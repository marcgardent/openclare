//! Relaxation: quads get squarer, stay on the surface, and nothing wears away.

use retopo::PolyMesh;
use retopo::math::{Rng, Vec3};
use retopo::mesh::primitives;
use retopo::relax::{self, Kind, Relaxer, Squaring};
use retopo::spatial::Bvh;

/// Mean distance of the corners of the quads of `mesh` from a right angle,
/// in degrees.
fn angle_deviation(mesh: &PolyMesh) -> f64 {
    let (mut sum, mut count) = (0.0, 0.0);
    for face in &mesh.faces {
        for k in 0..face.len() {
            let at = mesh.positions[face[k] as usize];
            let next = mesh.positions[face[(k + 1) % face.len()] as usize] - at;
            let previous = mesh.positions[face[(k + face.len() - 1) % face.len()] as usize] - at;
            sum += (next.angle_between(previous).to_degrees() - 90.0).abs();
            count += 1.0;
        }
    }
    sum / count
}

/// Whether some quad of `mesh`, seen from +z, folds over.
fn folds(mesh: &PolyMesh) -> bool {
    mesh.faces.iter().any(|face| {
        (0..face.len()).any(|k| {
            let at = mesh.positions[face[k] as usize];
            let next = mesh.positions[face[(k + 1) % face.len()] as usize] - at;
            let previous = mesh.positions[face[(k + face.len() - 1) % face.len()] as usize] - at;
            next.cross(previous).z <= 0.0
        })
    })
}

/// A flat grid whose inner vertices have been pushed about at random.
fn shaken_grid(segments: u32, amount: f64) -> PolyMesh {
    let mut mesh = primitives::plane_quads(segments);
    let step = 2.0 / segments as f64;
    let mut rng = Rng::new(3);
    for p in &mut mesh.positions {
        if p.x.abs() < 1.0 - 1e-9 && p.y.abs() < 1.0 - 1e-9 {
            *p += Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, 0.0) * (2.0 * amount * step);
        }
    }
    mesh
}

#[test]
fn a_shaken_grid_is_squared_up_without_leaving_its_plane() {
    let surface = Bvh::build(&primitives::plane(4));
    let mut mesh = shaken_grid(12, 0.3);
    let before = angle_deviation(&mesh);
    assert!(before > 8.0 && !folds(&mesh));
    Squaring::default().relax(&mut mesh, &surface, None);
    let after = angle_deviation(&mesh);
    assert!(after < 0.4 * before, "{before} degrees before, {after} after");
    assert!(!folds(&mesh));
    assert!(mesh.positions.iter().all(|p| p.z.abs() < 1e-9));
}

#[test]
fn a_perfect_grid_is_left_alone() {
    let surface = Bvh::build(&primitives::plane(4));
    let mut mesh = primitives::plane_quads(10);
    let before = mesh.positions.clone();
    Squaring::default().relax(&mut mesh, &surface, None);
    for (p, q) in mesh.positions.iter().zip(&before) {
        assert!(p.distance(*q) < 1e-9);
    }
}

#[test]
fn no_vertex_strays_further_than_its_reach() {
    let surface = Bvh::build(&primitives::plane(4));
    for reach in [0.1, 0.5] {
        let mut mesh = shaken_grid(12, 0.4);
        let start = mesh.positions.clone();
        Squaring { reach, ..Default::default() }.relax(&mut mesh, &surface, None);
        // Edges are at most the grid step plus twice the shake.
        let longest_edge = (2.0 / 12.0) * (1.0 + 2.0 * 0.4 * 2.0f64.sqrt());
        let furthest = mesh.positions.iter().zip(&start).map(|(p, q)| p.distance(*q)).fold(0.0, f64::max);
        assert!(furthest <= reach * longest_edge + 1e-9, "moved {furthest} with reach {reach}");
        assert!(furthest > 0.0);
    }
}

#[test]
fn thin_parts_are_not_worn_away() {
    // A cube mesh on a sphere of the same size: plain smoothing run for long
    // slides everything about; squaring keeps each vertex near its start.
    let source = primitives::icosphere(4);
    let surface = Bvh::build(&source);
    let on_sphere = |mesh: &mut PolyMesh| {
        for p in &mut mesh.positions {
            *p = p.normalize();
        }
    };
    let mut squared = primitives::cube_quads(6);
    on_sphere(&mut squared);
    let start = squared.positions.clone();
    let before = angle_deviation(&squared);
    Squaring::default().relax(&mut squared, &surface, None);
    assert!(angle_deviation(&squared) < before);
    for (p, q) in squared.positions.iter().zip(&start) {
        // Still on the sphere (to the accuracy of its triangles), and close by.
        assert!((p.length() - 1.0).abs() < 0.01);
        assert!(p.distance(*q) < 0.5 * 0.35);
    }
}

#[test]
fn relaxers_come_from_the_factory_by_name() {
    for &kind in Kind::ALL {
        assert_eq!(kind.name().parse::<Kind>().unwrap(), kind);
        assert_eq!(relax::create(kind).kind(), kind);
    }
    assert!("no-such-relaxer".parse::<Kind>().is_err());

    // Both leave a flat grid flat and unfolded.
    let surface = Bvh::build(&primitives::plane(4));
    for &kind in Kind::ALL {
        let mut mesh = shaken_grid(8, 0.2);
        relax::create(kind).relax(&mut mesh, &surface, None);
        assert!(!folds(&mesh), "{kind}");
    }
}
