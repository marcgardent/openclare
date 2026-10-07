//! Quantization: whole lengths on the arcs of the T-mesh that agree across
//! every rectangle, and stand for a sound quad mesh of the size asked for.

use std::collections::BTreeMap;

use retopo::TriMesh;
use retopo::crossfield::{self, CrossField};
use retopo::mesh::primitives;
use retopo::preprocess::{self, WorkMesh};
use retopo::quantization::{self, Kind, Lattice, Quantization, Request};
use retopo::tmesh::{self, NodeKind, TMesh};

fn work_mesh(source: &TriMesh, crease_angle: Option<f64>) -> WorkMesh {
    let request = preprocess::Request { triangles: 8000, crease_angle, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

fn field(work: &WorkMesh, alignment: f64) -> CrossField {
    let request = crossfield::Request { alignment, ..Default::default() };
    crossfield::create(crossfield::Kind::Power).solve(work, &request).unwrap()
}

/// The T-mesh of `source` for quads of `edge_length`.
fn layout(source: &TriMesh, crease_angle: Option<f64>, alignment: f64, edge_length: f64) -> TMesh {
    let work = work_mesh(source, crease_angle);
    let cross = field(&work, alignment);
    let tmesh =
        tmesh::create(tmesh::Kind::Motorcycle).build(&work, &cross, &tmesh::Request { edge_length }).unwrap().tmesh;
    assert!(tmesh.report.is_valid(), "{:?}", tmesh.report);
    tmesh
}

fn quantize(tmesh: &TMesh, quads: Option<usize>) -> Quantization {
    let request = Request { quads, ..Default::default() };
    quantization::create(Kind::WalkCancelling).quantize(tmesh, &request).unwrap()
}

/// Valence of each vertex of a lattice, and whether it is on a border.
fn valences(lattice: &Lattice) -> Vec<(usize, bool)> {
    let mut uses: BTreeMap<(u32, u32), usize> = BTreeMap::new();
    for quad in &lattice.quads {
        for k in 0..4 {
            let (a, b) = (quad[k], quad[(k + 1) % 4]);
            *uses.entry((a.min(b), a.max(b))).or_default() += 1;
        }
    }
    let mut out = vec![(0, false); lattice.sources.len()];
    for (&(a, b), &count) in &uses {
        assert!(count <= 2, "an edge between more than two quads");
        for vertex in [a, b] {
            out[vertex as usize].0 += 1;
            out[vertex as usize].1 |= count == 1;
        }
    }
    out
}

/// What must hold of any quantization, whatever the surface.
fn check(tmesh: &TMesh, quantized: &Quantization) -> Lattice {
    let report = &quantized.report;
    assert!(report.is_valid(), "{report:?}");
    assert_eq!(quantized.lengths.len(), tmesh.arcs.len());

    // Opposite sides of every rectangle add up to the same.
    let mut quads = 0;
    for (patch, size) in tmesh.patches.iter().zip(quantized.patch_sizes(tmesh)) {
        let mut sides = [0; 4];
        for (&half, &side) in patch.boundary.iter().zip(&patch.side_of_each().unwrap()) {
            sides[side as usize] += quantized.lengths[half as usize / 2];
        }
        assert_eq!((sides[0], sides[1]), (sides[2], sides[3]));
        assert_eq!(size, [sides[0], sides[1]]);
        quads += (size[0] * size[1]) as usize;
    }
    assert_eq!(report.quads, quads);

    // The quad mesh they stand for: the right number of quads, the topology
    // of the surface, and the singularities for its only irregular vertices.
    let lattice = quantized.lattice(tmesh);
    assert_eq!(lattice.quads.len(), quads);
    for quad in &lattice.quads {
        let mut corners = quad.to_vec();
        corners.sort_unstable();
        corners.dedup();
        assert_eq!(corners.len(), 4, "a quad with a repeated corner");
    }
    let valence = valences(&lattice);
    let edges: usize = valence.iter().map(|&(count, _)| count).sum::<usize>() / 2;
    assert_eq!(lattice.sources.len() as i64 - edges as i64 + quads as i64, tmesh.report.euler);

    let mut irregular = vec![false; valence.len()];
    for (node, data) in tmesh.nodes.iter().enumerate() {
        if matches!(data.kind, NodeKind::Singularity { .. }) {
            let vertex = lattice.node_vertex[node] as usize;
            assert!(!irregular[vertex], "two singularities in one vertex");
            irregular[vertex] = true;
            assert_eq!(valence[vertex], (data.arcs.len(), false));
        }
    }
    for (vertex, &(count, border)) in valence.iter().enumerate() {
        assert!(border || irregular[vertex] || count == 4, "a stray irregular vertex of valence {count}");
    }
    lattice
}

#[test]
fn a_sphere_gets_the_quads_asked_for() {
    let tmesh = layout(&primitives::icosphere(4), None, 0.0, 0.15);
    for wanted in [300, 1200, 5000] {
        let quantized = quantize(&tmesh, Some(wanted));
        check(&tmesh, &quantized);
        let error = quantized.report.budget_error.unwrap();
        assert!(error <= 0.05, "{} quads for {wanted}", quantized.report.quads);
        // The lengths follow the real ones, scaled: within an edge or two.
        assert!(quantized.report.deviation_mean < 0.6, "{:?}", quantized.report);
    }
}

#[test]
fn the_scale_of_the_tmesh_is_kept_unless_a_number_of_quads_is_given() {
    let tmesh = layout(&primitives::icosphere(4), None, 0.0, 0.15);
    let quantized = quantize(&tmesh, None);
    check(&tmesh, &quantized);
    assert_eq!((quantized.report.scale, quantized.report.budget_error), (1.0, None));
    // The sphere has area 4 pi: about 560 quads of side 0.15.
    let expected = 4.0 * std::f64::consts::PI / (0.15 * 0.15);
    assert!((quantized.report.quads as f64 - expected).abs() < 0.1 * expected, "{}", quantized.report.quads);
}

#[test]
fn a_cube_keeps_its_six_faces_and_eight_corners() {
    let tmesh = layout(&primitives::cube(8), Some(30.0), 0.0, 0.1);
    let quantized = quantize(&tmesh, Some(2400));
    let lattice = check(&tmesh, &quantized);
    let report = &quantized.report;
    assert_eq!((report.zero_arcs, report.collapsed_patches, report.held_arcs), (0, 0, 0));
    assert!(report.budget_error.unwrap() <= 0.02, "{} quads", report.quads);
    let corners = valences(&lattice).iter().filter(|&&(count, _)| count == 3).count();
    assert_eq!(corners, 8);
}

#[test]
fn an_open_sheet_gets_exactly_its_grid() {
    let tmesh = layout(&primitives::plane(24), None, 0.0, 0.1);
    let quantized = quantize(&tmesh, Some(400));
    let lattice = check(&tmesh, &quantized);
    assert_eq!(quantized.report.quads, 400);
    assert!(quantized.lengths.iter().all(|&length| length == 20));
    assert_eq!(lattice.sources.len(), 21 * 21);
    let border = valences(&lattice).iter().filter(|&&(_, border)| border).count();
    assert_eq!(border, 80);
}

#[test]
fn a_surface_without_singularity_has_nothing_to_quantize() {
    let tmesh = layout(&primitives::torus(1.0, 0.4, 96, 48), None, 0.05, 0.1);
    let quantized = quantize(&tmesh, Some(2000));
    assert!(quantized.lengths.is_empty());
    assert_eq!(quantized.report.quads, 0);
    assert!(quantized.report.is_valid());
}

#[test]
fn coarse_meshes_close_slivers_up_without_merging_singularities() {
    // Left to itself, the smoothest field of a fat torus has sixteen
    // singularities in two rings. With few quads to share out, many arcs
    // get no length at all; the singularities must still stay apart, and
    // the tube open.
    let torus = layout(&primitives::torus(1.0, 0.4, 96, 48), None, 0.0, 0.1);
    let sphere = layout(&primitives::icosphere(4), None, 0.05, 0.15);
    let mut closed = 0;
    for tmesh in [&torus, &sphere] {
        for wanted in [60, 150, 400] {
            let quantized = quantize(tmesh, Some(wanted));
            check(tmesh, &quantized);
            closed += quantized.report.zero_arcs;
        }
    }
    assert!(closed > 0);
}

#[test]
fn the_quantizer_from_the_factory_is_deterministic_and_checks_its_request() {
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(quantization::create(kind).kind(), kind);
    }
    assert!("no-such-quantizer".parse::<Kind>().is_err());

    let tmesh = layout(&primitives::icosphere(4), None, 0.05, 0.15);
    let (first, second) = (quantize(&tmesh, Some(900)), quantize(&tmesh, Some(900)));
    assert_eq!(first.lengths, second.lengths);
    assert_eq!(first.report, second.report);

    let quantizer = quantization::create(Kind::WalkCancelling);
    assert!(quantizer.quantize(&tmesh, &Request { quads: Some(0), ..Default::default() }).is_err());
    assert!(quantizer.quantize(&tmesh, &Request { tolerance: -1.0, ..Default::default() }).is_err());
}
