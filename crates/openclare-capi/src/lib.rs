//! C interface of openclare.
//!
//! One call remeshes; what it returns is a handle to read the result from,
//! or the reason there is none, and to free when done:
//!
//! ```c
//! OpenclareResult *result = openclare_remesh(
//!     positions, vertex_count, triangles, triangle_count,
//!     NULL, 0,                    /* triangles hold vertex indices */
//!     NULL,                       /* no painted density */
//!     NULL, NULL, NULL, 0,        /* no guide curve */
//!     "organic",                  /* preset */
//!     "quads = 3000\n",           /* settings on top of it */
//!     NULL, NULL);                /* nobody follows the run */
//! if (openclare_result_ok(result)) {
//!     size_t vertices = openclare_result_vertex_count(result);
//!     ...
//!     openclare_result_copy_positions(result, out_positions);
//!     openclare_result_copy_faces(result, out_sizes, out_corners);
//! } else {
//!     puts(openclare_result_error(result));
//! }
//! openclare_result_free(result);
//! ```
//!
//! The mesh goes in as flat arrays the caller owns, read where they are: no
//! file, no serialisation and no copy on the caller's side. They are laid out
//! the way Blender keeps a mesh in memory (positions as three floats per
//! vertex, triangles as three corners each, and the vertex of each corner),
//! so that an add-on hands over the addresses of Blender's own arrays. The
//! library then builds its working mesh from them, in double precision. Settings are the text a saved
//! configuration is made of (`key = value` lines, see
//! `RemeshConfig::to_text`), so that a setting added to the library needs
//! no change to this interface.
//!
//! Nothing here unwinds into the caller: a panic inside the library comes
//! back as an error of the result.
//!
//! The C header, `include/openclare.h`, is written by hand and changes with
//! this file: a function, a type or a constant added, removed or changed here
//! is changed there too. The release workflow compares the two through
//! cbindgen, and compiles `tests/c/smoke.c` against the header on each
//! system (`make capi-test` here).

use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::OnceLock;
use std::time::Instant;

use openclare::math::Vec3;
use openclare::recorder::{Recorder, Shape};
use openclare::uv;
use openclare::{GuideCurve, Guides, HardEdgeFinder, ImposedPole, Pole, PolyMesh, RemeshConfig, Target, TriMesh};

/// Called when a stage of the run begins, with its name ("cross field", or
/// "piece 3 of 12: cross field") and the `user` pointer given to
/// [`openclare_remesh`]. Return 0 to go on, anything else to stop the run:
/// the result then holds the error "stopped on request".
pub type OpenclareOnStage = Option<unsafe extern "C" fn(stage: *const c_char, user: *mut c_void) -> i32>;

/// Passes the stages on to the host, and its wish to stop back to the chain.
struct Watcher {
    on_stage: OpenclareOnStage,
    user: *mut c_void,
}

impl Recorder for Watcher {
    fn is_listening(&self) -> bool {
        false
    }

    fn record(&mut self, _: &str, _: Shape<'_>) {}

    fn begin(&mut self, stage: &str) -> openclare::Result<()> {
        let Some(on_stage) = self.on_stage else { return Ok(()) };
        let name = text(stage.to_string());
        // SAFETY: the host gave this function and this pointer to be called
        // together; the name is a NUL-terminated string that outlives the call.
        match unsafe { on_stage(name.as_ptr(), self.user) } {
            0 => Ok(()),
            _ => Err(openclare::Error::Cancelled),
        }
    }
}

/// What a call to [`openclare_remesh`] gives back.
pub struct OpenclareResult {
    mesh: PolyMesh,
    /// Edges of the mesh along a hard edge of the input.
    hard_edges: Vec<[u32; 2]>,
    /// Edges to keep sharp under subdivision, and how sharp.
    creases: Vec<([u32; 2], f64)>,
    /// Edges of the mesh along a guide curve, each with the index of the
    /// curve among those given.
    curve_edges: Vec<([u32; 2], u32)>,
    /// The irregular vertices of the mesh away from its borders.
    poles: Vec<Pole>,
    /// The group of the input each face lies in; empty if it had none.
    face_groups: Vec<u32>,
    /// Texture coordinates, worked out the first time they are asked for.
    layers: OnceLock<uv::Layers>,
    /// Empty when all went well.
    error: CString,
    /// What happened, as `key = value` lines.
    report: CString,
}

impl OpenclareResult {
    fn failed(message: String) -> Self {
        Self {
            mesh: PolyMesh::default(),
            hard_edges: Vec::new(),
            creases: Vec::new(),
            curve_edges: Vec::new(),
            poles: Vec::new(),
            face_groups: Vec::new(),
            layers: OnceLock::new(),
            error: text(message),
            report: CString::default(),
        }
    }
}

/// A C string from Rust text; a NUL inside it, which nothing here writes,
/// would end it early rather than fail.
fn text(message: String) -> CString {
    let bytes: Vec<u8> = message.into_bytes().into_iter().take_while(|&byte| byte != 0).collect();
    CString::new(bytes).unwrap_or_default()
}

/// Version of the library, as `major.minor.patch`.
#[unsafe(no_mangle)]
pub extern "C" fn openclare_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}

/// Remeshes a triangle mesh into quads.
///
/// - `positions`: `3 * vertex_count` coordinates, x y z per vertex.
/// - `triangles`: `3 * triangle_count` indices: of vertices if
///   `corner_vertices` is null, of corners otherwise.
/// - `corner_vertices`: null, or the vertex of each of `corner_count`
///   corners, which is how Blender stores faces.
/// - `density`: null, or one weight per vertex from 0 (least dense) to 1
///   (most dense).
/// - `triangle_groups`: null, or a group for each of the `triangle_count`
///   triangles: its material, say. The quad mesh has a line of edges where
///   two groups meet, and [`openclare_result_copy_face_groups`] gives the
///   group of each of its faces.
/// - `edges`, `edge_count`: null and 0, or `edge_count` edges of the mesh,
///   two vertices each, for the quad mesh to have lines of edges along and
///   keep as hard edges, whatever their angle: the edges a modeller marked
///   sharp, say. They go in as sharp guide curves through their vertices,
///   joined into chains: hard edges of the result, marked sharp and creased.
/// - `curve_points`, `curve_sizes`, `curve_flags`, `curve_count`: guide
///   curves, lines drawn on the surface for the quad mesh to have edges
///   along. `curve_count` curves, one after the other in `curve_points`
///   (x y z per point), with as many points each as `curve_sizes` says, and
///   in `curve_flags` what each is: [`OPENCLARE_CURVE_CLOSED`] if its last
///   point joins the first, plus [`OPENCLARE_CURVE_SHARP`] if the edges
///   along it are to be hard edges of the result. All null and 0 for none.
/// - `pole_points`, `pole_valences`, `pole_count`: poles placed by the user,
///   x y z each, and the valence the vertex of the quad mesh there is to
///   have, 3 or 5. The others are placed freely, and
///   [`openclare_result_copy_poles`] tells which of the result answers
///   which. Both null and 0 for none.
/// - `preset`: null for the default one, or the name of a shipped preset
///   (`organic`, `animation`, `hard-surface`).
/// - `settings`: null, or `key = value` lines that adjust the preset.
/// - `on_stage`: null, or a function to call as each stage begins, with
///   `user`; it is called from the thread that called this one.
///
/// Always returns a result to free with [`openclare_result_free`]; ask it
/// whether it holds a mesh with [`openclare_result_ok`].
///
/// # Safety
///
/// The arrays must hold what their counts say (for the curves, three numbers
/// for each point `curve_sizes` adds up to; for the poles, three numbers
/// each in `pole_points`), and the two texts, when not
/// null, must end with a NUL. `on_stage`, when not null, must be safe to call
/// with `user`.
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_remesh(
    positions: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    triangle_count: usize,
    corner_vertices: *const u32,
    corner_count: usize,
    density: *const f32,
    triangle_groups: *const u32,
    edges: *const u32,
    edge_count: usize,
    curve_points: *const f32,
    curve_sizes: *const u32,
    curve_flags: *const u8,
    curve_count: usize,
    pole_points: *const f32,
    pole_valences: *const u8,
    pole_count: usize,
    preset: *const c_char,
    settings: *const c_char,
    on_stage: OpenclareOnStage,
    user: *mut c_void,
) -> *mut OpenclareResult {
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        let mut watcher = Watcher { on_stage, user };
        // SAFETY: the caller vouches for the pointers and their lengths.
        unsafe {
            let corners =
                (!corner_vertices.is_null()).then(|| std::slice::from_raw_parts(corner_vertices, corner_count));
            let curves = curves_of(curve_points, curve_sizes, curve_flags, curve_count);
            let edges = (!edges.is_null()).then(|| std::slice::from_raw_parts(edges, 2 * edge_count));
            let poles = poles_of(pole_points, pole_valences, pole_count)?;
            remesh(
                positions,
                vertex_count,
                triangles,
                triangle_count,
                corners,
                density,
                triangle_groups,
                edges,
                &curves,
                &poles,
                preset,
                settings,
                &mut watcher,
            )
        }
    }));
    let result = match outcome {
        Ok(Ok(result)) => result,
        Ok(Err(message)) => OpenclareResult::failed(message),
        Err(panic) => {
            let what = panic
                .downcast_ref::<&str>()
                .map(|message| message.to_string())
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown cause".into());
            OpenclareResult::failed(format!("openclare failed unexpectedly: {what}"))
        }
    };
    Box::into_raw(Box::new(result))
}

#[allow(clippy::too_many_arguments)]
unsafe fn remesh(
    positions: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    triangle_count: usize,
    corner_vertices: Option<&[u32]>,
    density: *const f32,
    triangle_groups: *const u32,
    edges: Option<&[u32]>,
    curves: &[GuideCurve],
    poles: &[ImposedPole],
    preset: *const c_char,
    settings: *const c_char,
    watcher: &mut Watcher,
) -> Result<OpenclareResult, String> {
    // SAFETY: as the caller vouches.
    let input = unsafe { input_of(positions, vertex_count, triangles, triangle_count, corner_vertices)? };
    let painted: Option<Vec<f64>> = (!density.is_null()).then(|| {
        // SAFETY: one weight per vertex, as documented.
        unsafe { std::slice::from_raw_parts(density, vertex_count) }.iter().map(|&weight| weight as f64).collect()
    });
    // SAFETY: null or NUL-terminated, as documented.
    let config = unsafe { config_of(preset, settings)? };

    let clock = Instant::now();
    // SAFETY: null, or one group per triangle as the caller says.
    let groups =
        (!triangle_groups.is_null()).then(|| unsafe { std::slice::from_raw_parts(triangle_groups, triangle_count) });
    // The edges given go in as sharp curves through their vertices: lines of
    // edges kept as hard edges, whatever their angle.
    let mut curves = curves.to_vec();
    for (chain, closed) in chains_of(edges.unwrap_or_default(), vertex_count)? {
        let points = chain.iter().map(|&v| input.positions[v as usize]).collect();
        curves.push(GuideCurve { points, closed, sharp: true });
    }
    let guides = Guides { density: painted.as_deref(), curves: &curves, groups, lines: &[], held: &[], poles };
    let (mesh, done) = config.remesh_recorded(&input, &guides, watcher).map_err(|error| error.to_string())?;
    let seconds = clock.elapsed().as_secs_f64();

    let asked = match config.target {
        Target::QuadCount(count) => format!("quads_asked = {count}"),
        Target::EdgeLength(length) => format!("edge_length_asked = {length}"),
        Target::SizeRange { largest, smallest } => format!("largest_asked = {largest}\nsmallest_asked = {smallest}"),
    };
    let stats = &done.stats;
    let report = format!(
        "{asked}\nvertices = {}\nfaces = {}\nquads = {}\nirregular_vertices = {}\npieces = {}\n\
         pieces_through_fallback = {}\npieces_failed = {}\npieces_skipped = {}\npieces_boxed = {}\nthin_pieces = {}\nquads_for_thin_pieces = {}\nguide_edges = {}\nsharp_curve_edges = {}\nfaces_across_groups = {}\nwelded_vertices = {}\nedges_taken_apart = {}\nguide_gaps = {}\n\
         guides_let_go = {}\nquads_wanted = {}\nbelow_smallest = {}\nlargest_edge = {}\nsmallest_edge = {}\noff_the_plane = {}\n\
         crossing_faces = {}\npoles = {}\npoles_imposed = {}\nseconds = {seconds:.3}\n",
        stats.vertices,
        stats.faces,
        stats.quads,
        stats.irregular_vertices,
        done.pieces,
        done.pieces_fell_back,
        done.pieces_failed,
        done.pieces_skipped,
        done.pieces_boxed,
        done.thin_pieces,
        done.quads_for_thin_pieces,
        done.guide_edges.len(),
        done.sharp_curve_edges.len(),
        done.faces_across_groups,
        done.welded_vertices,
        done.edges_taken_apart,
        done.guide_gaps,
        done.guides_let_go,
        done.quads_wanted,
        done.below_smallest,
        done.largest_edge,
        done.smallest_edge,
        done.off_the_plane,
        done.crossing_faces,
        done.poles.len(),
        done.poles_imposed,
    );
    // Said only of a result whose flat parts were asked for as n-gons.
    let report = match config.faces {
        openclare::config::Faces::Ngons => format!("{report}merged_faces = {}\n", done.merged_faces),
        openclare::config::Faces::Quads => report,
    };
    Ok(OpenclareResult {
        mesh,
        hard_edges: done.hard_edges,
        face_groups: done.face_groups,
        creases: done.creases,
        curve_edges: done.curve_edges,
        poles: done.poles,
        layers: OnceLock::new(),
        error: CString::default(),
        report: text(report),
    })
}

/// The triangle mesh a host gave as flat arrays: positions, and triangles of
/// vertices, or of corners when `corner_vertices` gives the vertex of each.
///
/// # Safety
///
/// The arrays must hold what their counts say, or be null with a count of 0.
unsafe fn input_of(
    positions: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    triangle_count: usize,
    corner_vertices: Option<&[u32]>,
) -> Result<TriMesh, String> {
    if (positions.is_null() && vertex_count > 0) || (triangles.is_null() && triangle_count > 0) {
        return Err("the mesh arrays are missing".into());
    }
    // SAFETY: not null, and as long as the caller says.
    let (coordinates, indices) = unsafe {
        (
            if vertex_count == 0 { &[][..] } else { std::slice::from_raw_parts(positions, 3 * vertex_count) },
            if triangle_count == 0 { &[][..] } else { std::slice::from_raw_parts(triangles, 3 * triangle_count) },
        )
    };
    // Through the corners, if the triangles are given by their corners.
    let vertex_of = |index: u32| -> Result<u32, String> {
        match corner_vertices {
            None => Ok(index),
            Some(corners) => corners
                .get(index as usize)
                .copied()
                .ok_or_else(|| format!("a triangle names corner {index}, of {} corners", corners.len())),
        }
    };
    let mut faces = Vec::with_capacity(triangle_count);
    for t in indices.chunks_exact(3) {
        let face = [vertex_of(t[0])?, vertex_of(t[1])?, vertex_of(t[2])?];
        if let Some(&beyond) = face.iter().find(|&&v| v as usize >= vertex_count) {
            return Err(format!("a triangle names vertex {beyond}, of {vertex_count} vertices"));
        }
        faces.push(face);
    }
    Ok(TriMesh {
        positions: coordinates.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect(),
        triangles: faces,
    })
}

/// The configuration a host asked for: the preset of that name (the default
/// one for null or ""), adjusted by the `key = value` lines of `settings`.
///
/// # Safety
///
/// Both texts must be null or end with a NUL.
unsafe fn config_of(preset: *const c_char, settings: *const c_char) -> Result<RemeshConfig, String> {
    let read = |pointer: *const c_char, what: &str| -> Result<Option<String>, String> {
        if pointer.is_null() {
            return Ok(None);
        }
        // SAFETY: NUL-terminated, as documented.
        let text = unsafe { CStr::from_ptr(pointer) };
        text.to_str().map(|text| Some(text.to_string())).map_err(|_| format!("the {what} is not valid UTF-8"))
    };
    let mut config = match read(preset, "preset name")? {
        Some(name) if !name.is_empty() => {
            RemeshConfig::preset(&name, RemeshConfig::default().target).map_err(|error| error.to_string())?
        }
        _ => RemeshConfig::default(),
    };
    if let Some(lines) = read(settings, "settings")? {
        config.apply_text(&lines).map_err(|error| error.to_string())?;
    }
    Ok(config)
}

/// In the flags of a guide curve: its last point joins the first.
pub const OPENCLARE_CURVE_CLOSED: u8 = 1;
/// In the flags of a guide curve: the edges along it are hard edges of the
/// result, marked sharp and creased, and not only a line of edges.
pub const OPENCLARE_CURVE_SHARP: u8 = 2;

/// The poles a host gave as flat arrays: valences other than 3 and 5 are
/// refused.
///
/// # Safety
///
/// `points` must hold three numbers per pole and `valences` one, or both
/// be null.
unsafe fn poles_of(points: *const f32, valences: *const u8, count: usize) -> Result<Vec<ImposedPole>, String> {
    if points.is_null() || valences.is_null() || count == 0 {
        return Ok(Vec::new());
    }
    // SAFETY: as the caller vouches.
    let (points, valences) =
        unsafe { (std::slice::from_raw_parts(points, 3 * count), std::slice::from_raw_parts(valences, count)) };
    points
        .chunks_exact(3)
        .zip(valences)
        .map(|(p, &valence)| match valence {
            3 | 5 => Ok(ImposedPole { position: Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64), valence }),
            _ => Err(format!("a pole of valence {valence}: only 3 and 5 can be placed")),
        })
        .collect()
}

/// The guide curves a host gave as flat arrays.
///
/// # Safety
///
/// The arrays must hold what `count` and the sizes say, or be null.
unsafe fn curves_of(points: *const f32, sizes: *const u32, flags: *const u8, count: usize) -> Vec<GuideCurve> {
    if points.is_null() || sizes.is_null() || count == 0 {
        return Vec::new();
    }
    // SAFETY: `count` sizes, and three numbers for each point they add up to.
    let sizes = unsafe { std::slice::from_raw_parts(sizes, count) };
    let total: usize = sizes.iter().map(|&size| size as usize).sum();
    let coordinates = unsafe { std::slice::from_raw_parts(points, 3 * total) };
    let flags = (!flags.is_null()).then(|| unsafe { std::slice::from_raw_parts(flags, count) });
    let mut next = 0;
    sizes
        .iter()
        .enumerate()
        .map(|(index, &size)| {
            let slice = &coordinates[3 * next..3 * (next + size as usize)];
            next += size as usize;
            let flags = flags.map_or(0, |flags| flags[index]);
            GuideCurve {
                points: slice.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect(),
                closed: flags & OPENCLARE_CURVE_CLOSED != 0,
                sharp: flags & OPENCLARE_CURVE_SHARP != 0,
            }
        })
        .collect()
}

/// The edges given as pairs of vertices, joined into chains: each runs from
/// an end, or a vertex where more than two meet, to the next such vertex;
/// what is left are closed loops. With whether each closes on itself.
fn chains_of(edges: &[u32], vertex_count: usize) -> Result<Vec<(Vec<u32>, bool)>, String> {
    let mut next: HashMap<u32, Vec<u32>> = HashMap::new();
    for pair in edges.chunks_exact(2) {
        let [a, b] = [pair[0], pair[1]];
        if a as usize >= vertex_count || b as usize >= vertex_count || a == b {
            return Err(format!("edge ({a}, {b}) is not an edge of a mesh of {vertex_count} vertices"));
        }
        next.entry(a).or_default().push(b);
        next.entry(b).or_default().push(a);
    }
    let mut used: HashSet<(u32, u32)> = HashSet::new();
    let walk = |start: u32, first: u32, used: &mut HashSet<(u32, u32)>| {
        let mut chain = vec![start];
        let (mut previous, mut at) = (start, first);
        used.insert((start.min(first), start.max(first)));
        loop {
            chain.push(at);
            let onward = next[&at].iter().copied().find(|&v| v != previous && !used.contains(&(at.min(v), at.max(v))));
            match onward {
                Some(v) if next[&at].len() == 2 && at != start => {
                    used.insert((at.min(v), at.max(v)));
                    (previous, at) = (at, v);
                }
                _ => break,
            }
        }
        let closed = chain.len() > 2 && chain.last() == Some(&start);
        if closed {
            chain.pop();
        }
        (chain, closed)
    };
    // In a fixed order, so that the curves do not depend on hashing.
    let mut vertices: Vec<u32> = next.keys().copied().collect();
    vertices.sort_unstable();
    let mut chains = Vec::new();
    // From the ends and the forks first, then round what is left: loops.
    for pass in [false, true] {
        for &v in &vertices {
            if (next[&v].len() == 2) != pass {
                continue;
            }
            let mut around = next[&v].clone();
            around.sort_unstable();
            for w in around {
                if !used.contains(&(v.min(w), v.max(w))) {
                    chains.push(walk(v, w, &mut used));
                }
            }
        }
    }
    Ok(chains)
}

/// The result behind a handle; `None` for a null one.
unsafe fn held<'a>(result: *const OpenclareResult) -> Option<&'a OpenclareResult> {
    // SAFETY: null, or a handle from `openclare_remesh` not yet freed.
    unsafe { result.as_ref() }
}

/// 1 if the result holds a mesh, 0 if it holds an error (or is null).
///
/// # Safety
///
/// `result` must be null or a handle from [`openclare_remesh`] not yet freed;
/// the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_ok(result: *const OpenclareResult) -> i32 {
    unsafe { held(result) }.is_some_and(|result| result.error.as_bytes().is_empty()) as i32
}

/// Why there is no mesh; empty if there is one. Valid until the result is freed.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_error(result: *const OpenclareResult) -> *const c_char {
    match unsafe { held(result) } {
        Some(result) => result.error.as_ptr(),
        None => c"no result".as_ptr(),
    }
}

/// What happened, as `key = value` lines: quads made, pieces, seconds.
/// Valid until the result is freed.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_report(result: *const OpenclareResult) -> *const c_char {
    match unsafe { held(result) } {
        Some(result) => result.report.as_ptr(),
        None => c"".as_ptr(),
    }
}

/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_vertex_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.mesh.positions.len())
}

/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_face_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.mesh.faces.len())
}

/// Corners of all the faces together: four per quad.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_corner_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.mesh.faces.iter().map(Vec::len).sum())
}

/// Writes the coordinates of the vertices, x y z each, into `positions`.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `positions` must have room for three times
/// [`openclare_result_vertex_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_positions(result: *const OpenclareResult, positions: *mut f32) {
    let Some(result) = (unsafe { held(result) }) else { return };
    if positions.is_null() {
        return;
    }
    // SAFETY: room for three numbers per vertex, as documented.
    let out = unsafe { std::slice::from_raw_parts_mut(positions, 3 * result.mesh.positions.len()) };
    for (slot, point) in out.chunks_exact_mut(3).zip(&result.mesh.positions) {
        slot.copy_from_slice(&[point.x as f32, point.y as f32, point.z as f32]);
    }
}

/// Writes the faces: into `sizes`, how many corners each has; into
/// `corners`, the vertex of each corner, face after face.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `sizes` must have room for
/// [`openclare_result_face_count`] numbers and `corners` for
/// [`openclare_result_corner_count`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_faces(
    result: *const OpenclareResult,
    sizes: *mut u32,
    corners: *mut u32,
) {
    let Some(result) = (unsafe { held(result) }) else { return };
    if sizes.is_null() || corners.is_null() {
        return;
    }
    let total: usize = result.mesh.faces.iter().map(Vec::len).sum();
    // SAFETY: room as documented.
    let (sizes, corners) = unsafe {
        (std::slice::from_raw_parts_mut(sizes, result.mesh.faces.len()), std::slice::from_raw_parts_mut(corners, total))
    };
    let mut next = 0;
    for (size, face) in sizes.iter_mut().zip(&result.mesh.faces) {
        *size = face.len() as u32;
        corners[next..next + face.len()].copy_from_slice(face);
        next += face.len();
    }
}

/// Which layer of texture coordinates [`openclare_result_copy_uv`] writes.
/// The grid: every quad a unit square, coordinates in whole numbers of
/// quads, parts of the surface on top of each other. For patterns that
/// repeat along the edge flow.
pub const OPENCLARE_UV_GRID: u32 = 0;
/// The atlas: islands that cover nothing twice, packed into the unit square.
/// For painting; its cuts are given by [`openclare_result_copy_seams`].
pub const OPENCLARE_UV_ATLAS: u32 = 1;

/// Writes a layer of texture coordinates, u v for each corner of each face,
/// in the order of [`openclare_result_copy_faces`], into `coordinates`.
/// Returns 1 if it did, 0 if `layer` is not one of the two.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `coordinates` must have room for twice
/// [`openclare_result_corner_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_uv(
    result: *const OpenclareResult,
    layer: u32,
    coordinates: *mut f32,
) -> i32 {
    let Some(result) = (unsafe { held(result) }) else { return 0 };
    if coordinates.is_null() {
        return 0;
    }
    let layers = result.layers.get_or_init(|| uv::unfold(&result.mesh));
    let layer = match layer {
        OPENCLARE_UV_GRID => &layers.grid,
        OPENCLARE_UV_ATLAS => &layers.atlas,
        _ => return 0,
    };
    // SAFETY: room for two numbers per corner, as documented.
    let out = unsafe { std::slice::from_raw_parts_mut(coordinates, 2 * layer.len()) };
    for (slot, uv) in out.chunks_exact_mut(2).zip(layer) {
        slot.copy_from_slice(&[uv[0] as f32, uv[1] as f32]);
    }
    1
}

/// How many edges the atlas is cut along.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_seam_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.layers.get_or_init(|| uv::unfold(&result.mesh)).seams.len())
}

/// Writes the seams of the atlas, two vertices per edge, into `vertices`.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `vertices` must have room for twice
/// [`openclare_result_seam_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_seams(result: *const OpenclareResult, vertices: *mut u32) {
    let Some(result) = (unsafe { held(result) }) else { return };
    let seams = &result.layers.get_or_init(|| uv::unfold(&result.mesh)).seams;
    // SAFETY: room as documented.
    unsafe { copy_edges(seams, vertices) }
}

/// How many edges of the mesh run along a hard edge of the input.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_hard_edge_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.hard_edges.len())
}

/// Writes the edges along a hard edge of the input, two vertices per edge,
/// into `vertices`.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `vertices` must have room for twice
/// [`openclare_result_hard_edge_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_hard_edges(result: *const OpenclareResult, vertices: *mut u32) {
    let Some(result) = (unsafe { held(result) }) else { return };
    // SAFETY: room as documented.
    unsafe { copy_edges(&result.hard_edges, vertices) }
}

/// How many edges of the mesh run along a guide curve.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_curve_edge_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.curve_edges.len())
}

/// Writes the edges along a guide curve, two vertices per edge, into
/// `vertices`, and the index of the curve each runs along, among the curves
/// given to [`openclare_remesh`], into `curves`: what a host marks each edge
/// with, as its curve says. Either may be null.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `vertices` must have room for twice
/// [`openclare_result_curve_edge_count`] numbers, `curves` for as many.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_curve_edges(
    result: *const OpenclareResult,
    vertices: *mut u32,
    curves: *mut u32,
) {
    let Some(result) = (unsafe { held(result) }) else { return };
    let edges: Vec<[u32; 2]> = result.curve_edges.iter().map(|&(edge, _)| edge).collect();
    // SAFETY: room as documented.
    unsafe { copy_edges(&edges, vertices) }
    if !curves.is_null() {
        // SAFETY: room for one number per edge, as documented.
        let out = unsafe { std::slice::from_raw_parts_mut(curves, result.curve_edges.len()) };
        for (slot, &(_, curve)) in out.iter_mut().zip(&result.curve_edges) {
            *slot = curve;
        }
    }
}

/// How many irregular vertices the mesh has away from its borders.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_pole_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.poles.len())
}

/// Writes the irregular vertices of the mesh away from its borders: where
/// each lies on the input, as a triangle of it into `triangles`, three
/// barycentric coordinates into `barycentric` and the point, x y z, into
/// `points`; its valence into `valences`; and into `imposed` the index of
/// the pole given to [`openclare_remesh`] it answers, or -1 for one placed
/// freely. Any may be null.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `barycentric` and `points` must have room
/// for three times [`openclare_result_pole_count`] numbers, the others for
/// as many.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_poles(
    result: *const OpenclareResult,
    triangles: *mut u32,
    barycentric: *mut f32,
    points: *mut f32,
    valences: *mut u8,
    imposed: *mut i32,
) {
    let Some(result) = (unsafe { held(result) }) else { return };
    let count = result.poles.len();
    // SAFETY: each, when not null, has the room documented.
    unsafe {
        if !triangles.is_null() {
            let out = std::slice::from_raw_parts_mut(triangles, count);
            for (slot, pole) in out.iter_mut().zip(&result.poles) {
                *slot = pole.triangle;
            }
        }
        if !barycentric.is_null() {
            let out = std::slice::from_raw_parts_mut(barycentric, 3 * count);
            for (slot, pole) in out.chunks_exact_mut(3).zip(&result.poles) {
                for (value, &weight) in slot.iter_mut().zip(&pole.barycentric) {
                    *value = weight as f32;
                }
            }
        }
        if !points.is_null() {
            let out = std::slice::from_raw_parts_mut(points, 3 * count);
            for (slot, pole) in out.chunks_exact_mut(3).zip(&result.poles) {
                slot.copy_from_slice(&[pole.position.x as f32, pole.position.y as f32, pole.position.z as f32]);
            }
        }
        if !valences.is_null() {
            let out = std::slice::from_raw_parts_mut(valences, count);
            for (slot, pole) in out.iter_mut().zip(&result.poles) {
                *slot = pole.valence;
            }
        }
        if !imposed.is_null() {
            let out = std::slice::from_raw_parts_mut(imposed, count);
            for (slot, pole) in out.iter_mut().zip(&result.poles) {
                *slot = pole.imposed.map_or(-1, |k| k as i32);
            }
        }
    }
}

/// Writes the group of the input each face of the mesh lies in, one number
/// per face, into `groups`. Returns 0, and writes nothing, if the input came
/// without groups.
///
/// # Safety
///
/// See [`openclare_result_ok`]; `groups` must have room for
/// [`openclare_result_face_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_face_groups(result: *const OpenclareResult, groups: *mut u32) -> i32 {
    let Some(result) = (unsafe { held(result) }) else { return 0 };
    if result.face_groups.is_empty() || groups.is_null() {
        return 0;
    }
    // SAFETY: room as documented.
    unsafe { std::slice::from_raw_parts_mut(groups, result.face_groups.len()) }.copy_from_slice(&result.face_groups);
    1
}

/// How many edges of the mesh carry a crease: those to keep sharp, wholly or
/// in part, when the mesh is subdivided.
///
/// # Safety
///
/// See [`openclare_result_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_crease_count(result: *const OpenclareResult) -> usize {
    unsafe { held(result) }.map_or(0, |result| result.creases.len())
}

/// Writes the creased edges, two vertices per edge, into `vertices`, and
/// their weights, from 0 (smooth) to 1 (sharp), into `weights`.
///
/// # Safety
///
/// See [`openclare_result_ok`]; with [`openclare_result_crease_count`] as
/// the number of edges, `vertices` must have room for twice that many
/// numbers and `weights` for that many.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_copy_creases(
    result: *const OpenclareResult,
    vertices: *mut u32,
    weights: *mut f32,
) {
    let Some(result) = (unsafe { held(result) }) else { return };
    if vertices.is_null() || weights.is_null() {
        return;
    }
    let edges: Vec<[u32; 2]> = result.creases.iter().map(|&(edge, _)| edge).collect();
    // SAFETY: room as documented.
    unsafe {
        copy_edges(&edges, vertices);
        let out = std::slice::from_raw_parts_mut(weights, result.creases.len());
        for (slot, &(_, weight)) in out.iter_mut().zip(&result.creases) {
            *slot = weight as f32;
        }
    }
}

/// Writes edges as pairs of vertices; nothing if `vertices` is null.
unsafe fn copy_edges(edges: &[[u32; 2]], vertices: *mut u32) {
    if vertices.is_null() {
        return;
    }
    // SAFETY: the caller has room for two numbers per edge.
    let out = unsafe { std::slice::from_raw_parts_mut(vertices, 2 * edges.len()) };
    for (slot, edge) in out.chunks_exact_mut(2).zip(edges) {
        slot.copy_from_slice(edge);
    }
}

/// Frees a result. Null is fine.
///
/// # Safety
///
/// `result` must be null or a handle from [`openclare_remesh`] not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_result_free(result: *mut OpenclareResult) {
    if !result.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_remesh`.
        drop(unsafe { Box::from_raw(result) });
    }
}

/// What a call to [`openclare_bake_height`] gives back.
pub struct OpenclareBaked {
    heights: Vec<f32>,
    /// Empty when all went well.
    error: CString,
    /// What happened, as `key = value` lines.
    report: CString,
}

/// Bakes the height of a detailed surface above a plainer one into a
/// texture: for each texel, how far the source is from the target along the
/// normal of the target, signed, in the unit of the meshes.
///
/// - `positions`, `normals`: `3 * vertex_count` numbers each, the target.
/// - `triangles`: `3 * triangle_count` indices of its vertices.
/// - `uvs`: `6 * triangle_count` numbers, u and v for each corner of each
///   triangle, (0, 0) at the bottom left of the texture.
/// - `source_positions`, `source_triangles`: the detailed surface.
/// - `width`, `height`: the size of the texture in texels.
/// - `reach`: how far from the target the source is looked for.
/// - `padding`: texels filled round the faces from their neighbours.
///
/// Returns a handle to free with [`openclare_baked_free`], never null.
///
/// # Safety
///
/// Each pointer must be valid for the number of elements said of it, for
/// the time of the call.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn openclare_bake_height(
    positions: *const f32,
    normals: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    uvs: *const f32,
    triangle_count: usize,
    source_positions: *const f32,
    source_vertex_count: usize,
    source_triangles: *const u32,
    source_triangle_count: usize,
    width: u32,
    height: u32,
    reach: f32,
    padding: u32,
) -> *mut OpenclareBaked {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<OpenclareBaked, String> {
        let missing = positions.is_null() || normals.is_null() || triangles.is_null() || uvs.is_null();
        if missing || source_positions.is_null() || source_triangles.is_null() {
            return Err("the mesh arrays are missing".into());
        }
        let points = |at: *const f32, count: usize| -> Vec<Vec3> {
            // SAFETY: not null, and as long as the caller says.
            let numbers = unsafe { std::slice::from_raw_parts(at, 3 * count) };
            numbers.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect()
        };
        let faces = |at: *const u32, count: usize, vertices: usize| -> Result<Vec<[u32; 3]>, String> {
            // SAFETY: not null, and as long as the caller says.
            let indices = unsafe { std::slice::from_raw_parts(at, 3 * count) };
            match indices.iter().find(|&&v| v as usize >= vertices) {
                Some(beyond) => Err(format!("a triangle names vertex {beyond}, of {vertices} vertices")),
                None => Ok(indices.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect()),
            }
        };
        let (target_positions, target_normals) = (points(positions, vertex_count), points(normals, vertex_count));
        let target_triangles = faces(triangles, triangle_count, vertex_count)?;
        // SAFETY: not null, and as long as the caller says.
        let coordinates = unsafe { std::slice::from_raw_parts(uvs, 6 * triangle_count) };
        let target_uvs: Vec<[[f64; 2]; 3]> = coordinates
            .chunks_exact(6)
            .map(|c| [[c[0] as f64, c[1] as f64], [c[2] as f64, c[3] as f64], [c[4] as f64, c[5] as f64]])
            .collect();
        let source = TriMesh {
            positions: points(source_positions, source_vertex_count),
            triangles: faces(source_triangles, source_triangle_count, source_vertex_count)?,
        };
        let target = openclare::bake::Target {
            positions: &target_positions,
            normals: &target_normals,
            triangles: &target_triangles,
            uvs: &target_uvs,
        };
        let clock = Instant::now();
        let baked = openclare::bake::height_map(
            &target,
            &source,
            width as usize,
            height as usize,
            reach as f64,
            padding as usize,
        )
        .map_err(|error| error.to_string())?;
        let report = format!(
            "width = {}\nheight = {}\ncovered = {}\nastray = {}\nlow = {}\nhigh = {}\nseconds = {:.3}\n",
            baked.width,
            baked.height,
            baked.covered,
            baked.astray,
            baked.low,
            baked.high,
            clock.elapsed().as_secs_f64()
        );
        Ok(OpenclareBaked { heights: baked.heights, error: CString::default(), report: text(report) })
    }));
    let failed =
        |message: String| OpenclareBaked { heights: Vec::new(), error: text(message), report: CString::default() };
    let baked = match outcome {
        Ok(Ok(baked)) => baked,
        Ok(Err(message)) => failed(message),
        Err(_) => failed("openclare failed unexpectedly while baking".into()),
    };
    Box::into_raw(Box::new(baked))
}

/// 1 if there is a texture, 0 if not or if `baked` is null.
///
/// # Safety
///
/// `baked` must be null or a handle from [`openclare_bake_height`] not yet
/// freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_baked_ok(baked: *const OpenclareBaked) -> i32 {
    // SAFETY: null or a live handle, by contract.
    unsafe { baked.as_ref() }.is_some_and(|baked| baked.error.as_bytes().is_empty()) as i32
}

/// Why there is no texture; empty if there is one. Valid until freed.
///
/// # Safety
///
/// See [`openclare_baked_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_baked_error(baked: *const OpenclareBaked) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { baked.as_ref() } {
        Some(baked) => baked.error.as_ptr(),
        None => c"no result".as_ptr(),
    }
}

/// What happened, as `key = value` lines: texels covered, texels where the
/// source was not found along the normal, least and greatest height,
/// seconds. Valid until freed.
///
/// # Safety
///
/// See [`openclare_baked_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_baked_report(baked: *const OpenclareBaked) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { baked.as_ref() } {
        Some(baked) => baked.report.as_ptr(),
        None => c"".as_ptr(),
    }
}

/// Copies the heights, `width * height` numbers, row after row from the
/// bottom: not a number where no face of the target lies.
///
/// # Safety
///
/// See [`openclare_baked_ok`]; `heights` must have room for them.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_baked_copy(baked: *const OpenclareBaked, heights: *mut f32) {
    // SAFETY: null or a live handle, by contract.
    let Some(baked) = (unsafe { baked.as_ref() }) else { return };
    if heights.is_null() {
        return;
    }
    // SAFETY: room for as many as the texture has, by contract.
    unsafe { std::slice::from_raw_parts_mut(heights, baked.heights.len()) }.copy_from_slice(&baked.heights);
}

/// # Safety
///
/// `baked` must be null or a handle from [`openclare_bake_height`] not yet
/// freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_baked_free(baked: *mut OpenclareBaked) {
    if !baked.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_bake_height`.
        drop(unsafe { Box::from_raw(baked) });
    }
}

/// What a call to [`openclare_project_level`] gives back.
pub struct OpenclareLevel {
    positions: Vec<f32>,
    status: Vec<u8>,
    /// Per cage face, when the cage faces were given and the level measured.
    missing: Vec<f32>,
    verdict: Vec<u8>,
    /// Empty when all went well.
    error: CString,
    /// What happened, as `key = value` lines.
    report: CString,
}

/// Places a subdivision level on its source: each vertex onto the side of
/// the source that faces its way, no farther than its reach, a vertex
/// inside a creased line onto the source's hard edges, faces that fold
/// taken back (the guarded projection of the Multires, PRO #6).
///
/// - `positions`: `3 * vertex_count` numbers, the level as the subdivision
///   left it.
/// - `quads`: `4 * quad_count` indices of its vertices.
/// - `crease_edges`: `2 * crease_count` indices, the creased edges of the
///   level, and `crease_weights` their `crease_count` weights; both may be
///   null when `crease_count` is 0.
/// - `cage_face`: `quad_count` indices, for each face of the level the face
///   of the cage it was subdivided from, of `cage_face_count`; or null.
///   Given, the missing detail is also measured over each cage face, and
///   each told what it calls for ([`openclare_level_copy_missing`],
///   [`openclare_level_copy_verdict`]).
/// - `previous_missing`: `cage_face_count` numbers, what
///   [`openclare_level_copy_missing`] gave for the level below; or null, for
///   the first level.
/// - `source_positions`, `source_triangles`: the source.
/// - `settings`: `key = value` lines (reach, far, stretch, tilt,
///   crease_angle, samples, seed, target, stall, stuck), or null for the
///   defaults.
///
/// Returns a handle to free with [`openclare_level_free`], never null.
///
/// # Safety
///
/// Each pointer must be valid for the number of elements said of it, and
/// `settings` null or a text ending in a nul, for the time of the call.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn openclare_project_level(
    positions: *const f32,
    vertex_count: usize,
    quads: *const u32,
    quad_count: usize,
    crease_edges: *const u32,
    crease_weights: *const f32,
    crease_count: usize,
    cage_face: *const u32,
    cage_face_count: usize,
    previous_missing: *const f32,
    source_positions: *const f32,
    source_vertex_count: usize,
    source_triangles: *const u32,
    source_triangle_count: usize,
    settings: *const c_char,
) -> *mut OpenclareLevel {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<OpenclareLevel, String> {
        if positions.is_null() || quads.is_null() || source_positions.is_null() || source_triangles.is_null() {
            return Err("the mesh arrays are missing".into());
        }
        if crease_count > 0 && (crease_edges.is_null() || crease_weights.is_null()) {
            return Err("the crease arrays are missing".into());
        }
        let points = |at: *const f32, count: usize| -> Vec<Vec3> {
            // SAFETY: not null, and as long as the caller says.
            let numbers = unsafe { read(at, 3 * count) };
            numbers.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect()
        };
        // SAFETY: not null, and as long as the caller says.
        let corners = unsafe { read(quads, 4 * quad_count) };
        let level = PolyMesh {
            positions: points(positions, vertex_count),
            faces: corners.chunks_exact(4).map(<[u32]>::to_vec).collect(),
        };
        // SAFETY: not null when there are any, and as long as the caller says.
        let (ends, weights) = unsafe { (read(crease_edges, 2 * crease_count), read(crease_weights, crease_count)) };
        let creases: Vec<([u32; 2], f64)> =
            ends.chunks_exact(2).zip(weights).map(|(e, &w)| ([e[0], e[1]], w as f64)).collect();
        // SAFETY: not null, and as long as the caller says.
        let indices = unsafe { read(source_triangles, 3 * source_triangle_count) };
        if let Some(beyond) = indices.iter().find(|&&v| v as usize >= source_vertex_count) {
            return Err(format!("a triangle of the source names vertex {beyond}, of {source_vertex_count} vertices"));
        }
        let source = TriMesh {
            positions: points(source_positions, source_vertex_count),
            triangles: indices.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect(),
        };
        let settings: openclare::level::Settings = if settings.is_null() {
            Default::default()
        } else {
            // SAFETY: a text ending in a nul, by contract.
            let text = unsafe { CStr::from_ptr(settings) }.to_str().map_err(|_| "the settings are not UTF-8")?;
            text.parse().map_err(|e: openclare::Error| e.to_string())?
        };
        // SAFETY: null, or as long as the caller says.
        let face_of = if cage_face.is_null() { None } else { Some(unsafe { read(cage_face, quad_count) }) };
        let previous: Option<Vec<f64>> = (!previous_missing.is_null())
            // SAFETY: not null, and as long as the caller says.
            .then(|| unsafe { read(previous_missing, cage_face_count) }.iter().map(|&m| m as f64).collect());
        if previous.is_some() && face_of.is_none() {
            return Err("the level below is measured over cage faces, but none are named".into());
        }
        let cage = face_of.as_ref().map(|face_of| openclare::level::Cage {
            face_of,
            faces: cage_face_count,
            previous: previous.as_deref(),
        });
        let placed = openclare::level::place_on_cage(&level, &creases, &source, &settings, cage.as_ref())
            .map_err(|e| e.to_string())?;
        let faces = placed.faces.as_ref();
        Ok(OpenclareLevel {
            positions: placed.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect(),
            status: placed.status.iter().map(|&s| s as u8).collect(),
            missing: faces.map_or_else(Vec::new, |f| f.missing.iter().map(|&m| m as f32).collect()),
            verdict: faces.map_or_else(Vec::new, |f| f.verdict.iter().map(|&v| v as u8).collect()),
            error: CString::default(),
            report: text(placed.report()),
        })
    }));
    let failed = |message: String| OpenclareLevel {
        positions: Vec::new(),
        status: Vec::new(),
        missing: Vec::new(),
        verdict: Vec::new(),
        error: text(message),
        report: CString::default(),
    };
    let level = match outcome {
        Ok(Ok(level)) => level,
        Ok(Err(message)) => failed(message),
        Err(_) => failed("openclare failed unexpectedly while placing the level".into()),
    };
    Box::into_raw(Box::new(level))
}

/// 1 if the level was placed, 0 if not or if `level` is null.
///
/// # Safety
///
/// `level` must be null or a handle from [`openclare_project_level`] not
/// yet freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_ok(level: *const OpenclareLevel) -> i32 {
    // SAFETY: null or a live handle, by contract.
    unsafe { level.as_ref() }.is_some_and(|level| level.error.as_bytes().is_empty()) as i32
}

/// Why the level was not placed; empty if it was. Valid until freed.
///
/// # Safety
///
/// See [`openclare_level_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_error(level: *const OpenclareLevel) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { level.as_ref() } {
        Some(level) => level.error.as_ptr(),
        None => c"no result".as_ptr(),
    }
}

/// What happened, as `key = value` lines: the count of vertices in each
/// status, the missing detail and stretched edges of the level once placed
/// (unless `samples = 0`), seconds. Valid until freed.
///
/// # Safety
///
/// See [`openclare_level_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_report(level: *const OpenclareLevel) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { level.as_ref() } {
        Some(level) => level.report.as_ptr(),
        None => c"".as_ptr(),
    }
}

/// Copies where the vertices now are, 3 numbers each, in the order given.
///
/// # Safety
///
/// See [`openclare_level_ok`]; `positions` must have room for them.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_copy_positions(level: *const OpenclareLevel, positions: *mut f32) {
    // SAFETY: null or a live handle, by contract.
    let Some(level) = (unsafe { level.as_ref() }) else { return };
    if !positions.is_null() {
        // SAFETY: room for them, by contract.
        unsafe { std::slice::from_raw_parts_mut(positions, level.positions.len()) }.copy_from_slice(&level.positions);
    }
}

/// Copies what became of each vertex, one number each: 0 moved, 1 onto a
/// hard edge, 2 stopped by the reach, 3 nothing found, 4 taken back, 5 held.
///
/// # Safety
///
/// See [`openclare_level_ok`]; `status` must have room for them.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_copy_status(level: *const OpenclareLevel, status: *mut u8) {
    // SAFETY: null or a live handle, by contract.
    let Some(level) = (unsafe { level.as_ref() }) else { return };
    if !status.is_null() {
        // SAFETY: room for them, by contract.
        unsafe { std::slice::from_raw_parts_mut(status, level.status.len()) }.copy_from_slice(&level.status);
    }
}

/// Copies what the level misses of the source over each cage face, one
/// number each, in percent of the diagonal of the source's bounding box:
/// the 99th percentile of the distances from the points spread on the
/// source closest to that face's part of the level, 0 where no point is.
/// Copies nothing when no cage faces were given or nothing was measured
/// (`samples = 0`).
///
/// # Safety
///
/// See [`openclare_level_ok`]; `missing` must have room for
/// `cage_face_count` numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_copy_missing(level: *const OpenclareLevel, missing: *mut f32) {
    // SAFETY: null or a live handle, by contract.
    let Some(level) = (unsafe { level.as_ref() }) else { return };
    if !missing.is_null() && !level.missing.is_empty() {
        // SAFETY: room for them, by contract.
        unsafe { std::slice::from_raw_parts_mut(missing, level.missing.len()) }.copy_from_slice(&level.missing);
    }
}

/// Copies what each cage face calls for, a byte each: 0 its part of the
/// level misses less than the target; 1 above it and still gaining, another
/// level helps; 2 above it and no longer gaining, or its vertices found
/// little of the source: more quads round it in the cage. Copies nothing
/// when [`openclare_level_copy_missing`] does not.
///
/// # Safety
///
/// See [`openclare_level_ok`]; `verdict` must have room for
/// `cage_face_count` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_copy_verdict(level: *const OpenclareLevel, verdict: *mut u8) {
    // SAFETY: null or a live handle, by contract.
    let Some(level) = (unsafe { level.as_ref() }) else { return };
    if !verdict.is_null() && !level.verdict.is_empty() {
        // SAFETY: room for them, by contract.
        unsafe { std::slice::from_raw_parts_mut(verdict, level.verdict.len()) }.copy_from_slice(&level.verdict);
    }
}

/// Frees a handle from [`openclare_project_level`].
///
/// # Safety
///
/// `level` must be null or a handle from [`openclare_project_level`] not
/// yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_level_free(level: *mut OpenclareLevel) {
    if !level.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_project_level`.
        drop(unsafe { Box::from_raw(level) });
    }
}

/// What a call to [`openclare_analyse`] gives back: a shape measured once,
/// to be asked as often as wanted how many quads a largest and a smallest
/// quad make of it.
pub struct OpenclareAnalysis {
    analysis: Option<openclare::Analysis>,
    vertices: usize,
    /// Empty when all went well.
    error: CString,
}

/// Measures what a shape calls for, without meshing it.
///
/// - `positions`: `3 * vertex_count` numbers; `triangles`: `3 *
///   triangle_count` indices of vertices.
/// - `preset`, `settings`: as for [`openclare_remesh`], null for none. The
///   adaptivity, the quads round a thin part and the contrast of the paint
///   count; the target does not.
/// - `density`: null, or one weight per vertex, from 0 to 1.
///
/// Returns a handle to free with [`openclare_analysis_free`], never null.
///
/// # Safety
///
/// Each pointer must be null where allowed, or valid for the number of
/// elements said of it, for the time of the call; text NUL-terminated.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analyse(
    positions: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    triangle_count: usize,
    preset: *const c_char,
    settings: *const c_char,
    density: *const f32,
) -> *mut OpenclareAnalysis {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<openclare::Analysis, String> {
        if positions.is_null() || triangles.is_null() {
            return Err("the mesh arrays are missing".into());
        }
        // SAFETY: not null, and as long as the caller says.
        let coordinates = unsafe { std::slice::from_raw_parts(positions, 3 * vertex_count) };
        // SAFETY: likewise.
        let indices = unsafe { std::slice::from_raw_parts(triangles, 3 * triangle_count) };
        if let Some(&beyond) = indices.iter().find(|&&v| v as usize >= vertex_count) {
            return Err(format!("a triangle names vertex {beyond}, of {vertex_count} vertices"));
        }
        let input = TriMesh {
            positions: coordinates.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect(),
            triangles: indices.chunks_exact(3).map(|t| [t[0], t[1], t[2]]).collect(),
        };
        let painted: Option<Vec<f64>> = (!density.is_null()).then(|| {
            // SAFETY: one weight per vertex, as documented.
            unsafe { std::slice::from_raw_parts(density, vertex_count) }.iter().map(|&weight| weight as f64).collect()
        });
        let read = |pointer: *const c_char, what: &str| -> Result<Option<String>, String> {
            if pointer.is_null() {
                return Ok(None);
            }
            // SAFETY: NUL-terminated, as documented.
            let text = unsafe { CStr::from_ptr(pointer) };
            text.to_str().map(|text| Some(text.to_string())).map_err(|_| format!("the {what} is not valid UTF-8"))
        };
        let mut config = match read(preset, "preset name")? {
            Some(name) if !name.is_empty() => {
                RemeshConfig::preset(&name, RemeshConfig::default().target).map_err(|error| error.to_string())?
            }
            _ => RemeshConfig::default(),
        };
        if let Some(lines) = read(settings, "settings")? {
            config.apply_text(&lines).map_err(|error| error.to_string())?;
        }
        let guides = Guides { density: painted.as_deref(), ..Default::default() };
        openclare::Analysis::of(&input, &guides, &config).map_err(|error| error.to_string())
    }));
    let (analysis, error) = match outcome {
        Ok(Ok(analysis)) => (Some(analysis), CString::default()),
        Ok(Err(message)) => (None, text(message)),
        Err(_) => (None, text("openclare failed unexpectedly while analysing".to_string())),
    };
    Box::into_raw(Box::new(OpenclareAnalysis { analysis, vertices: vertex_count, error }))
}

/// 1 if the shape was analysed, 0 if not or if `analysis` is null.
///
/// # Safety
///
/// `analysis` must be null or a handle from [`openclare_analyse`] not yet
/// freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_ok(analysis: *const OpenclareAnalysis) -> i32 {
    // SAFETY: null or a live handle, by contract.
    unsafe { analysis.as_ref() }.is_some_and(|analysis| analysis.analysis.is_some()) as i32
}

/// Why the shape could not be analysed; empty if it was. Valid until freed.
///
/// # Safety
///
/// See [`openclare_analysis_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_error(analysis: *const OpenclareAnalysis) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { analysis.as_ref() } {
        Some(analysis) => analysis.error.as_ptr(),
        None => c"no analysis".as_ptr(),
    }
}

/// A largest and a smallest quad to start from, written to `largest` and
/// `smallest`. Returns 1, or 0 when there is no analysis.
///
/// # Safety
///
/// See [`openclare_analysis_ok`]; `largest` and `smallest` must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_proposal(
    analysis: *const OpenclareAnalysis,
    largest: *mut f64,
    smallest: *mut f64,
) -> i32 {
    // SAFETY: null or a live handle, by contract.
    let Some(analysis) = (unsafe { analysis.as_ref() }).and_then(|analysis| analysis.analysis.as_ref()) else {
        return 0;
    };
    if largest.is_null() || smallest.is_null() {
        return 0;
    }
    let (proposed, least) = analysis.proposal();
    // SAFETY: valid, by contract.
    unsafe { (*largest, *smallest) = (proposed, least) };
    1
}

/// How many quads quads between `smallest` and `largest` make, written to
/// `quads`, and the share of the surface that calls for smaller ones, from
/// 0 to 1, to `below`. Returns 1, or 0 when there is no analysis.
///
/// # Safety
///
/// See [`openclare_analysis_ok`]; `quads` and `below` must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_count(
    analysis: *const OpenclareAnalysis,
    largest: f64,
    smallest: f64,
    quads: *mut f64,
    below: *mut f64,
) -> i32 {
    // SAFETY: null or a live handle, by contract.
    let Some(analysis) = (unsafe { analysis.as_ref() }).and_then(|analysis| analysis.analysis.as_ref()) else {
        return 0;
    };
    if quads.is_null() || below.is_null() || !(smallest > 0.0 && largest >= smallest && largest.is_finite()) {
        return 0;
    }
    let made =
        catch_unwind(AssertUnwindSafe(|| (analysis.quads(largest, smallest), analysis.below(largest, smallest))));
    let Ok((made, under)) = made else { return 0 };
    // SAFETY: valid, by contract.
    unsafe { (*quads, *below) = (made, under) };
    1
}

/// For each vertex of the shape, the edge its quads would have with quads
/// between `smallest` and `largest`, to `sizes` (not a number for a piece
/// that could not be looked at), and 1 in `coarser` where its shape calls
/// for smaller quads than the smallest, 0 elsewhere. Returns 1, or 0 when
/// there is no analysis.
///
/// # Safety
///
/// See [`openclare_analysis_ok`]; `sizes` and `coarser` must have room for
/// one number per vertex given to [`openclare_analyse`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_vertices(
    analysis: *const OpenclareAnalysis,
    largest: f64,
    smallest: f64,
    sizes: *mut f32,
    coarser: *mut u8,
) -> i32 {
    // SAFETY: null or a live handle, by contract.
    let Some(handle) = (unsafe { analysis.as_ref() }) else { return 0 };
    let Some(analysis) = handle.analysis.as_ref() else { return 0 };
    if sizes.is_null() || coarser.is_null() || !(smallest > 0.0 && largest >= smallest && largest.is_finite()) {
        return 0;
    }
    let Ok((at, under)) = catch_unwind(AssertUnwindSafe(|| analysis.at_vertices(largest, smallest))) else { return 0 };
    // SAFETY: room for one per vertex, by contract.
    let (sizes, coarser) = unsafe {
        (
            std::slice::from_raw_parts_mut(sizes, handle.vertices),
            std::slice::from_raw_parts_mut(coarser, handle.vertices),
        )
    };
    for (v, (size, below)) in at.into_iter().zip(under).enumerate().take(handle.vertices) {
        (sizes[v], coarser[v]) = (size as f32, below as u8);
    }
    1
}

/// For each vertex of the shape, the edge its quads would have with `quads`
/// quads asked for, as a number and not as sizes, to `sizes` (not a number
/// for a piece that could not be looked at). Returns 1, or 0 when there is
/// no analysis or `quads` is not a number of quads.
///
/// # Safety
///
/// See [`openclare_analysis_ok`]; `sizes` must have room for one number per
/// vertex given to [`openclare_analyse`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_vertices_for(
    analysis: *const OpenclareAnalysis,
    quads: f64,
    sizes: *mut f32,
) -> i32 {
    // SAFETY: null or a live handle, by contract.
    let Some(handle) = (unsafe { analysis.as_ref() }) else { return 0 };
    let Some(analysis) = handle.analysis.as_ref() else { return 0 };
    if sizes.is_null() || !(quads.is_finite() && quads > 0.0) {
        return 0;
    }
    let Ok(at) = catch_unwind(AssertUnwindSafe(|| analysis.at_vertices_for(quads))) else { return 0 };
    // SAFETY: room for one per vertex, by contract.
    let sizes = unsafe { std::slice::from_raw_parts_mut(sizes, handle.vertices) };
    for (v, size) in at.into_iter().enumerate().take(handle.vertices) {
        sizes[v] = size as f32;
    }
    1
}

/// # Safety
///
/// `analysis` must be null or a handle from [`openclare_analyse`] not yet
/// freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_analysis_free(analysis: *mut OpenclareAnalysis) {
    if !analysis.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_analyse`.
        drop(unsafe { Box::from_raw(analysis) });
    }
}

/// What a call to [`openclare_hard_edges`] gives back: the hard edges a
/// remesh would follow, and those it would leave out, found without
/// remeshing.
pub struct OpenclareHardEdges {
    kept: Vec<[u32; 2]>,
    left_out: Vec<[u32; 2]>,
    /// Empty when all went well.
    error: CString,
}

/// Finds the hard edges [`openclare_remesh`] would follow on a mesh with the
/// same arguments, by the same code and at the size of the quads it would
/// make, without remeshing: what a host shows of the angle of the hard edges
/// before a run.
///
/// - `positions`, `vertex_count`, `triangles`, `triangle_count`,
///   `corner_vertices`, `corner_count`, `density`, `preset`, `settings`: as
///   for [`openclare_remesh`]. The target, the angles, the adaptivity, the
///   quads round a thin part and the paint count, as they set the size of
///   the quads of each piece of the mesh.
///
/// The edges are given as two vertices of the mesh each. Kept: those sharper
/// than the angle of the hard edges (`edge_angle`) that still are at the
/// size of the quads, and those sharper than `crease_angle`; open borders,
/// which are always followed, are not among them. Left out: those sharper
/// than `edge_angle` between their two triangles that are not at the size
/// of the quads, the grain of a sculpt. Where the mesh had vertices at the
/// same place along a torn border, which the remesh welds, an edge names
/// one of them at each end.
///
/// Returns a handle to free with [`openclare_hard_edges_free`], never null.
///
/// # Safety
///
/// As for [`openclare_remesh`].
#[allow(clippy::too_many_arguments)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges(
    positions: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    triangle_count: usize,
    corner_vertices: *const u32,
    corner_count: usize,
    density: *const f32,
    preset: *const c_char,
    settings: *const c_char,
) -> *mut OpenclareHardEdges {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<openclare::HardEdges, String> {
        // SAFETY: the caller vouches for the pointers and their lengths.
        unsafe {
            let corners =
                (!corner_vertices.is_null()).then(|| std::slice::from_raw_parts(corner_vertices, corner_count));
            let input = input_of(positions, vertex_count, triangles, triangle_count, corners)?;
            let painted: Option<Vec<f64>> = (!density.is_null()).then(|| {
                std::slice::from_raw_parts(density, vertex_count).iter().map(|&weight| weight as f64).collect()
            });
            let config = config_of(preset, settings)?;
            let guides = Guides { density: painted.as_deref(), ..Default::default() };
            config.hard_edges(&input, &guides).map_err(|error| error.to_string())
        }
    }));
    Box::into_raw(Box::new(hard_edges_of(outcome)))
}

/// The handle of hard edges for what finding them came to.
fn hard_edges_of(outcome: std::thread::Result<Result<openclare::HardEdges, String>>) -> OpenclareHardEdges {
    match outcome {
        Ok(Ok(found)) => OpenclareHardEdges { kept: found.kept, left_out: found.left_out, error: CString::default() },
        Ok(Err(message)) => OpenclareHardEdges { kept: Vec::new(), left_out: Vec::new(), error: text(message) },
        Err(_) => OpenclareHardEdges {
            kept: Vec::new(),
            left_out: Vec::new(),
            error: text("openclare failed unexpectedly while finding the hard edges".to_string()),
        },
    }
}

/// One mesh whose hard edges are found again and again under other
/// settings, by [`openclare_hard_edge_finder_find`]: what depends on the
/// mesh alone is done once, and the size of the quads is not looked for
/// again for settings already met.
pub struct OpenclareHardEdgeFinder {
    /// None when the mesh could not be read.
    finder: Option<HardEdgeFinder>,
    vertex_count: usize,
    /// Empty when all went well.
    error: CString,
}

/// Takes in a mesh to find the hard edges of with
/// [`openclare_hard_edge_finder_find`], as [`openclare_hard_edges`] finds
/// them, and quicker when called more than once.
///
/// - `positions`, `vertex_count`, `triangles`, `triangle_count`,
///   `corner_vertices`, `corner_count`: as for [`openclare_remesh`]. They
///   are copied: the caller may free them on return.
///
/// Returns a handle to free with [`openclare_hard_edge_finder_free`], never
/// null. A handle is used by one thread at a time.
///
/// # Safety
///
/// As for [`openclare_remesh`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edge_finder(
    positions: *const f32,
    vertex_count: usize,
    triangles: *const u32,
    triangle_count: usize,
    corner_vertices: *const u32,
    corner_count: usize,
) -> *mut OpenclareHardEdgeFinder {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<TriMesh, String> {
        // SAFETY: the caller vouches for the pointers and their lengths.
        unsafe {
            let corners =
                (!corner_vertices.is_null()).then(|| std::slice::from_raw_parts(corner_vertices, corner_count));
            input_of(positions, vertex_count, triangles, triangle_count, corners)
        }
    }));
    let (finder, error) = match outcome {
        Ok(Ok(input)) => (Some(HardEdgeFinder::new(input)), CString::default()),
        Ok(Err(message)) => (None, text(message)),
        Err(_) => (None, text("openclare failed unexpectedly while reading the mesh".to_string())),
    };
    Box::into_raw(Box::new(OpenclareHardEdgeFinder { finder, vertex_count, error }))
}

/// 1 if the mesh was read, 0 if not or if `finder` is null.
///
/// # Safety
///
/// `finder` must be null or a handle from [`openclare_hard_edge_finder`] not
/// yet freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edge_finder_ok(finder: *const OpenclareHardEdgeFinder) -> i32 {
    // SAFETY: null or a live handle, by contract.
    unsafe { finder.as_ref() }.is_some_and(|finder| finder.finder.is_some()) as i32
}

/// Why the mesh could not be read; empty if it was. Valid until freed.
///
/// # Safety
///
/// See [`openclare_hard_edge_finder_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edge_finder_error(finder: *const OpenclareHardEdgeFinder) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { finder.as_ref() } {
        Some(finder) => finder.error.as_ptr(),
        None => c"no mesh".as_ptr(),
    }
}

/// Finds the hard edges of the mesh of `finder` under `preset` and
/// `settings`, with `density` (null, or one weight per vertex): what
/// [`openclare_hard_edges`] gives with the same mesh and arguments.
///
/// Returns a handle to free with [`openclare_hard_edges_free`], never null.
///
/// # Safety
///
/// See [`openclare_hard_edge_finder_ok`]; `density` and the texts as for
/// [`openclare_hard_edges`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edge_finder_find(
    finder: *mut OpenclareHardEdgeFinder,
    density: *const f32,
    preset: *const c_char,
    settings: *const c_char,
) -> *mut OpenclareHardEdges {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<openclare::HardEdges, String> {
        // SAFETY: null or a live handle, by contract.
        let Some(finder) = (unsafe { finder.as_mut() }) else { return Err("no mesh".into()) };
        let vertex_count = finder.vertex_count;
        let Some(found) = finder.finder.as_mut() else {
            return Err(finder.error.to_string_lossy().into_owned());
        };
        // SAFETY: one weight per vertex, and texts null or NUL-terminated,
        // as the caller vouches.
        unsafe {
            let painted: Option<Vec<f64>> = (!density.is_null()).then(|| {
                std::slice::from_raw_parts(density, vertex_count).iter().map(|&weight| weight as f64).collect()
            });
            let config = config_of(preset, settings)?;
            let guides = Guides { density: painted.as_deref(), ..Default::default() };
            found.find(&guides, &config).map_err(|error| error.to_string())
        }
    }));
    Box::into_raw(Box::new(hard_edges_of(outcome)))
}

/// # Safety
///
/// `finder` must be null or a handle from [`openclare_hard_edge_finder`] not
/// yet freed, and no other thread may be using it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edge_finder_free(finder: *mut OpenclareHardEdgeFinder) {
    if !finder.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_hard_edge_finder`.
        drop(unsafe { Box::from_raw(finder) });
    }
}

/// 1 if the hard edges were found, 0 if not or if `found` is null.
///
/// # Safety
///
/// `found` must be null or a handle from [`openclare_hard_edges`] not yet
/// freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_ok(found: *const OpenclareHardEdges) -> i32 {
    // SAFETY: null or a live handle, by contract.
    unsafe { found.as_ref() }.is_some_and(|found| found.error.as_bytes().is_empty()) as i32
}

/// Why the hard edges could not be found; empty if they were. Valid until
/// freed.
///
/// # Safety
///
/// See [`openclare_hard_edges_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_error(found: *const OpenclareHardEdges) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { found.as_ref() } {
        Some(found) => found.error.as_ptr(),
        None => c"no hard edges".as_ptr(),
    }
}

/// How many edges a remesh would follow.
///
/// # Safety
///
/// See [`openclare_hard_edges_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_kept_count(found: *const OpenclareHardEdges) -> usize {
    // SAFETY: null or a live handle, by contract.
    unsafe { found.as_ref() }.map_or(0, |found| found.kept.len())
}

/// Writes the edges a remesh would follow, two vertices per edge, into
/// `vertices`.
///
/// # Safety
///
/// See [`openclare_hard_edges_ok`]; `vertices` must have room for twice
/// [`openclare_hard_edges_kept_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_copy_kept(found: *const OpenclareHardEdges, vertices: *mut u32) {
    // SAFETY: null or a live handle, by contract.
    let Some(found) = (unsafe { found.as_ref() }) else { return };
    // SAFETY: room as documented.
    unsafe { copy_edges(&found.kept, vertices) }
}

/// How many edges sharper than the angle a remesh would leave out.
///
/// # Safety
///
/// See [`openclare_hard_edges_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_left_out_count(found: *const OpenclareHardEdges) -> usize {
    // SAFETY: null or a live handle, by contract.
    unsafe { found.as_ref() }.map_or(0, |found| found.left_out.len())
}

/// Writes the edges sharper than the angle a remesh would leave out, two
/// vertices per edge, into `vertices`.
///
/// # Safety
///
/// See [`openclare_hard_edges_ok`]; `vertices` must have room for twice
/// [`openclare_hard_edges_left_out_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_copy_left_out(found: *const OpenclareHardEdges, vertices: *mut u32) {
    // SAFETY: null or a live handle, by contract.
    let Some(found) = (unsafe { found.as_ref() }) else { return };
    // SAFETY: room as documented.
    unsafe { copy_edges(&found.left_out, vertices) }
}

/// # Safety
///
/// `found` must be null or a handle from [`openclare_hard_edges`] not yet
/// freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_hard_edges_free(found: *mut OpenclareHardEdges) {
    if !found.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_hard_edges`.
        drop(unsafe { Box::from_raw(found) });
    }
}

/// What a call to [`openclare_reflection_check`] gives back: the faults of a
/// cage under Catmull-Clark, worst first, or why there are none.
pub struct OpenclareReflection {
    check: openclare::reflection::Check,
    /// Empty when all went well.
    error: CString,
}

/// The kinds of fault [`openclare_reflection_copy_faults`] writes, in the
/// order of `openclare::reflection::Kind`.
pub const OPENCLARE_FAULT_DEGENERATE: u8 = 0;
pub const OPENCLARE_FAULT_FOLDED: u8 = 1;
pub const OPENCLARE_FAULT_NOT_FLAT: u8 = 2;
pub const OPENCLARE_FAULT_PINCH: u8 = 3;
pub const OPENCLARE_FAULT_CURVED_POLE: u8 = 4;
pub const OPENCLARE_FAULT_THIN: u8 = 5;

/// The reflection check of a cage: what of it will make a reflection pinch
/// or wobble once Catmull-Clark subdivides it, fault by fault, worst first
/// (see `openclare::reflection`). Angles are measured on the cage, not on
/// the subdivided surface.
///
/// - `positions`: `3 * vertex_count` numbers.
/// - `face_sizes`: the number of corners of each of `face_count` faces;
///   `corner_vertices`: the vertex of each corner, `corner_count` of them,
///   face after face, as Blender keeps its polygons.
/// - `crease_edges`: `2 * crease_count` vertices, two per edge, and
///   `crease_weights` one weight from 0 to 1 per edge; both null, and
///   `crease_count` 0, for none: only the borders are then sharp.
/// - `flat_tolerance` (a share of the radius of a face), `angle_tolerance`
///   (degrees) and `thin_ratio`: the thresholds of what counts as a fault;
///   a negative or non-finite one takes its default (0.002, 1, 0.15).
///
/// Returns a handle to free with [`openclare_reflection_free`], never null.
///
/// # Safety
///
/// Each pointer must be null where allowed, or valid for the number of
/// elements said of it, for the time of the call.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn openclare_reflection_check(
    positions: *const f32,
    vertex_count: usize,
    face_sizes: *const u32,
    face_count: usize,
    corner_vertices: *const u32,
    corner_count: usize,
    crease_edges: *const u32,
    crease_weights: *const f32,
    crease_count: usize,
    flat_tolerance: f64,
    angle_tolerance: f64,
    thin_ratio: f64,
) -> *mut OpenclareReflection {
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<openclare::reflection::Check, String> {
        // SAFETY: as the caller says.
        let (mesh, creases) = unsafe {
            read_cage(
                positions,
                vertex_count,
                face_sizes,
                face_count,
                corner_vertices,
                corner_count,
                crease_edges,
                crease_weights,
                crease_count,
            )?
        };
        let settings = check_settings(flat_tolerance, angle_tolerance, thin_ratio);
        Ok(openclare::reflection::check(&mesh, &creases, &settings))
    }));
    let (check, error) = match outcome {
        Ok(Ok(check)) => (check, CString::default()),
        Ok(Err(message)) => (Default::default(), text(message)),
        Err(_) => (Default::default(), text("openclare failed unexpectedly while checking".to_string())),
    };
    Box::into_raw(Box::new(OpenclareReflection { check, error }))
}

/// 1 if the cage was checked, 0 if not or if `reflection` is null.
///
/// # Safety
///
/// `reflection` must be null or a handle from [`openclare_reflection_check`]
/// not yet freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_ok(reflection: *const OpenclareReflection) -> i32 {
    // SAFETY: null or a live handle, by contract.
    unsafe { reflection.as_ref() }.is_some_and(|reflection| reflection.error.as_bytes().is_empty()) as i32
}

/// Why the cage could not be checked; empty if it was. Valid until freed.
///
/// # Safety
///
/// See [`openclare_reflection_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_error(reflection: *const OpenclareReflection) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { reflection.as_ref() } {
        Some(reflection) => reflection.error.as_ptr(),
        None => c"no reflection check".as_ptr(),
    }
}

/// How many faults were found.
///
/// # Safety
///
/// See [`openclare_reflection_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_fault_count(reflection: *const OpenclareReflection) -> usize {
    // SAFETY: null or a live handle, by contract.
    unsafe { reflection.as_ref() }.map_or(0, |reflection| reflection.check.faults.len())
}

/// How many poles were looked at, on a curved zone or not.
///
/// # Safety
///
/// See [`openclare_reflection_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_pole_count(reflection: *const OpenclareReflection) -> usize {
    // SAFETY: null or a live handle, by contract.
    unsafe { reflection.as_ref() }.map_or(0, |reflection| reflection.check.poles)
}

/// How many parts (corners and edges of faces) the faults name in all.
///
/// # Safety
///
/// See [`openclare_reflection_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_part_count(reflection: *const OpenclareReflection) -> usize {
    // SAFETY: null or a live handle, by contract.
    unsafe { reflection.as_ref() }.map_or(0, |reflection| reflection.check.faults.iter().map(|f| f.parts.len()).sum())
}

/// Writes the faults, worst first, one number per fault into each array
/// that is not null:
///
/// - `kinds`: [`OPENCLARE_FAULT_DEGENERATE`] (0) for a face of no area,
///   [`OPENCLARE_FAULT_FOLDED`] (1) for corner quads that fold,
///   [`OPENCLARE_FAULT_NOT_FLAT`] (2), [`OPENCLARE_FAULT_PINCH`] (3),
///   [`OPENCLARE_FAULT_CURVED_POLE`] (4) for a pole on a curved zone,
///   [`OPENCLARE_FAULT_THIN`] (5) for thin corner quads;
/// - `elements`: 0 for a face, 1 for a vertex; `indices`: which one;
/// - `sides`: the sides of the face, or the valence of the vertex;
///   `counts`: how many of its parts the fault concerns;
/// - `values`: the number that ranks it, an angle in degrees for kinds 2, 3
///   and 4, the smallest triangle (centre, edge) over the mean one for 1
///   and 5, 0 for 0.
///
/// # Safety
///
/// See [`openclare_reflection_ok`]; each array not null must have room for
/// [`openclare_reflection_fault_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_copy_faults(
    reflection: *const OpenclareReflection,
    kinds: *mut u8,
    elements: *mut u8,
    indices: *mut u32,
    sides: *mut u32,
    counts: *mut u32,
    values: *mut f64,
) {
    use openclare::reflection::Element;
    // SAFETY: null or a live handle, by contract.
    let Some(reflection) = (unsafe { reflection.as_ref() }) else { return };
    for (i, fault) in reflection.check.faults.iter().enumerate() {
        let (element, index) = match fault.at {
            Element::Face(f) => (0, f),
            Element::Vertex(v) => (1, v),
        };
        // SAFETY: room for one number per fault in each array not null.
        unsafe {
            write(kinds, i, fault.kind as u8);
            write(elements, i, element);
            write(indices, i, index);
            write(sides, i, fault.sides as u32);
            write(counts, i, fault.count as u32);
            write(values, i, fault.value);
        }
    }
}

/// Writes where each fault is to be drawn, into each array that is not
/// null: `centres` and `normals`, three numbers per fault (the centre of the
/// face or the vertex; the unit normal of the face or the mean one round the
/// vertex, zero for a degenerate face); `inner` and `radius`, one per fault
/// (for a face, the distance from its centre to its nearest edge and to its
/// furthest vertex; for a vertex, the mean length of its edges, twice).
///
/// # Safety
///
/// See [`openclare_reflection_ok`]; `centres` and `normals`, if not null,
/// must have room for three times [`openclare_reflection_fault_count`]
/// numbers, `inner` and `radius` for that count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_copy_frames(
    reflection: *const OpenclareReflection,
    centres: *mut f32,
    normals: *mut f32,
    inner: *mut f32,
    radius: *mut f32,
) {
    // SAFETY: null or a live handle, by contract.
    let Some(reflection) = (unsafe { reflection.as_ref() }) else { return };
    for (i, fault) in reflection.check.faults.iter().enumerate() {
        let frame = &fault.frame;
        let (centre, normal) = (frame.centre.to_array(), frame.normal.to_array());
        // SAFETY: room as documented in each array not null.
        unsafe {
            for axis in 0..3 {
                write(centres, 3 * i + axis, centre[axis] as f32);
                write(normals, 3 * i + axis, normal[axis] as f32);
            }
            write(inner, i, frame.inner as f32);
            write(radius, i, frame.radius as f32);
        }
    }
}

/// Writes the parts of the faces at fault: `offsets`, fault count + 1
/// numbers, the parts of fault `i` being those from `offsets[i]` to
/// `offsets[i + 1]`; then for each part, its kind in `part_kinds` (0 a
/// corner, 1 an edge) and its place in the face in `part_indices` (corner
/// `k` is the face's vertex `k`; edge `k` runs from its vertex `k` to its
/// vertex `k + 1`). Any array may be null.
///
/// # Safety
///
/// See [`openclare_reflection_ok`]; `offsets` must have room for
/// [`openclare_reflection_fault_count`] + 1 numbers, `part_kinds` and
/// `part_indices` for [`openclare_reflection_part_count`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_copy_parts(
    reflection: *const OpenclareReflection,
    offsets: *mut u32,
    part_kinds: *mut u8,
    part_indices: *mut u32,
) {
    use openclare::reflection::Part;
    // SAFETY: null or a live handle, by contract.
    let Some(reflection) = (unsafe { reflection.as_ref() }) else { return };
    let mut at = 0;
    for (i, fault) in reflection.check.faults.iter().enumerate() {
        // SAFETY: room as documented in each array not null.
        unsafe { write(offsets, i, at as u32) };
        for part in &fault.parts {
            let (kind, index) = match *part {
                Part::Corner(k) => (0, k),
                Part::Edge(k) => (1, k),
            };
            // SAFETY: likewise.
            unsafe {
                write(part_kinds, at, kind);
                write(part_indices, at, index);
            }
            at += 1;
        }
    }
    // SAFETY: likewise.
    unsafe { write(offsets, reflection.check.faults.len(), at as u32) };
}

/// # Safety
///
/// `reflection` must be null or a handle from [`openclare_reflection_check`]
/// not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_reflection_free(reflection: *mut OpenclareReflection) {
    if !reflection.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_reflection_check`.
        drop(unsafe { Box::from_raw(reflection) });
    }
}

/// A cage from the flat arrays of [`openclare_reflection_check`] and
/// [`openclare_reflection_repair`], checked.
///
/// # Safety
///
/// Each pointer null where its count is 0, or valid for the number of
/// elements said of it.
#[allow(clippy::too_many_arguments)]
unsafe fn read_cage(
    positions: *const f32,
    vertex_count: usize,
    face_sizes: *const u32,
    face_count: usize,
    corner_vertices: *const u32,
    corner_count: usize,
    crease_edges: *const u32,
    crease_weights: *const f32,
    crease_count: usize,
) -> Result<(PolyMesh, Vec<([u32; 2], f64)>), String> {
    if (positions.is_null() && vertex_count > 0)
        || (face_sizes.is_null() && face_count > 0)
        || (corner_vertices.is_null() && corner_count > 0)
    {
        return Err("the mesh arrays are missing".into());
    }
    if crease_count > 0 && (crease_edges.is_null() || crease_weights.is_null()) {
        return Err("the crease arrays are missing".into());
    }
    // SAFETY (all five): not null where the count is not 0, and as long as
    // the caller says.
    let coordinates = unsafe { read(positions, 3 * vertex_count) };
    let sizes = unsafe { read(face_sizes, face_count) };
    let corners = unsafe { read(corner_vertices, corner_count) };
    let crease_pairs = unsafe { read(crease_edges, 2 * crease_count) };
    let weights = unsafe { read(crease_weights, crease_count) };
    let total: usize = sizes.iter().map(|&size| size as usize).sum();
    if total != corner_count {
        return Err(format!("the faces have {total} corners in all, and {corner_count} are given"));
    }
    if let Some(&beyond) = corners.iter().chain(crease_pairs).find(|&&v| v as usize >= vertex_count) {
        return Err(format!("vertex {beyond} is named, of {vertex_count} vertices"));
    }
    let mut faces = Vec::with_capacity(face_count);
    let mut start = 0;
    for &size in sizes {
        faces.push(corners[start..start + size as usize].to_vec());
        start += size as usize;
    }
    let mesh = PolyMesh {
        positions: coordinates.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect(),
        faces,
    };
    let creases =
        crease_pairs.chunks_exact(2).zip(weights).map(|(pair, &weight)| ([pair[0], pair[1]], weight as f64)).collect();
    Ok((mesh, creases))
}

/// A setting given, or its default where it is negative or not finite.
fn or_default(given: f64, default: f64) -> f64 {
    if given.is_finite() && given >= 0.0 { given } else { default }
}

/// The thresholds of the check, each its default where it is negative or
/// not finite.
fn check_settings(flat_tolerance: f64, angle_tolerance: f64, thin_ratio: f64) -> openclare::reflection::Settings {
    let defaults = openclare::reflection::Settings::default();
    openclare::reflection::Settings {
        flat_tolerance: or_default(flat_tolerance, defaults.flat_tolerance),
        angle_tolerance: or_default(angle_tolerance, defaults.angle_tolerance),
        thin_ratio: or_default(thin_ratio, defaults.thin_ratio),
        ..defaults
    }
}

/// What a call to [`openclare_reflection_repair`] gives back: the cage
/// repaired in place, the creases proposed, the scores before and after,
/// the faults left and why; or why there is none of that.
pub struct OpenclareRepair {
    repaired: Option<openclare::reflection::repair::Repaired>,
    /// Empty when all went well.
    error: CString,
}

/// The scores [`openclare_repair_copy_score`] writes: of the cage given;
/// after the moves, with the creases given; after the moves with the
/// proposed creases set too.
pub const OPENCLARE_REPAIR_BEFORE: u32 = 0;
pub const OPENCLARE_REPAIR_AFTER: u32 = 1;
pub const OPENCLARE_REPAIR_WITH_CREASES: u32 = 2;

/// Why a fault is left, as [`openclare_repair_copy_left`] writes it, in the
/// order of `openclare::reflection::repair::Reason`.
pub const OPENCLARE_REASON_CHANGE_TOPOLOGY: u8 = 0;
pub const OPENCLARE_REASON_TOO_FAR: u8 = 1;
pub const OPENCLARE_REASON_WOULD_WORSEN: u8 = 2;
pub const OPENCLARE_REASON_PROPOSED_CREASE: u8 = 3;
pub const OPENCLARE_REASON_OUTSIDE_SELECTION: u8 = 4;
pub const OPENCLARE_REASON_OUT_OF_PASSES: u8 = 5;

/// The reflection repair of a cage, in place (see
/// `openclare::reflection::repair`): every vertex, edge and face kept;
/// vertices moved to flatten the n-gons off their plane and to make the
/// faces whose corner quads fold or are thin star-shaped again; creases
/// proposed on the edges already sharp where they cure a pinch or a pole on
/// a curved zone, never set. Each repair is kept only if the score of the
/// reflection check gets better. Angles are measured on the cage, not on
/// the subdivided surface.
///
/// - The cage and its creases as for [`openclare_reflection_check`].
/// - `within`: `within_count` faces, the only ones repaired (only their
///   vertices move, only their edges get a crease); null and 0 for the
///   whole cage.
/// - `flat_tolerance`, `angle_tolerance` and `thin_ratio` as for the check;
///   `bound`, the largest displacement of a vertex as a share of the
///   shortest of its edges (default 0.1); `crease_fold`, the fold in
///   degrees between two faces from which an edge may get a crease
///   (default 30). A negative or non-finite one takes its default.
///
/// Returns a handle to free with [`openclare_repair_free`], never null.
///
/// # Safety
///
/// Each pointer must be null where allowed, or valid for the number of
/// elements said of it, for the time of the call.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn openclare_reflection_repair(
    positions: *const f32,
    vertex_count: usize,
    face_sizes: *const u32,
    face_count: usize,
    corner_vertices: *const u32,
    corner_count: usize,
    crease_edges: *const u32,
    crease_weights: *const f32,
    crease_count: usize,
    within: *const u32,
    within_count: usize,
    flat_tolerance: f64,
    angle_tolerance: f64,
    thin_ratio: f64,
    bound: f64,
    crease_fold: f64,
) -> *mut OpenclareRepair {
    use openclare::reflection::repair::{RepairSettings, repair};
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<openclare::reflection::repair::Repaired, String> {
        // SAFETY: as the caller says.
        let (mesh, creases) = unsafe {
            read_cage(
                positions,
                vertex_count,
                face_sizes,
                face_count,
                corner_vertices,
                corner_count,
                crease_edges,
                crease_weights,
                crease_count,
            )?
        };
        if within.is_null() && within_count > 0 {
            return Err("the faces to repair are missing".into());
        }
        // SAFETY: not null where the count is not 0, and as long as said.
        let chosen = unsafe { read(within, within_count) };
        if let Some(&beyond) = chosen.iter().find(|&&f| f as usize >= face_count) {
            return Err(format!("face {beyond} is named, of {face_count} faces"));
        }
        let defaults = RepairSettings::default();
        let settings = RepairSettings {
            check: check_settings(flat_tolerance, angle_tolerance, thin_ratio),
            bound: or_default(bound, defaults.bound),
            crease_fold: or_default(crease_fold, defaults.crease_fold),
            ..defaults
        };
        Ok(repair(&mesh, &creases, (!within.is_null()).then_some(chosen), &settings))
    }));
    let (repaired, error) = match outcome {
        Ok(Ok(repaired)) => (Some(repaired), CString::default()),
        Ok(Err(message)) => (None, text(message)),
        Err(_) => (None, text("openclare failed unexpectedly while repairing".to_string())),
    };
    Box::into_raw(Box::new(OpenclareRepair { repaired, error }))
}

/// The repair behind a handle, if there is one.
///
/// # Safety
///
/// `repair` must be null or a live handle.
unsafe fn repaired<'a>(repair: *const OpenclareRepair) -> Option<&'a openclare::reflection::repair::Repaired> {
    // SAFETY: null or a live handle, by contract.
    unsafe { repair.as_ref() }.and_then(|repair| repair.repaired.as_ref())
}

/// 1 if the cage was repaired (even with nothing to change), 0 if not or if
/// `repair` is null.
///
/// # Safety
///
/// `repair` must be null or a handle from [`openclare_reflection_repair`]
/// not yet freed; the same goes for every function below.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_ok(repair: *const OpenclareRepair) -> i32 {
    // SAFETY: by contract.
    unsafe { repaired(repair) }.is_some() as i32
}

/// Why the cage could not be repaired; empty if it was. Valid until freed.
///
/// # Safety
///
/// See [`openclare_repair_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_error(repair: *const OpenclareRepair) -> *const c_char {
    // SAFETY: null or a live handle, by contract.
    match unsafe { repair.as_ref() } {
        Some(repair) => repair.error.as_ptr(),
        None => c"no reflection repair".as_ptr(),
    }
}

/// How many vertices the cage has: those
/// [`openclare_repair_copy_positions`] writes.
///
/// # Safety
///
/// See [`openclare_repair_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_vertex_count(repair: *const OpenclareRepair) -> usize {
    // SAFETY: by contract.
    unsafe { repaired(repair) }.map_or(0, |r| r.positions.len())
}

/// How many of them moved.
///
/// # Safety
///
/// See [`openclare_repair_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_moved_count(repair: *const OpenclareRepair) -> usize {
    // SAFETY: by contract.
    unsafe { repaired(repair) }.map_or(0, |r| r.moved)
}

/// How far the furthest vertex moved; with `share` not null, the largest
/// displacement over the shortest edge of its vertex is written there.
///
/// # Safety
///
/// See [`openclare_repair_ok`]; `share` null or room for one number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_largest_displacement(repair: *const OpenclareRepair, share: *mut f64) -> f64 {
    // SAFETY: by contract.
    let Some(r) = (unsafe { repaired(repair) }) else { return 0.0 };
    // SAFETY: room for one number if not null.
    unsafe { write(share, 0, r.largest_share) };
    r.largest_displacement
}

/// How many repairs were kept; with `passes` not null, how many passes ran
/// is written there.
///
/// # Safety
///
/// See [`openclare_repair_ok`]; `passes` null or room for one number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_repair_count(repair: *const OpenclareRepair, passes: *mut usize) -> usize {
    // SAFETY: by contract.
    let Some(r) = (unsafe { repaired(repair) }) else { return 0 };
    // SAFETY: room for one number if not null.
    unsafe { write(passes, 0, r.passes) };
    r.repairs
}

/// Writes the positions of the repaired cage, three numbers per vertex,
/// numbered as given; a vertex not moved is written as it was given.
///
/// # Safety
///
/// See [`openclare_repair_ok`]; `positions` must have room for three times
/// [`openclare_repair_vertex_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_copy_positions(repair: *const OpenclareRepair, positions: *mut f32) {
    // SAFETY: by contract.
    let Some(r) = (unsafe { repaired(repair) }) else { return };
    for (i, p) in r.positions.iter().enumerate() {
        // SAFETY: room as documented.
        unsafe {
            write(positions, 3 * i, p.x as f32);
            write(positions, 3 * i + 1, p.y as f32);
            write(positions, 3 * i + 2, p.z as f32);
        }
    }
}

/// Writes one score, `stage` [`OPENCLARE_REPAIR_BEFORE`],
/// [`OPENCLARE_REPAIR_AFTER`] or [`OPENCLARE_REPAIR_WITH_CREASES`]: for each
/// of the six kinds of fault, in the order of the `OPENCLARE_FAULT_*`, how
/// many, the worst value and the median one (NaN where there is none), into
/// each array not null. Returns how many poles were looked at; 0 for a stage
/// unknown.
///
/// # Safety
///
/// See [`openclare_repair_ok`]; each array not null must have room for six
/// numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_copy_score(
    repair: *const OpenclareRepair,
    stage: u32,
    counts: *mut u32,
    worst: *mut f64,
    median: *mut f64,
) -> usize {
    // SAFETY: by contract.
    let Some(r) = (unsafe { repaired(repair) }) else { return 0 };
    let score = match stage {
        OPENCLARE_REPAIR_BEFORE => &r.before,
        OPENCLARE_REPAIR_AFTER => &r.after,
        OPENCLARE_REPAIR_WITH_CREASES => &r.with_creases,
        _ => return 0,
    };
    for (i, kind) in score.kinds.iter().enumerate() {
        // SAFETY: room for six numbers in each array not null.
        unsafe {
            write(counts, i, kind.count as u32);
            write(worst, i, kind.worst.unwrap_or(f64::NAN));
            write(median, i, kind.median.unwrap_or(f64::NAN));
        }
    }
    score.poles
}

/// How many creases are proposed.
///
/// # Safety
///
/// See [`openclare_repair_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_crease_count(repair: *const OpenclareRepair) -> usize {
    // SAFETY: by contract.
    unsafe { repaired(repair) }.map_or(0, |r| r.creases.len())
}

/// Writes the creases proposed, into each array not null: `edges`, two
/// vertices per crease; `folds`, the fold between its two faces in degrees;
/// and the fault each is proposed for: its kind (`OPENCLARE_FAULT_*`), its
/// element (0 a face, 1 a vertex) and index, its angle in degrees, and in
/// `cured` 1 if it is gone once every proposed crease is set.
///
/// # Safety
///
/// See [`openclare_repair_ok`]; `edges` must have room for twice
/// [`openclare_repair_crease_count`] numbers, the others for that count.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn openclare_repair_copy_creases(
    repair: *const OpenclareRepair,
    edges: *mut u32,
    folds: *mut f64,
    kinds: *mut u8,
    elements: *mut u8,
    indices: *mut u32,
    values: *mut f64,
    cured: *mut u8,
) {
    use openclare::reflection::Element;
    // SAFETY: by contract.
    let Some(r) = (unsafe { repaired(repair) }) else { return };
    for (i, crease) in r.creases.iter().enumerate() {
        let (element, index) = match crease.at {
            Element::Face(f) => (0, f),
            Element::Vertex(v) => (1, v),
        };
        // SAFETY: room as documented in each array not null.
        unsafe {
            write(edges, 2 * i, crease.edge[0]);
            write(edges, 2 * i + 1, crease.edge[1]);
            write(folds, i, crease.fold);
            write(kinds, i, crease.kind as u8);
            write(elements, i, element);
            write(indices, i, index);
            write(values, i, crease.value);
            write(cured, i, crease.cured as u8);
        }
    }
}

/// How many faults are left after the moves, with the creases given.
///
/// # Safety
///
/// See [`openclare_repair_ok`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_left_count(repair: *const OpenclareRepair) -> usize {
    // SAFETY: by contract.
    unsafe { repaired(repair) }.map_or(0, |r| r.left.len())
}

/// Writes the faults left, worst first, as
/// [`openclare_reflection_copy_faults`] does, and in `reasons` why each is
/// left (`OPENCLARE_REASON_*`); any array may be null.
///
/// # Safety
///
/// See [`openclare_repair_ok`]; each array not null must have room for
/// [`openclare_repair_left_count`] numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_copy_left(
    repair: *const OpenclareRepair,
    kinds: *mut u8,
    elements: *mut u8,
    indices: *mut u32,
    sides: *mut u32,
    values: *mut f64,
    reasons: *mut u8,
) {
    use openclare::reflection::Element;
    // SAFETY: by contract.
    let Some(r) = (unsafe { repaired(repair) }) else { return };
    for (i, left) in r.left.iter().enumerate() {
        let fault = &left.fault;
        let (element, index) = match fault.at {
            Element::Face(f) => (0, f),
            Element::Vertex(v) => (1, v),
        };
        // SAFETY: room for one number per fault in each array not null.
        unsafe {
            write(kinds, i, fault.kind as u8);
            write(elements, i, element);
            write(indices, i, index);
            write(sides, i, fault.sides as u32);
            write(values, i, fault.value);
            write(reasons, i, left.reason as u8);
        }
    }
}

/// # Safety
///
/// `repair` must be null or a handle from [`openclare_reflection_repair`]
/// not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openclare_repair_free(repair: *mut OpenclareRepair) {
    if !repair.is_null() {
        // SAFETY: made by `Box::into_raw` in `openclare_reflection_repair`.
        drop(unsafe { Box::from_raw(repair) });
    }
}

/// The `count` elements at `array`, none when `count` is 0.
///
/// # Safety
///
/// `array` must be valid for `count` elements when `count` is not 0.
unsafe fn read<'a, T>(array: *const T, count: usize) -> &'a [T] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: as the caller says.
        unsafe { std::slice::from_raw_parts(array, count) }
    }
}

/// Writes `value` at `array[index]`, unless `array` is null.
///
/// # Safety
///
/// `array` must be null or have room for `index + 1` elements.
unsafe fn write<T>(array: *mut T, index: usize, value: T) {
    if !array.is_null() {
        // SAFETY: room as the caller says.
        unsafe { array.add(index).write(value) };
    }
}
