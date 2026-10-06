use retopo::math::Vec3;
use retopo::mesh::{Topology, primitives};
use retopo::metrics::measure;
use retopo::preprocess::{Kind, Request, WorkMesh, create, quality};
use retopo::{PolyMesh, TriMesh};

fn isotropic(source: &TriMesh, triangles: usize, crease_angle: Option<f64>) -> WorkMesh {
    let request = Request { triangles, crease_angle, ..Default::default() };
    create(Kind::Isotropic).preprocess(source, &request).unwrap()
}

fn euler_characteristic(mesh: &TriMesh) -> i64 {
    let topology = Topology::build(mesh);
    mesh.positions.len() as i64 - topology.edge_count() as i64 + mesh.triangles.len() as i64
}

fn assert_closed_manifold(mesh: &TriMesh, euler: i64) {
    let topology = Topology::build(mesh);
    assert!(topology.is_closed(), "holes appeared");
    assert_eq!(topology.non_manifold_edges, 0);
    assert_eq!(euler_characteristic(mesh), euler);
}

/// Two-sided distance to the source, relative to its bounding diagonal.
fn hausdorff(source: &TriMesh, mesh: &TriMesh) -> f64 {
    let polygons =
        PolyMesh { positions: mesh.positions.clone(), faces: mesh.triangles.iter().map(|t| t.to_vec()).collect() };
    measure(source, &polygons, None).hausdorff
}

fn within(count: usize, target: usize, tolerance: f64) -> bool {
    (count as f64 - target as f64).abs() <= tolerance * target as f64
}

#[test]
fn slivers_become_well_shaped_triangles_on_the_same_surface() {
    // 200 x 16 segments: triangles about eight times longer than wide.
    let source = primitives::torus(1.0, 0.4, 16, 200);
    assert!(quality(&source).min_angle_mean < 15.0);

    let work = isotropic(&source, 8000, None);
    let shape = quality(&work.mesh);
    assert!(shape.min_angle_mean > 45.0, "{shape:?}");
    assert!(shape.min_angle_p01 > 25.0, "{shape:?}");
    assert!(shape.edge_ratio < 3.5, "{shape:?}");
    assert!(within(work.mesh.triangles.len(), 8000, 0.3), "{} triangles", work.mesh.triangles.len());

    assert_closed_manifold(&work.mesh, 0);
    assert!(hausdorff(&source, &work.mesh) < 0.01);
    assert!((work.mesh.signed_volume() / source.signed_volume() - 1.0).abs() < 0.02);
}

#[test]
fn a_twelve_triangle_cube_keeps_its_edges_and_corners() {
    let work = isotropic(&primitives::cube(1), 4000, Some(30.0));
    assert_closed_manifold(&work.mesh, 2);
    assert!(quality(&work.mesh).min_angle_p01 > 25.0, "{:?}", quality(&work.mesh));

    // Flat faces and straight creases: the volume is exact, not approximate.
    assert!((work.mesh.signed_volume() - 8.0).abs() < 1e-9);
    let at_unit = |p: &Vec3| p.abs().to_array().iter().filter(|c| (**c - 1.0).abs() < 1e-12).count();
    assert_eq!(work.mesh.positions.iter().filter(|p| at_unit(p) == 3).count(), 8, "corners");
    assert!(work.mesh.positions.iter().all(|p| at_unit(p) >= 1), "a vertex left the surface");

    // The 12 creases are still marked, end to end.
    assert!(work.feature_edges.iter().all(|&[a, b]| {
        at_unit(&work.mesh.positions[a as usize]) >= 2 && at_unit(&work.mesh.positions[b as usize]) >= 2
    }));
    let crease_length: f64 = work
        .feature_edges
        .iter()
        .map(|&[a, b]| work.mesh.positions[a as usize].distance(work.mesh.positions[b as usize]))
        .sum();
    assert!((crease_length - 24.0).abs() < 1e-9, "{crease_length}");
}

#[test]
fn an_open_surface_keeps_its_border() {
    let work = isotropic(&primitives::plane(6), 2000, None);
    assert!((work.mesh.surface_area() - 4.0).abs() < 1e-9);
    assert_eq!(euler_characteristic(&work.mesh), 1);
    let corners = work.mesh.positions.iter().filter(|p| p.x.abs() == 1.0 && p.y.abs() == 1.0).count();
    assert_eq!(corners, 4);
    assert!(work.mesh.positions.iter().all(|p| p.z == 0.0));
}

#[test]
fn a_dense_input_is_decimated_then_remeshed() {
    // 81 920 triangles for a request of 2 000: far above the decimation ceiling.
    let source = primitives::icosphere(6);
    let work = isotropic(&source, 2000, None);
    assert!(within(work.mesh.triangles.len(), 2000, 0.3), "{} triangles", work.mesh.triangles.len());
    assert_closed_manifold(&work.mesh, 2);
    assert!(quality(&work.mesh).min_angle_p01 > 25.0);
    assert!(work.mesh.positions.iter().all(|p| (p.length() - 1.0).abs() < 0.005));
}

#[test]
fn genus_is_preserved() {
    // Decimation and remeshing both collapse edges around a thin handle.
    let source = primitives::torus(1.0, 0.15, 256, 64);
    let work = isotropic(&source, 1500, None);
    assert_closed_manifold(&work.mesh, 0);
    assert!(hausdorff(&source, &work.mesh) < 0.02);
}

#[test]
fn curvature_is_right_on_the_remeshed_surface() {
    let (major, minor) = (1.0, 0.4);
    let work = isotropic(&primitives::torus(major, minor, 96, 48), 12000, None);
    let n = work.mesh.positions.len();
    let mut accurate = 0;
    for (v, p) in work.mesh.positions.iter().enumerate() {
        let ring_tangent = Vec3::Z.cross(Vec3::new(p.x, p.y, 0.0).normalize());
        let magnitude_ok = (work.curvature.k1[v] - 1.0 / minor).abs() < 0.1 / minor;
        let direction_ok = work.curvature.dir1[v].dot(ring_tangent).abs() < 0.15;
        accurate += (magnitude_ok && direction_ok) as usize;
    }
    assert!(accurate as f64 > 0.97 * n as f64, "{accurate} of {n} vertices");

    // The tube bends far more than the ring: directions are trustworthy
    // nearly everywhere, except on the two circles where the ring curvature
    // changes sign... and there they are still well defined.
    let confident = work.confidence.iter().filter(|&&c| c > 0.4).count();
    assert!(confident as f64 > 0.9 * n as f64);

    // On a sphere no direction stands out.
    let sphere = isotropic(&primitives::icosphere(4), 6000, None);
    let mean = sphere.confidence.iter().sum::<f64>() / sphere.confidence.len() as f64;
    assert!(mean < 0.1, "mean confidence on a sphere: {mean}");
}

#[test]
fn split_vertices_are_welded() {
    // The same cube as a triangle soup: every triangle has its own vertices.
    let cube = primitives::cube(3);
    let soup = TriMesh {
        positions: cube.triangles.iter().flatten().map(|&v| cube.positions[v as usize]).collect(),
        triangles: (0..cube.triangles.len() as u32).map(|t| [3 * t, 3 * t + 1, 3 * t + 2]).collect(),
    };
    assert!(!Topology::build(&soup).is_closed());

    let work = create(Kind::Passthrough).preprocess(&soup, &Request::default()).unwrap();
    assert_eq!(work.mesh.positions.len(), cube.positions.len());
    assert_eq!(work.mesh.triangles.len(), cube.triangles.len());
    assert_closed_manifold(&work.mesh, 2);
}

#[test]
fn every_preprocessor_is_reachable_by_name_and_deterministic() {
    let source = primitives::torus(1.0, 0.4, 48, 24);
    let request = Request { triangles: 3000, ..Default::default() };
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        let preprocessor = create(kind);
        assert_eq!(preprocessor.kind(), kind);
        let (first, second) =
            (preprocessor.preprocess(&source, &request).unwrap(), preprocessor.preprocess(&source, &request).unwrap());
        assert_eq!(first.mesh.triangles, second.mesh.triangles);
        assert_eq!(first.mesh.positions, second.mesh.positions);
    }
    assert!("nope".parse::<Kind>().is_err());

    let empty = TriMesh::default();
    assert!(create(Kind::Isotropic).preprocess(&empty, &request).is_err());
    let none = Request { triangles: 0, ..Default::default() };
    assert!(create(Kind::Isotropic).preprocess(&source, &none).is_err());
}

/// Not part of the regular suite (a few seconds in release, far more in
/// debug): `cargo test --release -- --ignored --nocapture`.
#[test]
#[ignore = "performance check on a 1.3 million triangle input"]
fn a_million_triangles_are_brought_down_to_a_work_mesh() {
    let source = primitives::icosphere(8);
    assert_eq!(source.triangles.len(), 1_310_720);
    let start = std::time::Instant::now();
    let work = isotropic(&source, 100_000, None);
    println!(
        "{} -> {} triangles in {:.1} s",
        source.triangles.len(),
        work.mesh.triangles.len(),
        start.elapsed().as_secs_f64()
    );
    assert!(within(work.mesh.triangles.len(), 100_000, 0.3));
    assert_closed_manifold(&work.mesh, 2);
    assert!(quality(&work.mesh).min_angle_p01 > 25.0);
    assert!(work.mesh.positions.iter().all(|p| (p.length() - 1.0).abs() < 1e-3));
}
