//! Sizing: quads smaller where the shape bends or where the user painted
//! more of them, for the same number in total.

use retopo::math::Vec3;
use retopo::mesh::primitives;
use retopo::metrics::measure;
use retopo::preprocess::{self, WorkMesh};
use retopo::sizing::{self, Kind, Painted, Request};
use retopo::{Guides, PolyMesh, RemeshConfig, Target, TriMesh};

fn work_mesh(source: &TriMesh) -> WorkMesh {
    let request = preprocess::Request { triangles: 8000, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

/// A sphere stretched three times along z: tips that bend much more than
/// its waist.
fn spindle() -> TriMesh {
    let mut mesh = primitives::icosphere(4);
    for p in &mut mesh.positions {
        p.z *= 3.0;
    }
    mesh
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

/// Mean length of the edges of `mesh` whose middle `keep`s.
fn mean_edge(mesh: &PolyMesh, keep: impl Fn(Vec3) -> bool) -> f64 {
    let (mut sum, mut count) = (0.0, 0.0);
    for face in &mesh.faces {
        for k in 0..face.len() {
            let (a, b) = (mesh.positions[face[k] as usize], mesh.positions[face[(k + 1) % face.len()] as usize]);
            if keep(0.5 * (a + b)) {
                sum += a.distance(b);
                count += 1.0;
            }
        }
    }
    sum / count
}

#[test]
fn without_adaptivity_or_paint_every_quad_is_the_size_asked_for() {
    let work = work_mesh(&spindle());
    assert!(work.sizing.iter().all(|&s| s == 1.0));
    let request = Request { edge_length: 0.15, adaptivity: 0.0, painted: None, quads_around: 0.0 };
    let scale = sizing::create(Kind::Curvature).size(&work, &request).unwrap();
    assert!(scale.iter().all(|&s| s == 1.0));
}

#[test]
fn quads_get_smaller_where_the_surface_bends_more() {
    let work = work_mesh(&spindle());
    let sizer = sizing::create(Kind::Curvature);
    let at = |adaptivity: f64| {
        let scale =
            sizer.size(&work, &Request { edge_length: 0.15, adaptivity, painted: None, quads_around: 0.0 }).unwrap();
        // The number of quads is unchanged: on average, 1 / s² is 1.
        let density: Vec<f64> = scale.iter().map(|s| 1.0 / (s * s)).collect();
        assert!((mean_where(&work, &density, |_| true) - 1.0).abs() < 1e-9);
        (mean_where(&work, &scale, |p| p.z.abs() > 2.6), mean_where(&work, &scale, |p| p.z.abs() < 0.5))
    };
    let (tips, waist) = at(0.5);
    assert!(tips < 0.9 && waist > 1.02, "tips {tips}, waist {waist}");
    // The more adaptive, the wider the gap; never beyond the allowed range.
    let (sharper_tips, wider_waist) = at(1.0);
    assert!(sharper_tips < tips && wider_waist > waist);
    assert!(sharper_tips > 1.0 / 3.5);
}

#[test]
fn painted_density_sets_the_size_and_blends_across_its_border() {
    let source = primitives::icosphere(4);
    let work = work_mesh(&source);
    let sizer = sizing::create(Kind::Curvature);
    let sized = |painted: Painted| {
        sizer.size(&work, &Request { edge_length: 0.1, adaptivity: 0.0, painted: Some(painted), quads_around: 0.0 })
    };
    // The northern half painted at 1, the southern left at 0, sixteen times
    // the quads per area between the two: edges four times shorter.
    let weights: Vec<f64> = source.positions.iter().map(|p| if p.z > 0.0 { 1.0 } else { 0.0 }).collect();
    let scale = sized(Painted { source: &source, weights: &weights, contrast: 16.0 }).unwrap();
    let (north, south) = (mean_where(&work, &scale, |p| p.z > 0.6), mean_where(&work, &scale, |p| p.z < -0.6));
    assert!((south / north - 4.0).abs() < 0.3, "north {north}, south {south}");
    // No jump at the equator: neighbours differ by little.
    let equator = mean_where(&work, &scale, |p| p.z.abs() < 0.03);
    assert!(equator > 1.3 * north && equator < south / 1.3, "equator {equator}");

    // Only the difference between weights counts: paint everything alike,
    // or ask for no contrast, and nothing changes.
    let even = vec![0.7; weights.len()];
    let unchanged = sized(Painted { source: &source, weights: &even, contrast: 16.0 }).unwrap();
    assert!(unchanged.iter().all(|s| (s - 1.0).abs() < 1e-9));
    let unchanged = sized(Painted { source: &source, weights: &weights, contrast: 1.0 }).unwrap();
    assert!(unchanged.iter().all(|s| (s - 1.0).abs() < 1e-9));

    assert!(sized(Painted { source: &source, weights: &weights[1..], contrast: 16.0 }).is_err());
    assert!(sized(Painted { source: &source, weights: &vec![1.5; weights.len()], contrast: 16.0 }).is_err());
    assert!(sized(Painted { source: &source, weights: &weights, contrast: 0.5 }).is_err());
    assert!(
        sizer.size(&work, &Request { edge_length: 0.1, adaptivity: 2.0, painted: None, quads_around: 0.0 }).is_err()
    );
}

#[test]
fn the_mesh_follows_the_painted_density_and_keeps_its_budget() {
    let source = primitives::icosphere(4);
    let weights: Vec<f64> = source.positions.iter().map(|p| if p.z > 0.0 { 1.0 } else { 0.0 }).collect();
    let config =
        RemeshConfig { adaptivity: 0.0, density_contrast: 9.0, ..RemeshConfig::for_organic(Target::QuadCount(2000)) };
    let (mesh, _) = config.remesh_guided(&source, &Guides { density: Some(&weights) }).unwrap();
    let metrics = measure(&source, &mesh, Some(2000));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces", metrics.faces);
    // Nine times the density: edges three times shorter, give or take.
    let (north, south) = (mean_edge(&mesh, |p| p.z > 0.5), mean_edge(&mesh, |p| p.z < -0.5));
    assert!(south / north > 2.2 && south / north < 3.8, "north {north}, south {south}");

    // Without paint the same settings give quads of one size.
    let (plain, _) = config.remesh(&source).unwrap();
    let (north, south) = (mean_edge(&plain, |p| p.z > 0.5), mean_edge(&plain, |p| p.z < -0.5));
    assert!((south / north - 1.0).abs() < 0.15, "north {north}, south {south}");
}

#[test]
fn the_mesh_is_finer_at_the_tips_of_a_spindle_when_adaptive() {
    let source = spindle();
    let ratio = |adaptivity: f64| {
        let config = RemeshConfig { adaptivity, ..RemeshConfig::for_organic(Target::QuadCount(1500)) };
        let (mesh, _) = config.remesh(&source).unwrap();
        let metrics = measure(&source, &mesh, Some(1500));
        assert!(metrics.is_valid(), "{metrics:?}");
        assert!(metrics.budget_error.unwrap() <= 0.05, "{} faces", metrics.faces);
        (mean_edge(&mesh, |p| p.z.abs() > 2.5) / mean_edge(&mesh, |p| p.z.abs() < 0.8), metrics.hausdorff)
    };
    let (uniform, coarse_error) = ratio(0.0);
    let (adaptive, fine_error) = ratio(1.0);
    // (Even uniform, the caps are finer than the waist: each holds four
    // singularities in little room.)
    assert!(adaptive < 0.9 * uniform, "tips over waist: {uniform} uniform, {adaptive} adaptive");
    // Smaller quads where it bends: the tips are followed more closely.
    assert!(fine_error < coarse_error, "{coarse_error} uniform, {fine_error} adaptive");
}
