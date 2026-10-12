//! Sizing: quads smaller where the shape bends or where the user painted
//! more of them, for the same number in total.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::preprocess::{self, WorkMesh};
use openclare::sizing::{self, Kind, Painted, Request};
use openclare::{Guides, PolyMesh, RemeshConfig, Target, TriMesh};

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
    let request = Request { edge_length: 0.15, adaptivity: 0.0, painted: None, quads_around: 0.0, bounds: None };
    let scale = sizing::create(Kind::Curvature).size(&work, &request).unwrap();
    assert!(scale.iter().all(|&s| s == 1.0));
}

#[test]
fn quads_get_smaller_where_the_surface_bends_more() {
    let work = work_mesh(&spindle());
    let sizer = sizing::create(Kind::Curvature);
    let at = |adaptivity: f64| {
        let scale =
            sizer.size(&work, &Request { edge_length: 0.15, adaptivity, painted: None, quads_around: 0.0, bounds: None }).unwrap();
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
        sizer.size(&work, &Request { edge_length: 0.1, adaptivity: 0.0, painted: Some(painted), quads_around: 0.0, bounds: None })
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
        sizer.size(&work, &Request { edge_length: 0.1, adaptivity: 2.0, painted: None, quads_around: 0.0, bounds: None }).is_err()
    );
}

#[test]
fn the_mesh_follows_the_painted_density_and_keeps_its_budget() {
    let source = primitives::icosphere(4);
    let weights: Vec<f64> = source.positions.iter().map(|p| if p.z > 0.0 { 1.0 } else { 0.0 }).collect();
    let config =
        RemeshConfig { adaptivity: 0.0, density_contrast: 9.0, ..RemeshConfig::for_organic(Target::QuadCount(2000)) };
    let (mesh, _) = config.remesh_guided(&source, &Guides { density: Some(&weights), ..Default::default() }).unwrap();
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

// The size asked for, between a largest and a smallest quad, and the number
// of quads as what that makes.

use openclare::sizing::{Curvature, Demand};

#[test]
fn a_ball_calls_for_the_same_number_of_quads_whatever_its_size() {
    // The largest quad as long as the radius. Twelve of them would go round
    // a ball of radius 1.9: this one bends more, and for an even error gets
    // an edge half way, as a logarithm, between the largest and a twelfth of
    // its great circle. That is 0.72 of its radius, and 24 quads over it,
    // be it a pea or a planet.
    for radius in [0.5, 1.0, 4.0] {
        let mut ball = primitives::icosphere(4);
        for p in &mut ball.positions {
            *p *= radius;
        }
        let work = work_mesh(&ball);
        let (largest, smallest) = (radius, 0.001 * radius);
        let request = Request {
            edge_length: 0.1 * radius,
            bounds: Some((largest, smallest)),
            adaptivity: 1.0,
            painted: None,
            quads_around: 0.0,
        };
        let tuning = Curvature::default();
        assert_eq!(tuning.quads_a_turn, 12.0);
        let demand = Demand::measure(&tuning, &work, &request).unwrap();
        let wanted = radius * (std::f64::consts::TAU / 12.0).sqrt();
        for size in demand.sizes(largest, largest, smallest) {
            assert!((size / wanted - 1.0).abs() < 0.03, "radius {radius}: an edge of {size} for {wanted}");
        }
        let quads = demand.quads(largest, largest, smallest);
        assert!((quads / 24.0 - 1.0).abs() < 0.05, "radius {radius}: {quads} quads for 24");
        // Nothing of it asks for less than the smallest; all of it asks for
        // less than a smallest set above that edge.
        assert_eq!(demand.below(largest, smallest), 0.0);
        assert_eq!(demand.below(largest, 1.1 * wanted), 1.0);
        // Without adaptivity it does not ask: the largest quad everywhere.
        let deaf = Demand::measure(&tuning, &work, &Request { adaptivity: 0.0, ..request }).unwrap();
        assert!(deaf.sizes(largest, largest, smallest).iter().all(|&size| size == largest));
    }
}

#[test]
fn the_bounds_are_kept_and_tighter_ones_make_more_quads() {
    let work = work_mesh(&spindle());
    let request =
        Request { edge_length: 0.15, bounds: Some((0.4, 0.05)), adaptivity: 1.0, painted: None, quads_around: 0.0 };
    let demand = Demand::measure(&Curvature::default(), &work, &request).unwrap();
    // The very tips bend along a circle of radius a third, which the
    // largest quad would go round in five: an edge of 0.26 there for an
    // even error, and more a little way off, where it bends less already.
    // The waist bends along a circle of radius 1, which the largest goes
    // round in more than twelve: flat.
    let sizes = demand.sizes(0.4, 0.4, 0.05);
    assert!(sizes.iter().all(|&size| (0.05..=0.4).contains(&size)));
    let (tips, waist) = (mean_where(&work, &sizes, |p| p.z.abs() > 2.9), mean_where(&work, &sizes, |p| p.z.abs() < 0.3));
    let least = sizes.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(least > 0.26 && least <= tips && tips < 0.36, "an edge of {least} at the very tips, {tips} round them");
    assert!((waist - 0.4).abs() < 1e-9, "an edge of {waist} at the waist");
    // A smallest quad above what the tips ask is what they get.
    let floored = demand.sizes(0.4, 0.4, 0.35);
    assert!(floored.iter().all(|&size| (0.35..=0.4).contains(&size)));
    assert!((mean_where(&work, &floored, |p| p.z.abs() > 2.9) - 0.35).abs() < 1e-9);
    let below = demand.below(0.4, 0.35);
    assert!(below > 0.01 && below < 0.5, "{below}");

    // The number of quads: more with a smaller largest quad, fewer with a
    // larger smallest one, and what quads of one size make when the two
    // are the same.
    let quads = |largest: f64, smallest: f64| demand.quads(largest, largest, smallest);
    assert!(quads(0.3, 0.05) > 1.2 * quads(0.4, 0.05));
    assert!(quads(0.4, 0.35) < quads(0.4, 0.05) && quads(0.4, 0.05) <= quads(0.4, 0.01));
    let area: f64 = work.areas.iter().sum();
    assert!((quads(0.2, 0.2) - area / 0.04).abs() < 1e-6 * area);
}

#[test]
fn asked_for_sizes_the_mesh_has_them_and_the_report_says_how_many_quads_that_made() {
    let source = spindle();
    let config = RemeshConfig {
        adaptivity: 1.0,
        quads_around: 0.0,
        ..RemeshConfig::for_organic(Target::SizeRange { largest: 0.3, smallest: 0.1 })
    };
    let (mesh, report) = config.remesh(&source).unwrap();
    assert!(measure(&source, &mesh, None).is_valid());
    assert_eq!((report.largest_edge, report.smallest_edge), (0.3, 0.1));
    // The quads the shape was found to call for are the quads made.
    let made = mesh.faces.len() as f64;
    assert!((made / report.quads_wanted - 1.0).abs() < 0.1, "{made} quads for {} called for", report.quads_wanted);
    assert_eq!(report.below_smallest, 0.0);
    // The edges of the mesh follow, as far as its layout lets them: a
    // spindle has its eight singularities at its tips, and as many quads
    // round its waist as round its ends, which makes those of the waist
    // longer than asked and those of the tips shorter.
    let (tips, waist) = (mean_edge(&mesh, |p| p.z.abs() > 2.7), mean_edge(&mesh, |p| p.z.abs() < 0.8));
    assert!(tips < 0.7 * waist && tips > 0.1, "edges of {tips} at the tips");
    assert!(waist > 0.3 && waist < 0.4, "edges of {waist} at the waist");

    // Quads of one size when the two bounds are the same.
    let even = RemeshConfig { target: Target::SizeRange { largest: 0.2, smallest: 0.2 }, ..config.clone() };
    let (mesh, report) = even.remesh(&source).unwrap();
    let (tips, waist) = (mean_edge(&mesh, |p| p.z.abs() > 2.5), mean_edge(&mesh, |p| p.z.abs() < 0.8));
    assert!(waist > 0.2 && waist < 0.27, "edges of {waist} at the waist");
    assert!(tips > 0.1 && tips < 0.2, "edges of {tips} at the tips");
    assert!((mesh.faces.len() as f64 / (source.surface_area() / 0.04) - 1.0).abs() < 0.1);
    assert!((report.quads_wanted / (source.surface_area() / 0.04) - 1.0).abs() < 0.05);

    // More quads called for than may be made: every quad larger, and said.
    let capped = RemeshConfig { most_quads: 150, ..config.clone() };
    let (mesh, report) = capped.remesh(&source).unwrap();
    assert!(report.quads_wanted > 300.0, "{} quads called for", report.quads_wanted);
    assert!((mesh.faces.len() as f64 / 150.0 - 1.0).abs() < 0.1, "{} quads", mesh.faces.len());
    let larger = (report.quads_wanted / 150.0).sqrt();
    assert!((report.largest_edge - 0.3 * larger).abs() < 1e-9 && (report.smallest_edge - 0.1 * larger).abs() < 1e-9);

    // Sizes that are not sizes.
    for (largest, smallest) in [(0.1, 0.3), (0.3, 0.0), (0.3, -0.1), (f64::NAN, 0.1), (f64::INFINITY, 0.1)] {
        let wrong = RemeshConfig { target: Target::SizeRange { largest, smallest }, ..config.clone() };
        assert!(wrong.remesh(&source).is_err(), "quads from {smallest} to {largest}");
    }
}
