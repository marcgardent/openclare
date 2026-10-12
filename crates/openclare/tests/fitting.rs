//! Fitting: the mesh keeps the volume of the shape instead of being
//! inscribed in it.

use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::{RemeshConfig, Target, TriMesh};

fn remesh(source: &TriMesh, quads: usize, volume_fit: f64) -> (openclare::PolyMesh, openclare::metrics::Metrics) {
    let config = RemeshConfig { adaptivity: 0.0, volume_fit, ..RemeshConfig::for_organic(Target::QuadCount(quads)) };
    let (mesh, _) = config.remesh(source).unwrap();
    let metrics = measure(source, &mesh, Some(quads));
    (mesh, metrics)
}

#[test]
fn with_its_vertices_on_the_surface_a_mesh_is_smaller_than_the_shape() {
    let source = primitives::icosphere(5);
    let (_, inscribed) = remesh(&source, 300, 0.0);
    let kept = inscribed.volume_ratio.unwrap();
    assert!(kept < 0.985, "volume kept: {kept}");
}

#[test]
fn a_fitted_mesh_keeps_the_volume() {
    for (name, source, quads) in
        [("ball", primitives::icosphere(5), 300), ("ring", primitives::torus(1.0, 0.4, 96, 48), 400)]
    {
        let (on_surface, inscribed) = remesh(&source, quads, 0.0);
        let (fitted, fit) = remesh(&source, quads, 1.0);
        assert!(fit.is_valid(), "{name}");
        // The same mesh, its vertices moved.
        assert_eq!(fitted.faces, on_surface.faces, "{name}");
        let (before, after) = (inscribed.volume_ratio.unwrap(), fit.volume_ratio.unwrap());
        assert!((after - 1.0).abs() < 0.004, "{name}: volume kept {after} (it was {before})");
        assert!((after - 1.0).abs() < 0.3 * (before - 1.0).abs(), "{name}: volume kept {after} (it was {before})");
        // No farther from the shape for that: closer on the whole.
        assert!(
            fit.mean_distance < inscribed.mean_distance,
            "{name}: {} then {}",
            inscribed.mean_distance,
            fit.mean_distance
        );
        assert!(fit.hausdorff < 1.05 * inscribed.hausdorff, "{name}: {} then {}", inscribed.hausdorff, fit.hausdorff);
    }
}

#[test]
fn half_way_is_half_way() {
    let source = primitives::icosphere(5);
    let (on_surface, _) = remesh(&source, 300, 0.0);
    let (half, _) = remesh(&source, 300, 0.5);
    let (fitted, _) = remesh(&source, 300, 1.0);
    for v in 0..on_surface.positions.len() {
        let middle = 0.5 * (on_surface.positions[v] + fitted.positions[v]);
        assert!(half.positions[v].distance(middle) < 1e-12);
    }
}

#[test]
fn flat_faces_and_hard_edges_are_left_where_they_are() {
    let source = primitives::cube(8);
    let base = RemeshConfig::for_hard_surface(Target::QuadCount(600));
    assert_eq!(base.volume_fit, 1.0);
    let (fitted, _) = base.remesh(&source).unwrap();
    let (exact, _) = RemeshConfig { volume_fit: 0.0, ..base }.remesh(&source).unwrap();
    assert_eq!(fitted.faces, exact.faces);
    for v in 0..exact.positions.len() {
        assert!(fitted.positions[v].distance(exact.positions[v]) < 1e-9);
    }
}

// The fitter on its own, on meshes whose answer is known.

use openclare::fitting::{Fitter, LeastSquares};
use openclare::math::Vec3;
use openclare::spatial::Bvh;
use openclare::PolyMesh;

/// The cube of side 2 in quads, four to a side, scaled along each axis.
fn block(scale: Vec3) -> PolyMesh {
    let mut mesh = primitives::cube_quads(4);
    for p in &mut mesh.positions {
        *p = Vec3::new(p.x * scale.x, p.y * scale.y, p.z * scale.z);
    }
    mesh
}

/// The same block in triangles, finer, scaled and moved.
fn solid(scale: Vec3, moved: Vec3) -> TriMesh {
    let mut mesh = primitives::cube(8);
    for p in &mut mesh.positions {
        *p = Vec3::new(p.x * scale.x, p.y * scale.y, p.z * scale.z) + moved;
    }
    mesh
}

/// The vertex of `mesh` nearest to `p`.
fn vertex_at(mesh: &PolyMesh, p: Vec3) -> usize {
    let nearest = (0..mesh.positions.len()).min_by(|&a, &b| {
        mesh.positions[a].distance(p).partial_cmp(&mesh.positions[b].distance(p)).unwrap()
    });
    let v = nearest.unwrap();
    assert!(mesh.positions[v].distance(p) < 1e-9, "no vertex at {p}");
    v
}

#[test]
fn a_vertex_goes_no_farther_than_its_leash() {
    // A cube of side 2 inside one of side 2.6: every face is 0.3 under the
    // surface, more than a vertex may move, which is a quarter of its
    // shortest edge: 0.125.
    let start = block(Vec3::ONE);
    let source = solid(Vec3::splat(1.3), Vec3::ZERO);
    let surface = Bvh::build(&source);
    let fitter = LeastSquares::default();
    assert_eq!(fitter.reach, 0.25);
    let mut fitted = start.clone();
    fitter.fit(&mut fitted, &source, &surface, None, 1.0);
    for v in 0..start.positions.len() {
        let moved = fitted.positions[v].distance(start.positions[v]);
        assert!(moved <= 0.125 + 1e-12, "vertex {v} moved by {moved}");
    }
    // In the middle of a face, straight out and all the leash allows.
    for axis in 0..3 {
        for side in [-1.0, 1.0] {
            let mut middle = Vec3::ZERO;
            middle[axis] = side;
            let v = vertex_at(&start, middle);
            let mut expected = middle;
            expected[axis] = side * 1.125;
            assert!(fitted.positions[v].distance(expected) < 1e-9, "{} for {expected}", fitted.positions[v]);
        }
    }

    // Half the way is half the move, and none of it is none.
    let mut half = start.clone();
    fitter.fit(&mut half, &source, &surface, None, 0.5);
    let v = vertex_at(&start, Vec3::Z);
    assert!((half.positions[v].z - 1.0625).abs() < 1e-9, "{}", half.positions[v]);
    // No way at all, less than none or no number: the mesh as it was.
    for amount in [0.0, -1.0, f64::NAN] {
        let mut none = start.clone();
        fitter.fit(&mut none, &source, &surface, None, amount);
        assert_eq!(none.positions, start.positions, "amount {amount}");
    }
    let mut empty = PolyMesh::default();
    fitter.fit(&mut empty, &source, &surface, None, 1.0);
    assert!(empty.positions.is_empty());
}

#[test]
fn on_a_thin_part_a_face_is_fitted_to_the_sheet_that_faces_its_way() {
    // A slab 0.1 thick, and a source 0.15 thick moved up: the top of the
    // source is 0.1 above the top of the slab, its underside 0.05 under it,
    // nearer. The top goes up to the top all the same; the underside of the
    // slab goes up too, to the underside of the source.
    let start = block(Vec3::new(1.0, 1.0, 0.05));
    let source = solid(Vec3::new(1.0, 1.0, 0.075), Vec3::new(0.0, 0.0, 0.075));
    let surface = Bvh::build(&source);
    let mut fitted = start.clone();
    LeastSquares::default().fit(&mut fitted, &source, &surface, None, 1.0);
    let top = vertex_at(&start, Vec3::new(0.0, 0.0, 0.05));
    let underside = vertex_at(&start, Vec3::new(0.0, 0.0, -0.05));
    // Not all the way: the rim, held short by the thin edges it is on,
    // weighs on its neighbours. But up, both of them, by more than half of
    // what there is to go, when the nearest sheet is down for the top.
    let (top, underside) = (fitted.positions[top].z, fitted.positions[underside].z);
    assert!(top > 0.05 + 0.05 && top < 0.15 + 1e-9, "top at {top}");
    assert!(underside > -0.05 + 0.025 && underside < 1e-9, "underside at {underside}");
}

#[test]
fn a_face_that_would_turn_too_much_is_put_back() {
    // The same slab and source. Round the rim the vertices are held short
    // by the thin edges they are on, and the faces next to them lean by
    // some 8 degrees. A fitter that allows 1 degree gives all of it up.
    let start = block(Vec3::new(1.0, 1.0, 0.05));
    let source = solid(Vec3::new(1.0, 1.0, 0.075), Vec3::new(0.0, 0.0, 0.075));
    let surface = Bvh::build(&source);
    let facing = |mesh: &PolyMesh, face: &[u32]| {
        let at = |k: usize| mesh.positions[face[k] as usize];
        (at(2) - at(0)).cross(at(3) - at(1)).normalize()
    };
    let most_turned = |mesh: &PolyMesh| {
        start.faces.iter().map(|face| facing(&start, face).dot(facing(mesh, face)).clamp(-1.0, 1.0).acos()).fold(0.0, f64::max)
    };

    let mut loose = start.clone();
    LeastSquares::default().fit(&mut loose, &source, &surface, None, 1.0);
    let leaning = most_turned(&loose).to_degrees();
    assert!(leaning > 5.0 && leaning < 20.0, "faces lean by up to {leaning} degrees");

    let mut strict = start.clone();
    LeastSquares { tilt: 1f64.to_radians(), ..Default::default() }.fit(&mut strict, &source, &surface, None, 1.0);
    let leaning = most_turned(&strict).to_degrees();
    assert!(leaning <= 1.0 + 1e-9, "faces lean by up to {leaning} degrees");
}

#[test]
fn the_border_of_an_open_sheet_stays_where_it_is() {
    // A sheet of side 2, eight quads a side, 0.05 under a plane.
    let start = primitives::plane_quads(8);
    let mut source = primitives::plane(16);
    for p in &mut source.positions {
        (p.x, p.y, p.z) = (2.0 * p.x, 2.0 * p.y, 0.05);
    }
    let surface = Bvh::build(&source);
    let mut fitted = start.clone();
    LeastSquares::default().fit(&mut fitted, &source, &surface, None, 1.0);
    let (mut held, mut moved) = (0, 0);
    for (v, p) in start.positions.iter().enumerate() {
        if p.x.abs() == 1.0 || p.y.abs() == 1.0 {
            assert_eq!(fitted.positions[v], *p);
            held += 1;
        } else {
            assert!(fitted.positions[v].z > 0.04 && fitted.positions[v].z < 0.0625 + 1e-12, "{}", fitted.positions[v]);
            moved += 1;
        }
    }
    assert_eq!((held, moved), (32, 49));
}
