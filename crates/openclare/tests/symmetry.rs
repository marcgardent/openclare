//! Symmetry: a model the same on both sides of a plane through its origin
//! gets a quad mesh the same on both sides of it, closed where the model
//! is, the quads along the plane as good as the others (DT-028).

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::symmetry::{self, Axis};
use openclare::{RemeshConfig, Target, TriMesh};

fn config(quads: usize, axis: Axis) -> RemeshConfig {
    RemeshConfig { symmetry: Some(axis), ..RemeshConfig::for_organic(Target::QuadCount(quads)) }
}

fn diagonal(mesh: &TriMesh) -> f64 {
    let (low, high) =
        mesh.positions.iter().fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    (high - low).length()
}

/// Remeshes `source` symmetric across `axis` and checks the result: a
/// valid quad mesh, closed as the source is, its own mirror image, and near
/// the number of quads asked for.
fn symmetric(source: &TriMesh, axis: Axis, quads: usize) {
    let (mesh, report) = config(quads, axis).remesh(source).unwrap();
    let metrics = measure(source, &mesh, Some(quads));
    assert!(metrics.is_valid(), "{axis}: {metrics:?}");
    assert_eq!(metrics.boundary_edges, 0, "{axis}: the halves are not sewn together");
    let off = symmetry::asymmetry(&mesh, axis);
    assert!(off <= 1e-9 * diagonal(source), "{axis}: a vertex has no image, {off} away");
    assert_eq!(report.off_the_plane, 0, "{axis}: the half is cut across its quads");
    // The whole model is meshed for that number; its positive side need not
    // hold exactly half.
    assert!(metrics.budget_error.unwrap() <= 0.08, "{axis}: {} faces for {quads}", metrics.faces);
}

#[test]
fn primitives_come_out_symmetric_across_each_axis() {
    let shapes = [
        ("sphere", primitives::icosphere(4)),
        ("torus", primitives::torus(1.0, 0.35, 64, 24)),
        ("cube", primitives::cube(16)),
        ("cylinder", primitives::cylinder(32)),
    ];
    for (name, shape) in &shapes {
        for &axis in Axis::ALL {
            eprintln!("{name} across {axis}");
            symmetric(shape, axis, 1000);
        }
    }
}

#[test]
fn the_plane_is_drawn_into_the_model_as_a_ring_of_edges() {
    let cube = primitives::cube(5);
    let symmetry::Split { mesh: drawn, lines, .. } = symmetry::split(&cube, Axis::X, 0.0);
    // Nothing added nor taken away: the same surface, cut into more pieces.
    let ratio = drawn.surface_area() / cube.surface_area();
    assert!((ratio - 1.0).abs() < 1e-9, "{ratio}");
    // The plane meets the cube in a square of side 2: its edges are drawn.
    let length: f64 = lines.iter().map(|[a, b]| a.distance(*b)).sum();
    assert!((length - 8.0).abs() < 1e-9, "{length}");
    assert!(lines.iter().flatten().all(|p| p.x == 0.0));
    // And each is an edge of the drawn mesh.
    let edges: std::collections::HashSet<(u32, u32)> = drawn
        .triangles
        .iter()
        .flat_map(|tri| (0..3).map(move |k| (tri[k].min(tri[(k + 1) % 3]), tri[k].max(tri[(k + 1) % 3]))))
        .collect();
    let at = |p: Vec3| drawn.positions.iter().position(|&q| q == p).unwrap() as u32;
    assert!(lines.iter().all(|&[a, b]| edges.contains(&(at(a).min(at(b)), at(a).max(at(b))))));
}

#[test]
fn a_model_on_one_side_of_the_plane_is_refused() {
    let mut sphere = primitives::icosphere(3);
    for p in &mut sphere.positions {
        p.x += 5.0;
    }
    assert!(config(500, Axis::X).remesh(&sphere).is_err());
}

#[test]
fn a_model_not_the_same_on_both_sides_is_refused() {
    // A ball with a bump on its negative side only.
    let mut sphere = primitives::icosphere(3);
    for p in &mut sphere.positions {
        if p.x < -0.5 && p.y.abs() < 0.4 && p.z.abs() < 0.4 {
            p.x -= 0.2;
        }
    }
    let error = config(500, Axis::X).remesh(&sphere).unwrap_err().to_string();
    assert!(error.contains("not the same on both sides"), "{error}");
    // Across the other axes it is the same on both sides.
    assert!(config(500, Axis::Y).remesh(&sphere).is_ok());
}

#[test]
fn hard_edges_and_creases_are_said_of_the_mirrored_mesh() {
    let cube = primitives::cube(16);
    for &axis in Axis::ALL {
        let config = RemeshConfig { symmetry: Some(axis), ..RemeshConfig::for_hard_surface(Target::QuadCount(600)) };
        let (mesh, report) = config.remesh(&cube).unwrap();
        let edges: std::collections::HashSet<(u32, u32)> = mesh
            .faces
            .iter()
            .flat_map(|face| (0..face.len()).map(move |k| (face[k], face[(k + 1) % face.len()])))
            .map(|(a, b)| (a.min(b), a.max(b)))
            .collect();
        let key = |[a, b]: [u32; 2]| (a.min(b), a.max(b));
        let hard: std::collections::HashSet<(u32, u32)> = report.hard_edges.iter().map(|&e| key(e)).collect();
        assert!(hard.len() >= 12, "{axis}: {} hard edges", hard.len());
        assert_eq!(hard.len(), report.hard_edges.len(), "{axis}: an edge said twice");
        assert!(hard.iter().all(|e| edges.contains(e)), "{axis}: a hard edge that is not an edge of the mesh");
        // Along the edges of the cube, and each with its mirror image.
        let corner = |v: u32| mesh.positions[v as usize];
        let on_the_cube_edges = |p: Vec3| (0..3).filter(|&k| (p[k].abs() - 1.0).abs() < 1e-4).count() >= 2;
        assert!(hard.iter().all(|&(a, b)| on_the_cube_edges(corner(a)) && on_the_cube_edges(corner(b))), "{axis}");
        let at = |p: Vec3| mesh.positions.iter().position(|&q| q.distance(p) < 1e-9).unwrap() as u32;
        let image = |v: u32| at(axis.mirror(corner(v)));
        assert!(hard.iter().all(|&(a, b)| hard.contains(&key([image(a), image(b)]))), "{axis}: no image");
        assert!(report.creases.iter().all(|&(e, _)| edges.contains(&key(e))), "{axis}: a crease off the mesh");
    }
}

/// Remeshes `source` symmetric across `axis` with `guides`, and returns the
/// mesh and its report, checked to be its own mirror image.
fn symmetric_with(
    source: &TriMesh,
    axis: Axis,
    config: RemeshConfig,
    guides: &openclare::Guides,
) -> (openclare::PolyMesh, openclare::pipeline::Report) {
    let config = RemeshConfig { symmetry: Some(axis), ..config };
    let (mesh, report) = config.remesh_guided(source, guides).unwrap();
    let off = symmetry::asymmetry(&mesh, axis);
    assert!(off <= 1e-9 * diagonal(source), "{axis}: a vertex has no image, {off} away");
    (mesh, report)
}

#[test]
fn a_painted_density_is_kept_through_the_cut() {
    // Painted at 1 on the top of a ball, 0 elsewhere: across x, the top is
    // on both sides of the plane, and stays the denser part.
    let ball = primitives::icosphere(4);
    let weights: Vec<f64> = ball.positions.iter().map(|p| if p.z > 0.3 { 1.0 } else { 0.0 }).collect();
    let guides = openclare::Guides { density: Some(&weights), ..Default::default() };
    let config = RemeshConfig { density_contrast: 9.0, ..RemeshConfig::for_organic(Target::QuadCount(800)) };
    let (mesh, _) = symmetric_with(&ball, Axis::X, config, &guides);
    let (top, bottom) = mesh.faces.iter().fold((0usize, 0usize), |(top, bottom), face| {
        let z = face.iter().map(|&v| mesh.positions[v as usize].z).sum::<f64>() / face.len() as f64;
        if z > 0.5 {
            (top + 1, bottom)
        } else if z < -0.5 {
            (top, bottom + 1)
        } else {
            (top, bottom)
        }
    });
    assert!(top > 3 * bottom, "{top} faces on top against {bottom} below");
}

#[test]
fn groups_are_kept_through_the_cut_and_mirrored() {
    // A ball whose upper half is of a second group.
    let ball = primitives::icosphere(4);
    let groups: Vec<u32> = ball
        .triangles
        .iter()
        .map(|tri| (tri.iter().map(|&v| ball.positions[v as usize].z).sum::<f64>() > 0.0) as u32)
        .collect();
    let guides = openclare::Guides { groups: Some(&groups), ..Default::default() };
    let (mesh, report) = symmetric_with(&ball, Axis::X, RemeshConfig::for_organic(Target::QuadCount(800)), &guides);
    assert_eq!(report.face_groups.len(), mesh.faces.len());
    for (face, &group) in mesh.faces.iter().zip(&report.face_groups) {
        let z = face.iter().map(|&v| mesh.positions[v as usize].z).sum::<f64>() / face.len() as f64;
        if z.abs() > 0.2 {
            assert_eq!(group, (z > 0.0) as u32, "a face at z = {z:.2} in group {group}");
        }
    }
}

#[test]
fn sizes_asked_for_come_out_symmetric_too() {
    let cube = primitives::cube(16);
    let config = RemeshConfig::for_organic(Target::SizeRange { largest: 0.2, smallest: 0.05 });
    let (mesh, report) = symmetric_with(&cube, Axis::Y, config, &openclare::Guides::default());
    assert!(mesh.faces.len() > 100, "{} faces", mesh.faces.len());
    assert_eq!(report.off_the_plane, 0);
}

#[test]
fn symmetry_is_written_and_read_back() {
    let config = RemeshConfig { symmetry: Some(Axis::Z), ..RemeshConfig::default() };
    let text = config.to_text();
    assert!(text.contains("symmetry = z"), "{text}");
    assert_eq!(RemeshConfig::from_text(&text).unwrap().symmetry, Some(Axis::Z));
    assert_eq!(RemeshConfig::from_text("symmetry = none").unwrap().symmetry, None);
    assert!(RemeshConfig::from_text("symmetry = w").is_err());
}

#[test]
fn a_hole_on_one_side_only_is_seen_though_its_corners_mirror_onto_a_face() {
    // A slab of two big faces, with the wall of a square hole through it on
    // the positive side only. Every corner of the wall, mirrored, lands on
    // one of the two faces; the middle of the wall lands inside the slab.
    let mut positions = vec![];
    let mut triangles = vec![];
    for z in [0.0, 0.2] {
        let base = positions.len() as u32;
        positions.extend([
            Vec3::new(-1.0, -1.0, z),
            Vec3::new(1.0, -1.0, z),
            Vec3::new(1.0, 1.0, z),
            Vec3::new(-1.0, 1.0, z),
        ]);
        triangles.extend([[base, base + 1, base + 2], [base, base + 2, base + 3]]);
    }
    let square = [(0.4, -0.1), (0.6, -0.1), (0.6, 0.1), (0.4, 0.1)];
    let base = positions.len() as u32;
    for z in [0.0, 0.2] {
        positions.extend(square.iter().map(|&(x, y)| Vec3::new(x, y, z)));
    }
    for k in 0..4u32 {
        let (a, b) = (base + k, base + (k + 1) % 4);
        triangles.extend([[a, b, b + 4], [a, b + 4, a + 4]]);
    }
    let slab = TriMesh { positions, triangles };
    assert!(symmetry::surface_asymmetry(&slab, Axis::X) > 0.05, "the hole went unseen");
    assert!(symmetry::surface_asymmetry(&slab, Axis::Y) < 1e-9, "across y the slab and its hole are the same");
}
