//! Procedural test shapes. All are closed, manifold and wound outward.

use std::collections::HashMap;
use std::f64::consts::TAU;

use super::{PolyMesh, TriMesh};
use crate::math::Vec3;

/// Torus around the z axis: genus 1, so a singularity-free field exists.
pub fn torus(major_radius: f64, minor_radius: f64, major_segments: u32, minor_segments: u32) -> TriMesh {
    torus_quads(major_radius, minor_radius, major_segments, minor_segments).triangulated()
}

/// The same torus as a regular quad grid: a quad mesh with no irregular
/// vertex at all, to calibrate quality metrics.
pub fn torus_quads(major_radius: f64, minor_radius: f64, major_segments: u32, minor_segments: u32) -> PolyMesh {
    let (nu, nv) = (major_segments, minor_segments);
    let mut mesh = PolyMesh::default();
    for i in 0..nu {
        let u = TAU * i as f64 / nu as f64;
        for j in 0..nv {
            let v = TAU * j as f64 / nv as f64;
            let ring = major_radius + minor_radius * v.cos();
            mesh.positions.push(Vec3::new(ring * u.cos(), ring * u.sin(), minor_radius * v.sin()));
        }
    }
    let index = |i: u32, j: u32| (i % nu) * nv + (j % nv);
    for i in 0..nu {
        for j in 0..nv {
            mesh.faces.push(vec![index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
        }
    }
    mesh
}

/// Unit sphere from a subdivided icosahedron: genus 0, isotropic triangles
/// and no preferred direction, the canonical hard case for a cross field.
pub fn icosphere(subdivisions: u32) -> TriMesh {
    let t = (1.0 + 5.0f64.sqrt()) / 2.0;
    #[rustfmt::skip]
    let mut positions: Vec<Vec3> = [
        [-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0],
        [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t],
        [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0],
    ]
    .iter()
    .map(|&p| Vec3::from_array(p).normalize())
    .collect();
    #[rustfmt::skip]
    let mut triangles: Vec<[u32; 3]> = vec![
        [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
        [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
        [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
        [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
    ];

    for _ in 0..subdivisions {
        let mut midpoints: HashMap<(u32, u32), u32> = HashMap::new();
        let mut midpoint = |a: u32, b: u32| {
            *midpoints.entry((a.min(b), a.max(b))).or_insert_with(|| {
                positions.push((positions[a as usize] + positions[b as usize]).normalize());
                (positions.len() - 1) as u32
            })
        };
        triangles = triangles
            .iter()
            .flat_map(|&[a, b, c]| {
                let (ab, bc, ca) = (midpoint(a, b), midpoint(b, c), midpoint(c, a));
                [[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]]
            })
            .collect();
    }
    TriMesh { positions, triangles }
}

/// Axis-aligned cube `[-1, 1]³` with `segments` cells per side and welded
/// edges: the basic hard-surface case (12 creases, 8 corners).
pub fn cube(segments: u32) -> TriMesh {
    cube_quads(segments).triangulated()
}

/// The same cube as six quad grids: the 8 corners are its only irregular
/// vertices, which is the topological minimum for a sphere.
pub fn cube_quads(segments: u32) -> PolyMesh {
    let n = segments.max(1);
    let mut mesh = PolyMesh::default();
    let mut indices: HashMap<[u32; 3], u32> = HashMap::new();
    let mut vertex = |grid: [u32; 3]| {
        *indices.entry(grid).or_insert_with(|| {
            mesh.positions.push(Vec3::from_array(grid.map(|c| 2.0 * c as f64 / n as f64 - 1.0)));
            (mesh.positions.len() - 1) as u32
        })
    };

    let mut faces = Vec::new();
    for axis in 0..3 {
        for side in [0, n] {
            for a in 0..n {
                for b in 0..n {
                    let mut corner = |da: u32, db: u32| {
                        let mut grid = [0; 3];
                        grid[axis] = side;
                        grid[(axis + 1) % 3] = a + da;
                        grid[(axis + 2) % 3] = b + db;
                        vertex(grid)
                    };
                    let mut quad = [corner(0, 0), corner(1, 0), corner(1, 1), corner(0, 1)];
                    if side == 0 {
                        quad.reverse();
                    }
                    faces.push(quad.to_vec());
                }
            }
        }
    }
    mesh.faces = faces;
    mesh
}

/// Flat `[-1, 1]²` grid in the xy plane: the simplest open surface, for
/// boundary alignment.
pub fn plane(segments: u32) -> TriMesh {
    plane_quads(segments).triangulated()
}

/// The same plane as a grid of unit-aspect squares.
pub fn plane_quads(segments: u32) -> PolyMesh {
    let n = segments.max(1);
    let mut mesh = PolyMesh::default();
    for j in 0..=n {
        for i in 0..=n {
            mesh.positions.push(Vec3::new(2.0 * i as f64 / n as f64 - 1.0, 2.0 * j as f64 / n as f64 - 1.0, 0.0));
        }
    }
    let index = |i: u32, j: u32| j * (n + 1) + i;
    for j in 0..n {
        for i in 0..n {
            mesh.faces.push(vec![index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)]);
        }
    }
    mesh
}

/// The surface of a set of unit cubes on the integer lattice, each given by
/// its lowest corner, with `segments` quads along every unit edge. The cubes
/// must touch by whole faces and leave no edge shared by more than two
/// outer faces. Three cubes in an L make the simplest shape with an edge
/// that turns inwards.
pub fn blocks_quads(cells: &[[i32; 3]], segments: u32) -> PolyMesh {
    let n = segments.max(1) as i32;
    let filled: std::collections::HashSet<[i32; 3]> = cells.iter().copied().collect();
    let mut mesh = PolyMesh::default();
    let mut indices: HashMap<[i32; 3], u32> = HashMap::new();
    let mut vertex = |mesh: &mut PolyMesh, at: [i32; 3]| {
        *indices.entry(at).or_insert_with(|| {
            mesh.positions.push(Vec3::new(at[0] as f64, at[1] as f64, at[2] as f64) / n as f64);
            (mesh.positions.len() - 1) as u32
        })
    };
    for &cell in cells {
        for axis in 0..3 {
            for (side, step) in [(0, -1), (1, 1)] {
                let mut neighbour = cell;
                neighbour[axis] += step;
                if filled.contains(&neighbour) {
                    continue;
                }
                // The two other axes, in the order that winds the face
                // outwards.
                let (u, v) =
                    if side == 1 { ((axis + 1) % 3, (axis + 2) % 3) } else { ((axis + 2) % 3, (axis + 1) % 3) };
                for j in 0..n {
                    for i in 0..n {
                        let corner = |di: i32, dj: i32| {
                            let mut at = [cell[0] * n, cell[1] * n, cell[2] * n];
                            at[axis] += side * n;
                            at[u] += i + di;
                            at[v] += j + dj;
                            at
                        };
                        let face = [corner(0, 0), corner(1, 0), corner(1, 1), corner(0, 1)];
                        let face = face.map(|at| vertex(&mut mesh, at));
                        mesh.faces.push(face.to_vec());
                    }
                }
            }
        }
    }
    mesh
}

/// An L-shaped block: three unit cubes, with one edge that turns inwards.
pub fn l_block(segments: u32) -> TriMesh {
    blocks_quads(&[[0, 0, 0], [1, 0, 0], [0, 1, 0]], segments).triangulated()
}

/// A cylinder of radius 1 and height 2 around the z axis, with flat caps:
/// two hard edges that are closed curves without a corner. Each cap is a
/// grid of `segments` by `segments` quads.
pub fn cylinder(segments: u32) -> TriMesh {
    let n = segments.max(2);
    let layers = (n as f64 * 2.0 / std::f64::consts::FRAC_PI_2).round().max(1.0) as u32;
    let mut mesh = PolyMesh::default();
    // A square grid squeezed into a disk: its rim lands on the circle.
    let cap = |mesh: &mut PolyMesh, z: f64, upwards: bool| {
        let first = mesh.positions.len() as u32;
        for j in 0..=n {
            for i in 0..=n {
                let (x, y) = (2.0 * i as f64 / n as f64 - 1.0, 2.0 * j as f64 / n as f64 - 1.0);
                mesh.positions.push(Vec3::new(x * (1.0 - 0.5 * y * y).sqrt(), y * (1.0 - 0.5 * x * x).sqrt(), z));
            }
        }
        let index = |i: u32, j: u32| first + j * (n + 1) + i;
        for j in 0..n {
            for i in 0..n {
                let mut face = vec![index(i, j), index(i + 1, j), index(i + 1, j + 1), index(i, j + 1)];
                if !upwards {
                    face.reverse();
                }
                mesh.faces.push(face);
            }
        }
        // Its rim, counter-clockwise seen from above.
        let mut rim: Vec<u32> = (0..n).map(|i| index(i, 0)).collect();
        rim.extend((0..n).map(|j| index(n, j)));
        rim.extend((0..n).map(|i| index(n - i, n)));
        rim.extend((0..n).map(|j| index(0, n - j)));
        rim
    };
    let top = cap(&mut mesh, 1.0, true);
    let bottom = cap(&mut mesh, -1.0, false);
    let mut ring = top.clone();
    for layer in 1..=layers {
        let below: Vec<u32> = if layer == layers {
            bottom.clone()
        } else {
            let z = 1.0 - 2.0 * layer as f64 / layers as f64;
            top.iter()
                .map(|&v| {
                    let p = mesh.positions[v as usize];
                    mesh.positions.push(Vec3::new(p.x, p.y, z));
                    (mesh.positions.len() - 1) as u32
                })
                .collect()
        };
        for i in 0..ring.len() {
            let next = (i + 1) % ring.len();
            mesh.faces.push(vec![below[i], below[next], ring[next], ring[i]]);
        }
        ring = below;
    }
    mesh.triangulated()
}
