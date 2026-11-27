//! Guide curves: lines drawn on the surface that the quad mesh has edges
//! along.

use std::collections::HashMap;

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::preprocess::{self, Kind};
use openclare::{GuideCurve, Guides, PolyMesh, RemeshConfig, Report, Target, TriMesh};

fn remesh(source: &TriMesh, quads: usize, curves: &[GuideCurve]) -> (PolyMesh, Report) {
    let config = RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(quads)) };
    config.remesh_guided(source, &Guides { curves, ..Default::default() }).unwrap()
}

/// The vertices of `edges`, each with the vertices it is joined to.
fn joined(edges: &[[u32; 2]]) -> HashMap<u32, Vec<u32>> {
    let mut joined: HashMap<u32, Vec<u32>> = HashMap::new();
    for &[a, b] in edges {
        joined.entry(a).or_default().push(b);
        joined.entry(b).or_default().push(a);
    }
    joined
}

/// A circle of latitude on the unit sphere, at height `z`.
fn parallel(z: f64) -> GuideCurve {
    let radius = (1.0 - z * z).sqrt();
    let points = (0..48)
        .map(|k| {
            let turn = std::f64::consts::TAU * k as f64 / 48.0;
            Vec3::new(radius * turn.cos(), radius * turn.sin(), z)
        })
        .collect();
    GuideCurve { points, closed: true, sharp: false }
}

#[test]
fn a_curve_is_laid_into_the_work_mesh_as_one_unbroken_line() {
    let source = primitives::icosphere(5);
    let curves = [parallel(0.3)];
    let request = preprocess::Request { triangles: 8000, curves: &curves, ..Default::default() };
    let work = preprocess::create(Kind::Isotropic).preprocess(&source, &request).unwrap();
    assert_eq!(work.guide_gaps, 0);
    assert!(!work.guide_lines.is_empty());
    // A closed line: every vertex on it has two neighbours on it, and they
    // all sit on the circle.
    let line = joined(&work.feature_edges);
    assert!(line.len() > 40, "{} vertices on the line", line.len());
    for (&v, neighbours) in &line {
        let p = work.mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - 0.3).abs() < 2e-3 && (p.length() - 1.0).abs() < 2e-3, "vertex {v} at {p:?}");
    }
    // And the mesh is still a closed surface.
    assert_eq!(work.mesh.pieces().len(), 1);
    work.mesh.validate().unwrap();
}

#[test]
fn the_quad_mesh_has_an_edge_loop_on_a_closed_curve() {
    let source = primitives::icosphere(5);
    // Not at 0.3: the sphere has the layout of a cube, and one of its rings
    // runs there of itself.
    let height = 0.35;
    let (free, _) = remesh(&source, 600, &[]);
    let (mesh, report) = remesh(&source, 600, &[parallel(height)]);
    assert!(!report.fell_back, "the layout chain gave up: {:?}", report.fallback_causes);
    assert!(measure(&source, &mesh, Some(600)).is_valid());
    assert_eq!(report.guide_gaps, 0);

    // The edges along the curve close into one loop, on the circle.
    let line = joined(&report.guide_edges);
    assert!(line.len() >= 12, "{} vertices on the loop", line.len());
    for (&v, neighbours) in &line {
        let p = mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - height).abs() < 5e-3, "vertex {v} at {p:?}");
    }
    // Without the curve nothing of the sort is there.
    let on_the_circle = |mesh: &PolyMesh| mesh.positions.iter().filter(|p| (p.z - height).abs() < 5e-3).count();
    assert!(on_the_circle(&free) < line.len() / 2);
    assert_eq!(on_the_circle(&mesh), line.len());
    // A line drawn is not a hard edge: nothing to mark sharp or to crease.
    assert!(report.hard_edges.is_empty() && report.creases.is_empty());
}

#[test]
fn an_open_curve_gives_a_line_of_edges_from_end_to_end() {
    // A line across a flat sheet, off its own grid.
    let source = primitives::plane(24);
    let curve = GuideCurve {
        points: vec![Vec3::new(-0.61, 0.13, 0.0), Vec3::new(0.57, 0.13, 0.0)],
        closed: false,
        sharp: false,
    };
    let (mesh, report) = remesh(&source, 400, &[curve]);
    assert!(measure(&source, &mesh, Some(400)).is_valid());
    let line = joined(&report.guide_edges);
    let xs: Vec<f64> = line.keys().map(|&v| mesh.positions[v as usize].x).collect();
    for &v in line.keys() {
        assert!((mesh.positions[v as usize].y - 0.13).abs() < 1e-6);
    }
    let (low, high) = (xs.iter().copied().fold(f64::INFINITY, f64::min), xs.iter().copied().fold(-1.0, f64::max));
    assert!(low < -0.5 && high > 0.45, "the line runs from {low} to {high}");
    // One line: two ends, every other vertex with two neighbours.
    assert_eq!(line.values().filter(|n| n.len() == 1).count(), 2);
    assert!(line.values().all(|n| n.len() <= 2));
}

#[test]
fn two_curves_that_cross_share_a_vertex() {
    let source = primitives::plane(24);
    let across =
        GuideCurve { points: vec![Vec3::new(-0.8, 0.13, 0.0), Vec3::new(0.8, 0.13, 0.0)], closed: false, sharp: false };
    let up = GuideCurve {
        points: vec![Vec3::new(-0.21, -0.8, 0.0), Vec3::new(-0.21, 0.8, 0.0)],
        closed: false,
        sharp: false,
    };
    let (mesh, report) = remesh(&source, 400, &[across, up]);
    assert!(measure(&source, &mesh, Some(400)).is_valid());
    let line = joined(&report.guide_edges);
    let crossings: Vec<Vec3> =
        line.iter().filter(|(_, n)| n.len() == 4).map(|(&v, _)| mesh.positions[v as usize]).collect();
    assert_eq!(crossings.len(), 1, "{crossings:?}");
    assert!((crossings[0] - Vec3::new(-0.21, 0.13, 0.0)).length() < 1e-6, "{:?}", crossings[0]);
}

#[test]
fn a_curve_on_a_coarse_mesh_gives_a_line_of_edges() {
    // The cube of twelve triangles (DT-164): a face is two triangles, which
    // the line crosses once. A line of points closer together than an edge
    // of the work mesh, as the drawing tools make them.
    let source = primitives::cube(1);
    let points = (0..31).map(|i| Vec3::new(-0.9 + 0.06 * f64::from(i), -1.0, 0.3)).collect();
    let curve = GuideCurve { points, closed: false, sharp: false };
    let (mesh, report) = remesh(&source, 600, &[curve]);
    assert!(measure(&source, &mesh, Some(600)).is_valid());
    let line = joined(&report.guide_edges);
    for &v in line.keys() {
        let p = mesh.positions[v as usize];
        assert!((p.y + 1.0).abs() < 1e-6 && (p.z - 0.3).abs() < 1e-6, "{p:?}");
    }
    let xs: Vec<f64> = line.keys().map(|&v| mesh.positions[v as usize].x).collect();
    let (low, high) = (xs.iter().copied().fold(f64::INFINITY, f64::min), xs.iter().copied().fold(-1.0, f64::max));
    assert!(low < -0.6 && high > 0.6, "the line runs from {low} to {high}");
    assert_eq!(line.values().filter(|n| n.len() == 1).count(), 2);
}

#[test]
fn a_ring_round_a_coarse_cube_is_one_loop_of_edges() {
    let source = primitives::cube(1);
    let corners =
        [Vec3::new(-1.0, -1.0, 0.3), Vec3::new(1.0, -1.0, 0.3), Vec3::new(1.0, 1.0, 0.3), Vec3::new(-1.0, 1.0, 0.3)];
    let points = (0..40).map(|k| corners[k / 10].lerp(corners[(k / 10 + 1) % 4], (k % 10) as f64 / 10.0)).collect();
    let curve = GuideCurve { points, closed: true, sharp: false };
    let (mesh, report) = remesh(&source, 600, &[curve]);
    assert!(measure(&source, &mesh, Some(600)).is_valid());
    let line = joined(&report.guide_edges);
    assert!(line.len() > 30, "{} vertices on the ring", line.len());
    for (&v, neighbours) in &line {
        let p = mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - 0.3).abs() < 1e-6, "{p:?}");
    }
}

#[test]
fn a_diagonal_across_a_face_is_followed_with_no_singularity_for_it() {
    // At 45 degrees from the sides of the face, the field may turn either
    // way to meet the line: it used to turn one way on one side and the
    // other way on the other, with a pair of singularities at the two ends
    // that folded the map and let the line go (DT-164).
    let source = primitives::cube(1);
    let points = (0..31).map(|i| -0.6 + 0.04 * f64::from(i)).map(|t| Vec3::new(t, t, 1.0)).collect();
    let curve = GuideCurve { points, closed: false, sharp: false };
    let (mesh, report) = remesh(&source, 1000, &[curve]);
    assert!(measure(&source, &mesh, Some(1000)).is_valid());
    assert_eq!(report.singularities, (8, 0), "the corners of the cube, nothing more");
    let line = joined(&report.guide_edges);
    for &v in line.keys() {
        let p = mesh.positions[v as usize];
        assert!((p.z - 1.0).abs() < 1e-6 && (p.x - p.y).abs() < 1e-6, "{p:?}");
    }
    let xs: Vec<f64> = line.keys().map(|&v| mesh.positions[v as usize].x).collect();
    let (low, high) = (xs.iter().copied().fold(f64::INFINITY, f64::min), xs.iter().copied().fold(-1.0, f64::max));
    assert!(low < -0.55 && high > 0.55, "the line runs from {low} to {high}");
    assert_eq!(line.values().filter(|n| n.len() == 1).count(), 2);
}

/// A closed curve on the top face of the cube of twelve triangles, as the
/// drawing tools make one: an ellipse of half-axes `a` and `b` round
/// `centre`, turned by `turn`, its radius wobbling by up to `wobble` of
/// itself.
fn loop_on_top(a: f64, b: f64, centre: [f64; 2], turn: f64, wobble: f64) -> GuideCurve {
    let points = (0..100)
        .map(|k| {
            let t = std::f64::consts::TAU * f64::from(k) / 100.0;
            let w = 1.0
                + wobble * ((2.0 * t + 1.6).sin() + 0.75 * (3.0 * t + 1.3).sin() + 0.5 * (5.0 * t + 0.2).sin()) / 2.25;
            let (x, y) = (a * w * t.cos(), b * w * t.sin());
            Vec3::new(centre[0] + x * turn.cos() - y * turn.sin(), centre[1] + x * turn.sin() + y * turn.cos(), 1.0)
        })
        .collect();
    GuideCurve { points, closed: true, sharp: false }
}

/// The edges of `report` along the curve form one closed loop of edges on
/// it, of at least `fewest` vertices.
fn is_one_loop_on(mesh: &PolyMesh, report: &Report, curve: &GuideCurve, fewest: usize) {
    assert_eq!(report.guides_let_go, 0, "{:?}", report.line_causes);
    let line = joined(&report.guide_edges);
    assert!(line.len() >= fewest, "{} vertices on the loop", line.len());
    let points = &curve.points;
    let off = |p: Vec3| {
        (0..points.len())
            .map(|k| {
                let (a, b) = (points[k], points[(k + 1) % points.len()]);
                let t = ((p - a).dot(b - a) / (b - a).length_squared()).clamp(0.0, 1.0);
                p.distance(a.lerp(b, t))
            })
            .fold(f64::INFINITY, f64::min)
    };
    for (&v, neighbours) in &line {
        let p = mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - 1.0).abs() < 1e-6 && off(p) < 0.01, "{p:?} is off the loop by {}", off(p));
    }
    // One loop, not several.
    let start = *line.keys().next().unwrap();
    let (mut previous, mut current, mut steps) = (start, line[&start][0], 1);
    while current != start {
        let next = line[&current].iter().copied().find(|&n| n != previous).unwrap();
        (previous, current, steps) = (current, next, steps + 1);
    }
    assert_eq!(steps, line.len());
}

/// How long the edges of `mesh` along each of the twelve edges of the cube
/// of side 2 are, in all: 2 for an edge kept whole.
fn along_the_edges_of_the_cube(mesh: &PolyMesh) -> Vec<f64> {
    let mut lengths = Vec::new();
    for axis in 0..3 {
        let others = [(axis + 1) % 3, (axis + 2) % 3];
        for [s, t] in [[-1.0, -1.0], [-1.0, 1.0], [1.0, -1.0], [1.0, 1.0]] {
            let on = |p: Vec3| (p[others[0]] - s).abs() < 1e-6 && (p[others[1]] - t).abs() < 1e-6;
            let mut seen = std::collections::HashSet::new();
            let mut length = 0.0;
            for face in &mesh.faces {
                for k in 0..face.len() {
                    let (a, b) = (face[k], face[(k + 1) % face.len()]);
                    let (p, q) = (mesh.positions[a as usize], mesh.positions[b as usize]);
                    if on(p) && on(q) && seen.insert((a.min(b), a.max(b))) {
                        length += p.distance(q);
                    }
                }
            }
            lengths.push(length);
        }
    }
    lengths
}

#[test]
fn a_loop_on_a_face_of_a_cube_is_one_loop_of_edges() {
    // The singularities of the field round the loop lie half a quad from
    // it: arcs of no length join them to it, and the vertex they share
    // used to stay at the singularity, off the loop (#195).
    let source = primitives::cube(1);
    let curve = loop_on_top(0.45, 0.35, [0.0, 0.0], 0.0, 0.0);
    let (mesh, report) = remesh(&source, 550, std::slice::from_ref(&curve));
    assert!(measure(&source, &mesh, Some(550)).is_valid());
    is_one_loop_on(&mesh, &report, &curve, 8);
    assert!(along_the_edges_of_the_cube(&mesh).iter().all(|&length| (length - 2.0).abs() < 1e-6));
}

#[test]
fn a_loop_drawn_by_hand_on_a_face_of_a_cube_is_held() {
    // Off the middle, turned and uneven. Cancelling a pair of
    // singularities across the loop left the field inside it a quarter
    // turn instead of a whole one round, which folded the map: the loop
    // was let go (#195).
    let source = primitives::cube(1);
    let curve = loop_on_top(0.51, 0.4, [-0.09, -0.08], 2.6, 0.06);
    let (mesh, report) = remesh(&source, 550, std::slice::from_ref(&curve));
    assert!(measure(&source, &mesh, Some(550)).is_valid());
    is_one_loop_on(&mesh, &report, &curve, 8);
    assert!(along_the_edges_of_the_cube(&mesh).iter().all(|&length| (length - 2.0).abs() < 1e-6));
}

#[test]
fn a_loop_that_cannot_be_held_is_let_go_alone() {
    // Smaller than a quad: it is let go, and the edges of the cube are kept
    // whole. Letting go of a line used to take the hard edges near it
    // along, and the top of the cube came out rounded (#195).
    let source = primitives::cube(1);
    let curve = loop_on_top(0.06, 0.06, [0.0, 0.0], 0.0, 0.0);
    let (mesh, report) = remesh(&source, 550, &[curve]);
    assert!(measure(&source, &mesh, Some(550)).is_valid());
    assert_eq!(report.guides_let_go, 1);
    let lengths = along_the_edges_of_the_cube(&mesh);
    assert!(lengths.iter().all(|&length| (length - 2.0).abs() < 1e-6), "{lengths:?}");
}

#[test]
fn a_loop_near_the_edges_of_a_cube_leaves_them_whole() {
    // A quarter of a quad from two edges of the top: an arc of no length
    // joins the loop to them, and the vertex they share used to be put
    // halfway between, off the edge of the cube (#195).
    let source = primitives::cube(1);
    let lengths: Vec<Vec<f64>> =
        [loop_on_top(0.3, 0.3, [0.65, 0.65], 0.0, 0.0), loop_on_top(0.95, 0.95, [0.0, 0.0], 0.0, 0.0)]
            .into_iter()
            .map(|curve| {
                let (mesh, report) = remesh(&source, 550, &[curve]);
                assert!(measure(&source, &mesh, Some(550)).is_valid());
                assert!(!report.guide_edges.is_empty());
                along_the_edges_of_the_cube(&mesh)
            })
            .collect();
    assert!(lengths.iter().flatten().all(|&length| (length - 2.0).abs() < 1e-6), "{lengths:?}");
}

#[test]
fn a_curve_away_from_the_surface_changes_nothing() {
    let source = primitives::icosphere(4);
    let far =
        GuideCurve { points: vec![Vec3::new(5.0, 0.0, 0.0), Vec3::new(5.0, 1.0, 0.0)], closed: false, sharp: false };
    // It is brought back onto the surface, where it is one short line.
    let (mesh, report) = remesh(&source, 300, &[far]);
    assert!(measure(&source, &mesh, Some(300)).is_valid());
    assert!(report.guide_edges.len() < 12);
}

#[test]
fn a_curve_drawn_as_sharp_gives_hard_edges_creased_in_full() {
    let source = primitives::icosphere(5);
    // As a guide, the loop is a line of edges and nothing more.
    let (_, soft) = remesh(&source, 600, &[parallel(0.3)]);
    assert!(soft.hard_edges.is_empty() && soft.sharp_curve_edges.is_empty());
    assert!(soft.creases.is_empty(), "{} creases on a sphere", soft.creases.len());

    // Drawn as sharp, the same loop of edges is made of hard edges.
    let (mesh, report) = remesh(&source, 600, &[GuideCurve { sharp: true, ..parallel(0.3) }]);
    assert!(measure(&source, &mesh, Some(600)).is_valid());
    assert!(report.guide_edges.is_empty());
    assert_eq!(report.hard_edges, report.sharp_curve_edges);
    assert_eq!(report.hard_edges.len(), soft.guide_edges.len());
    let line = joined(&report.hard_edges);
    assert!(line.len() >= 12, "{} vertices on the loop", line.len());
    for (&v, neighbours) in &line {
        let p = mesh.positions[v as usize];
        assert_eq!(neighbours.len(), 2, "vertex {v} at {p:?}");
        assert!((p.z - 0.3).abs() < 5e-3, "vertex {v} at {p:?}");
    }
    // And each is creased in full, to stay sharp when the mesh is subdivided.
    for edge in &report.hard_edges {
        let weight = report.creases.iter().find(|(creased, _)| creased == edge).map(|&(_, weight)| weight);
        assert_eq!(weight, Some(1.0), "edge {edge:?}");
    }

    // One of each: the two loops are told apart.
    let curves = [parallel(0.3), GuideCurve { sharp: true, ..parallel(-0.3) }];
    let (mesh, report) = remesh(&source, 600, &curves);
    assert!(!report.guide_edges.is_empty() && !report.hard_edges.is_empty());
    let height = |edges: &[[u32; 2]]| edges.iter().map(|&[a, _]| mesh.positions[a as usize].z).collect::<Vec<_>>();
    assert!(height(&report.guide_edges).iter().all(|z| (z - 0.3).abs() < 5e-3));
    assert!(height(&report.hard_edges).iter().all(|z| (z + 0.3).abs() < 5e-3));
}

#[test]
fn the_edges_of_a_box_are_not_taken_for_a_curve_drawn_as_sharp() {
    // A sharp diagonal across the top of a cube: the edges of the cube are
    // nearer to it than to any other curve, yet they are not along it
    // (DT-164).
    let source = primitives::cube(1);
    let points = (0..31).map(|i| -0.6 + 0.04 * f64::from(i)).map(|t| Vec3::new(t, t, 1.0)).collect();
    let curve = GuideCurve { points, closed: false, sharp: true };
    let (mesh, report) = remesh(&source, 1000, &[curve]);
    assert!(measure(&source, &mesh, Some(1000)).is_valid());
    let line = joined(&report.sharp_curve_edges);
    assert!(line.len() >= 8, "{} vertices along the curve", line.len());
    for &v in line.keys() {
        let p = mesh.positions[v as usize];
        assert!((p.z - 1.0).abs() < 1e-6 && (p.x - p.y).abs() < 1e-6, "{p:?}");
    }
}

#[test]
fn no_edge_along_a_guide_curve_is_taken_for_a_hard_edge() {
    // Small loops, drawn with few points: the line the mesh follows, on the
    // surface, strays from the polygon that was drawn. The edges along it
    // are still those of a guide curve, none of them a hard edge.
    let source = primitives::icosphere(5);
    for (z, points) in [(0.8, 10), (0.9, 8), (0.93, 6)] {
        let radius = (1.0f64 - z * z).sqrt();
        let loop_ = GuideCurve {
            points: (0..points)
                .map(|k| std::f64::consts::TAU * k as f64 / points as f64)
                .map(|turn| Vec3::new(radius * turn.cos(), radius * turn.sin(), z))
                .collect(),
            closed: true,
            sharp: false,
        };
        let (mesh, report) = remesh(&source, 2400, &[loop_]);
        assert!(measure(&source, &mesh, Some(2400)).is_valid());
        assert!(report.guide_edges.len() >= 6, "at {z}: {} edges along the loop", report.guide_edges.len());
        assert!(report.hard_edges.is_empty(), "at {z}: {} hard edges", report.hard_edges.len());
        assert!(report.creases.is_empty(), "at {z}: {} creases", report.creases.len());
    }
}

#[test]
fn the_map_is_shown_to_whoever_listens() {
    // What a viewer gets of the map: its grid lines over the surface, and
    // the lines it kept.
    struct Paths(Vec<String>);
    impl openclare::recorder::Recorder for Paths {
        fn record(&mut self, path: &str, _: openclare::recorder::Shape<'_>) {
            self.0.push(path.to_string());
        }
    }
    let source = primitives::icosphere(5);
    let config = RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(600)) };
    let mut paths = Paths(Vec::new());
    config.remesh_recorded(&source, &Guides { curves: &[parallel(0.3)], ..Default::default() }, &mut paths).unwrap();
    for layer in ["map/surface", "map/u-lines", "map/v-lines", "map/lines-kept", "notes/map"] {
        assert!(paths.0.iter().any(|path| path == layer), "{layer} is missing from {:?}", paths.0);
    }
    // The map comes before the layout cut from it.
    let first = |stage: &str| paths.0.iter().position(|path| path.starts_with(stage)).unwrap();
    assert!(first("map/") < first("layout/"));
    assert!(!paths.0.iter().any(|path| path == "map/lines-let-go" || path == "map/folded-with-every-line"));
}

#[test]
fn each_edge_along_a_curve_says_which_curve() {
    // A point alone (no curve, but counted), a loop round the cube and a
    // sharp diagonal across its top: each edge of the result along a curve
    // comes with the index of that curve among those given.
    let source = primitives::cube(1);
    let alone = GuideCurve { points: vec![Vec3::new(0.0, -1.0, 0.0)], closed: false, sharp: false };
    let corners =
        [Vec3::new(-1.0, -1.0, 0.3), Vec3::new(1.0, -1.0, 0.3), Vec3::new(1.0, 1.0, 0.3), Vec3::new(-1.0, 1.0, 0.3)];
    let points = (0..40).map(|k| corners[k / 10].lerp(corners[(k / 10 + 1) % 4], (k % 10) as f64 / 10.0)).collect();
    let ring = GuideCurve { points, closed: true, sharp: false };
    let points = (0..31).map(|i| -0.6 + 0.04 * f64::from(i)).map(|t| Vec3::new(t, t, 1.0)).collect();
    let diagonal = GuideCurve { points, closed: false, sharp: true };
    let (mesh, report) = remesh(&source, 1000, &[alone, ring, diagonal]);
    let of = |curve: u32| -> Vec<[u32; 2]> {
        report.curve_edges.iter().filter(|&&(_, c)| c == curve).map(|&(edge, _)| edge).collect()
    };
    assert!(of(0).is_empty());
    let (ring_edges, diagonal_edges) = (of(1), of(2));
    assert_eq!(report.curve_edges.len(), ring_edges.len() + diagonal_edges.len());
    assert!(ring_edges.len() >= 30, "{} edges along the loop", ring_edges.len());
    for &[a, b] in &ring_edges {
        for v in [a, b] {
            assert!((mesh.positions[v as usize].z - 0.3).abs() < 1e-6, "{:?}", mesh.positions[v as usize]);
        }
    }
    assert!(diagonal_edges.len() >= 8, "{} edges along the diagonal", diagonal_edges.len());
    for &[a, b] in &diagonal_edges {
        for v in [a, b] {
            let p = mesh.positions[v as usize];
            assert!((p.z - 1.0).abs() < 1e-6 && (p.x - p.y).abs() < 1e-6, "{p:?}");
        }
    }
    // The same edges as the report lists by kind.
    let set = |edges: &[[u32; 2]]| edges.iter().copied().collect::<std::collections::HashSet<_>>();
    assert_eq!(set(&ring_edges), set(&report.guide_edges));
    assert_eq!(set(&diagonal_edges), set(&report.sharp_curve_edges));
}

#[test]
fn a_ring_drawn_along_the_edges_of_the_input_is_kept() {
    // A ring round a cube at the height of one of its rows of edges: where it
    // turns the corners it goes through vertices of the input, which must be
    // taken as they are, not doubled by a split next to them.
    let source = primitives::cube(40);
    for z in [0.0, 0.05, 0.5] {
        let points = (0..64)
            .map(|k| {
                let turn = std::f64::consts::TAU * k as f64 / 64.0;
                let (x, y) = (turn.cos(), turn.sin());
                let s = x.abs().max(y.abs());
                Vec3::new(x / s, y / s, z)
            })
            .collect();
        let (mesh, report) = remesh(&source, 2000, &[GuideCurve { points, closed: true, sharp: false }]);
        assert_eq!(report.guides_let_go, 0, "at {z}: {:?}", report.line_causes);
        let line = joined(&report.guide_edges);
        assert!(line.len() >= 60, "at {z}: {} vertices on the loop", line.len());
        for (&v, neighbours) in &line {
            let p = mesh.positions[v as usize];
            assert_eq!(neighbours.len(), 2, "at {z}: vertex {v} at {p:?}");
            assert!((p.z - z).abs() < 1e-6, "at {z}: vertex {v} at {p:?}");
        }
    }
}
