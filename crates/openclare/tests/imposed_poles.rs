//! Poles imposed by the user (DT-181): the field has its singularities
//! there, and the others are free.

use openclare::crossfield::{self, CrossField};
use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::preprocess::{self, WorkMesh};

fn sphere() -> WorkMesh {
    let request = preprocess::Request { triangles: 8_000, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(&primitives::icosphere(4), &request).unwrap()
}

fn nearest(work: &WorkMesh, p: Vec3) -> u32 {
    let positions = &work.mesh.positions;
    (0..positions.len()).min_by(|&a, &b| positions[a].distance(p).total_cmp(&positions[b].distance(p))).unwrap() as u32
}

fn field(work: &WorkMesh) -> CrossField {
    crossfield::create(crossfield::Kind::Power)
        .solve(work, &crossfield::Request { alignment: 0.0, ..Default::default() })
        .unwrap()
}

fn singular(field: &CrossField) -> Vec<(u32, i8)> {
    field.indices.iter().enumerate().filter(|(_, i)| **i != 0).map(|(v, &i)| (v as u32, i)).collect()
}

#[test]
fn eight_poles_at_the_corners_of_a_cube() {
    let mut work = sphere();
    work.imposed = cube_corners().iter().map(|&p| (nearest(&work, p), 1)).collect();
    let field = field(&work);
    let mut found = singular(&field);
    found.sort();
    let mut wanted = work.imposed.clone();
    wanted.sort();
    assert_eq!(found, wanted);
}

#[test]
fn one_pole_moved_and_the_others_free() {
    let mut work = sphere();
    let free = singular(&field(&work));
    println!("free: {free:?}");
    let p = Vec3::new(0.3, 0.2, 0.93).normalize();
    let pinned = nearest(&work, p);
    work.imposed = vec![(pinned, 1)];
    let field = field(&work);
    let found = singular(&field);
    println!("with one imposed: {found:?}");
    assert!(found.contains(&(pinned, 1)));
    assert_eq!(field.total_index(), 8);
}

fn cube_corners() -> Vec<Vec3> {
    (0..8).map(|k| Vec3::new([-1.0, 1.0][k & 1], [-1.0, 1.0][(k >> 1) & 1], [-1.0, 1.0][k >> 2]).normalize()).collect()
}

#[test]
fn a_ball_remeshed_round_poles_placed_where_no_one_would() {
    use openclare::{Guides, ImposedPole, RemeshConfig, Target};
    // The corners of a cube turned a little: not where the free field puts them.
    let turn = |p: Vec3| Vec3::new(p.x * 0.8 - p.y * 0.6, p.x * 0.6 + p.y * 0.8, p.z);
    let poles: Vec<ImposedPole> =
        cube_corners().into_iter().map(|p| ImposedPole { position: turn(p), valence: 3 }).collect();
    let source = primitives::icosphere(4);
    let config = RemeshConfig::for_organic(Target::QuadCount(1500));
    let (mesh, report) = config.remesh_guided(&source, &Guides { poles: &poles, ..Default::default() }).unwrap();
    println!(
        "{} quads, {} imposed, poles {:?}",
        mesh.faces.len(),
        report.poles_imposed,
        report.poles.iter().map(|p| (p.valence, p.imposed)).collect::<Vec<_>>()
    );
    assert_eq!(report.poles_imposed, 8);
    assert_eq!(report.poles.len(), 8);
    let mut answered: Vec<u32> = report.poles.iter().filter_map(|p| p.imposed).collect();
    answered.sort();
    assert_eq!(answered, (0..8).collect::<Vec<u32>>());
    // Left free, the poles are elsewhere.
    let (_, free) = config.remesh_guided(&source, &Guides::default()).unwrap();
    let near = |p: Vec3| poles.iter().any(|pole| pole.position.distance(p) <= 2.0 * report.edge_length);
    println!("free poles near the placed ones: {}", free.poles.iter().filter(|p| near(p.position)).count());
    assert!(free.poles.iter().filter(|p| near(p.position)).count() < 4);
}
