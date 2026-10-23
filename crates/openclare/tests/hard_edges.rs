//! The hard edges a chain follows, found without remeshing: what the add-on
//! shows in the viewport before a run (`RemeshConfig::hard_edges`).

use std::collections::HashSet;

use openclare::math::{Rng, Vec3};
use openclare::mesh::primitives;
use openclare::recorder::{Recorder, Shape};
use openclare::{Guides, HardEdges, RemeshConfig, Target, TriMesh};

fn organic(quads: usize) -> RemeshConfig {
    RemeshConfig::preset("organic", Target::QuadCount(quads)).unwrap()
}

fn sorted(mesh: &TriMesh, config: &RemeshConfig) -> HardEdges {
    config.hard_edges(mesh, &Guides::default()).unwrap()
}

/// A sphere whose vertices are moved in and out at random: many of its
/// triangles meet at more than 30 degrees, none of that shows at the size
/// of the quads.
fn grainy_sphere(at: Vec3, radius: f64) -> TriMesh {
    let mut sphere = primitives::icosphere(5);
    let mut random = Rng::new(3);
    for p in &mut sphere.positions {
        *p = at + *p * radius * (1.0 + 0.012 * (random.next_f64() - 0.5));
    }
    sphere
}

/// A slab of 8 by 8 by 2 unit cells with a step of one cell across the
/// middle of its top: the step is a hard edge for quads smaller than it,
/// and not for quads several times larger.
fn stepped_slab() -> TriMesh {
    let mut cells = Vec::new();
    for x in 0..24 {
        for y in 0..24 {
            cells.push([x, y, 0]);
            if x >= 12 {
                cells.push([x, y, 1]);
            }
        }
    }
    primitives::blocks_quads(&cells, 2).triangulated()
}

fn together(meshes: &[TriMesh]) -> TriMesh {
    let mut all = TriMesh::default();
    for mesh in meshes {
        let offset = all.positions.len() as u32;
        all.positions.extend(&mesh.positions);
        all.triangles.extend(mesh.triangles.iter().map(|tri| tri.map(|v| v + offset)));
    }
    all
}

fn key([a, b]: [u32; 2]) -> (u32, u32) {
    (a.min(b), a.max(b))
}

#[test]
fn every_edge_of_a_cube_is_kept() {
    let cube = primitives::cube(1);
    assert_eq!(cube.positions.len(), 8);
    let found = sorted(&cube, &organic(3000));
    assert!(found.left_out.is_empty(), "{:?}", found.left_out);
    let kept: HashSet<(u32, u32)> = found.kept.iter().copied().map(key).collect();
    assert_eq!((found.kept.len(), kept.len()), (12, 12), "{:?}", found.kept);
    // Each is an edge of the cube: its ends differ along one axis only.
    for [a, b] in found.kept {
        let along = cube.positions[a as usize] - cube.positions[b as usize];
        let axes = along.to_array().iter().filter(|c| c.abs() > 1e-9).count();
        assert_eq!(axes, 1, "{along:?}");
    }
}

#[test]
fn a_cube_torn_apart_is_read_stitched_and_told_in_its_own_vertices() {
    // Every triangle with corners of its own, as an export split along
    // its hard edges gives it: the chain welds them back first.
    let welded = primitives::cube(1);
    let mut torn = TriMesh::default();
    for tri in &welded.triangles {
        let first = torn.positions.len() as u32;
        torn.positions.extend(tri.map(|v| welded.positions[v as usize]));
        torn.triangles.push([first, first + 1, first + 2]);
    }
    let found = sorted(&torn, &organic(3000));
    assert_eq!((found.kept.len(), found.left_out.len()), (12, 0));
    for [a, b] in found.kept {
        let along = torn.positions[a as usize] - torn.positions[b as usize];
        assert_eq!(along.to_array().iter().filter(|c| c.abs() > 1e-9).count(), 1, "{along:?}");
    }
}

#[test]
fn every_chain_and_target_keeps_the_edges_of_a_cube() {
    use openclare::Method;
    let cube = primitives::cube(4);
    let targets = [Target::QuadCount(500), Target::EdgeLength(0.1), Target::SizeRange { largest: 0.2, smallest: 0.05 }];
    for method in [Method::QuadLayout, Method::IntegerGrid] {
        for target in targets {
            let found = sorted(&cube, &RemeshConfig { method, target, ..organic(500) });
            assert_eq!((found.kept.len(), found.left_out.len()), (48, 0), "{method} {target:?}");
        }
    }
    // The baseline chain only has creases, at their own angle.
    let config = RemeshConfig { method: Method::InstantMeshes, crease_angle: Some(30.0), ..organic(500) };
    assert_eq!(sorted(&cube, &config).kept.len(), 48);
    let config = RemeshConfig { method: Method::InstantMeshes, ..organic(500) };
    assert_eq!(sorted(&cube, &config), HardEdges::default());
}

#[test]
fn without_hard_edges_nothing_is_kept_nor_left_out() {
    let config = RemeshConfig { edge_angle: None, ..organic(3000) };
    assert_eq!(sorted(&primitives::cube(1), &config), HardEdges::default());
}

#[test]
fn the_grain_of_a_sculpt_is_left_out() {
    let sphere = grainy_sphere(Vec3::ZERO, 1.0);
    let found = sorted(&sphere, &organic(3000));
    assert!(found.kept.is_empty(), "{} kept", found.kept.len());
    assert!(found.left_out.len() > 100, "{} left out", found.left_out.len());
    // Each sharper than the angle between its two triangles.
    let topology = openclare::mesh::Topology::build(&sphere);
    let creased = openclare::geometry::feature_edges(&sphere, &topology, Some(30f64.to_radians()), false);
    let creased: HashSet<(u32, u32)> = creased.into_iter().map(key).collect();
    assert!(found.left_out.iter().all(|&edge| creased.contains(&key(edge))));
    assert_eq!(found.left_out.len(), creased.len());
}

#[test]
fn a_step_is_kept_at_the_size_of_small_quads_only() {
    let slab = stepped_slab();
    let fine = sorted(&slab, &organic(3000));
    let coarse = sorted(&slab, &organic(30));
    // The step runs across the slab at the top, 48 edges, and at its foot,
    // 48 more; at the size of small quads it is kept with the rest.
    assert!(fine.left_out.is_empty(), "{} left out", fine.left_out.len());
    assert_eq!(coarse.left_out.len(), 96);
    assert_eq!(fine.kept.len(), coarse.kept.len() + 96);
    let kept: HashSet<(u32, u32)> = fine.kept.iter().copied().map(key).collect();
    assert!(coarse.kept.iter().chain(&coarse.left_out).all(|&edge| kept.contains(&key(edge))));
}

/// Keeps the hard edges of the work meshes a run shows, piece by piece.
#[derive(Default)]
struct WorkHardEdges {
    lines: Vec<[Vec3; 2]>,
}

impl Recorder for WorkHardEdges {
    fn record(&mut self, path: &str, shape: Shape<'_>) {
        if let (true, Shape::Lines { strips, .. }) = (path.starts_with("work/hard-edges"), shape) {
            self.lines.extend(strips.iter().map(|strip| [strip[0], strip[1]]));
        }
    }
}

fn distance_to_segment(p: Vec3, [a, b]: [Vec3; 2]) -> f64 {
    let along = b - a;
    let t = ((p - a).dot(along) / along.length_squared().max(f64::MIN_POSITIVE)).clamp(0.0, 1.0);
    p.distance(a + along * t)
}

#[test]
fn what_is_kept_is_what_the_remesh_follows() {
    // Two pieces, each meshed apart with its share of the quads: at that
    // size the step of the slab is left out, as is the grain of the ball.
    let slab = stepped_slab();
    let on_slab = slab.positions.len() as u32;
    let ball = grainy_sphere(Vec3::new(12.0, 12.0, 30.0), 8.0);
    let input = together(&[slab, ball]);
    let config = organic(60);
    let found = sorted(&input, &config);
    let (slab_left_out, ball_left_out) = found
        .left_out
        .iter()
        .fold((0, 0), |(slab, ball), edge| if edge[0] < on_slab { (slab + 1, ball) } else { (slab, ball + 1) });
    assert_eq!(slab_left_out, 96, "the step");
    assert!(ball_left_out > 100, "{ball_left_out} edges of grain left out");
    assert!(found.kept.iter().all(|edge| edge.iter().all(|&v| v < on_slab)), "the ball has no hard edge");

    // The work meshes of the run have their hard edges along those kept,
    // and along nothing else.
    let mut recorder = WorkHardEdges::default();
    config.remesh_recorded(&input, &Guides::default(), &mut recorder).unwrap();
    let kept: Vec<[Vec3; 2]> =
        found.kept.iter().map(|&[a, b]| [input.positions[a as usize], input.positions[b as usize]]).collect();
    let near = |p: Vec3, lines: &[[Vec3; 2]]| {
        lines.iter().map(|&line| distance_to_segment(p, line)).fold(f64::INFINITY, f64::min)
    };
    let middle = |[a, b]: [Vec3; 2]| 0.5 * (a + b);
    let followed_not_kept = recorder.lines.iter().map(|&line| near(middle(line), &kept)).fold(0.0, f64::max);
    let kept_not_followed = kept.iter().map(|&line| near(middle(line), &recorder.lines)).fold(0.0, f64::max);
    assert!(!recorder.lines.is_empty());
    assert!(followed_not_kept < 1e-9 && kept_not_followed < 1e-9, "{followed_not_kept} {kept_not_followed}");
}
