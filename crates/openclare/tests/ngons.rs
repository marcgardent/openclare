//! Flat parts as n-gons: what the whole chain makes of shapes whose answer
//! is known.

use std::collections::BTreeMap;

use openclare::config::Faces;
use openclare::mesh::primitives;
use openclare::{PolyMesh, RemeshConfig, Report, Target, TriMesh};

fn remesh(source: &TriMesh, quads: usize, faces: Faces) -> (PolyMesh, Report) {
    let config = RemeshConfig { faces, ..RemeshConfig::for_hard_surface(Target::QuadCount(quads)) };
    let (mesh, report) = config.remesh(source).unwrap();
    assert!(!report.fell_back);
    let stats = mesh.stats();
    assert_eq!((stats.boundary_edges, stats.non_manifold_edges, stats.euler_characteristic()), (0, 0, 2));
    assert_eq!(stats, report.stats);
    (mesh, report)
}

/// The weight of each creased edge, every one of them an edge of the mesh.
fn creases(mesh: &PolyMesh, report: &Report) -> BTreeMap<(u32, u32), f64> {
    let edges = mesh.edge_uses();
    report
        .creases
        .iter()
        .map(|&([a, b], weight)| {
            assert!(edges.contains_key(&(a.min(b), a.max(b))), "a crease on no edge");
            ((a.min(b), a.max(b)), weight)
        })
        .collect()
}

#[test]
fn a_cube_comes_out_as_its_six_faces() {
    let (mesh, report) = remesh(&primitives::cube(8), 300, Faces::Ngons);
    assert_eq!((mesh.faces.len(), report.merged_faces), (6, 6));
    // Every edge left is one of the cube, fully creased.
    let creases = creases(&mesh, &report);
    assert_eq!(creases.len(), mesh.stats().edges);
    assert!(creases.values().all(|&weight| weight == 1.0));
}

#[test]
fn a_cylinder_has_one_face_for_each_cap_and_all_its_quads_round_it() {
    let source = primitives::cylinder(24);
    let (quads, _) = remesh(&source, 1000, Faces::Quads);
    let (mesh, report) = remesh(&source, 1000, Faces::Ngons);
    assert_eq!(report.merged_faces, 2);
    let caps: Vec<&Vec<u32>> = mesh.faces.iter().filter(|face| face.len() != 4).collect();
    assert_eq!(caps.len(), 2);
    // The caps are not meshed: the quads asked for all go round the side,
    // more of them a turn than when the caps took their share.
    let around = caps[0].len();
    assert_eq!(caps[1].len(), around);
    assert_eq!((mesh.faces.len() - 2) % around, 0);
    assert!((mesh.faces.len() as f64 / 1000.0 - 1.0).abs() < 0.05, "{} faces", mesh.faces.len());
    let on_the_rim = quads.positions.iter().filter(|p| (p.z - 1.0).abs() < 1e-6 && p.truncate().length() > 0.9).count();
    assert!(around > on_the_rim, "{around} round it, {on_the_rim} with the caps meshed");
    // The rim of each cap is fully creased, and a hard edge.
    let creases = creases(&mesh, &report);
    for cap in caps {
        for k in 0..cap.len() {
            let (a, b) = (cap[k], cap[(k + 1) % cap.len()]);
            assert_eq!(creases.get(&(a.min(b), a.max(b))), Some(&1.0));
            assert!(report.hard_edges.iter().any(|&[c, d]| (a.min(b), a.max(b)) == (c.min(d), c.max(d))));
        }
    }
}

#[test]
fn a_cylinder_with_caps_is_as_many_quads_round_as_asked() {
    // With its caps meshed it gets two thirds of them: 8 for 12.
    for turn in [12.0, 16.0] {
        let config = RemeshConfig {
            faces: Faces::Ngons,
            quads_a_turn: turn,
            ..RemeshConfig::for_organic(Target::SizeRange { largest: 0.6, smallest: 0.05 })
        };
        let (mesh, _) = config.remesh(&primitives::cylinder(24)).unwrap();
        let caps: Vec<usize> = mesh.faces.iter().map(Vec::len).filter(|&sides| sides != 4).collect();
        assert_eq!(caps, [turn as usize; 2]);
        assert_eq!(mesh.stats().boundary_edges, 0);
    }
}

#[test]
fn an_l_block_comes_out_as_convex_faces_creased_all_round() {
    let (mesh, report) = remesh(&primitives::l_block(8), 1000, Faces::Ngons);
    // Six rectangles round it, and its two L-shaped faces in two each.
    assert_eq!(mesh.faces.len(), 10);
    let creases = creases(&mesh, &report);
    assert_eq!(creases.len(), mesh.stats().edges);
    assert!(creases.values().all(|&weight| weight == 1.0));
}

#[test]
fn asking_for_quads_gives_the_mesh_it_always_gave() {
    let source = primitives::cube(8);
    let (mesh, report) = remesh(&source, 300, Faces::Quads);
    assert_eq!((report.merged_faces, mesh.stats().ngons), (0, 0));
}
