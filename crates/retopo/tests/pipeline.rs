use retopo::field::{Constraints, orientation};
use retopo::geometry::{vertex_areas, vertex_normals};
use retopo::hierarchy::{Graph, Hierarchy, Level};
use retopo::math::{Rng, Vec3};
use retopo::mesh::subdivide::split_long_edges;
use retopo::mesh::{Topology, primitives};
use retopo::{Params, PolyMesh, Target, TriMesh, remesh};

fn euler_characteristic(mesh: &TriMesh) -> i64 {
    let topology = Topology::build(mesh);
    mesh.positions.len() as i64 - topology.edge_count() as i64 + mesh.triangles.len() as i64
}

/// Sum of the singularity indices, in quarter turns, of a freshly solved
/// orientation field.
fn total_index(mesh: &TriMesh, seed: u64) -> i64 {
    let topology = Topology::build(mesh);
    let level = Level::new(
        mesh.positions.clone(),
        vertex_normals(mesh),
        vertex_areas(mesh),
        Graph::uniform(&topology.neighbors),
        Constraints::none(mesh.positions.len()),
    );
    let mut hierarchy = Hierarchy::build(level);
    orientation::solve(&mut hierarchy, 6, &mut Rng::new(seed));
    let finest = hierarchy.finest();
    orientation::singularities(mesh, &finest.normals, &finest.orientation).iter().map(|s| s.index as i64).sum()
}

fn params(quads: usize) -> Params {
    Params { target: Target::QuadCount(quads), ..Default::default() }
}

fn assert_closed_quad_mesh(mesh: &PolyMesh, euler: i64) {
    let stats = mesh.stats();
    assert_eq!(stats.quads, stats.faces, "not a pure quad mesh: {stats:?}");
    assert_eq!(stats.boundary_edges, 0, "mesh has holes: {stats:?}");
    assert_eq!(stats.non_manifold_edges, 0);
    assert_eq!(stats.euler_characteristic(), euler);
}

#[test]
fn primitives_are_closed_and_wound_outward() {
    for (mesh, euler) in
        [(primitives::torus(1.0, 0.4, 32, 16), 0), (primitives::icosphere(2), 2), (primitives::cube(4), 2)]
    {
        let topology = Topology::build(&mesh);
        assert!(topology.is_closed());
        assert_eq!(topology.non_manifold_edges, 0);
        assert_eq!(euler_characteristic(&mesh), euler);
        assert!(mesh.signed_volume() > 0.0);
    }
}

#[test]
fn subdivision_bounds_edge_length_and_keeps_the_surface() {
    let mesh = primitives::cube(2);
    let fine = split_long_edges(&mesh, 0.3);
    assert!(fine.edge_length_stats().2 <= 0.3);
    assert!((fine.surface_area() - mesh.surface_area()).abs() < 1e-9);
    let topology = Topology::build(&fine);
    assert!(topology.is_closed());
    assert_eq!(topology.non_manifold_edges, 0);
}

#[test]
fn orientation_field_obeys_poincare_hopf() {
    // Whatever the solver converges to, the indices must add up to 4 χ.
    for seed in 1..=3 {
        assert_eq!(total_index(&primitives::icosphere(3), seed), 8);
        assert_eq!(total_index(&primitives::torus(1.0, 0.4, 48, 24), seed), 0);
    }
}

#[test]
fn sphere_becomes_a_closed_quad_mesh_on_the_surface() {
    let (mesh, report) = remesh(&primitives::icosphere(4), &params(1000)).unwrap();
    assert_closed_quad_mesh(&mesh, 2);
    let faces = mesh.faces.len() as f64;
    assert!((750.0..1250.0).contains(&faces), "{faces} quads for a target of 1000");
    assert_eq!(report.singularities.0 as i64 - report.singularities.1 as i64, 8);

    for p in &mesh.positions {
        assert!((p.length() - 1.0).abs() < 0.01, "vertex off the surface: {p}");
    }
    // Faces keep the outward winding of the input.
    for face in &mesh.faces {
        let [a, b, c, d] = [0, 1, 2, 3].map(|k| mesh.positions[face[k] as usize]);
        assert!((c - a).cross(d - b).dot(a + b + c + d) > 0.0);
    }
}

#[test]
fn torus_keeps_its_genus() {
    let (mesh, _) = remesh(&primitives::torus(1.0, 0.4, 96, 48), &params(1500)).unwrap();
    assert_closed_quad_mesh(&mesh, 0);
}

#[test]
fn plane_becomes_a_regular_grid_up_to_its_border() {
    let (mesh, report) = remesh(&primitives::plane(24), &params(400)).unwrap();
    let stats = &report.stats;
    assert_eq!(stats.quads, stats.faces);
    assert_eq!(stats.irregular_vertices, 0);
    assert_eq!(report.singularities, (0, 0));
    // One boundary loop, and it is the border of the input square.
    assert_eq!(stats.euler_characteristic(), 1);
    let (min, max) =
        mesh.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    assert!(min.distance(Vec3::new(-1.0, -1.0, 0.0)) < 0.02, "{min}");
    assert!(max.distance(Vec3::new(1.0, 1.0, 0.0)) < 0.02, "{max}");
}

#[test]
fn creases_are_followed() {
    let source = primitives::cube(16);
    let options = Params { crease_angle: Some(30.0), ..params(600) };
    let (mesh, report) = remesh(&source, &options).unwrap();
    assert_closed_quad_mesh(&mesh, 2);
    assert!(report.feature_edges >= 12 * 16);

    // Hard edges survive: the 8 corners are output vertices, and whole rows
    // of vertices sit exactly on the 12 edges (two coordinates at +-1).
    let sharp = |p: &Vec3| p.abs().to_array().iter().filter(|c| (**c - 1.0).abs() < 1e-9).count();
    assert_eq!(mesh.positions.iter().filter(|p| sharp(p) == 3).count(), 8);
    assert!(mesh.positions.iter().filter(|p| sharp(p) == 2).count() >= 12 * 6);
}

#[test]
fn quad_dominant_output_is_mostly_quads() {
    let options = Params { pure_quad: false, ..params(1000) };
    let (mesh, report) = remesh(&primitives::icosphere(4), &options).unwrap();
    assert!(report.stats.quad_ratio() > 0.85, "{:?}", report.stats);
    assert_eq!(report.stats.boundary_edges, 0);
    assert_eq!(mesh.stats().euler_characteristic(), 2);
}

#[test]
fn remeshing_is_deterministic() {
    let source = primitives::torus(1.0, 0.4, 48, 24);
    let (a, _) = remesh(&source, &params(500)).unwrap();
    let (b, _) = remesh(&source, &params(500)).unwrap();
    assert_eq!(a.faces, b.faces);
    assert_eq!(a.positions, b.positions);
}

#[test]
fn bad_input_is_rejected() {
    assert!(remesh(&TriMesh::default(), &Params::default()).is_err());
    let degenerate = TriMesh { positions: vec![Vec3::ZERO; 3], triangles: vec![[0, 1, 2]] };
    assert!(remesh(&degenerate, &Params::default()).is_err());
    let zero = Params { target: Target::QuadCount(0), ..Default::default() };
    assert!(remesh(&primitives::icosphere(1), &zero).is_err());
}
