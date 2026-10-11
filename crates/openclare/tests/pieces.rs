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
fn a_piece_thin_all_along_gets_the_quads_its_thickness_calls_for() {
    // A ball, and beside it a ring of wire with a hundredth of its area.
    let aside = Vec3::new(4.0, 0.0, 0.0);
    let ring = primitives::torus(0.5, 0.01, 96, 12);
    let mesh = assembly(&[(primitives::icosphere(4), Vec3::ZERO, 1.0), (ring, aside, 1.0)]);
    let ring_faces = |quads: &PolyMesh| faces_where(quads, |p| p.x > 2.0);

    // The number asked for comes first: the ring has the least a ring can
    // be made of, with edges several times longer than the wire is thick.
    let kept = RemeshConfig { preserve_volumes: false, ..config(600) };
    let (quads, report) = kept.remesh(&mesh).unwrap();
    assert_eq!((report.thin_pieces, report.quads_for_thin_pieces), (0, 0));
    assert!(ring_faces(&quads) < 40, "{} faces on the ring", ring_faces(&quads));

    // The shape comes first: quads no longer than the wire is thick, on top
    // of the number asked for, and nothing taken from the ball for it.
    let (quads, report) = config(600).remesh(&mesh).unwrap();
    assert_eq!(report.thin_pieces, 1);
    let on_ring = ring_faces(&quads);
    assert!(on_ring > 300, "{on_ring} faces on the ring");
    assert!((report.quads_for_thin_pieces as f64 - on_ring as f64).abs() < 0.2 * on_ring as f64);
    let on_ball = quads.faces.len() - on_ring;
    assert!((on_ball as f64 - 600.0).abs() < 60.0, "{on_ball} faces on the ball");
    let stats = quads.stats();
    assert_eq!((stats.quads, stats.non_manifold_edges, stats.boundary_edges), (stats.faces, 0, 0));
}
