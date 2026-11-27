//! The guarded projection of a subdivision level, on shapes whose answer is
//! known: a sphere, a thin slab, a cube with its hard edges.

use openclare::PolyMesh;
use openclare::level::{Settings, Status, place};
use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::subdivision;

fn subdivided(level: &PolyMesh) -> PolyMesh {
    let refined = subdivision::refine(&level.faces, level.positions.len());
    let positions = refined.rows.iter().map(|row| subdivision::apply(row, &level.positions)).collect();
    PolyMesh { positions, faces: refined.faces }
}

#[test]
fn a_sphere_takes_every_vertex_at_every_level() {
    let source = primitives::icosphere(6);
    let mut level = primitives::cube_quads(2);
    for p in &mut level.positions {
        *p = p.normalize();
    }
    let mut missing = Vec::new();
    for _ in 0..3 {
        level = subdivided(&level);
        let placed = place(&level, &[], &source, &Settings::default()).unwrap();
        assert_eq!(placed.count(Status::Moved), level.positions.len(), "{}", placed.report());
        // On the icosphere: within its facets of the unit sphere.
        for p in &placed.positions {
            assert!((p.length() - 1.0).abs() < 2e-3, "{p:?} is off the sphere");
        }
        let detail = placed.detail.clone().unwrap();
        assert_eq!(detail.stretched, 0);
        missing.push(detail.missing_p99);
        level.positions = placed.positions;
    }
    for pair in missing.windows(2) {
        assert!(pair[1] < pair[0], "{missing:?}");
    }
}

#[test]
fn a_level_sunk_in_a_thin_part_goes_to_its_own_side() {
    // A slab a tenth thick, and a level facing up that the subdivision has
    // sunk into it, nearer its bottom than its top: what Catmull-Clark does
    // to a finger. The nearest sheet along the normal is the bottom; the
    // level faces the top, and that is where it must go.
    let mut source = primitives::cube(8);
    source.positions.iter_mut().for_each(|p| p.z *= 0.05);
    let mut level = primitives::plane_quads(8);
    level.positions.iter_mut().for_each(|p| *p = Vec3::new(0.8 * p.x, 0.8 * p.y, -0.02));
    let settings = Settings { samples: 0, ..Settings::default() };
    let placed = place(&level, &[], &source, &settings).unwrap();
    assert_eq!(placed.count(Status::Held), 32, "the border holds: {}", placed.report());
    for (v, &status) in placed.status.iter().enumerate() {
        if status != Status::Held {
            assert_eq!(status, Status::Moved);
            assert!((placed.positions[v].z - 0.05).abs() < 1e-9, "{:?}", placed.positions[v]);
        }
    }
}

#[test]
fn a_tip_left_short_comes_out_along_its_normal() {
    // A post three high, and a level of it that the subdivision left short
    // at the top, by two and a half of its shortest edges: a fingertip.
    // Along the normal, the top vertices find the top of the post past the
    // reach; they go there, and with one edge of reach they stay short.
    let mut source = primitives::cube(8);
    source.positions.iter_mut().for_each(|p| p.z *= 3.0);
    let mut level = primitives::cube_quads(8);
    level.positions.iter_mut().for_each(|p| *p = Vec3::new(0.95 * p.x, 0.95 * p.y, 2.4 * p.z));
    let top: Vec<usize> = (0..level.positions.len())
        .filter(|&v| {
            let p = level.positions[v];
            p.z > 2.39 && p.x.abs() < 0.9 && p.y.abs() < 0.9
        })
        .collect();
    assert_eq!(top.len(), 7 * 7);
    let settings = Settings { samples: 0, ..Settings::default() };
    let placed = place(&level, &[], &source, &settings).unwrap();
    for &v in &top {
        assert_eq!(placed.status[v], Status::Moved, "{}", placed.report());
        assert!((placed.positions[v].z - 3.0).abs() < 1e-9, "{:?}", placed.positions[v]);
    }
    let near = Settings { far: 1.0, ..settings };
    let placed = place(&level, &[], &source, &near).unwrap();
    assert!(top.iter().all(|&v| placed.positions[v].z < 2.9));
}

#[test]
fn a_stretched_edge_that_hugs_the_source_is_kept() {
    // The post of the fingertip, its tip brought out: the edges from the top
    // to the rim of the top grow longer. With the stretch guard at 1.5 and
    // every stretched edge taken back, the tip stays short; with the edges
    // whose middle lies near the source kept, it comes out. (Whether that
    // lets through a vertex thrown across the air is the bench's to say: on
    // the ogre of #6, hug = 0.5 left its stretched edges at L4 as they were.)
    let mut source = primitives::cube(8);
    source.positions.iter_mut().for_each(|p| p.z *= 3.0);
    let mut level = primitives::cube_quads(8);
    level.positions.iter_mut().for_each(|p| *p = Vec3::new(0.95 * p.x, 0.95 * p.y, 2.4 * p.z));
    let top: Vec<usize> = (0..level.positions.len())
        .filter(|&v| {
            let p = level.positions[v];
            p.z > 2.39 && p.x.abs() < 0.9 && p.y.abs() < 0.9
        })
        .collect();
    let tight = Settings { samples: 0, stretch: 1.5, hug: 0.0, ..Settings::default() };
    let placed = place(&level, &[], &source, &tight).unwrap();
    assert!(top.iter().all(|&v| placed.positions[v].z < 2.9), "{}", placed.report());
    let hugging = Settings { hug: 4.0, ..tight };
    let placed = place(&level, &[], &source, &hugging).unwrap();
    for &v in &top {
        assert!((placed.positions[v].z - 3.0).abs() < 1e-9, "{:?} {}", placed.positions[v], placed.report());
    }
}

#[test]
fn creased_lines_go_to_the_hard_edges_and_corners_hold() {
    // The cube's grid shrunk: its edge lines run inside the cube, a tenth
    // from its hard edges, and are creased.
    let source = primitives::cube(8);
    let mut level = primitives::cube_quads(4);
    level.positions.iter_mut().for_each(|p| *p *= 0.9);
    let on_edge = |p: Vec3| p.to_array().iter().filter(|c| (c.abs() - 0.9).abs() < 1e-9).count() >= 2;
    let creases: Vec<_> = level
        .edge_uses()
        .into_keys()
        .filter(|&(a, b)| {
            let (p, q) = (level.positions[a as usize], level.positions[b as usize]);
            // Both ends on the same edge line of the cube.
            on_edge(p) && on_edge(q) && (0..3).filter(|&k| (p[k] - q[k]).abs() < 1e-9 && p[k].abs() > 0.89).count() == 2
        })
        .map(|(a, b)| ([a, b], 1.0))
        .collect();
    assert_eq!(creases.len(), 12 * 4);
    let settings = Settings { samples: 0, ..Settings::default() };
    let placed = place(&level, &creases, &source, &settings).unwrap();
    assert_eq!(placed.count(Status::Held), 8, "{}", placed.report());
    assert_eq!(placed.count(Status::OnLine), 12 * 3, "{}", placed.report());
    for (v, &status) in placed.status.iter().enumerate() {
        let p = placed.positions[v];
        match status {
            Status::OnLine => {
                let at_one = p.to_array().iter().filter(|c| (c.abs() - 1.0).abs() < 1e-6).count();
                assert!(at_one >= 2, "{p:?} is not on a hard edge");
            }
            Status::Held => assert_eq!(p, level.positions[v]),
            _ => {}
        }
    }
}

#[test]
fn a_bad_level_is_refused() {
    let source = primitives::icosphere(2);
    let level = PolyMesh { positions: vec![Vec3::ZERO; 3], faces: vec![vec![0, 1, 2, 7]] };
    assert!(place(&level, &[], &source, &Settings::default()).is_err());
    let level = primitives::cube_quads(1);
    assert!(place(&level, &[([0, 99], 1.0)], &source, &Settings::default()).is_err());
}
