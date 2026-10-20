//! The yardstick has to be right before anything is measured with it: each
//! metric is checked on a mesh whose score is known in closed form.

use openclare::math::Vec3;
use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::{Params, Target, remesh};

#[test]
fn flat_grid_is_a_perfect_quad_mesh() {
    let metrics = measure(&primitives::plane(24), &primitives::plane_quads(12), Some(144));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.irregular_vertices, 0);
    assert_eq!(metrics.base_complex_patches, Some(1));
    assert_eq!(metrics.budget_error, Some(0.0));
    assert!(metrics.angle_deviation_p95 < 1e-9);
    assert!((metrics.aspect_p95 - 1.0).abs() < 1e-9);
    assert!(metrics.area_variation < 1e-9);
    assert!(metrics.hausdorff < 1e-9);
    // A plane has no curvature direction to align to.
    assert_eq!(metrics.alignment, None);
    // It is open, and so is its source: that is not a defect.
    assert!(metrics.boundary_edges > 0 && metrics.source_boundary_edges > 0);
}

#[test]
fn cube_grid_has_the_minimum_of_a_sphere() {
    let metrics = measure(&primitives::cube(8), &primitives::cube_quads(8), None);
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.irregular_vertices, 8);
    assert_eq!(metrics.topological_minimum, Some(8));
    assert_eq!(metrics.irregular_excess(), Some(0));
    // The separatrices from the 8 corners are the 12 edges: 6 faces.
    assert_eq!(metrics.base_complex_patches, Some(6));
    assert_eq!((metrics.components, metrics.source_components), (1, 1));
}

#[test]
fn torus_grid_needs_no_irregular_vertex_and_follows_curvature() {
    let source = primitives::torus(1.0, 0.4, 96, 48);
    let metrics = measure(&source, &primitives::torus_quads(1.0, 0.4, 48, 24), None);
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.irregular_vertices, 0);
    assert_eq!(metrics.topological_minimum, Some(0));
    assert_eq!(metrics.base_complex_patches, Some(1));
    // Its edges run exactly along the principal directions.
    assert!(metrics.alignment.unwrap() < 1.0, "{:?}", metrics.alignment);
    assert!(metrics.hausdorff < 0.01);
}

#[test]
fn edges_at_45_degrees_score_the_worst_alignment() {
    // Same torus, with every quad replaced by the one joining its edge
    // midpoints... which is the grid of diagonals.
    let grid = primitives::torus_quads(1.0, 0.4, 48, 24);
    let diagonal = grid.split_to_quads();
    let source = primitives::torus(1.0, 0.4, 96, 48);
    let straight = measure(&source, &diagonal, None).alignment.unwrap();
    assert!(straight < 2.0, "splitting keeps the directions: {straight}");

    let (nu, nv) = (48u32, 24u32);
    let index = |i: u32, j: u32| (i % nu) * nv + (j % nv);
    let mut skewed = grid.clone();
    // Diamonds over a 2 x 2 block of cells: all edges are cell diagonals.
    skewed.faces = (0..nu)
        .flat_map(|i| (0..nv).map(move |j| (i, j)))
        .filter(|(i, j)| (i + j) % 2 == 0)
        .map(|(i, j)| vec![index(i, j), index(i + 1, j + 1), index(i, j + 2), index(i + nu - 1, j + 1)])
        .collect();
    let skew = measure(&source, &skewed, None).alignment.unwrap();
    assert!(skew > 30.0, "diagonal edges should be far from the principal directions: {skew}");
}

#[test]
fn defects_are_caught() {
    let source = primitives::cube(8);
    let good = primitives::cube_quads(8);

    let mut flipped = good.clone();
    flipped.faces[10].reverse();
    let metrics = measure(&source, &flipped, None);
    assert_eq!(metrics.inverted_faces, 1);
    assert_eq!(metrics.misoriented_edges, 4);
    assert!(!metrics.is_valid());
    assert_eq!(metrics.base_complex_patches, None);

    let mut holed = good.clone();
    holed.faces.remove(10);
    let metrics = measure(&source, &holed, None);
    assert_eq!(metrics.boundary_edges, 4);
    assert!(!metrics.is_valid(), "a hole the source does not have is a defect");

    let mut triangulated = good.clone();
    let quad = triangulated.faces.remove(10);
    triangulated.faces.push(vec![quad[0], quad[1], quad[2]]);
    triangulated.faces.push(vec![quad[0], quad[2], quad[3]]);
    let metrics = measure(&source, &triangulated, None);
    assert_eq!(metrics.quads + 2, metrics.faces);
    assert!(!metrics.is_valid());

    let mut collapsed = good.clone();
    let corner = collapsed.faces[10][0] as usize;
    for k in 1..4 {
        let v = collapsed.faces[10][k] as usize;
        collapsed.positions[v] = collapsed.positions[corner];
    }
    assert!(measure(&source, &collapsed, None).degenerate_faces >= 1);
}

#[test]
fn distance_to_the_source_is_measured_both_ways() {
    let source = primitives::icosphere(4);
    let mut inflated = primitives::cube_quads(8);
    for p in &mut inflated.positions {
        *p = p.normalize() * 1.1;
    }
    let metrics = measure(&source, &inflated, None);
    // Bounding box diagonal of the unit sphere is 2 sqrt(3).
    let expected = 0.1 / (2.0 * 3.0f64.sqrt());
    assert!((metrics.mean_distance - expected).abs() < 0.1 * expected, "{}", metrics.mean_distance);

    // Half a cube: nothing is far from the sphere, but half the sphere is far
    // from it. Only the source-to-mesh direction sees that.
    let mut half = primitives::cube_quads(8);
    half.faces.retain(|face| face.iter().all(|&v| half.positions[v as usize].z >= 0.0));
    for p in &mut half.positions {
        *p = p.normalize();
    }
    assert!(measure(&source, &half, None).hausdorff > 0.2);
}

#[test]
fn a_skewed_and_stretched_quad_is_scored_as_such() {
    let source = primitives::plane(4);
    let mut mesh = openclare::PolyMesh::default();
    // A 2 x 1 parallelogram leaning at 60 degrees.
    let lean = Vec3::new(0.5, 3.0f64.sqrt() / 2.0, 0.0);
    mesh.positions = vec![Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0) + lean, lean];
    mesh.faces = vec![vec![0, 1, 2, 3]];
    let metrics = measure(&source, &mesh, None);
    assert!((metrics.angle_deviation_mean - 30.0).abs() < 1e-6);
    assert!((metrics.aspect_mean - 2.0).abs() < 1e-9);
}

#[test]
fn the_baseline_passes_validity_and_reports_its_excess() {
    let source = primitives::icosphere(4);
    let params = Params { target: Target::QuadCount(1000), ..Default::default() };
    let (mesh, _) = remesh(&source, &params).unwrap();
    let metrics = measure(&source, &mesh, Some(1000));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.topological_minimum, Some(8));
    assert!(metrics.irregular_excess().unwrap() >= 0);
    assert!(metrics.base_complex_patches.unwrap() >= 6);
    assert!(metrics.budget_error.unwrap() < 0.25);
    assert!(metrics.to_json().contains("\"valid\": true"));
}

#[test]
fn edge_loops_of_a_torus_grid_all_close() {
    // 48 rings one way, 24 the other, each its own loop.
    let metrics = measure(&primitives::torus(1.0, 0.4, 96, 48), &primitives::torus_quads(1.0, 0.4, 48, 24), None);
    let loops = metrics.edge_loops.expect("a pure quad manifold");
    assert_eq!((loops.closed, loops.spiral, loops.open, loops.border), (48 + 24, 0, 0, 0), "{loops:?}");
    assert!((loops.closed_share - 1.0).abs() < 1e-12);
}

#[test]
fn edge_loops_of_a_cube_grid_close_but_along_its_edges() {
    // Round the cube, 3 x 7 rings; along its 12 edges, loops from corner to
    // corner, which are irregular.
    let metrics = measure(&primitives::cube(8), &primitives::cube_quads(8), None);
    let loops = metrics.edge_loops.expect("a pure quad manifold");
    assert_eq!((loops.closed, loops.spiral, loops.open, loops.border), (21, 0, 12, 0), "{loops:?}");
    assert_eq!(loops.longest_open, 8);
    assert!((loops.open_share - 12.0 * 8.0 / (12.0 * 8.0 * 8.0)).abs() < 1e-12, "{loops:?}");
}

/// The torus grid closed with a twist of `shift` rings: across the seam
/// j = nv - 1 to 0, `shift` rings further along u.
fn twisted_torus(nu: u32, nv: u32, shift: u32) -> openclare::PolyMesh {
    let mut twisted = primitives::torus_quads(1.0, 0.4, nu, nv);
    let index = |i: u32, j: u32| (i % nu) * nv + (j % nv);
    twisted.faces.clear();
    for i in 0..nu {
        for j in 0..nv {
            let (up_i, up_j) = if j + 1 == nv { (i + shift, 0) } else { (i, j + 1) };
            twisted.faces.push(vec![index(i, j), index(i + 1, j), index(up_i + 1, up_j), index(up_i, up_j)]);
        }
    }
    twisted
}

#[test]
fn edge_loops_of_a_twisted_torus_spiral() {
    // Twisted by one ring, the loops that went round the tube go round it
    // and on to the next ring, a helix, beside itself a quad further. Those
    // round the axis still close.
    let (nu, nv) = (24u32, 12u32);
    let twisted = twisted_torus(nu, nv, 1);
    let metrics = measure(&twisted.triangulated(), &twisted, None);
    assert!(metrics.is_valid(), "{metrics:?}");
    let loops = metrics.edge_loops.expect("a pure quad manifold");
    assert_eq!(loops.spiral, 1, "{loops:?}");
    assert_eq!(loops.closed, nv as usize, "{loops:?}");
    assert!((loops.spiral_share - 0.5).abs() < 1e-12, "{loops:?}");
}

#[test]
fn edge_loops_wound_together_spiral_too() {
    // Twisted by two rings: two helices wound together, each beside the
    // other, not itself. Both spiral.
    let (nu, nv) = (24u32, 12u32);
    let twisted = twisted_torus(nu, nv, 2);
    let metrics = measure(&twisted.triangulated(), &twisted, None);
    let loops = metrics.edge_loops.expect("a pure quad manifold");
    assert_eq!((loops.spiral, loops.closed), (2, nv as usize), "{loops:?}");
    assert!((loops.spiral_share - 0.5).abs() < 1e-12, "{loops:?}");
    // Five wound together are beyond what is looked for: they go as loops
    // that close, each after going round five times.
    let five = twisted_torus(25, nv, 5);
    let loops = measure(&five.triangulated(), &five, None).edge_loops.unwrap();
    assert_eq!(loops.spiral, 0, "{loops:?}");
}

#[test]
fn edge_loops_met_again_over_a_cap_are_rings() {
    // On a cube of two quads a side, a strip leaving a ring across goes over
    // the top and meets the ring again within four quads, but the other way
    // round: still a ring.
    let metrics = measure(&primitives::cube(4), &primitives::cube_quads(2), None);
    let loops = metrics.edge_loops.expect("a pure quad manifold");
    assert_eq!((loops.closed, loops.spiral), (3, 0), "{loops:?}");
}

#[test]
fn edge_loops_of_a_grid_run_from_border_to_border() {
    let metrics = measure(&primitives::plane(24), &primitives::plane_quads(12), None);
    let loops = metrics.edge_loops.expect("a pure quad manifold");
    // 11 inner lines each way; the border itself is on no loop.
    assert_eq!((loops.closed, loops.spiral, loops.open, loops.border), (0, 0, 0, 22), "{loops:?}");
}

#[test]
fn edge_loops_stop_at_a_face_that_is_not_a_quad() {
    // The torus grid with one quad split in two triangles: the four loops
    // through its corners now stop there, its edges and the diagonal on none.
    let mut grid = primitives::torus_quads(1.0, 0.4, 48, 24);
    let split = grid.faces.swap_remove(0);
    grid.faces.push(vec![split[0], split[1], split[2]]);
    grid.faces.push(vec![split[0], split[2], split[3]]);
    let metrics = measure(&primitives::torus(1.0, 0.4, 96, 48), &grid, None);
    assert!(!metrics.is_valid());
    let loops = metrics.edge_loops.expect("still a manifold");
    assert_eq!((loops.closed, loops.spiral, loops.open, loops.border), (48 + 24 - 4, 0, 4, 0), "{loops:?}");
}

/// An open tube of `nu` columns of quads round and `nv` rows along, its
/// last column closed onto the first only at the rows `seam`, and
/// `shift` rows higher: with a shift, the loops round it climb into
/// helices. Only its connectivity counts.
fn open_tube(nu: u32, nv: u32, seam: std::ops::Range<u32>, shift: u32) -> openclare::PolyMesh {
    let index = |i: u32, j: u32| i * (nv + 1) + j;
    let mut tube = openclare::PolyMesh::default();
    for i in 0..nu {
        for j in 0..=nv {
            let angle =
                std::f64::consts::TAU * (i as f64 + j as f64 / nv as f64 * shift as f64 / nv as f64) / nu as f64;
            tube.positions.push(Vec3::new(angle.cos(), angle.sin(), j as f64 * 0.2));
        }
    }
    for i in 0..nu - 1 {
        for j in 0..nv {
            tube.faces.push(vec![index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
        }
    }
    for j in seam.filter(|j| j + shift < nv) {
        tube.faces.push(vec![index(nu - 1, j), index(0, j + shift), index(0, j + shift + 1), index(nu - 1, j + 1)]);
    }
    tube
}

#[test]
fn a_tube_of_rings_and_a_flat_grid_have_no_helix() {
    use openclare::metrics::edge_loops;
    // The torus grid: a strip leaving a ring goes round the torus back to
    // the edge it left, which is no pass.
    for (mesh, rings) in [(open_tube(24, 10, 0..10, 0), 9), (primitives::torus_quads(1.0, 0.4, 48, 24), 72)] {
        let loops = edge_loops(&mesh).expect("a pure quad manifold");
        assert_eq!(loops.closed, rings, "{loops:?}");
        assert_eq!((loops.helix, loops.helix_pitch_max()), (0, 0), "{loops:?}");
    }
    let loops = edge_loops(&primitives::plane_quads(12)).unwrap();
    assert_eq!((loops.border, loops.helix), (22, 0), "{loops:?}");
}

#[test]
fn a_helix_of_pitch_one_is_seen_by_both_counts() {
    use openclare::metrics::edge_loops;
    // Climbing one row a turn: a single helix from one end of the tube to
    // the other, beside itself a quad further.
    let loops = edge_loops(&open_tube(24, 10, 0..10, 1)).expect("a pure quad manifold");
    assert_eq!((loops.spiral, loops.helix, loops.helix_missed), (1, 1, 0), "{loops:?}");
    assert_eq!(loops.helix_pitch_shares, vec![loops.spiral_share]);
    // Round the twisted torus: the helix goes round the tube; the rings round
    // the axis are met again one ring along, but across the whole tube: the
    // turn is the strip's, and they are no helices.
    let loops = edge_loops(&twisted_torus(24, 12, 1)).unwrap();
    assert_eq!((loops.closed, loops.helix, loops.helix_pitch_max()), (12, 1, 1), "{loops:?}");
    assert!((loops.helix_share - 0.5).abs() < 1e-12, "{loops:?}");
}

#[test]
fn helices_of_pitch_six_escape_the_spiral_count() {
    use openclare::metrics::edge_loops;
    // Six helices wound together, each beside itself six quads further:
    // beyond SPIRAL_ORDER, yet helices.
    for mesh in [open_tube(24, 30, 0..30, 6), twisted_torus(24, 12, 6)] {
        let loops = edge_loops(&mesh).expect("a pure quad manifold");
        assert_eq!((loops.spiral, loops.helix, loops.helix_missed), (0, 6, 6), "{loops:?}");
        assert_eq!(loops.helix_pitch_max(), 6);
        assert_eq!(loops.helix_pitch_shares[..5], [0.0; 5]);
        assert!((loops.helix_missed_share - loops.helix_share).abs() < 1e-12, "{loops:?}");
    }
    let five = edge_loops(&twisted_torus(25, 12, 5)).unwrap();
    assert_eq!((five.spiral, five.helix, five.helix_pitch_max()), (0, 5, 5), "{five:?}");
}

#[test]
fn a_loop_beside_itself_for_less_than_a_turn_is_no_helix() {
    use openclare::metrics::edge_loops;
    // Closed at a few rows only, two rows higher: the loop along row 2 goes
    // round, on along row 4, round again and on along row 6, where it stops.
    // Its middle turn is beside both others: a helix. (The loop along row 3
    // goes on along row 5 only, and the edge across the seam between them
    // has nothing beside it: a spiral, not a helix.)
    let (nu, nv) = (24u32, 8u32);
    let tube = open_tube(nu, nv, 1..5, 2);
    let loops = edge_loops(&tube).expect("a pure quad manifold");
    assert_eq!((loops.spiral, loops.helix, loops.helix_pitch_max()), (2, 1, 2), "{loops:?}");
    // Row 4 cut open half way round: the first loop now stops there, beside
    // itself for half a turn only. Still a spiral, no longer a helix.
    let mut cut = tube;
    cut.faces.retain(|face| {
        let (i, j) = (face[0] / (nv + 1), face[0] % (nv + 1));
        !(j == 4 && (nu / 2..nu - 1).contains(&i))
    });
    let loops = edge_loops(&cut).expect("a pure quad manifold");
    assert_eq!((loops.spiral, loops.helix), (2, 0), "{loops:?}");
}
