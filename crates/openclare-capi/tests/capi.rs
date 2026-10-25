//! The C interface, called the way a host calls it.

use std::ffi::{CStr, CString, c_char, c_void};
use std::ptr::{null, null_mut};

use openclare::math::{Rng, Vec3};
use openclare::mesh::primitives;
use openclare_capi::*;

/// A ball as flat arrays: positions, and its triangles as vertex indices.
fn ball() -> (Vec<f32>, Vec<u32>) {
    let mesh = primitives::icosphere(4);
    let positions = mesh.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let triangles = mesh.triangles.iter().flatten().copied().collect();
    (positions, triangles)
}

fn text(pointer: *const c_char) -> String {
    unsafe { CStr::from_ptr(pointer) }.to_str().unwrap().to_string()
}

/// The faces of a result: how many corners each has, and their vertices.
fn faces_of(result: *const OpenclareResult) -> (Vec<u32>, Vec<u32>) {
    unsafe {
        let mut sizes = vec![0u32; openclare_result_face_count(result)];
        let mut corners = vec![0u32; openclare_result_corner_count(result)];
        openclare_result_copy_faces(result, sizes.as_mut_ptr(), corners.as_mut_ptr());
        (sizes, corners)
    }
}

#[test]
fn a_mesh_goes_in_as_arrays_and_quads_come_out() {
    let (positions, triangles) = ball();
    let settings = CString::new("quads = 600\n").unwrap();
    unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            c"organic".as_ptr(),
            settings.as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!(openclare_result_ok(result), 1, "{}", text(openclare_result_error(result)));
        assert_eq!(text(openclare_result_error(result)), "");

        let vertices = openclare_result_vertex_count(result);
        let (sizes, corners) = faces_of(result);
        assert!((sizes.len() as f64 - 600.0).abs() < 60.0, "{} faces", sizes.len());
        assert!(sizes.iter().all(|&size| size == 4));
        assert_eq!(corners.len(), 4 * sizes.len());
        assert!(corners.iter().all(|&v| (v as usize) < vertices));

        let mut out = vec![0f32; 3 * vertices];
        openclare_result_copy_positions(result, out.as_mut_ptr());
        // A ball of radius 1, give or take what fitting does.
        for point in out.chunks_exact(3) {
            let radius = (point[0] * point[0] + point[1] * point[1] + point[2] * point[2]).sqrt();
            assert!((radius - 1.0).abs() < 0.03, "radius {radius}");
        }

        let report = text(openclare_result_report(result));
        assert!(
            report.contains("quads_asked = 600\n") && report.contains(&format!("quads = {}\n", sizes.len())),
            "{report}"
        );
        openclare_result_free(result);
    }
}

#[test]
fn triangles_may_be_given_by_their_corners_as_blender_keeps_them() {
    let (positions, triangles) = ball();
    // Corners in an order of their own: the triangles then name corners.
    let corner_vertices: Vec<u32> = triangles.iter().rev().copied().collect();
    let last = corner_vertices.len() as u32 - 1;
    let by_corner: Vec<u32> = (0..triangles.len() as u32).map(|k| last - k).collect();
    unsafe {
        let direct = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            c"quads = 300".as_ptr(),
            None,
            null_mut(),
        );
        let through = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            by_corner.as_ptr(),
            by_corner.len() / 3,
            corner_vertices.as_ptr(),
            corner_vertices.len(),
            null(),
            null(),
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            c"quads = 300".as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!((openclare_result_ok(direct), openclare_result_ok(through)), (1, 1));
        assert_eq!(faces_of(direct), faces_of(through));
        openclare_result_free(direct);
        openclare_result_free(through);
    }
}

#[test]
fn what_goes_wrong_is_said_not_thrown() {
    let (positions, triangles) = ball();
    let attempt = |triangles: &[u32], preset: &CStr, settings: &CStr| unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            preset.as_ptr(),
            settings.as_ptr(),
            None,
            null_mut(),
        );
        let outcome = (openclare_result_ok(result), text(openclare_result_error(result)));
        assert_eq!(openclare_result_vertex_count(result), 0);
        openclare_result_free(result);
        outcome
    };
    let (ok, error) = attempt(&triangles, c"no-such-preset", c"");
    assert_eq!(ok, 0);
    assert!(error.contains("no-such-preset"), "{error}");

    let (ok, error) = attempt(&triangles, c"organic", c"sharpness = 3");
    assert_eq!(ok, 0);
    assert!(error.contains("sharpness"), "{error}");

    let mut beyond = triangles.clone();
    beyond[0] = 1_000_000;
    let (ok, error) = attempt(&beyond, c"organic", c"quads = 100");
    assert_eq!(ok, 0);
    assert!(error.contains("1000000"), "{error}");

    let (ok, _) = attempt(&[], c"organic", c"quads = 100");
    assert_eq!(ok, 0);
}

/// Notes each stage in the list `user` points to; stops at "layout" if the
/// list starts with the word "stop".
unsafe extern "C" fn note(stage: *const c_char, user: *mut c_void) -> i32 {
    let stages = unsafe { &mut *user.cast::<Vec<String>>() };
    let name = text(stage);
    let stop = stages.first().is_some_and(|first| first == "stop") && name == "layout";
    stages.push(name);
    stop as i32
}

#[test]
fn a_host_follows_the_stages_and_can_stop_the_run() {
    let (positions, triangles) = ball();
    let run = |stages: &mut Vec<String>| unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            c"quads = 300".as_ptr(),
            Some(note),
            (stages as *mut Vec<String>).cast(),
        );
        let outcome = (openclare_result_ok(result), text(openclare_result_error(result)));
        openclare_result_free(result);
        outcome
    };
    let mut stages = Vec::new();
    assert_eq!(run(&mut stages).0, 1);
    assert_eq!(stages.first().map(String::as_str), Some("preprocessing"));
    assert_eq!(stages.last().map(String::as_str), Some("fitting"));
    assert!(stages.len() >= 8, "{stages:?}");

    let mut stages = vec!["stop".to_string()];
    let (ok, error) = run(&mut stages);
    assert_eq!((ok, error.as_str()), (0, "stopped on request"));
    assert_eq!(stages.last().map(String::as_str), Some("layout"));
}

#[test]
fn a_result_carries_its_texture_coordinates_and_its_hard_edges() {
    let cube = primitives::cube(8);
    let positions: Vec<f32> = cube.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let triangles: Vec<u32> = cube.triangles.iter().flatten().copied().collect();
    unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            c"hard-surface".as_ptr(),
            c"quads = 600".as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!(openclare_result_ok(result), 1, "{}", text(openclare_result_error(result)));
        let (vertices, corners) = (openclare_result_vertex_count(result), openclare_result_corner_count(result));

        let mut grid = vec![-1f32; 2 * corners];
        let mut atlas = vec![-1f32; 2 * corners];
        assert_eq!(openclare_result_copy_uv(result, OPENCLARE_UV_GRID, grid.as_mut_ptr()), 1);
        assert_eq!(openclare_result_copy_uv(result, OPENCLARE_UV_ATLAS, atlas.as_mut_ptr()), 1);
        assert_eq!(openclare_result_copy_uv(result, 7, atlas.as_mut_ptr()), 0);
        assert!(grid.iter().all(|&c| c >= 0.0 && c == c.round()));
        assert!(atlas.iter().all(|&c| (0.0..=1.0).contains(&c)));

        let mut seams = vec![u32::MAX; 2 * openclare_result_seam_count(result)];
        openclare_result_copy_seams(result, seams.as_mut_ptr());
        assert!(!seams.is_empty() && seams.iter().all(|&v| (v as usize) < vertices));

        // The twelve edges of the cube.
        let mut hard = vec![u32::MAX; 2 * openclare_result_hard_edge_count(result)];
        openclare_result_copy_hard_edges(result, hard.as_mut_ptr());
        assert!(hard.len() >= 2 * 12 && hard.iter().all(|&v| (v as usize) < vertices), "{} numbers", hard.len());
        // Creases: the same twelve edges, at full weight.
        let count = openclare_result_crease_count(result);
        let (mut creased, mut weights) = (vec![u32::MAX; 2 * count], vec![-1f32; count]);
        openclare_result_copy_creases(result, creased.as_mut_ptr(), weights.as_mut_ptr());
        assert_eq!(2 * count, hard.len());
        assert!(creased.iter().all(|&v| (v as usize) < vertices) && weights.iter().all(|&w| w == 1.0));
        openclare_result_free(result);

        assert_eq!(openclare_result_crease_count(null()), 0);
        assert_eq!(openclare_result_seam_count(null()), 0);
        assert_eq!(openclare_result_hard_edge_count(null()), 0);
        assert_eq!(openclare_result_copy_uv(null(), OPENCLARE_UV_GRID, grid.as_mut_ptr()), 0);
    }
}

#[test]
fn guide_curves_go_in_as_flat_arrays() {
    let (positions, triangles) = ball();
    // A circle of latitude and a short open line, one after the other.
    let radius = (1.0f32 - 0.09).sqrt();
    let mut points: Vec<f32> = (0..32)
        .flat_map(|k| {
            let turn = std::f32::consts::TAU * k as f32 / 32.0;
            [radius * turn.cos(), radius * turn.sin(), 0.3]
        })
        .collect();
    points.extend([0.0, 0.3, -0.954, 0.0, -0.3, -0.954]);
    let (sizes, closed) = ([32u32, 2], [1u8, 0]);
    unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            points.as_ptr(),
            sizes.as_ptr(),
            closed.as_ptr(),
            2,
            null(),
            c"quads = 600".as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!(openclare_result_ok(result), 1, "{}", text(openclare_result_error(result)));
        let report = text(openclare_result_report(result));
        let edges: usize = report
            .lines()
            .find_map(|line| line.strip_prefix("guide_edges = "))
            .and_then(|count| count.parse().ok())
            .unwrap();
        assert!(edges >= 14, "{report}");
        // The loop is there: vertices on the circle of latitude.
        let mut out = vec![0f32; 3 * openclare_result_vertex_count(result)];
        openclare_result_copy_positions(result, out.as_mut_ptr());
        let on_the_circle = out.chunks_exact(3).filter(|p| (p[2] - 0.3).abs() < 5e-3).count();
        assert!(on_the_circle >= 12, "{on_the_circle} vertices on the circle");
        assert_eq!(openclare_result_hard_edge_count(result), 0);
        openclare_result_free(result);

        // The same with the circle drawn as sharp: its edges are hard edges.
        let flags = [OPENCLARE_CURVE_CLOSED | OPENCLARE_CURVE_SHARP, 0];
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            null(),
            0,
            points.as_ptr(),
            sizes.as_ptr(),
            flags.as_ptr(),
            2,
            null(),
            c"quads = 600".as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!(openclare_result_ok(result), 1, "{}", text(openclare_result_error(result)));
        let report = text(openclare_result_report(result));
        let count = |key: &str| -> usize {
            report.lines().find_map(|line| line.strip_prefix(key)).and_then(|count| count.parse().ok()).unwrap()
        };
        assert!(count("sharp_curve_edges = ") >= 12, "{report}");
        assert!(count("guide_edges = ") + count("sharp_curve_edges = ") == edges, "{report}");
        assert_eq!(openclare_result_hard_edge_count(result), count("sharp_curve_edges = "));
        openclare_result_free(result);
    }
}

#[test]
fn a_null_result_is_harmless() {
    unsafe {
        assert_eq!(openclare_result_ok(null()), 0);
        assert_eq!(openclare_result_vertex_count(null()), 0);
        assert_eq!(openclare_result_face_count(null()), 0);
        assert_eq!(text(openclare_result_error(null())), "no result");
        openclare_result_copy_positions(null(), null_mut());
        openclare_result_free(null_mut());
    }
    assert_eq!(text(openclare_version()), env!("CARGO_PKG_VERSION"));
}

#[test]
fn a_shape_is_analysed_once_and_asked_as_often_as_wanted() {
    let (positions, triangles) = ball();
    let vertices = positions.len() / 3;
    let settings = CString::new("adaptivity = 0\n").unwrap();
    unsafe {
        let analysis = openclare_analyse(
            positions.as_ptr(),
            vertices,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            settings.as_ptr(),
            null(),
        );
        assert_eq!(openclare_analysis_ok(analysis), 1, "{}", text(openclare_analysis_error(analysis)));
        assert_eq!(text(openclare_analysis_error(analysis)), "");

        // A ball of radius 1 with quads of one size: its area over theirs.
        let (mut quads, mut below) = (0.0, 0.0);
        assert_eq!(openclare_analysis_count(analysis, 0.2, 0.2, &mut quads, &mut below), 1);
        let area = 4.0 * std::f64::consts::PI;
        assert!((quads / (area / 0.04) - 1.0).abs() < 0.03, "{quads} quads");
        assert!((0.0..=1.0).contains(&below));
        let (mut largest, mut smallest) = (0.0, 0.0);
        assert_eq!(openclare_analysis_proposal(analysis, &mut largest, &mut smallest), 1);
        assert!(largest > 0.0 && smallest > 0.0 && smallest <= largest, "{smallest} to {largest}");

        // At each vertex: the size, and whether it is coarser than asked.
        let (mut sizes, mut coarser) = (vec![0f32; vertices], vec![9u8; vertices]);
        assert_eq!(openclare_analysis_vertices(analysis, 0.2, 0.2, sizes.as_mut_ptr(), coarser.as_mut_ptr()), 1);
        assert!(sizes.iter().all(|&size| (size - 0.2).abs() < 1e-5), "{:?}", &sizes[..4]);
        assert!(coarser.iter().all(|&flag| flag <= 1));
        // For a number of quads: those of one size, their area over the
        // number, with no curvature asked to count.
        let mut for_count = vec![0f32; vertices];
        assert_eq!(openclare_analysis_vertices_for(analysis, 1000.0, for_count.as_mut_ptr()), 1);
        let edge = (area / 1000.0).sqrt() as f32;
        assert!(for_count.iter().all(|&size| (size / edge - 1.0).abs() < 0.03), "{:?} for {edge}", &for_count[..4]);
        assert_eq!(openclare_analysis_vertices_for(analysis, 0.0, for_count.as_mut_ptr()), 0);

        // Bounds that are not bounds are refused, and said by the return.
        assert_eq!(openclare_analysis_count(analysis, 0.1, 0.2, &mut quads, &mut below), 0);
        assert_eq!(openclare_analysis_vertices(analysis, 0.2, 0.0, sizes.as_mut_ptr(), coarser.as_mut_ptr()), 0);
        openclare_analysis_free(analysis);

        // A shape that cannot be analysed is said so.
        let none = openclare_analyse(positions.as_ptr(), vertices, triangles.as_ptr(), 0, null(), null(), null());
        assert_eq!(openclare_analysis_ok(none), 0);
        assert!(!text(openclare_analysis_error(none)).is_empty());
        assert_eq!(openclare_analysis_count(none, 0.2, 0.2, &mut quads, &mut below), 0);
        openclare_analysis_free(none);
        openclare_analysis_free(null_mut());
    }
}

#[test]
fn the_hard_edges_a_remesh_would_follow_are_found_without_remeshing() {
    // A cube of six quads, given as Blender gives it: each triangle by its
    // corners, and the vertex of each corner.
    let cube = primitives::cube_quads(1);
    let positions: Vec<f32> = cube.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let corners: Vec<u32> = cube.faces.iter().flatten().copied().collect();
    let triangles: Vec<u32> =
        (0..cube.faces.len() as u32).flat_map(|f| [4 * f, 4 * f + 1, 4 * f + 2, 4 * f, 4 * f + 2, 4 * f + 3]).collect();
    let settings = CString::new("quads = 3000\n").unwrap();
    unsafe {
        let found = openclare_hard_edges(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            corners.as_ptr(),
            corners.len(),
            null(),
            c"organic".as_ptr(),
            settings.as_ptr(),
        );
        assert_eq!(openclare_hard_edges_ok(found), 1, "{}", text(openclare_hard_edges_error(found)));
        assert_eq!(text(openclare_hard_edges_error(found)), "");
        assert_eq!((openclare_hard_edges_kept_count(found), openclare_hard_edges_left_out_count(found)), (12, 0));
        let mut kept = vec![0u32; 24];
        openclare_hard_edges_copy_kept(found, kept.as_mut_ptr());
        let mut edges: Vec<(u32, u32)> =
            kept.chunks_exact(2).map(|edge| (edge[0].min(edge[1]), edge[0].max(edge[1]))).collect();
        edges.sort_unstable();
        edges.dedup();
        assert_eq!(edges.len(), 12);
        assert!(edges.iter().all(|&(a, b)| b < 8 && a != b));
        openclare_hard_edges_copy_left_out(found, null_mut());
        openclare_hard_edges_free(found);

        // Hard edges turned off: nothing either way.
        let off = CString::new("quads = 3000\nedge_angle = none\n").unwrap();
        let found = openclare_hard_edges(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            corners.as_ptr(),
            corners.len(),
            null(),
            c"organic".as_ptr(),
            off.as_ptr(),
        );
        assert_eq!(openclare_hard_edges_ok(found), 1);
        assert_eq!((openclare_hard_edges_kept_count(found), openclare_hard_edges_left_out_count(found)), (0, 0));
        openclare_hard_edges_free(found);

        // What goes wrong is said.
        let found = openclare_hard_edges(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            corners.as_ptr(),
            corners.len(),
            null(),
            c"no such preset".as_ptr(),
            null(),
        );
        assert_eq!(openclare_hard_edges_ok(found), 0);
        assert!(text(openclare_hard_edges_error(found)).contains("no such preset"));
        assert_eq!(openclare_hard_edges_kept_count(found), 0);
        openclare_hard_edges_free(found);
        assert_eq!(openclare_hard_edges_ok(null()), 0);
        assert_eq!(openclare_hard_edges_kept_count(null()), 0);
        openclare_hard_edges_free(null_mut());
    }
}

/// The edges a handle gives, kept then left out, two vertices each.
unsafe fn edges_of(found: *mut OpenclareHardEdges) -> (Vec<u32>, Vec<u32>) {
    unsafe {
        assert_eq!(openclare_hard_edges_ok(found), 1, "{}", text(openclare_hard_edges_error(found)));
        let mut kept = vec![0u32; 2 * openclare_hard_edges_kept_count(found)];
        let mut left_out = vec![0u32; 2 * openclare_hard_edges_left_out_count(found)];
        openclare_hard_edges_copy_kept(found, kept.as_mut_ptr());
        openclare_hard_edges_copy_left_out(found, left_out.as_mut_ptr());
        openclare_hard_edges_free(found);
        (kept, left_out)
    }
}

#[test]
fn a_finder_gives_the_hard_edges_found_afresh_whatever_the_settings_and_their_order() {
    // Two pieces: a cube, and a small ball apart whose vertices are moved in
    // and out at random, a grain of edges sharper than the angle.
    let mut mesh = primitives::cube(2);
    let apart = primitives::icosphere(3);
    let mut random = Rng::new(5);
    let offset = mesh.positions.len() as u32;
    mesh.positions.extend(
        apart.positions.iter().map(|p| *p * 0.5 * (1.0 + 0.05 * (random.next_f64() - 0.5)) + Vec3::new(3.0, 0.0, 0.0)),
    );
    mesh.triangles.extend(apart.triangles.iter().map(|t| t.map(|v| v + offset)));
    let positions: Vec<f32> = mesh.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let triangles: Vec<u32> = mesh.triangles.iter().flatten().copied().collect();
    let density: Vec<f32> = (0..mesh.positions.len()).map(|v| (v % 3) as f32 / 2.0).collect();
    let all = [
        ("quads = 3000\n", false),
        ("quads = 30\n", false),
        ("quads = 3000\nedge_angle = 60\n", false),
        ("quads = 3000\n", true),
        ("quads = 3000\nedge_angle = none\n", false),
        ("quads = 3000\n", false),
        ("quads = 3000\n", true),
    ];
    unsafe {
        let finder = openclare_hard_edge_finder(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
        );
        assert_eq!(openclare_hard_edge_finder_ok(finder), 1);
        assert_eq!(text(openclare_hard_edge_finder_error(finder)), "");
        let mut grain = 0;
        for (settings, painted) in all {
            let lines = CString::new(settings).unwrap();
            let painted = if painted { density.as_ptr() } else { null() };
            let afresh = edges_of(openclare_hard_edges(
                positions.as_ptr(),
                positions.len() / 3,
                triangles.as_ptr(),
                triangles.len() / 3,
                null(),
                0,
                painted,
                c"organic".as_ptr(),
                lines.as_ptr(),
            ));
            let found = edges_of(openclare_hard_edge_finder_find(finder, painted, c"organic".as_ptr(), lines.as_ptr()));
            assert_eq!(found, afresh, "{settings}");
            grain += found.1.len();
        }
        assert!(grain > 0, "no edge left out: the grain is not looked at");
        // What goes wrong is said, and the finder is still good after.
        let found = openclare_hard_edge_finder_find(finder, null(), c"no such preset".as_ptr(), null());
        assert_eq!(openclare_hard_edges_ok(found), 0);
        assert!(text(openclare_hard_edges_error(found)).contains("no such preset"));
        openclare_hard_edges_free(found);
        assert!(!edges_of(openclare_hard_edge_finder_find(finder, null(), c"organic".as_ptr(), null())).0.is_empty());
        openclare_hard_edge_finder_free(finder);

        // A mesh that cannot be read is said so, and finds nothing.
        let broken = [7u32, 0, 1];
        let finder = openclare_hard_edge_finder(positions.as_ptr(), 3, broken.as_ptr(), 1, null(), 0);
        assert_eq!(openclare_hard_edge_finder_ok(finder), 0);
        assert!(text(openclare_hard_edge_finder_error(finder)).contains("vertex 7"));
        let found = openclare_hard_edge_finder_find(finder, null(), null(), null());
        assert_eq!(openclare_hard_edges_ok(found), 0);
        openclare_hard_edges_free(found);
        openclare_hard_edge_finder_free(finder);
        assert_eq!(openclare_hard_edge_finder_ok(null()), 0);
        let found = openclare_hard_edge_finder_find(null_mut(), null(), null(), null());
        assert_eq!(openclare_hard_edges_ok(found), 0);
        openclare_hard_edges_free(found);
        openclare_hard_edge_finder_free(null_mut());
    }
}

#[test]
fn edges_given_are_followed_and_kept_hard_whatever_their_angle() {
    // A smooth torus: no edge sharper than any angle. Its outer equator is a
    // closed loop of its edges, given as the edges a modeller marked sharp.
    let (nu, nv) = (48u32, 24u32);
    let mesh = primitives::torus(1.0, 0.4, nu, nv);
    let positions: Vec<f32> = mesh.positions.iter().flat_map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    let triangles: Vec<u32> = mesh.triangles.iter().flatten().copied().collect();
    let equator: Vec<u32> = (0..nu).flat_map(|i| [i * nv, ((i + 1) % nu) * nv]).collect();
    let settings = CString::new("quads = 800\nedge_angle = none\n").unwrap();
    let hard_edges = |edges: &[u32]| unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            if edges.is_empty() { null() } else { edges.as_ptr() },
            edges.len() / 2,
            null(),
            null(),
            null(),
            0,
            c"organic".as_ptr(),
            settings.as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!(openclare_result_ok(result), 1, "{}", text(openclare_result_error(result)));
        let mut hard = vec![u32::MAX; 2 * openclare_result_hard_edge_count(result)];
        openclare_result_copy_hard_edges(result, hard.as_mut_ptr());
        let mut points = vec![0f32; 3 * openclare_result_vertex_count(result)];
        openclare_result_copy_positions(result, points.as_mut_ptr());
        openclare_result_free(result);
        hard.iter()
            .map(|&v| Vec3::new(points[3 * v as usize] as f64, 0.0, points[3 * v as usize + 2] as f64))
            .collect::<Vec<_>>()
    };
    assert!(hard_edges(&[]).is_empty(), "a smooth torus has no hard edge");
    let along = hard_edges(&equator);
    // Round the whole loop, and on it: z = 0 at the outer radius 1.4.
    assert!(along.len() >= 2 * 20, "{} ends of hard edges", along.len());
    assert!(along.iter().all(|p| p.z.abs() < 1e-3), "a hard edge off the equator");

    // An edge that is none is refused, not followed.
    let wrong = [0u32, positions.len() as u32];
    unsafe {
        let result = openclare_remesh(
            positions.as_ptr(),
            positions.len() / 3,
            triangles.as_ptr(),
            triangles.len() / 3,
            null(),
            0,
            null(),
            null(),
            wrong.as_ptr(),
            1,
            null(),
            null(),
            null(),
            0,
            c"organic".as_ptr(),
            settings.as_ptr(),
            None,
            null_mut(),
        );
        assert_eq!(openclare_result_ok(result), 0);
        assert!(text(openclare_result_error(result)).contains("is not an edge"));
        openclare_result_free(result);
    }
}
