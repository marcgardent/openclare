//! Symmetry-aware comparison of field values at two neighbouring vertices.
//!
//! A cross is stored as one unit tangent `q`; the other three directions are
//! its rotations by 90° about the normal. A lattice is stored as one of its
//! points `o`; the others are `o + scale * (a * q + b * (n × q))` for integer
//! `a`, `b`. "Compatible" means: the representatives of each side that are
//! closest to one another in 3D.

use crate::math::Vec3;

/// The representatives of crosses `(q0, n0)` and `(q1, n1)` that are best
/// aligned with each other.
#[inline]
pub fn compat_orientation(q0: Vec3, n0: Vec3, q1: Vec3, n1: Vec3) -> (Vec3, Vec3) {
    let a = [q0, n0.cross(q0)];
    let b = [q1, n1.cross(q1)];
    let (i, j) = best_axis_pair(&a, &b);
    (a[i], b[j] * a[i].dot(b[j]).signum())
}

/// Same matching as [`compat_orientation`], returned as the number of quarter
/// turns applied to each cross. Summed around a loop, the differences give
/// the index of the field inside it.
#[inline]
pub fn compat_orientation_index(q0: Vec3, n0: Vec3, q1: Vec3, n1: Vec3) -> (i32, i32) {
    let a = [q0, n0.cross(q0)];
    let b = [q1, n1.cross(q1)];
    let (i, j) = best_axis_pair(&a, &b);
    let flip = if a[i].dot(b[j]) < 0.0 { 2 } else { 0 };
    (i as i32, j as i32 + flip)
}

#[inline]
fn best_axis_pair(a: &[Vec3; 2], b: &[Vec3; 2]) -> (usize, usize) {
    let mut best = (0, 0);
    let mut best_score = f64::NEG_INFINITY;
    for i in 0..2 {
        for j in 0..2 {
            let score = a[i].dot(b[j]).abs();
            if score > best_score {
                best_score = score;
                best = (i, j);
            }
        }
    }
    best
}

/// Point closest to both `p0` and `p1` among those lying on both tangent
/// planes: where two neighbouring lattices can be compared without the
/// curvature between them getting in the way.
#[inline]
pub fn middle_point(p0: Vec3, n0: Vec3, p1: Vec3, n1: Vec3) -> Vec3 {
    let (n0p0, n0p1) = (n0.dot(p0), n0.dot(p1));
    let (n1p0, n1p1) = (n1.dot(p0), n1.dot(p1));
    let n0n1 = n0.dot(n1);
    // The epsilon keeps the parallel-plane case finite; the two multipliers
    // then cancel out and the result is the plain midpoint.
    let denom = 1.0 / (1.0 - n0n1 * n0n1 + 1e-4);
    let lambda0 = 2.0 * (n0p1 - n0p0 - n0n1 * (n1p0 - n1p1)) * denom;
    let lambda1 = 2.0 * (n1p0 - n1p1 - n0n1 * (n0p1 - n0p0)) * denom;
    0.5 * (p0 + p1) - 0.25 * (n0 * lambda0 + n1 * lambda1)
}

/// Integer cell of the lattice `(o, q, t)` that contains `p`.
#[inline]
fn lattice_floor_index(o: Vec3, q: Vec3, t: Vec3, p: Vec3, inv_scale: f64) -> [i32; 2] {
    let d = p - o;
    [(q.dot(d) * inv_scale).floor() as i32, (t.dot(d) * inv_scale).floor() as i32]
}

/// Lattice point of `(o, q, n)` nearest to `p`.
#[inline]
pub fn lattice_round(o: Vec3, q: Vec3, n: Vec3, p: Vec3, scale: f64, inv_scale: f64) -> Vec3 {
    let t = n.cross(q);
    let d = p - o;
    o + q * ((q.dot(d) * inv_scale).round() * scale) + t * ((t.dot(d) * inv_scale).round() * scale)
}

/// Integer translations of each lattice that bring their points closest
/// together near the edge midpoint, and the squared distance left between
/// them. Equal translations mean both vertices snap to the same lattice
/// point; translations one step apart mean their points share a lattice edge.
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn compat_position_index(
    p0: Vec3,
    n0: Vec3,
    q0: Vec3,
    o0: Vec3,
    p1: Vec3,
    n1: Vec3,
    q1: Vec3,
    o1: Vec3,
    scale: f64,
    inv_scale: f64,
) -> ([i32; 2], [i32; 2], f64) {
    let (t0, t1) = (n0.cross(q0), n1.cross(q1));
    let middle = middle_point(p0, n0, p1, n1);
    let f0 = lattice_floor_index(o0, q0, t0, middle, inv_scale);
    let f1 = lattice_floor_index(o1, q1, t1, middle, inv_scale);

    // Compare the four corners of the cell around the midpoint on each side.
    let mut best = ([0; 2], [0; 2], f64::INFINITY);
    for i in 0..4 {
        let c0 = [f0[0] + (i & 1), f0[1] + (i >> 1)];
        let x0 = o0 + (q0 * c0[0] as f64 + t0 * c0[1] as f64) * scale;
        for j in 0..4 {
            let c1 = [f1[0] + (j & 1), f1[1] + (j >> 1)];
            let x1 = o1 + (q1 * c1[0] as f64 + t1 * c1[1] as f64) * scale;
            let cost = x0.distance_squared(x1);
            if cost < best.2 {
                best = (c0, c1, cost);
            }
        }
    }
    best
}

/// The lattice points of each side matched by [`compat_position_index`].
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn compat_position(
    p0: Vec3,
    n0: Vec3,
    q0: Vec3,
    o0: Vec3,
    p1: Vec3,
    n1: Vec3,
    q1: Vec3,
    o1: Vec3,
    scale: f64,
    inv_scale: f64,
) -> (Vec3, Vec3) {
    let (c0, c1, _) = compat_position_index(p0, n0, q0, o0, p1, n1, q1, o1, scale, inv_scale);
    let (t0, t1) = (n0.cross(q0), n1.cross(q1));
    (o0 + (q0 * c0[0] as f64 + t0 * c0[1] as f64) * scale, o1 + (q1 * c1[0] as f64 + t1 * c1[1] as f64) * scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_matching_is_invariant_to_quarter_turns() {
        let n = Vec3::Z;
        let q0 = Vec3::new(1.0, 0.1, 0.0).normalize();
        // The same cross as q0, stored through another of its four arms.
        let q1 = n.cross(n.cross(n.cross(q0)));
        let (a, b) = compat_orientation(q0, n, q1, n);
        assert!(a.dot(b) > 1.0 - 1e-12);
    }

    #[test]
    fn middle_point_lies_on_both_planes() {
        let (p0, n0) = (Vec3::new(0.0, 1.0, 0.3), Vec3::X);
        let (p1, n1) = (Vec3::new(1.0, 0.0, -0.2), Vec3::Y);
        let m = middle_point(p0, n0, p1, n1);
        assert!(n0.dot(m - p0).abs() < 1e-3);
        assert!(n1.dot(m - p1).abs() < 1e-3);
    }

    #[test]
    fn identical_lattices_have_zero_offset() {
        let (n, q, scale) = (Vec3::Z, Vec3::X, 0.5);
        let o = Vec3::new(0.1, 0.2, 0.0);
        // Same lattice, stored through a different point and a rotated arm.
        let o_shifted = o + Vec3::new(3.0 * scale, -2.0 * scale, 0.0);
        let (p0, p1) = (Vec3::new(0.2, 0.2, 0.0), Vec3::new(0.3, 0.25, 0.0));
        let (a, b) = compat_position(p0, n, q, o, p1, n, Vec3::Y, o_shifted, scale, 1.0 / scale);
        assert!(a.distance(b) < 1e-12);
    }
}
