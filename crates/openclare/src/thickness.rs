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

/// How much inside a piece has for its surface, in parts of what it is
/// across, from which it is a sheet and no longer a strap: a little over
/// twice as wide as it is thick.
const SHEET: f64 = 0.35;

/// How much surface a piece may have, in squares of the diagonal of its
/// bounding box, and still be thin all along. A wire has little for the
/// room it takes, whichever way it is bent: a ring thirty times its wire
/// across has 0.16, a rod ten times its width long 0.3. What is compact has
/// much, a ball 1, a disc 1.5, a cube 2, and is not thin all along however
/// small: six quads make a rivet, where they crumple a ring.
const SLENDER: f64 = 0.5;

/// Vertices looked at to tell how thick a piece is as a whole.
const TYPICAL_SAMPLES: usize = 2000;

/// The girth of a piece as a whole, looking no farther than `reach`: small
/// only for what is thin all along, a wire, a ring, a strap. Infinite for
/// a piece that is thick, for one that is compact (a ball, a disc, a short
/// bolt), for a sheet however thin, and for one that is not closed.
///
/// Across is measured as everywhere else, and it is the one half of the
/// surface is under (vertices taken evenly, each counting for the area
/// round it, so that the finely cut rim of a plate does not speak for the
/// plate). The other width is not: a chord through a thin wall that curves,
/// the band round a lens, leaves it at once and finds it as narrow as it is
/// thin. It comes from how much inside the piece has for its surface: a
/// part `a` across and `w` wide holds `a w` for `2 (a + w)` of surface per
/// unit of length, so that a wire holds a quarter of `a` per unit of
/// surface, and a sheet a half. Near a half the width cannot be told from
/// it (a lens-shaped section of any width holds 0.45), and need not be: a
/// piece is then a sheet.
pub fn typical_girth(mesh: &TriMesh, reach: f64) -> f64 {
    let area = mesh.surface_area();
    let centre = mesh.positions.iter().fold(Vec3::ZERO, |sum, &p| sum + p) / mesh.positions.len().max(1) as f64;
    let volume: f64 = (mesh.triangles.iter())
        .map(|tri| {
            let [a, b, c] = tri.map(|v| mesh.positions[v as usize] - centre);
            a.dot(b.cross(c)) / 6.0
        })
        .sum::<f64>()
        .abs();
    let (low, high) = (mesh.positions.iter())
        .fold((Vec3::INFINITY, Vec3::NEG_INFINITY), |(low, high), &p| (low.min(p), high.max(p)));
    if area >= SLENDER * (high - low).length_squared() {
        return f64::INFINITY;
    }
    // What is said of the inside of a piece holds for a closed one only: a
    // sash of cloth, open all round, has no inside to speak of.
    if !Topology::build(mesh).is_closed() {
        return f64::INFINITY;
    }
    // Across is at least about twice the volume over the area (a slab;
    // more for a rod or a ball). A piece that is thick by that count is not
    // looked at closer, which spares a sculpt of millions of triangles its
    // rays.
    if area <= 0.0 || std::f64::consts::TAU * volume / area >= reach {
        return f64::INFINITY;
    }

    let surface = Bvh::build(mesh);
    let normals = crate::geometry::vertex_normals(mesh);
    let step_in = 1e-7 * reach;
    let out = |origin: Vec3, direction: Vec3| {
        surface.first_hit(origin, direction, reach, |_, front| !front).map_or(f64::INFINITY, |(distance, _)| distance)
    };
    let mut around = vec![0.0; mesh.positions.len()];
    for (f, tri) in mesh.triangles.iter().enumerate() {
        let third = mesh.area_vector(f).length() / 3.0;
        for &v in tri {
            around[v as usize] += third;
        }
    }
    let step = mesh.positions.len().div_ceil(TYPICAL_SAMPLES).max(1);
    let mut found: Vec<(f64, f64)> = (0..mesh.positions.len())
        .step_by(step)
        .map(|v| {
            let inward = -normals[v];
            if inward.length_squared() < 0.5 {
                return (f64::INFINITY, around[v]);
            }
            let (side, other) = tangents(inward);
            let origin = mesh.positions[v] + inward * step_in;
            let mut rays = vec![out(origin, inward)];
            for k in 0..CONE_RAYS {
                let turn = std::f64::consts::TAU * k as f64 / CONE_RAYS as f64;
                let aside = side * turn.cos() + other * turn.sin();
                rays.push(out(origin, inward * CONE.cos() + aside * CONE.sin()));
            }
            (median(&mut rays), around[v])
        })
        .collect();
    found.sort_by(|a, b| a.0.total_cmp(&b.0));
    let half = 0.5 * found.iter().map(|&(_, area)| area).sum::<f64>();
    let mut below = 0.0;
    let across = found.iter().find(|&&(_, area)| {
        below += area;
        below >= half
    });
    let Some(&(across, _)) = across.filter(|(across, _)| across.is_finite()) else {
        return f64::INFINITY;
    };
    // The inside there is for the surface, in parts of what is across: a
    // quarter for a wire, towards a half for a sheet.
    let inside = volume / (area * across);
    if inside >= SHEET {
        return f64::INFINITY;
    }
    let width = across * 2.0 * inside / (1.0 - 2.0 * inside);
    std::f64::consts::FRAC_PI_2 * (across + width)
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
    fn a_ring_is_thin_as_a_whole_and_a_ball_a_slab_or_a_band_is_not() {
        let ring = primitives::torus(2.0, 0.2, 96, 24);
        let girth = typical_girth(&ring, 10.0);
        assert!((girth - 0.4 * std::f64::consts::PI).abs() < 0.2, "girth {girth}");
        // Not looked at farther than the reach: thick enough.
        assert!(typical_girth(&ring, 0.1).is_infinite());
        assert!(typical_girth(&primitives::icosphere(3), 1.0).is_infinite());
        // Neither is a rivet, however small for the reach: it is compact.
        assert!(typical_girth(&primitives::icosphere(3), 100.0).is_infinite());
        assert!(typical_girth(&primitives::torus(2.0, 1.0, 48, 24), 100.0).is_infinite());
        let mut slab = primitives::cube(8);
        for p in &mut slab.positions {
            p.z *= 0.1;
        }
        // Thin across, but wide: far more round than a rod of that thickness
        // (0.2 π).
        assert!(typical_girth(&slab, 10.0).is_infinite());
        // A band round a lens: a ring whose section is squashed to a wall a
        // hundredth of its height. A chord through the wall would find it
        // as narrow as it is thin, where it curves; it is a sheet.
        let mut band = primitives::torus(2.0, 0.3, 96, 24);
        for p in &mut band.positions {
            let from_axis = p.truncate().length();
            let thinner = (2.0 + 0.01 * (from_axis - 2.0)) / from_axis;
            (p.x, p.y) = (p.x * thinner, p.y * thinner);
        }
        assert!(typical_girth(&band, 10.0).is_infinite());
        assert!(typical_girth(&primitives::plane(4), 10.0).is_infinite());
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
