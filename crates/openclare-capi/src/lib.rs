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

use std::ffi::{CStr, CString, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::OnceLock;
use std::time::Instant;

use openclare::math::Vec3;
use openclare::recorder::{Recorder, Shape};
use openclare::uv;
use openclare::{Guides, PolyMesh, RemeshConfig, Target, TriMesh};

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
/// The arrays must hold what their counts say, and the two texts, when not
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
            remesh(positions, vertex_count, triangles, triangle_count, corners, density, preset, settings, &mut watcher)
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
    preset: *const c_char,
    settings: *const c_char,
    watcher: &mut Watcher,
) -> Result<OpenclareResult, String> {
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
    let input = TriMesh {
        positions: coordinates.chunks_exact(3).map(|p| Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)).collect(),
        triangles: faces,
    };
    let painted: Option<Vec<f64>> = (!density.is_null()).then(|| {
        // SAFETY: one weight per vertex, as documented.
        unsafe { std::slice::from_raw_parts(density, vertex_count) }.iter().map(|&weight| weight as f64).collect()
    });

    // SAFETY: NUL-terminated, as documented.
    let read = |pointer: *const c_char, what: &str| -> Result<Option<String>, String> {
        if pointer.is_null() {
            return Ok(None);
        }
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

    let clock = Instant::now();
    let guides = Guides { density: painted.as_deref(), ..Default::default() };
    let (mesh, done) = config.remesh_recorded(&input, &guides, watcher).map_err(|error| error.to_string())?;
    let seconds = clock.elapsed().as_secs_f64();

    let asked = match config.target {
        Target::QuadCount(count) => format!("quads_asked = {count}"),
        Target::EdgeLength(length) => format!("edge_length_asked = {length}"),
    };
    let stats = &done.stats;
    let report = format!(
        "{asked}\nvertices = {}\nfaces = {}\nquads = {}\nirregular_vertices = {}\npieces = {}\n\
         pieces_through_fallback = {}\npieces_failed = {}\npieces_skipped = {}\nseconds = {seconds:.3}\n",
        stats.vertices,
        stats.faces,
        stats.quads,
        stats.irregular_vertices,
        done.pieces,
        done.pieces_fell_back,
        done.pieces_failed,
        done.pieces_skipped,
    );
    Ok(OpenclareResult {
        mesh,
        hard_edges: done.hard_edges,
        creases: done.creases,
        layers: OnceLock::new(),
        error: CString::default(),
        report: text(report),
    })
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
