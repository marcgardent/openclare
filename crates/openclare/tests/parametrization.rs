use openclare::TriMesh;
use openclare::crossfield::{self, CrossField};
use openclare::math::Vec3;
use openclare::mesh::{Topology, primitives};
use openclare::parametrization::{Kind, Parametrization, Request, create};
use openclare::preprocess::{self, WorkMesh};

fn work_mesh(source: &TriMesh, triangles: usize, crease_angle: Option<f64>) -> WorkMesh {
    let request = preprocess::Request { triangles, crease_angle, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

fn field(work: &WorkMesh, alignment: f64) -> CrossField {
    let request = crossfield::Request { alignment, ..Default::default() };
    crossfield::create(crossfield::Kind::Power).solve(work, &request).unwrap()
}

fn parametrize(work: &WorkMesh, field: &CrossField, edge_length: f64) -> Parametrization {
    let request = Request { edge_length, stiffening_rounds: 40, distortion_passes: 0 };
    create(Kind::Seamless).parametrize(work, field, &request).unwrap()
}

/// Re-derives, from the coordinates alone, how far each seam is from the
/// condition it must meet: across it the map may only turn by the stated
/// number of quarter turns and shift.
fn worst_seam_mismatch(work: &WorkMesh, map: &Parametrization) -> f64 {
    let topology = Topology::build(&work.mesh);
    let next = |corner: usize| 3 * (corner / 3) + (corner % 3 + 1) % 3;
    let mut worst = 0.0f64;
    for h in 0..map.cut.seam.len() {
        if !map.cut.seam[h] {
            continue;
        }
        let t = topology.opposite[h] as usize;
        let (fa, fb, ga, gb) = (map.uv[h], map.uv[next(h)], map.uv[next(t)], map.uv[t]);
        let (mut du, mut dv) = (fa[0] - fb[0], fa[1] - fb[1]);
        for _ in 0..map.cut.transition[h] {
            (du, dv) = (dv, -du);
        }
        worst = worst.max((ga[0] - gb[0] - du).abs()).max((ga[1] - gb[1] - dv).abs());
    }
    worst
}

/// Away from the cut the field must be combed: neighbouring faces carry the
/// same arm, with no quarter turn left between them.
fn assert_combed(work: &WorkMesh, cross: &CrossField, map: &Parametrization) {
    let topology = Topology::build(&work.mesh);
    for h in 0..map.cut.seam.len() {
        let t = topology.opposite[h];
        if t == u32::MAX || map.cut.seam[h] {
            continue;
        }
        let (f, g) = (h / 3, t as usize / 3);
        let left = cross.period_jumps[h] - map.cut.turns[f] + map.cut.turns[g];
        assert_eq!(left.rem_euclid(4), 0, "half-edge {h} is not on the cut but keeps a quarter turn");
    }
}

/// `V - E + F` of the surface once cut open.
fn euler_of_cut_mesh(work: &WorkMesh, map: &Parametrization) -> i64 {
    let topology = Topology::build(&work.mesh);
    let edges = topology.edge_count() + map.cut.seam_edges();
    map.cut.wedge_count as i64 - edges as i64 + work.mesh.triangles.len() as i64
}

#[test]
fn a_flat_sheet_is_mapped_rigidly() {
    let work = work_mesh(&primitives::plane(6), 3000, None);
    let edge_length = 0.1;
    let map = parametrize(&work, &field(&work, 0.0), edge_length);
    assert_eq!(map.report.seam_edges, 0, "a disk needs no cut");
    assert_eq!(map.report.flipped, 0);
    // Rigid up to how accurately the field itself was solved.
    assert!(map.report.distortion_mean < 1e-3, "{:?}", map.report);

    // 2 units wide, so 20 quad edges across, along both grid directions.
    for axis in 0..2 {
        let (low, high) =
            map.uv.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p[axis]), hi.max(p[axis])));
        assert!((high - low - 20.0).abs() < 1e-2, "extent {} along axis {axis}", high - low);
    }
}

#[test]
fn a_cube_unfolds_without_any_distortion() {
    // Flat faces, a field along the edges, the eight corners as cones: an
    // exact seamless map exists, so the solver has to find it.
    let work = work_mesh(&primitives::cube(4), 6000, Some(30.0));
    let cross = field(&work, 0.0);
    let map = parametrize(&work, &cross, 0.1);

    assert_eq!(map.report.flipped_first, 0);
    assert!(map.report.distortion_mean < 1e-3, "{:?}", map.report);
    assert!(map.report.stretch_max < 1.001, "{:?}", map.report);
    assert!(worst_seam_mismatch(&work, &map) < 1e-8);
    assert_eq!(euler_of_cut_mesh(&work, &map), 1, "the cut must open the cube into a disk");
    assert_combed(&work, &cross, &map);

    // Every corner of the cube, being a singularity, sits on the cut.
    let mut on_seam = vec![false; work.mesh.positions.len()];
    for h in 0..map.cut.seam.len() {
        if map.cut.seam[h] {
            on_seam[work.mesh.triangles[h / 3][h % 3] as usize] = true;
        }
    }
    for (v, &index) in cross.indices.iter().enumerate() {
        assert!(index == 0 || on_seam[v], "singular vertex {v} is off the cut");
    }
}

#[test]
fn a_torus_is_cut_along_two_loops_and_wraps_seamlessly() {
    let (major, minor) = (1.0, 0.4);
    let work = work_mesh(&primitives::torus(major, minor, 96, 48), 8000, None);
    let cross = field(&work, 0.05);
    assert_eq!(cross.singularities(), (0, 0, 0));
    let edge_length = 0.1;
    let map = parametrize(&work, &cross, edge_length);

    assert_eq!(map.report.flipped, 0);
    assert_eq!(euler_of_cut_mesh(&work, &map), 1);
    assert!(worst_seam_mismatch(&work, &map) < 1e-8);
    // A torus cannot be flattened without stretch, but it stays moderate.
    assert!(map.report.distortion_mean < 0.35, "{:?}", map.report);

    // Crossing a seam shifts the map by a period of the torus. The shortest
    // period is once round the tube: its circumference, counted in edges.
    let topology = Topology::build(&work.mesh);
    let mut shortest = f64::INFINITY;
    for h in 0..map.cut.seam.len() {
        if map.cut.seam[h] {
            assert_eq!(map.cut.transition[h], 0, "no singularity, so no rotation across the cut");
            let (here, there) = (
                map.uv[h],
                map.uv[3 * (topology.opposite[h] as usize / 3) + (topology.opposite[h] as usize % 3 + 1) % 3],
            );
            let shift = (there[0] - here[0]).hypot(there[1] - here[1]);
            if shift > 1.0 {
                shortest = shortest.min(shift);
            }
        }
    }
    let expected = std::f64::consts::TAU * minor / edge_length;
    assert!((shortest / expected - 1.0).abs() < 0.1, "period {shortest} for an expected {expected}");
}

#[test]
fn a_sphere_with_its_eight_cones_has_no_inverted_triangle() {
    let work = work_mesh(&primitives::icosphere(4), 8000, None);
    let cross = field(&work, 0.0);
    assert_eq!(cross.singularities(), (8, 0, 0));
    let map = parametrize(&work, &cross, 0.1);
    assert_eq!(map.report.flipped, 0, "{:?}", map.report);
    assert_eq!(euler_of_cut_mesh(&work, &map), 1);
    assert!(worst_seam_mismatch(&work, &map) < 1e-8);
    assert!(map.report.distortion_mean < 0.35, "{:?}", map.report);
    assert_combed(&work, &cross, &map);
}

#[test]
fn a_busy_field_still_gives_a_valid_map() {
    // Bumps add dozens of singularities of both signs; the first solve folds
    // some triangles over, and stiffening has to undo every one.
    let work = bumpy_sphere();
    let cross = field(&work, 2.0);
    let (positive, negative, _) = cross.singularities();
    assert!(positive + negative > 20);

    let map = parametrize(&work, &cross, 0.1);
    assert!(map.report.flipped_first > 0, "the case should need stiffening: {:?}", map.report);
    assert_eq!(map.report.flipped, 0, "{:?}", map.report);
    assert_eq!(euler_of_cut_mesh(&work, &map), 1);
    assert!(worst_seam_mismatch(&work, &map) < 1e-8);
    assert_combed(&work, &cross, &map);
}

#[test]
fn parametrizers_are_reachable_by_name_deterministic_and_check_their_input() {
    let work = work_mesh(&primitives::torus(1.0, 0.4, 48, 24), 2000, None);
    let cross = field(&work, 0.05);
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(create(kind).kind(), kind);
    }
    assert!("nope".parse::<Kind>().is_err());

    let (first, second) = (parametrize(&work, &cross, 0.15), parametrize(&work, &cross, 0.15));
    assert_eq!(first.uv, second.uv);
    for edge_length in [0.0, -1.0, f64::NAN] {
        let request = Request { edge_length, stiffening_rounds: 0, distortion_passes: 0 };
        assert!(create(Kind::Seamless).parametrize(&work, &cross, &request).is_err());
    }
}

fn parametrize_with(
    work: &WorkMesh,
    field: &CrossField,
    edge_length: f64,
    kind: Kind,
    passes: usize,
) -> Parametrization {
    let request = Request { edge_length, stiffening_rounds: 40, distortion_passes: passes };
    create(kind).parametrize(work, field, &request).unwrap()
}

/// How the map shapes the quads to come, re-derived from the surface and the
/// coordinates alone: on each face the gradients of `u` and `v`, in units of
/// the local edge length, should have length 1 and be at right angles.
/// Returns the share of the area where a gradient is off by more than a
/// factor 2 or the angle by more than 20 degrees, the worst length factor
/// (in logarithm) and the worst skew (in degrees), over the faces the map
/// keeps the right way round.
fn shape_of_quads(work: &WorkMesh, map: &Parametrization, edge_length: f64) -> (f64, f64, f64) {
    let mesh = &work.mesh;
    let (mut beyond, mut total, mut worst_length, mut worst_skew) = (0.0, 0.0, 0.0f64, 0.0f64);
    for (f, tri) in mesh.triangles.iter().enumerate() {
        let p = mesh.corners(f);
        let normal = (p[1] - p[0]).cross(p[2] - p[0]);
        let area = 0.5 * normal.length();
        if map.set_aside[f] || area <= 0.0 || map.area(f) <= 0.0 {
            continue;
        }
        let normal = normal / (2.0 * area);
        let local = edge_length * (tri.iter().map(|&v| work.sizing[v as usize].ln()).sum::<f64>() / 3.0).exp();
        let gradient = |axis: usize| {
            (0..3).fold(Vec3::ZERO, |sum, i| {
                let opposite = p[(i + 2) % 3] - p[(i + 1) % 3];
                sum + normal.cross(opposite) * (map.uv[3 * f + i][axis] / (2.0 * area))
            }) * local
        };
        let (gu, gv) = (gradient(0), gradient(1));
        let length = gu.length().ln().abs().max(gv.length().ln().abs());
        let skew = (gu.dot(gv) / (gu.length() * gv.length())).clamp(-1.0, 1.0).asin().abs().to_degrees();
        if length > std::f64::consts::LN_2 || skew > 20.0 {
            beyond += area;
        }
        total += area;
        worst_length = worst_length.max(length);
        worst_skew = worst_skew.max(skew);
    }
    (beyond / total, worst_length, worst_skew)
}

/// The bumpy sphere of the busy field: dozens of singularities, folds at the
/// first solve, and faces still squashed once they are undone.
fn bumpy_sphere() -> WorkMesh {
    let mut bumpy = primitives::icosphere(5);
    for p in &mut bumpy.positions {
        *p *= 1.0 + 0.06 * (7.0 * p.x).sin() * (9.0 * p.y).sin() * (5.0 * p.z).cos();
    }
    work_mesh(&bumpy, 12_000, None)
}

#[test]
fn against_distortion_a_plane_and_a_cube_stay_rigid() {
    // Nothing on them is stretched past the tolerance: the passes find
    // nothing to do, and the map is the one made without them, bit for bit.
    let plane = work_mesh(&primitives::plane(6), 3000, None);
    let cube = work_mesh(&primitives::cube(4), 6000, Some(30.0));
    for work in [&plane, &cube] {
        let cross = field(work, 0.0);
        let (without, with) = (
            parametrize_with(work, &cross, 0.1, Kind::Seamless, 0),
            parametrize_with(work, &cross, 0.1, Kind::Seamless, 3),
        );
        assert_eq!(with.report.distortion_passes, 0, "{:?}", with.report);
        assert_eq!(with.report.flipped, 0);
        assert!(with.report.distortion_mean < 1e-3, "{:?}", with.report);
        assert_eq!(with.uv, without.uv);
    }
}

#[test]
fn against_distortion_the_worst_faces_of_a_busy_field_get_less_stretched() {
    let work = bumpy_sphere();
    let cross = field(&work, 2.0);
    let without = parametrize_with(&work, &cross, 0.1, Kind::Seamless, 0);
    let with = parametrize_with(&work, &cross, 0.1, Kind::Seamless, 2);
    assert_eq!(without.report.flipped, 0);
    assert_eq!(with.report.flipped, 0, "{:?}", with.report);
    assert!(with.report.distortion_passes > 0, "{:?}", with.report);

    // Measured from the surface, not by the module: the worst gradient
    // length (about 5.8 times the size asked for without the passes) and the
    // worst skew (near 90 degrees) both come down. The mean goes up a
    // little: the distortion the cones force is spread, not removed.
    let (_, length_without, skew_without) = shape_of_quads(&work, &without, 0.1);
    let (_, length_with, skew_with) = shape_of_quads(&work, &with, 0.1);
    assert!(length_with < 0.8 * length_without, "x{:.2} against x{:.2}", length_with.exp(), length_without.exp());
    assert!(skew_with < skew_without - 20.0, "{skew_with:.1} against {skew_without:.1} degrees");
    assert!(with.report.stretch_max < 0.25 * without.report.stretch_max, "{:?} {:?}", with.report, without.report);
    assert!(with.report.distortion_mean < 1.25 * without.report.distortion_mean);

    // Still a seamless map of the same cut.
    assert!(worst_seam_mismatch(&work, &with) < 1e-8);
    assert_eq!(euler_of_cut_mesh(&work, &with), 1);
    assert_combed(&work, &cross, &with);
}

#[test]
fn against_distortion_no_fold_is_added_and_the_map_is_deterministic() {
    // A sizing field that grades the quads by a factor 20 across a sheet
    // cannot be followed exactly: the seamless map is valid but skewed, the
    // integer map folds a few faces whatever is done.
    let mut graded = work_mesh(&primitives::plane(6), 3000, None);
    for (v, p) in graded.mesh.positions.iter().enumerate() {
        graded.sizing[v] = (1.5 * p.x).exp();
    }
    let cross = field(&graded, 0.0);
    for kind in [Kind::Seamless, Kind::IntegerGrid] {
        let without = parametrize_with(&graded, &cross, 0.1, kind, 0);
        let with = parametrize_with(&graded, &cross, 0.1, kind, 3);
        // The passes only act on a map with no fold, and the map kept has
        // the fewest folds first: never more than without them.
        assert!(with.report.flipped <= without.report.flipped, "{kind}: {:?} {:?}", with.report, without.report);
        let (_, _, skew_without) = shape_of_quads(&graded, &without, 0.1);
        let (_, _, skew_with) = shape_of_quads(&graded, &with, 0.1);
        assert!(skew_with <= skew_without, "{kind}: {skew_with:.1} against {skew_without:.1} degrees");
        assert_eq!(with.uv, parametrize_with(&graded, &cross, 0.1, kind, 3).uv, "{kind} is not deterministic");
    }
    let seamless = parametrize_with(&graded, &cross, 0.1, Kind::Seamless, 3);
    assert_eq!(seamless.report.flipped, 0);
    assert!(seamless.report.distortion_passes > 0, "{:?}", seamless.report);
}
