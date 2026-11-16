//! The option for a result to be subdivided (#330): singularities slid
//! towards flatter places, the mesh repaired in place for the reflection
//! check with its creases, and nothing else changed.

use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::reflection::{Kind, Settings, check};
use openclare::{RemeshConfig, Target};

#[test]
fn a_cube_keeps_its_faces_and_its_creased_edges() {
    // Every singularity of a cube sits on a corner, on the lines of its
    // edges, where none slides; its quads are flat, and none folds or is
    // thin. So the option gives the faces it gives without, every crease
    // with them, and the repair finds nothing to do.
    let cube = primitives::cube(16);
    let plain = RemeshConfig::for_hard_surface(Target::QuadCount(600));
    let (without, report_without) = plain.remesh(&cube).unwrap();
    let (with, report) = RemeshConfig { for_subdivision: true, ..plain }.remesh(&cube).unwrap();
    assert_eq!(with.faces, without.faces);
    assert_eq!(report.singularities_slid, 0);
    assert_eq!(report.creases, report_without.creases);
    let repair = report.subdivision_repair.unwrap();
    assert_eq!((repair.folded, repair.thin), ((0, 0), (0, 0)));
    assert_eq!(repair.moved, 0);
    assert_eq!(with.positions, without.positions);
    // With its creases, the twelve edges of the cube are sharp, and the
    // corners, where three creases meet, are no pole on a curved zone.
    let score = check(&with, &report.creases, &Settings::default()).score(Some(report.creases.len()));
    assert_eq!(score.of(Kind::CurvedPole).count, 0);
    assert!(report_without.subdivision_repair.is_none());
}

#[test]
fn a_smooth_shape_comes_out_valid_with_the_option() {
    let torus = primitives::torus(1.0, 0.35, 64, 24);
    let config = RemeshConfig { for_subdivision: true, ..RemeshConfig::for_organic(Target::QuadCount(800)) };
    let (mesh, report) = config.remesh(&torus).unwrap();
    assert!(measure(&torus, &mesh, Some(800)).is_valid());
    let repair = report.subdivision_repair.unwrap();
    assert!(repair.folded.1 <= repair.folded.0 && repair.thin.1 <= repair.thin.0, "{repair:?}");
    // No vertex moves beyond a tenth of its shortest edge.
    assert!(repair.largest_share <= 0.1 + 1e-9, "{}", repair.largest_share);
}
