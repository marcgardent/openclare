use std::collections::HashMap;

use super::TriMesh;
use crate::math::Vec3;

/// Splits every edge longer than `max_length` until none is left.
///
/// The fields are sampled at the vertices of the input, so it has to be at
/// least as dense as the lattice we want to recover from it. Midpoints are
/// shared per edge, which keeps the result conforming (no T-junctions).
pub fn split_long_edges(mesh: &TriMesh, max_length: f64) -> TriMesh {
    let mut mesh = mesh.clone();
    let max_sq = max_length * max_length;

    // Each pass halves the marked edges, so this bound is never the limit for
    // sane inputs; it only guards against a degenerate `max_length`.
    for _ in 0..32 {
        let mut midpoints: HashMap<(u32, u32), u32> = HashMap::new();
        let mut triangles = Vec::with_capacity(mesh.triangles.len());
        let positions = &mut mesh.positions;

        for &tri in &mesh.triangles {
            // mid[k] splits the edge leaving corner k.
            let mid: [Option<u32>; 3] = std::array::from_fn(|k| {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                let (pa, pb) = (positions[a as usize], positions[b as usize]);
                (pa.distance_squared(pb) > max_sq).then(|| {
                    *midpoints.entry((a.min(b), a.max(b))).or_insert_with(|| {
                        positions.push(0.5 * (pa + pb));
                        (positions.len() - 1) as u32
                    })
                })
            });

            match mid {
                [None, None, None] => triangles.push(tri),
                [Some(m0), Some(m1), Some(m2)] => {
                    triangles.push([tri[0], m0, m2]);
                    triangles.push([m0, tri[1], m1]);
                    triangles.push([m2, m1, tri[2]]);
                    triangles.push([m0, m1, m2]);
                }
                _ => {
                    let split = mid.iter().filter(|m| m.is_some()).count();
                    if split == 1 {
                        // Rotate so the split edge is a -> b.
                        let k = mid.iter().position(Option::is_some).unwrap();
                        let (a, b, c) = (tri[k], tri[(k + 1) % 3], tri[(k + 2) % 3]);
                        let m = mid[k].unwrap();
                        triangles.push([a, m, c]);
                        triangles.push([m, b, c]);
                    } else {
                        // Rotate so the intact edge is a -> b.
                        let k = mid.iter().position(Option::is_none).unwrap();
                        let (a, b, c) = (tri[k], tri[(k + 1) % 3], tri[(k + 2) % 3]);
                        let (mb, mc) = (mid[(k + 1) % 3].unwrap(), mid[(k + 2) % 3].unwrap());
                        triangles.push([mc, mb, c]);
                        // Split the remaining quad a-b-mb-mc along its shorter diagonal.
                        let p = |v: u32| -> Vec3 { positions[v as usize] };
                        if p(a).distance_squared(p(mb)) < p(b).distance_squared(p(mc)) {
                            triangles.push([a, b, mb]);
                            triangles.push([a, mb, mc]);
                        } else {
                            triangles.push([a, b, mc]);
                            triangles.push([b, mb, mc]);
                        }
                    }
                }
            }
        }

        mesh.triangles = triangles;
        if midpoints.is_empty() {
            break;
        }
    }
    mesh
}
