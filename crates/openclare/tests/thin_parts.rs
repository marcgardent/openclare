//! Thin parts: a finger keeps its volume because enough quads go round it.

use openclare::math::Vec3;
use openclare::metrics::measure;
use openclare::preprocess::{self, WorkMesh};
use openclare::sizing::{self, Kind, Request};
use openclare::{PolyMesh, RemeshConfig, Target, TriMesh};

const FINGER_RADIUS: f64 = 0.1;
const FINGER_TIP: f64 = 2.4;

/// A ball of radius 1 with a finger out of its top: a rod of radius 0.1
/// along z, as long as the ball is wide over all, with a round tip.
fn ball_with_a_finger() -> TriMesh {
    // The outline, from the bottom of the ball to the tip of the finger.
    let mut outline: Vec<(f64, f64)> = Vec::new();
    let joint = (1.0 - FINGER_RADIUS * FINGER_RADIUS).sqrt();
    let rings = 48;
    for i in 1..rings {
        let z = -1.0 + (joint + 1.0) * i as f64 / rings as f64;
        outline.push((z, (1.0 - z * z).max(0.0).sqrt()));
    }
    let straight = FINGER_TIP - FINGER_RADIUS;
    for i in 0..=40 {
        outline.push((joint + (straight - joint) * i as f64 / 40.0, FINGER_RADIUS));
    }
    for i in 1..6 {
        let angle = std::f64::consts::FRAC_PI_2 * i as f64 / 6.0;
        outline.push((straight + FINGER_RADIUS * angle.sin(), FINGER_RADIUS * angle.cos()));
    }

    let around = 48u32;
    let mut mesh = TriMesh::default();
    for &(z, radius) in &outline {
        for k in 0..around {
            let turn = std::f64::consts::TAU * k as f64 / around as f64;
            mesh.positions.push(Vec3::new(radius * turn.cos(), radius * turn.sin(), z));
        }
    }
    let (bottom, top) = (mesh.positions.len() as u32, mesh.positions.len() as u32 + 1);
    mesh.positions.push(Vec3::new(0.0, 0.0, -1.0));
    mesh.positions.push(Vec3::new(0.0, 0.0, FINGER_TIP));
    let last = outline.len() as u32 - 1;
    for k in 0..around {
        let next = (k + 1) % around;
        mesh.triangles.push([bottom, next, k]);
        mesh.triangles.push([top, last * around + k, last * around + next]);
        for ring in 0..last {
            let (a, b) = (ring * around + k, ring * around + next);
            mesh.triangles.push([a, b, b + around]);
            mesh.triangles.push([a, b + around, a + around]);
        }
    }
    mesh
}

fn work_mesh(source: &TriMesh) -> WorkMesh {
    let request = preprocess::Request { triangles: 12_000, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

/// Area-weighted mean of `values` over the vertices of `work` that `keep`.
fn mean_where(work: &WorkMesh, values: &[f64], keep: impl Fn(Vec3) -> bool) -> f64 {
    let (mut sum, mut weight) = (0.0, 0.0);
    for v in (0..values.len()).filter(|&v| keep(work.mesh.positions[v])) {
        sum += values[v] * work.areas[v];
        weight += work.areas[v];
    }
    sum / weight
}

fn on_the_finger(p: Vec3) -> bool {
    p.z > 1.3 && p.z < FINGER_TIP - 0.2
}

/// Quads round the finger, counted as the faces a plane across it cuts.
fn quads_round_the_finger(mesh: &PolyMesh, height: f64) -> usize {
    let cut = |face: &Vec<u32>| {
        let above = face.iter().filter(|&&v| mesh.positions[v as usize].z > height).count();
        above > 0 && above < face.len()
    };
    mesh.faces.iter().filter(|face| cut(face)).count()
}

#[test]
fn the_test_shape_is_closed_and_the_right_way_out() {
    let mesh = ball_with_a_finger();
    mesh.validate().unwrap();
    assert_eq!(mesh.pieces().len(), 1);
    let ball = 4.0 / 3.0 * std::f64::consts::PI;
    assert!((mesh.signed_volume() - ball).abs() < 0.08, "volume {}", mesh.signed_volume());
}

#[test]
fn a_thin_part_gets_quads_small_enough_to_go_round_it() {
    let work = work_mesh(&ball_with_a_finger());
    let sizer = sizing::create(Kind::Curvature);
    let edge_length = 0.2;
    let at = |quads_around: f64| {
        let request = Request { edge_length, adaptivity: 0.0, painted: None, quads_around, bounds: None };
        let scale = sizer.size(&work, &request).unwrap();
        // The number of quads is unchanged: on average, 1 / s² is 1.
        let density: Vec<f64> = scale.iter().map(|s| 1.0 / (s * s)).collect();
        assert!((mean_where(&work, &density, |_| true) - 1.0).abs() < 1e-9);
        (mean_where(&work, &scale, on_the_finger), mean_where(&work, &scale, |p| p.z < 0.0))
    };
    assert_eq!(at(0.0), (1.0, 1.0));
    // Six quads round a finger of radius 0.1: edges of about 0.1.
    let girth = std::f64::consts::TAU * FINGER_RADIUS;
    let (finger, ball) = at(6.0);
    assert!((finger * edge_length - girth / 6.0).abs() < 0.03, "edges of {} on the finger", finger * edge_length);
    // The rest pays for it, a little.
    assert!(ball > 1.0 && ball < 1.3, "factor {ball} on the ball");
    // More quads round it, smaller ones still.
    assert!(at(8.0).0 < 0.85 * finger);
}

#[test]
fn a_shape_with_nothing_thin_is_left_alone() {
    let work = work_mesh(&openclare::mesh::primitives::icosphere(4));
    let request = Request { edge_length: 0.15, adaptivity: 0.0, painted: None, quads_around: 6.0, bounds: None };
    let scale = sizing::create(Kind::Curvature).size(&work, &request).unwrap();
    assert!(scale.iter().all(|&s| (s - 1.0).abs() < 1e-9));
}

#[test]
fn a_finger_keeps_its_volume() {
    let source = ball_with_a_finger();
    let remesh = |quads_around: f64| {
        let config =
            RemeshConfig { adaptivity: 0.0, quads_around, ..RemeshConfig::for_organic(Target::QuadCount(300)) };
        let (mesh, report) = config.remesh(&source).unwrap();
        assert!(!report.fell_back, "the layout chain gave up");
        let metrics = measure(&source, &mesh, Some(300));
        assert!(metrics.is_valid());
        assert!(metrics.budget_error.unwrap() < 0.1, "{} faces", metrics.faces);
        (quads_round_the_finger(&mesh, 1.8), metrics.thin_distance.unwrap())
    };
    let (few, far) = remesh(0.0);
    let (enough, near) = remesh(6.0);
    assert!(enough >= 5 && enough > few, "{few} quads round the finger without, {enough} with");
    assert!(near < 0.6 * far, "thin parts {far} quad edges away without, {near} with");
}

#[test]
fn asked_for_sizes_the_finger_gets_its_quads_and_the_ball_comes_down_to_them_gently() {
    let work = work_mesh(&ball_with_a_finger());
    let request = Request { edge_length: 0.2, bounds: Some((0.3, 0.02)), adaptivity: 0.0, painted: None, quads_around: 6.0 };
    let tuning = sizing::Curvature::default();
    let demand = sizing::Demand::measure(&tuning, &work, &request).unwrap();
    let sizes = demand.sizes(&sizing::Reading::between(0.3, 0.02));
    // Round the finger, a sixth of its girth of 0.63; under the ball, far
    // from it and thick, the largest quad.
    let finger = mean_where(&work, &sizes, |p| on_the_finger(p) && p.z > 1.5 && p.z < 2.0);
    let bottom = mean_where(&work, &sizes, |p| p.z < -0.5);
    assert!((finger - 0.105).abs() < 0.015, "an edge of {finger} round the finger");
    assert!((bottom - 0.3).abs() < 1e-9, "an edge of {bottom} under the ball");
    // From the finger down the ball, no size grows faster than the
    // gradation lets it, and somewhere it grows that fast.
    let (mut steepest, mut at_the_limit) = (0.0f64, 0);
    for tri in &work.mesh.triangles {
        for k in 0..3 {
            let (a, b) = (tri[k] as usize, tri[(k + 1) % 3] as usize);
            let length = work.mesh.positions[a].distance(work.mesh.positions[b]);
            let rate = (sizes[a] - sizes[b]).abs() / length;
            steepest = steepest.max(rate);
            at_the_limit += (rate > 0.9 * tuning.gradation) as usize;
        }
    }
    assert!(steepest <= tuning.gradation + 1e-9, "the size grows by {steepest} per length");
    assert!(at_the_limit > 10, "{at_the_limit} edges where it grows as fast as allowed");
}
