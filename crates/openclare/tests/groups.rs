//! Groups of triangles (materials, say): the quad mesh has a line of edges
//! where two of them meet, and says which group each of its faces lies in.
//! And the mending of the input that comes first.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::{Guides, RemeshConfig, Target, TriMesh};

/// The group of each triangle of `mesh`: 1 where `inside` says so of its
/// middle, 0 elsewhere.
fn groups_of(mesh: &TriMesh, inside: impl Fn(Vec3) -> bool) -> Vec<u32> {
    (0..mesh.triangles.len()).map(|f| inside(mesh.corners(f).iter().sum::<Vec3>() / 3.0) as u32).collect()
}

#[test]
fn a_line_of_edges_runs_where_two_groups_meet() {
    // A flat sheet whose left third is of another material: the border is a
    // straight line of edges of the sheet, off the middle.
    let source = primitives::plane(24);
    let (low, high) =
        source.positions.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p.x), hi.max(p.x)));
    let border = low + (high - low) * 8.0 / 24.0;
    let groups = groups_of(&source, |p| p.x < border);
    let config = RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(400)) };
    let (mesh, report) =
        config.remesh_guided(&source, &Guides { groups: Some(&groups), ..Default::default() }).unwrap();
    assert!(measure(&source, &mesh, Some(400)).is_valid());
    assert!(!report.fell_back && report.guides_let_go == 0);

    // The edges along the border run the whole height of the sheet, on it.
    assert!(report.guide_edges.len() >= 15, "{} edges along the border", report.guide_edges.len());
    for &[a, b] in &report.guide_edges {
        for v in [a, b] {
            assert!(
                (mesh.positions[v as usize].x - border).abs() < 1e-6,
                "vertex {v} at {:?}",
                mesh.positions[v as usize]
            );
        }
    }
    // No face straddles the border, and each says which side it is on.
    assert_eq!(report.face_groups.len(), mesh.faces.len());
    for (face, &group) in mesh.faces.iter().zip(&report.face_groups) {
        let xs: Vec<f64> = face.iter().map(|&v| mesh.positions[v as usize].x).collect();
        let left = xs.iter().all(|&x| x <= border + 1e-6);
        let right = xs.iter().all(|&x| x >= border - 1e-6);
        assert!(left || right, "a face from {xs:?} straddles the border at {border}");
        assert_eq!(group, left as u32);
    }
    // Without groups, nothing of the sort.
    let (_, free) = config.remesh_guided(&source, &Guides::default()).unwrap();
    assert!(free.guide_edges.is_empty() && free.face_groups.is_empty());
    // As many groups as triangles, or none.
    assert!(config.remesh_guided(&source, &Guides { groups: Some(&groups[1..]), ..Default::default() }).is_err());
}

#[test]
fn borders_torn_apart_are_stitched() {
    // A ball written the way some formats do, every triangle with three
    // vertices of its own: one piece again once stitched, and closed.
    let ball = primitives::icosphere(3);
    let mut torn = TriMesh::default();
    for f in 0..ball.triangles.len() {
        let first = torn.positions.len() as u32;
        torn.positions.extend(ball.corners(f));
        torn.triangles.push([first, first + 1, first + 2]);
    }
    assert_eq!(torn.pieces().len(), ball.triangles.len());
    let mended = torn.stitched().expect("a torn mesh is mended");
    assert_eq!((mended.welded, mended.taken_apart), (torn.positions.len() - ball.positions.len(), 0));
    let (stitched, vertex_was, triangle_was) = (mended.mesh, mended.vertex_was, mended.triangle_was);
    assert_eq!(stitched.pieces().len(), 1);
    assert_eq!(stitched.positions.len(), ball.positions.len());
    assert_eq!(stitched.triangles.len(), ball.triangles.len());
    assert!((0..stitched.triangles.len() as u32).eq(triangle_was.iter().copied()));
    for (v, &was) in vertex_was.iter().enumerate() {
        assert_eq!(stitched.positions[v], torn.positions[was as usize]);
    }
    // A mesh that is whole has nothing to mend.
    assert!(ball.stitched().is_none());

    // And the remesher takes the torn ball as the ball it is.
    let config = RemeshConfig::for_organic(Target::QuadCount(300));
    let (mesh, report) = config.remesh_guided(&torn, &Guides::default()).unwrap();
    assert_eq!(report.pieces, 1);
    assert_eq!((report.welded_vertices, report.edges_taken_apart), (torn.positions.len() - ball.positions.len(), 0));
    assert!(measure(&ball, &mesh, Some(300)).is_valid());
    // The ball itself has no defect, and is not touched.
    let (_, untouched) = config.remesh_guided(&ball, &Guides::default()).unwrap();
    assert_eq!((untouched.welded_vertices, untouched.edges_taken_apart), (0, 0));
}

#[test]
fn faces_that_share_an_edge_in_threes_are_taken_apart() {
    // Three square sheets that meet along one line, like the pages of a
    // book: no surface as it is, three surfaces once taken apart.
    let mut book = TriMesh::default();
    let spine = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)];
    book.positions.extend(spine);
    for turn in [0.0f64, 2.0, 4.0] {
        let out = Vec3::new(turn.cos(), turn.sin(), 0.0);
        let first = book.positions.len() as u32;
        book.positions.extend([spine[0] + out, spine[1] + out]);
        book.triangles.extend([[0, first, first + 1], [0, first + 1, 1]]);
    }
    assert_eq!(book.pieces().len(), 1);
    let mended = book.stitched().expect("three sheets on a line are taken apart");
    assert_eq!((mended.welded, mended.taken_apart), (0, 1));
    let (apart, triangle_was) = (mended.mesh, mended.triangle_was);
    assert_eq!(apart.pieces().len(), 3);
    assert_eq!(apart.positions.len(), 12);
    assert_eq!(triangle_was.len(), 6);
    // Two sheets that meet along a line stay one surface.
    let mut fold = book.clone();
    fold.triangles.truncate(4);
    fold.positions.truncate(6);
    assert!(fold.stitched().is_none());
}

#[test]
fn two_sheets_that_cross_stay_two_sheets() {
    // Two square sheets through one another along a line: four faces round
    // each edge of that line, which are two sheets, not four half sheets.
    let mut cross = TriMesh::default();
    let spine = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)];
    cross.positions.extend(spine);
    let wings = [Vec3::X, Vec3::Y, -Vec3::X, -Vec3::Y];
    for (k, out) in wings.into_iter().enumerate() {
        let first = cross.positions.len() as u32;
        cross.positions.extend([spine[0] + out, spine[1] + out]);
        // Each sheet runs one way through the line: its two halves turn
        // round it in opposite senses.
        cross.triangles.extend(if k < 2 {
            [[0, first, first + 1], [0, first + 1, 1]]
        } else {
            [[0, first + 1, first], [0, 1, first + 1]]
        });
    }
    let mended = cross.stitched().expect("four faces round an edge are sorted out");
    assert_eq!(mended.mesh.pieces().len(), 2);
    assert_eq!((mended.welded, mended.taken_apart), (0, 1));
    // Each sheet is whole: six vertices, and no border along the line.
    assert_eq!(mended.mesh.positions.len(), 12);
    for (sheet, _) in mended.mesh.pieces() {
        assert_eq!((sheet.positions.len(), sheet.triangles.len()), (6, 4));
    }
}
