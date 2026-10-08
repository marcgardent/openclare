//! Thickness: how much shape there is under each vertex.
//!
//! Two lengths, both measured through the inside of the shape:
//!
//! - *across*: how far to the opposite side, straight in from the vertex.
//!   A finger, an ear and a sheet of cloth are all thin in that sense.
//! - *girth*: how far it is round the part the vertex sits on, taken at the
//!   narrowest. Half way across, the shortest chord that runs along the
//!   surface gives the other width of the part; a finger is as wide as it
//!   is thick, an ear or a sheet much wider. The girth is then that of the
//!   ellipse with those two widths.
//!
//! The girth is what tells how small quads must be for a part to keep its
//! volume: with too few quads round it, a finger comes out flat. Across is
//! the shape diameter function of Shapira, Shamir and Cohen-Or (2008), with
//! fewer rays.
//!
//! Both are infinite where no opposite side is found: on an open sheet, or
//! farther than the reach given.

use crate::math::Vec3;
use crate::mesh::{Topology, TriMesh};
use crate::spatial::Bvh;

/// Thickness at each vertex of a mesh.
#[derive(Clone, Debug, Default)]
pub struct Thickness {
    pub across: Vec<f64>,
    /// The other width of the part: its shortest chord half way across.
    pub width: Vec<f64>,
    pub girth: Vec<f64>,
}

/// How far off the inward normal the rays that measure across spread. A
/// slanted ray finds a slab thicker and a ball thinner than they are: at 20
/// degrees, by 6 % either way.
const CONE: f64 = std::f64::consts::PI / 9.0;
/// Rays round the cone, besides the one down its axis.
const CONE_RAYS: usize = 6;
/// Chords tried half way across, evenly turned.
const CHORDS: usize = 4;

/// Measures the thickness under each vertex of `mesh`, looking no farther
/// than `reach`.
pub fn measure(mesh: &TriMesh, normals: &[Vec3], reach: f64) -> Thickness {
    let surface = Bvh::build(mesh);
    let vertices = mesh.positions.len();
    // A ray that starts inside the shape leaves it through the back of a
    // triangle.
    let out = |origin: Vec3, direction: Vec3, reach: f64| {
        surface.first_hit(origin, direction, reach, |_, front| !front).map_or(f64::INFINITY, |(distance, _)| distance)
    };
    let step_in = 1e-7 * reach;

    let mut across = vec![f64::INFINITY; vertices];
    for v in 0..vertices {
        let inward = -normals[v];
        if inward.length_squared() < 0.5 {
            continue;
        }
        let (side, other) = tangents(inward);
        let origin = mesh.positions[v] + inward * step_in;
        let mut found = vec![out(origin, inward, reach)];
        for k in 0..CONE_RAYS {
            let turn = std::f64::consts::TAU * k as f64 / CONE_RAYS as f64;
            let aside = side * turn.cos() + other * turn.sin();
            let direction = inward * CONE.cos() + aside * CONE.sin();
            found.push(out(origin, direction, reach));
        }
        across[v] = median(&mut found);
    }
    // One vertex alone can be fooled by a fold of the surface next to it.
    let topology = Topology::build(mesh);
    let raw = across.clone();
    for v in 0..vertices {
        let mut around: Vec<f64> = topology.neighbors[v].iter().map(|&nb| raw[nb as usize]).collect();
        around.push(raw[v]);
        across[v] = median(&mut around);
    }

    let mut girth = vec![f64::INFINITY; vertices];
    let mut widths = vec![f64::INFINITY; vertices];
    for v in 0..vertices {
        if !across[v].is_finite() {
            continue;
        }
        let inward = -normals[v];
        let (side, other) = tangents(inward);
        let middle = mesh.positions[v] + inward * (0.5 * across[v]);
        let width = (0..CHORDS)
            .map(|k| {
                let turn = std::f64::consts::PI * k as f64 / CHORDS as f64;
                let along = side * turn.cos() + other * turn.sin();
                out(middle, along, reach) + out(middle, -along, reach)
            })
            .fold(f64::INFINITY, f64::min);
        // Round an ellipse of those two widths, near enough.
        widths[v] = width;
        girth[v] = std::f64::consts::FRAC_PI_2 * (across[v] + width);
    }
    Thickness { across, width: widths, girth }
}

/// Two unit vectors that make a frame with `axis`.
fn tangents(axis: Vec3) -> (Vec3, Vec3) {
    let helper = if axis.x.abs() < 0.7 { Vec3::X } else { Vec3::Y };
    let side = axis.cross(helper).normalize();
    (side, axis.cross(side))
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vertex_normals;
    use crate::mesh::primitives;

    #[test]
    fn a_ball_is_as_thick_as_it_is_wide() {
        let mesh = primitives::icosphere(3);
        let thickness = measure(&mesh, &vertex_normals(&mesh), 10.0);
        for v in 0..mesh.positions.len() {
            assert!((thickness.across[v] - 2.0).abs() < 0.15, "across {}", thickness.across[v]);
            assert!((thickness.girth[v] - std::f64::consts::TAU).abs() < 0.4, "girth {}", thickness.girth[v]);
        }
    }

    #[test]
    fn a_ring_is_thin_whatever_its_size() {
        // A torus of tube radius 0.2: 0.4 across, 0.4 π round the tube.
        let mesh = primitives::torus(2.0, 0.2, 96, 24);
        let thickness = measure(&mesh, &vertex_normals(&mesh), 10.0);
        for v in 0..mesh.positions.len() {
            assert!((thickness.across[v] - 0.4).abs() < 0.04, "across {}", thickness.across[v]);
            assert!((thickness.girth[v] - 0.4 * std::f64::consts::PI).abs() < 0.15, "girth {}", thickness.girth[v]);
        }
    }

    #[test]
    fn a_slab_is_thin_across_but_wide_around() {
        // The unit cube squashed to a tenth of its height.
        let mut mesh = primitives::cube(8);
        for p in &mut mesh.positions {
            p.z *= 0.1;
        }
        let thickness = measure(&mesh, &vertex_normals(&mesh), 10.0);
        for v in (0..mesh.positions.len()).filter(|&v| mesh.positions[v].truncate().abs().max_element() < 0.5) {
            assert!((thickness.across[v] - 0.2).abs() < 0.02, "across {}", thickness.across[v]);
            assert!(thickness.girth[v] > 3.0, "girth {}", thickness.girth[v]);
        }
    }

    #[test]
    fn an_open_sheet_has_no_thickness_to_speak_of() {
        let mesh = primitives::plane(4);
        let thickness = measure(&mesh, &vertex_normals(&mesh), 10.0);
        assert!(thickness.across.iter().chain(&thickness.girth).all(|length| length.is_infinite()));
    }

    #[test]
    fn nothing_is_found_beyond_the_reach() {
        let mesh = primitives::icosphere(2);
        let thickness = measure(&mesh, &vertex_normals(&mesh), 1.0);
        assert!(thickness.across.iter().all(|length| length.is_infinite()));
    }
}
