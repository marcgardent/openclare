//! A model made of several separate pieces: each is meshed on its own, with
//! its share of the quads.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::{Guides, PolyMesh, RemeshConfig, Target, TriMesh};

/// `pieces` in one mesh, each moved by its offset and scaled.
fn assembly(pieces: &[(TriMesh, Vec3, f64)]) -> TriMesh {
    let mut mesh = TriMesh::default();
    for (piece, offset, scale) in pieces {
        let first = mesh.positions.len() as u32;
        mesh.positions.extend(piece.positions.iter().map(|&p| p * *scale + *offset));
        mesh.triangles.extend(piece.triangles.iter().map(|tri| tri.map(|v| v + first)));
    }
    mesh
}

/// Faces of `mesh` whose first vertex `keep`s.
fn faces_where(mesh: &PolyMesh, keep: impl Fn(Vec3) -> bool) -> usize {
    mesh.faces.iter().filter(|face| keep(mesh.positions[face[0] as usize])).count()
}

fn config(quads: usize) -> RemeshConfig {
    RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(quads)) }
}

#[test]
fn a_mesh_is_split_into_its_connected_pieces() {
    let far = Vec3::new(10.0, 0.0, 0.0);
    let mesh = assembly(&[(primitives::icosphere(2), Vec3::ZERO, 1.0), (primitives::icosphere(2), far, 3.0)]);
    let pieces = mesh.pieces();
    assert_eq!(pieces.len(), 2);
    // Largest first, and every vertex traced back to where it was.
    assert!(pieces[0].0.surface_area() > 8.0 * pieces[1].0.surface_area());
    for (piece, origin) in &pieces {
        assert_eq!(piece.positions.len(), origin.len());
        for (p, &from) in piece.positions.iter().zip(origin) {
            assert_eq!(*p, mesh.positions[from as usize]);
        }
        assert_eq!(piece.pieces().len(), 1);
    }
    let triangles: usize = pieces.iter().map(|(piece, _)| piece.triangles.len()).sum();
    assert_eq!(triangles, mesh.triangles.len());
}

#[test]
fn each_piece_gets_quads_in_proportion_to_its_area() {
    // A sphere and another twice as large: four times the area.
    let far = Vec3::new(10.0, 0.0, 0.0);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (primitives::icosphere(4), far, 2.0)]);
    let (quads, report) = config(2000).remesh(&mesh).unwrap();
    assert_eq!((report.pieces, report.pieces_fell_back, report.pieces_failed), (2, 0, 0));
    let stats = quads.stats();
    assert_eq!((stats.quads, stats.non_manifold_edges, stats.boundary_edges), (stats.faces, 0, 0));
    assert!((stats.faces as f64 - 2000.0).abs() < 100.0, "{} faces", stats.faces);
    let (small, large) = (faces_where(&quads, |p| p.x < 5.0), faces_where(&quads, |p| p.x > 5.0));
    assert!((large as f64 / small as f64 - 4.0).abs() < 0.5, "{small} and {large} faces");
}

#[test]
fn a_ring_next_to_a_ball_does_not_send_the_ball_to_the_other_chain() {
    let far = Vec3::new(10.0, 0.0, 0.0);
    let mesh =
        assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (primitives::torus(1.0, 0.4, 96, 48), far, 1.0)]);
    let (quads, report) = config(2500).remesh(&mesh).unwrap();
    assert_eq!((report.pieces, report.pieces_fell_back, report.pieces_failed), (2, 0, 0));
    assert!(!report.fell_back);
    let stats = quads.stats();
    assert_eq!((stats.quads, stats.non_manifold_edges), (stats.faces, 0));
    // The ball has its singularities, the ring none.
    assert_eq!(stats.irregular_vertices, report.singularities.0 + report.singularities.1);
}

#[test]
fn a_piece_too_small_for_its_share_still_gets_a_mesh() {
    // A bead a hundredth the size of the ball: its fair share of 1000 quads
    // is a tenth of a quad. It gets the six a closed shape needs.
    let far = Vec3::new(10.0, 0.0, 0.0);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (primitives::icosphere(3), far, 0.01)]);
    let (quads, report) = config(1000).remesh(&mesh).unwrap();
    assert_eq!((report.pieces, report.pieces_failed), (2, 0));
    let bead = faces_where(&quads, |p| p.x > 5.0);
    assert!((6..=24).contains(&bead), "{bead} faces on the bead");
    // And the ball keeps nearly all of the budget.
    assert!(faces_where(&quads, |p| p.x < 5.0) > 900);
}

#[test]
fn pieces_too_small_can_be_left_out_instead() {
    let far = Vec3::new(10.0, 0.0, 0.0);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (primitives::icosphere(3), far, 0.01)]);
    let settings = RemeshConfig { skip_pieces_under: 1.0, ..config(1000) };
    let (quads, report) = settings.remesh(&mesh).unwrap();
    assert_eq!((report.pieces, report.pieces_skipped, report.pieces_failed), (2, 1, 0));
    assert_eq!(faces_where(&quads, |p| p.x > 5.0), 0);
    assert!((quads.faces.len() as f64 - 1000.0).abs() < 50.0, "{} faces", quads.faces.len());
    // The setting survives being saved and read back.
    assert_eq!(RemeshConfig::from_text(&settings.to_text()).unwrap(), settings);
}

#[test]
fn painting_one_piece_denser_takes_quads_from_the_others() {
    let far = Vec3::new(10.0, 0.0, 0.0);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (primitives::icosphere(4), far, 1.0)]);
    // The second ball painted at 1, the first left at 0: four times the
    // quads per area with the default contrast.
    let weights: Vec<f64> = mesh.positions.iter().map(|p| if p.x > 5.0 { 1.0 } else { 0.0 }).collect();
    let (quads, _) =
        config(2000).remesh_guided(&mesh, &Guides { density: Some(&weights), ..Default::default() }).unwrap();
    let (plain, painted) = (faces_where(&quads, |p| p.x < 5.0), faces_where(&quads, |p| p.x > 5.0));
    assert!((painted as f64 / plain as f64 - 4.0).abs() < 0.6, "{plain} plain, {painted} painted");
    assert!(((plain + painted) as f64 - 2000.0).abs() < 100.0);

    // One weight too few is refused, whatever the number of pieces.
    assert!(config(2000).remesh_guided(&mesh, &Guides { density: Some(&weights[1..]), ..Default::default() }).is_err());
}

#[test]
fn a_sliver_with_next_to_no_area_does_not_run_away() {
    // Next to a ball, a strip ten units long and a millionth of one wide,
    // then one with no width to speak of: what a boolean leaves between two
    // parts. The size of triangle such a strip asks for is next to nothing,
    // and tiling its length with it once took all the memory of a machine.
    for (width, pieces) in [(1e-6, 2), (1e-14, 1)] {
        let mut mesh = primitives::icosphere(4);
        let first = mesh.positions.len() as u32;
        mesh.positions.extend([
            Vec3::new(-5.0, 0.0, 3.0),
            Vec3::new(5.0, 0.0, 3.0),
            Vec3::new(5.0, width, 3.0),
            Vec3::new(-5.0, width, 3.0),
        ]);
        mesh.triangles.extend([[first, first + 1, first + 2], [first, first + 2, first + 3]]);
        let (quads, report) = config(1000).remesh_guided(&mesh, &Guides::default()).unwrap();
        // The narrower one is no surface at all once its two sides, which
        // are at the same place for any purpose, are stitched.
        assert_eq!(report.pieces, pieces, "width {width}");
        // The ball is meshed as if it were alone; the strip takes next to
        // nothing, whether it is left out or not.
        assert!(quads.faces.len() >= 900 && quads.faces.len() <= 1100, "{} faces", quads.faces.len());
        assert!(report.working_vertices < 200_000, "{} vertices in the work meshes", report.working_vertices);
    }
}

#[test]
fn a_piece_thin_all_along_gets_the_quads_its_thickness_calls_for_out_of_the_number() {
    // A ball, and beside it a ring of wire with a hundredth of its area: by
    // area it would have six quads of the 600, each many times longer than
    // the wire is thick, and come out crumpled.
    let aside = Vec3::new(4.0, 0.0, 0.0);
    let ring = primitives::torus(0.5, 0.01, 96, 12);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (ring, aside, 1.0)]);
    let (quads, report) = config(600).remesh(&mesh).unwrap();
    let on_ring = faces_where(&quads, |p| p.x > 2.0);
    let on_ball = quads.faces.len() - on_ring;
    // The ring is given quads for its thickness, as far as that leaves the
    // ball half of the number; and the number asked for is kept.
    assert_eq!(report.thin_pieces, 1);
    assert!(on_ring > 200, "{on_ring} faces on the ring");
    assert!((report.quads_for_thin_pieces as f64 - on_ring as f64).abs() < 0.2 * on_ring as f64);
    assert!(on_ball >= 270, "{on_ball} faces on the ball");
    assert!((quads.faces.len() as f64 - 600.0).abs() < 60.0, "{} faces in all", quads.faces.len());
    let stats = quads.stats();
    assert_eq!((stats.quads, stats.non_manifold_edges, stats.boundary_edges), (stats.faces, 0, 0));

    // With quads to spare the ring has all it calls for, and the ball the
    // rest.
    let (quads, report) = config(6000).remesh(&mesh).unwrap();
    let on_ring = faces_where(&quads, |p| p.x > 2.0);
    assert_eq!(report.thin_pieces, 1);
    assert!((quads.faces.len() as f64 - 6000.0).abs() < 300.0, "{} faces in all", quads.faces.len());
    assert!(on_ring > 600 && on_ring < 3000, "{on_ring} faces on the ring");
}

/// Listens to what the stages show, and keeps none of it.
struct Keeps;

impl openclare::recorder::Recorder for Keeps {
    fn record(&mut self, _: &str, _: openclare::recorder::Shape<'_>) {}
}

/// Follows a run, and stops it at the stage of that number.
struct StopAt {
    stages: usize,
    stop: usize,
}

impl openclare::recorder::Recorder for StopAt {
    fn is_listening(&self) -> bool {
        false
    }

    fn record(&mut self, _: &str, _: openclare::recorder::Shape<'_>) {}

    fn begin(&mut self, _: &str) -> openclare::Result<()> {
        self.stages += 1;
        if self.stages == self.stop { Err(openclare::Error::Cancelled) } else { Ok(()) }
    }
}

#[test]
fn pieces_meshed_at_the_same_time_give_the_same_mesh_and_stop_when_told() {
    // Twelve balls of several sizes in a row.
    let balls: Vec<(TriMesh, Vec3, f64)> = (0..12)
        .map(|k| (primitives::icosphere(3), Vec3::new(4.0 * k as f64, 0.0, 0.0), 0.5 + 0.1 * (k % 4) as f64))
        .collect();
    let mesh = assembly(&balls);
    // One after the other is what a recorder that keeps shapes gets; several
    // at a time is what the others get on a machine with more than one core.
    let (apart, _) = config(1500).remesh_recorded(&mesh, &Guides::default(), &mut Keeps).unwrap();
    let mut follower = StopAt { stages: 0, stop: usize::MAX };
    let (together, report) = config(1500).remesh_recorded(&mesh, &Guides::default(), &mut follower).unwrap();
    assert_eq!((report.pieces, report.pieces_failed), (12, 0));
    assert_eq!((&together.positions, &together.faces), (&apart.positions, &apart.faces));
    // Every stage of every piece was told, from the thread that asked.
    assert!(follower.stages >= 12 * 8, "{} stages told", follower.stages);

    // Stopped half way: no mesh, and the run says it was stopped.
    let mut follower = StopAt { stages: 0, stop: 40 };
    let stopped = config(1500).remesh_recorded(&mesh, &Guides::default(), &mut follower);
    assert!(matches!(stopped, Err(openclare::Error::Cancelled)));
}

/// A band with a half turn in it, round the z axis: one side only, which no
/// chain can lay a grid on.
fn one_sided_band(around: u32, across: u32) -> TriMesh {
    let mut mesh = TriMesh::default();
    for i in 0..around {
        let turn = i as f64 / around as f64 * std::f64::consts::TAU;
        for j in 0..=across {
            let off = (j as f64 / across as f64 - 0.5) * 0.8;
            let radius = 1.0 + off * (turn / 2.0).cos();
            mesh.positions.push(Vec3::new(radius * turn.cos(), radius * turn.sin(), off * (turn / 2.0).sin()));
        }
    }
    let row = across + 1;
    for i in 0..around {
        for j in 0..across {
            let (a, b) = (i * row + j, i * row + j + 1);
            // The last row meets the first one the other way up.
            let (c, d) = if i + 1 < around { ((i + 1) * row + j, (i + 1) * row + j + 1) } else { (across - j, across - j - 1) };
            mesh.triangles.push([a, c, b]);
            mesh.triangles.push([b, c, d]);
        }
    }
    mesh
}

#[test]
fn a_piece_that_cannot_be_meshed_is_counted_and_the_others_are_meshed() {
    let band = one_sided_band(96, 12);
    assert_eq!(band.pieces().len(), 1);
    // On its own there is nothing to give back: an error.
    assert!(config(600).remesh(&band).is_err());

    // Next to a ball, the ball is meshed and the band counted as missing.
    let far = Vec3::new(4.0, 0.0, 0.0);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (band, far, 1.0)]);
    let (quads, report) = config(1200).remesh(&mesh).unwrap();
    assert_eq!((report.pieces, report.pieces_failed, report.pieces_boxed, report.pieces_skipped), (2, 1, 0, 0));
    let stats = quads.stats();
    assert_eq!((stats.quads, stats.boundary_edges, stats.non_manifold_edges), (stats.faces, 0, 0));
    assert_eq!(stats.euler_characteristic(), 2);
    assert!(quads.positions.iter().all(|p| (p.length() - 1.0).abs() < 0.02), "a vertex away from the ball");
    // The ball has its share of the quads, by its area, and no more.
    let ball = 4.0 * std::f64::consts::PI;
    let share = 1200.0 * ball / mesh.surface_area();
    assert!((stats.faces as f64 - share).abs() < 0.1 * share, "{} quads for a share of {share}", stats.faces);
}
