//! The C interface, called the way a host calls it.

use std::ffi::{CStr, CString, c_char, c_void};
use std::ptr::{null, null_mut};

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
        openclare_result_free(result);

        assert_eq!(openclare_result_seam_count(null()), 0);
        assert_eq!(openclare_result_hard_edge_count(null()), 0);
        assert_eq!(openclare_result_copy_uv(null(), OPENCLARE_UV_GRID, grid.as_mut_ptr()), 0);
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
