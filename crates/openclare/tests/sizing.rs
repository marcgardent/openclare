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
        // Sizes alone: closing the loops round the spindle may stretch
        // the lengths a little, which is not what is looked at here.
        let config =
            RemeshConfig { adaptivity, close_loops: false, ..RemeshConfig::for_organic(Target::QuadCount(1500)) };
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

use openclare::sizing::{Curvature, Demand, Reading, Sizer};

#[test]
fn a_ball_calls_for_the_same_number_of_quads_whatever_its_size() {
    // The largest quad as long as the radius. Twelve of them would go round
    // a ball of radius 1.9: this one bends more, and for an even error gets
    // an edge half way, as a logarithm, between the largest and a twelfth of
    // its great circle (of a circle a twentieth tighter: what is added to
    // every curvature for the flat not to be flatter than quads can feel).
    // That is 0.71 of its radius, and 25 quads over it, be it a pea or a
    // planet.
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
        let wanted = radius * (std::f64::consts::TAU / 12.0 / (1.0 + tuning.felt)).sqrt();
        for size in demand.sizes(&Reading::between(largest, smallest)) {
            assert!((size / wanted - 1.0).abs() < 0.03, "radius {radius}: an edge of {size} for {wanted}");
        }
        let quads = demand.quads(&Reading::between(largest, smallest));
        let expected = 24.0 * (1.0 + tuning.felt);
        assert!((quads / expected - 1.0).abs() < 0.05, "radius {radius}: {quads} quads for {expected}");
        // Nothing of it asks for less than the smallest; all of it asks for
        // less than a smallest set above that edge.
        assert_eq!(demand.below(&Reading::between(largest, smallest)), 0.0);
        assert_eq!(demand.below(&Reading::between(largest, 1.1 * wanted)), 1.0);
        // Without adaptivity it does not ask: the largest quad everywhere.
        let deaf = Demand::measure(&tuning, &work, &Request { adaptivity: 0.0, ..request }).unwrap();
        assert!(deaf.sizes(&Reading::between(largest, smallest)).iter().all(|&size| size == largest));
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
    let sizes = demand.sizes(&Reading::between(0.4, 0.05));
    assert!(sizes.iter().all(|&size| (0.05..=0.4).contains(&size)));
    let (tips, waist) = (mean_where(&work, &sizes, |p| p.z.abs() > 2.9), mean_where(&work, &sizes, |p| p.z.abs() < 0.3));
    let least = sizes.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(least > 0.26 && least <= tips && tips < 0.36, "an edge of {least} at the very tips, {tips} round them");
    assert!((waist - 0.4).abs() < 1e-9, "an edge of {waist} at the waist");
    // A smallest quad above what the tips ask is what they get.
    let floored = demand.sizes(&Reading::between(0.4, 0.35));
    assert!(floored.iter().all(|&size| (0.35..=0.4).contains(&size)));
    assert!((mean_where(&work, &floored, |p| p.z.abs() > 2.9) - 0.35).abs() < 1e-9);
    let below = demand.below(&Reading::between(0.4, 0.35));
    assert!(below > 0.01 && below < 0.5, "{below}");

    // The number of quads: more with a smaller largest quad, fewer with a
    // larger smallest one, and what quads of one size make when the two
    // are the same.
    let quads = |largest: f64, smallest: f64| demand.quads(&Reading::between(largest, smallest));
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
    // (So few quads come in steps of whole rows: up to 15% fewer.)
    let ratio = mesh.faces.len() as f64 / 150.0;
    assert!(ratio > 0.85 && ratio < 1.1, "{} quads", mesh.faces.len());
    let larger = (report.quads_wanted / 150.0).sqrt();
    assert!((report.largest_edge - 0.3 * larger).abs() < 1e-9 && (report.smallest_edge - 0.1 * larger).abs() < 1e-9);

    // Sizes that are not sizes.
    for (largest, smallest) in [(0.1, 0.3), (0.3, 0.0), (0.3, -0.1), (f64::NAN, 0.1), (f64::INFINITY, 0.1)] {
        let wrong = RemeshConfig { target: Target::SizeRange { largest, smallest }, ..config.clone() };
        assert!(wrong.remesh(&source).is_err(), "quads from {smallest} to {largest}");
    }
}

#[test]
fn a_shape_is_analysed_once_and_any_bounds_are_then_counted() {
    use openclare::Analysis;
    let source = spindle();
    let config = RemeshConfig { adaptivity: 1.0, quads_around: 6.0, ..RemeshConfig::for_organic(Target::QuadCount(1)) };
    let analysis = Analysis::of(&source, &Guides::default(), &config).unwrap();
    assert!((analysis.area() / source.surface_area() - 1.0).abs() < 0.01);
    // Quads of one size: the area over their own.
    assert!((analysis.quads(0.2, 0.2) / (source.surface_area() / 0.04) - 1.0).abs() < 0.01);
    // What the analysis counts is what the chain then makes.
    let wanted = analysis.quads(0.3, 0.1);
    let sized = RemeshConfig { target: Target::SizeRange { largest: 0.3, smallest: 0.1 }, ..config.clone() };
    let (mesh, _) = sized.remesh(&source).unwrap();
    assert!((mesh.faces.len() as f64 / wanted - 1.0).abs() < 0.1, "{} quads for {wanted} counted", mesh.faces.len());
    // A lower smallest quad never makes fewer quads, nor leaves more of
    // the surface asking for less.
    let mut before = (0.0, 1.0);
    for over in [1.0, 1.5, 2.0, 3.0, 4.0, 8.0] {
        let now = (analysis.quads(0.4, 0.4 / over), analysis.below(0.4, 0.4 / over));
        assert!(now.0 >= before.0 && now.1 <= before.1, "1/{over}: {now:?} after {before:?}");
        before = now;
    }
    assert!(analysis.below(0.4, 0.4) > 0.02 && before.1 == 0.0);
    // The proposal: the largest quad of a mesh of 1,500, and a smallest
    // between it and a sixteenth of it, where coming down one more step
    // would not be worth it.
    let (largest, smallest) = analysis.proposal();
    assert!((largest - (analysis.area() / 1500.0).sqrt()).abs() < 1e-12);
    assert!(smallest <= largest && smallest >= largest / 16.0);
    let finer = smallest / 1.5;
    if finer >= largest / 16.0 {
        let gained = 100.0 * (analysis.below(largest, smallest) - analysis.below(largest, finer));
        let cost = 100.0 * (analysis.quads(largest, finer) / analysis.quads(largest, smallest) - 1.0);
        assert!(gained < 0.25 * cost, "one more step: {gained} points of the surface for {cost} % of quads");
    }

    // Asked for a number instead: the sizes the chain gives them, smaller
    // at the tips of the spindle, where it bends, than round its middle,
    // and of about the edge of as many quads of one size.
    let sizes = analysis.at_vertices_for(800.0);
    assert_eq!(sizes.len(), source.positions.len());
    assert!(sizes.iter().all(|size| size.is_finite() && *size > 0.0));
    let mean_of = |keep: &dyn Fn(Vec3) -> bool| {
        let kept: Vec<f64> = (0..sizes.len()).filter(|&v| keep(source.positions[v])).map(|v| sizes[v]).collect();
        kept.iter().sum::<f64>() / kept.len() as f64
    };
    let (tips, middle) = (mean_of(&|p: Vec3| p.z.abs() > 2.7), mean_of(&|p: Vec3| p.z.abs() < 0.5));
    let edge = (analysis.area() / 800.0).sqrt();
    assert!(tips < 0.8 * middle, "{tips} at the tips, {middle} round the middle");
    assert!(middle > 0.7 * edge && tips < 1.4 * edge, "{tips} and {middle} for {edge}");

    // A ring of wire beside a ball: thin all along, a sixth of its girth is
    // far under the largest quad, and on a twentieth of the surface: a part,
    // which the proposal comes down to whatever it costs.
    let mut both = primitives::icosphere(4);
    let first = both.positions.len() as u32;
    let ring = primitives::torus(0.6, 0.02, 96, 12);
    both.positions.extend(ring.positions.iter().map(|&p| p + Vec3::new(4.0, 0.0, 0.0)));
    both.triangles.extend(ring.triangles.iter().map(|t| t.map(|v| v + first)));
    let analysis = Analysis::of(&both, &Guides::default(), &config).unwrap();
    let (largest, smallest) = analysis.proposal();
    let wire = std::f64::consts::TAU * 0.02 / 6.0;
    assert!(smallest <= 1.1 * wire && wire < 0.5 * largest, "quads from {smallest} to {largest} for a wire asking {wire}");

    // What cannot be analysed.
    assert!(Analysis::of(&TriMesh::default(), &Guides::default(), &config).is_err());
}

#[test]
fn what_the_sizer_cannot_size_is_refused() {
    let work = work_mesh(&spindle());
    let fine = Request { edge_length: 0.15, bounds: Some((0.3, 0.1)), adaptivity: 0.5, painted: None, quads_around: 6.0 };
    let tuning = Curvature::default();
    assert!(Demand::measure(&tuning, &work, &fine).is_ok() && tuning.size(&work, &fine).is_ok());
    for quads_around in [f64::NAN, -1.0, f64::INFINITY] {
        assert!(Demand::measure(&tuning, &work, &Request { quads_around, ..fine }).is_err(), "{quads_around} round");
    }
    for edge_length in [0.0, -0.1, f64::NAN, f64::INFINITY] {
        assert!(Demand::measure(&tuning, &work, &Request { edge_length, ..fine }).is_err(), "an edge of {edge_length}");
    }
    let nan = f64::NAN;
    for bounds in [(nan, 0.1), (0.3, nan), (0.1, 0.3), (f64::INFINITY, 0.1), (0.3, 0.0), (0.3, -0.1)] {
        assert!(tuning.size(&work, &Request { bounds: Some(bounds), ..fine }).is_err(), "bounds {bounds:?}");
    }
}

#[test]
fn painted_with_sizes_asked_for_the_paint_is_read_where_each_point_is() {
    // A coarse ball, painted from 0 at its bottom to 1 at its top, linearly
    // in z: between two of its vertices the paint is read across the
    // triangle, as a linear weight is, and a point of the work mesh at
    // height z has the weight (z + 1) / 2. Sixteen times the quads at the
    // top: an edge four times shorter, a factor 4 to the power of the
    // weight in between.
    let source = primitives::icosphere(2);
    let work = work_mesh(&source);
    let weights: Vec<f64> = source.positions.iter().map(|p| 0.5 * (p.z + 1.0)).collect();
    let painted = Painted { source: &source, weights: &weights, contrast: 16.0 };
    let request =
        Request { edge_length: 0.1, bounds: Some((0.4, 0.01)), adaptivity: 0.0, painted: Some(painted), quads_around: 0.0 };
    let demand = Demand::measure(&Curvature::default(), &work, &request).unwrap();
    let sizes = demand.sizes(&Reading::between(0.4, 0.01));
    let mut worst = 0.0f64;
    for (v, p) in work.mesh.positions.iter().enumerate() {
        if p.z.abs() < 0.8 {
            let expected = 0.4 * 4f64.powf(-0.5 * (p.z + 1.0));
            worst = worst.max((sizes[v] / expected - 1.0).abs());
        }
    }
    assert!(worst < 0.05, "an edge {worst} off what the paint asks");

    // Painted all alike, the paint asks for nothing: the largest quad.
    let even = vec![0.7; weights.len()];
    let request = Request { painted: Some(Painted { weights: &even, ..painted }), ..request };
    let demand = Demand::measure(&Curvature::default(), &work, &request).unwrap();
    assert!(demand.sizes(&Reading::between(0.4, 0.01)).iter().all(|&size| (size - 0.4).abs() < 1e-9));
}

#[test]
fn asked_for_a_number_a_bend_takes_a_quad_no_farther_than_the_range_from_the_level() {
    // The rims of a cylinder bend as sharply as can be, its caps not at
    // all: with a range of 1.5 either way, the sizes span no more than 1.5
    // squared.
    let work = work_mesh(&primitives::cylinder(12));
    let tuning = Curvature { shape_range: 1.5, ..Default::default() };
    let request = Request { edge_length: 0.15, bounds: None, adaptivity: 1.0, painted: None, quads_around: 0.0 };
    let span = |tuning: &Curvature| {
        let scale = tuning.size(&work, &request).unwrap();
        let (least, most) = scale.iter().fold((f64::INFINITY, 0.0f64), |(least, most), &s| (least.min(s), most.max(s)));
        most / least
    };
    // 1.92 measured on 14 October 2026, and 2.90 with a range of 3: the
    // bends reach the range, and the smoothing takes a little off.
    let narrow = span(&tuning);
    assert!(narrow > 1.7 && narrow <= 2.25 + 1e-9, "sizes over a span of {narrow}");
    // Wider with a wider range.
    let wide = span(&Curvature::default());
    assert!(wide > narrow + 0.2, "sizes over a span of {wide} with a range of 3, {narrow} with 1.5");
}
