//! The T-mesh: the cuts from the singularities leave nothing but rectangles
//! of the map, and those cover the surface exactly.

use retopo::TriMesh;
use retopo::crossfield::{self, CrossField};
use retopo::mesh::primitives;
use retopo::parametrization::{self, Parametrization};
use retopo::preprocess::{self, WorkMesh};
use retopo::tmesh::{self, Kind, TMesh};

fn work_mesh(source: &TriMesh, triangles: usize, crease_angle: Option<f64>) -> WorkMesh {
    let request = preprocess::Request { triangles, crease_angle, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

fn field(work: &WorkMesh, alignment: f64) -> CrossField {
    let request = crossfield::Request { alignment, ..Default::default() };
    crossfield::create(crossfield::Kind::Power).solve(work, &request).unwrap()
}

fn seamless(work: &WorkMesh, field: &CrossField, edge_length: f64) -> Parametrization {
    let request = parametrization::Request { edge_length, stiffening_rounds: 40 };
    parametrization::create(parametrization::Kind::Seamless).parametrize(work, field, &request).unwrap()
}

fn layout(source: &TriMesh, crease_angle: Option<f64>, alignment: f64, edge_length: f64) -> (CrossField, TMesh) {
    let work = work_mesh(source, 8000, crease_angle);
    let cross = field(&work, alignment);
    let map = seamless(&work, &cross, edge_length);
    assert_eq!(map.report.flipped, 0);
    let tmesh = TMesh::trace(&work, &cross, &map, 1000.0);
    (cross, tmesh)
}

/// What must hold of any T-mesh, whatever the surface.
fn check_structure(tmesh: &TMesh) {
    let report = &tmesh.report;
    assert!(report.is_valid(), "{report:?}");
    assert_eq!((report.nodes, report.arcs, report.patches), (tmesh.nodes.len(), tmesh.arcs.len(), tmesh.patches.len()));

    // Every half-arc is on the rim of exactly one patch, the one its arc
    // names on that side; only a border has nothing on its right.
    let mut uses = vec![0; 2 * tmesh.arcs.len()];
    for (index, patch) in tmesh.patches.iter().enumerate() {
        assert_eq!(patch.boundary.len(), patch.angles.len());
        assert_eq!(patch.angles.iter().filter(|&&angle| angle == 1).count(), 4);
        let sides = patch.sides.expect("a rectangle");
        assert!((sides[0] - sides[2]).abs() < 1e-6 && (sides[1] - sides[3]).abs() < 1e-6, "{sides:?}");
        for &half in &patch.boundary {
            uses[half as usize] += 1;
            assert_eq!(tmesh.arcs[half as usize / 2].patches[half as usize % 2], index as u32);
        }
        // The rim is a closed walk: each half-arc starts where the last ended.
        for (&half, &next) in patch.boundary.iter().zip(patch.boundary.iter().cycle().skip(1)) {
            let end = tmesh.arcs[half as usize / 2].nodes[1 - half as usize % 2];
            let start = tmesh.arcs[next as usize / 2].nodes[next as usize % 2];
            assert_eq!(end, start);
        }
    }
    for (arc, data) in tmesh.arcs.iter().enumerate() {
        assert!(data.length >= 0.0);
        assert_eq!(uses[2 * arc], 1);
        assert_eq!(uses[2 * arc + 1], if data.border { 0 } else { 1 });
    }
    // Round a node, every arc that touches it is listed once.
    let listed: usize = tmesh.nodes.iter().map(|node| node.arcs.len()).sum();
    assert_eq!(listed, 2 * tmesh.arcs.len());
}

#[test]
fn a_sphere_is_cut_into_rectangles() {
    let (cross, tmesh) = layout(&primitives::icosphere(4), None, 0.0, 0.15);
    check_structure(&tmesh);
    let report = &tmesh.report;
    assert!(report.singularities >= 8);
    // A singularity of index k sends out 4 - k traces.
    let expected: i64 = cross.indices.iter().filter(|&&index| index != 0).map(|&index| 4 - index as i64).sum();
    assert_eq!(report.traces as i64, expected);
    assert_eq!(report.euler, 2);
    // Each trace ends on another: one T-junction per trace, unless two met
    // nose to nose.
    assert!(report.t_junctions <= report.traces);
    assert!(report.patches >= 6);
}

#[test]
fn the_cuts_of_a_cube_are_its_edges() {
    let (_, tmesh) = layout(&primitives::cube(8), Some(30.0), 0.0, 0.1);
    check_structure(&tmesh);
    let report = &tmesh.report;
    // Its hard edges are cuts from the start, corner to corner; nothing
    // else is needed, and its corners send no trace out.
    assert_eq!((report.singularities, report.traces, report.off_grid_edges), (8, 0, 0));
    assert_eq!((report.nodes, report.arcs, report.patches), (8, 12, 6));
    assert_eq!(report.t_junctions, 0);
    for arc in &tmesh.arcs {
        assert!(arc.feature && !arc.border);
        // An edge of a cube of side 2, in quads of 0.1.
        assert!((arc.length - 20.0).abs() < 1e-3, "{}", arc.length);
    }
    for patch in &tmesh.patches {
        for side in patch.sides.unwrap() {
            assert!((side - 20.0).abs() < 1e-3, "{side}");
        }
    }
}

#[test]
fn an_edge_that_turns_inwards_sends_traces_out() {
    // An L-shaped block: on each of its two L faces, the inner corner spans
    // three quarter turns and sends two traces across the face.
    let (_, tmesh) = layout(&primitives::l_block(8), Some(30.0), 0.0, 0.08);
    check_structure(&tmesh);
    let report = &tmesh.report;
    assert_eq!((report.singularities, report.traces), (12, 4));
    // Each L face is cut into three rectangles; the six sides are whole.
    assert_eq!(report.patches, 12);
    assert_eq!(report.t_junctions, 4);
    let walls = tmesh.arcs.iter().filter(|arc| arc.feature).count();
    assert_eq!(tmesh.arcs.len() - walls, 4);
    assert_eq!(report.euler, 2);
}

#[test]
fn a_hard_edge_that_closes_on_itself_is_a_cut_too() {
    // The two rims of a cylinder: closed curves without a corner.
    let (_, tmesh) = layout(&primitives::cylinder(12), Some(30.0), 0.0, 0.12);
    check_structure(&tmesh);
    let report = &tmesh.report;
    assert_eq!(report.off_grid_edges, 0);
    // Each flat cap needs four singularities to turn its grid into a disk.
    assert_eq!(report.singularities, 8);
    // Both rims are whole in the T-mesh, and the same length in the map: the
    // grid lines that leave one cap all reach the other. (That length is
    // less than the 2 pi / 0.12 of the circle: the caps, whose rings shrink
    // towards their centre, pull it down.)
    let rim_at = |top: bool| -> f64 {
        let on_it = |node: u32| (tmesh.nodes[node as usize].position.z > 0.0) == top;
        tmesh.arcs.iter().filter(|arc| arc.feature && on_it(arc.nodes[0])).map(|arc| arc.length).sum()
    };
    let rims = [rim_at(true), rim_at(false)];
    assert!((rims[0] - rims[1]).abs() < 1e-3, "{rims:?}");
    let circle = std::f64::consts::TAU / 0.12;
    assert!(rims[0] > 0.5 * circle && rims[0] < 1.1 * circle, "{rims:?} for a circle of {circle}");
}

#[test]
fn an_open_sheet_is_one_rectangle_bounded_by_its_border() {
    let (_, tmesh) = layout(&primitives::plane(24), None, 0.0, 0.1);
    check_structure(&tmesh);
    let report = &tmesh.report;
    assert_eq!((report.singularities, report.traces, report.off_grid_edges), (0, 0, 0));
    assert_eq!((report.nodes, report.arcs, report.patches), (4, 4, 1));
    assert!(tmesh.arcs.iter().all(|arc| arc.border && arc.patches == [0, u32::MAX]));
    for side in tmesh.patches[0].sides.unwrap() {
        assert!((side - 20.0).abs() < 1e-3, "{side}");
    }
    assert_eq!(report.euler, 1);
}

#[test]
fn a_torus_without_singularity_is_cut_from_one_of_its_vertices() {
    // Following the curvature, the field of a torus has no singularity (the
    // smoothest one, left to itself, has sixteen): nothing to cut it from.
    // Any vertex then serves, sending a trace along each of its four
    // directions.
    let (_, tmesh) = layout(&primitives::torus(1.0, 0.4, 96, 48), None, 0.05, 0.1);
    check_structure(&tmesh);
    let report = &tmesh.report;
    assert_eq!((report.singularities, report.traces, report.uncut_triangles), (0, 4, 0));
    assert_eq!(report.euler, 0);
    assert!(report.patches >= 1 && report.patches <= 4, "{report:?}");
}

#[test]
fn a_torus_with_singularities_is_cut_into_rectangles() {
    // Left to itself, the smoothest field of a fat torus has singularities.
    let (cross, tmesh) = layout(&primitives::torus(1.0, 0.4, 96, 48), None, 0.0, 0.1);
    assert!(cross.indices.iter().any(|&index| index != 0));
    check_structure(&tmesh);
    assert_eq!(tmesh.report.euler, 0);
}

#[test]
fn the_builder_from_the_factory_gives_the_same_layout_every_time() {
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(tmesh::create(kind).kind(), kind);
    }
    assert!("no-such-builder".parse::<Kind>().is_err());

    let work = work_mesh(&primitives::icosphere(4), 8000, None);
    let cross = field(&work, 0.05);
    let builder = tmesh::create(Kind::Motorcycle);
    let request = tmesh::Request { edge_length: 0.15 };
    let (first, second) =
        (builder.build(&work, &cross, &request).unwrap().tmesh, builder.build(&work, &cross, &request).unwrap().tmesh);
    check_structure(&first);
    assert_eq!(first.report, second.report);
    let lengths = |tmesh: &TMesh| tmesh.arcs.iter().map(|arc| arc.length).collect::<Vec<_>>();
    assert_eq!(lengths(&first), lengths(&second));

    // The same as tracing the seamless map by hand.
    let direct = TMesh::trace(&work, &cross, &seamless(&work, &cross, 0.15), 1000.0);
    assert_eq!(lengths(&first), lengths(&direct));
}
