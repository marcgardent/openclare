//! Baking: the relief of a detailed surface, read onto the texture of a
//! plainer one that lies along it.
//!
//! The plain surface (the *target*) has texture coordinates; the detailed
//! one (the *source*) is the sculpt it was made from. For each texel of the
//! texture that a face of the target covers, the point of the target it
//! stands for is found, and from there the source is looked for along the
//! normal of the target, both ways. How far it is, signed, is the height:
//! moved by that much along its normal, the point is on the source.
//!
//! The usual trouble of such a bake is a ray that finds the wrong sheet: the
//! other side of a finger, the next fold. Two things keep it away here. The
//! search goes no farther than a `reach` set from the size of the quads, a
//! target made by this engine being that close to its source; and a surface
//! is only taken if it faces the way the target does there, which the far
//! side of a thin part never does.

use crate::error::{Error, Result};
use crate::math::Vec3;
use crate::mesh::TriMesh;
use crate::spatial::Bvh;

/// The surface a texture is baked for: triangles with a normal at each
/// vertex and texture coordinates at each corner.
#[derive(Clone, Copy, Debug)]
pub struct Target<'a> {
    pub positions: &'a [Vec3],
    /// One per vertex: the direction the relief is measured along.
    pub normals: &'a [Vec3],
    pub triangles: &'a [[u32; 3]],
    /// For each triangle, the coordinates of its three corners in the
    /// texture, (0, 0) at the bottom left and (1, 1) at the top right.
    pub uvs: &'a [[[f64; 2]; 3]],
}

/// The height of the source above the target, texel by texel.
#[derive(Clone, Debug, Default)]
pub struct HeightMap {
    pub width: usize,
    pub height: usize,
    /// Row after row from the bottom: the signed distance from the target
    /// to the source along the normal of the target, in the unit of the
    /// meshes. Not a number where no face of the target lies and the
    /// padding did not reach.
    pub heights: Vec<f32>,
    /// Texels under a face of the target.
    pub covered: usize,
    /// Of those, the texels where the source was not found along the normal
    /// within the reach: the nearest point of the source was taken instead.
    pub astray: usize,
    /// The least and the greatest height among the texels written.
    pub low: f32,
    pub high: f32,
}

/// Where a texel is on the target: its triangle, and how much of each of
/// the first two corners it is made of.
#[derive(Clone, Copy)]
struct Place {
    texel: usize,
    triangle: u32,
    weights: [f64; 2],
    /// For a texel past the edge of a face: the point of that edge nearest
    /// to it, likewise, and how far past it the texel is, in texels.
    past: Option<([f64; 2], f64)>,
}

/// Bakes the height of `source` above `target` into a texture of `width` by
/// `height` texels, looking for the source no farther than `reach` from the
/// target along its normal. The texels within `padding` of a face go on
/// from it, so that a renderer that reads between two texels on the edge of
/// an island reads the height of that edge, the same from either side of a
/// seam.
pub fn height_map(
    target: &Target,
    source: &TriMesh,
    width: usize,
    height: usize,
    reach: f64,
    padding: usize,
) -> Result<HeightMap> {
    if width == 0 || height == 0 {
        return Err(Error::InvalidParams("a texture of no size".into()));
    }
    if !(reach.is_finite() && reach > 0.0) {
        return Err(Error::InvalidParams(format!("reach {reach} is not positive")));
    }
    if target.normals.len() != target.positions.len() || target.uvs.len() != target.triangles.len() {
        return Err(Error::InvalidParams(format!(
            "{} normals for {} vertices, {} sets of texture coordinates for {} triangles",
            target.normals.len(),
            target.positions.len(),
            target.uvs.len(),
            target.triangles.len()
        )));
    }
    if target.triangles.iter().flatten().any(|&v| v as usize >= target.positions.len()) {
        return Err(Error::InvalidMesh("a triangle of the target names a vertex it does not have".into()));
    }
    source.validate()?;

    // Which face each texel is under: its centre is what counts.
    let mut places: Vec<Place> = Vec::new();
    let mut taken = vec![false; width * height];
    for (triangle, corners) in target.uvs.iter().enumerate() {
        let [a, b, c] = corners.map(|[u, v]| [u * width as f64, v * height as f64]);
        let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if !area.is_finite() || area.abs() < 1e-12 {
            continue;
        }
        let low = [a[0].min(b[0]).min(c[0]), a[1].min(b[1]).min(c[1])];
        let high = [a[0].max(b[0]).max(c[0]), a[1].max(b[1]).max(c[1])];
        let columns = (low[0].floor().max(0.0) as usize)..(high[0].ceil().min(width as f64) as usize);
        let rows = (low[1].floor().max(0.0) as usize)..(high[1].ceil().min(height as f64) as usize);
        for row in rows {
            for column in columns.clone() {
                let p = [column as f64 + 0.5, row as f64 + 0.5];
                let of_a = ((b[0] - p[0]) * (c[1] - p[1]) - (b[1] - p[1]) * (c[0] - p[0])) / area;
                let of_b = ((c[0] - p[0]) * (a[1] - p[1]) - (c[1] - p[1]) * (a[0] - p[0])) / area;
                let texel = row * width + column;
                // A texel on a shared edge belongs to whichever face comes
                // first: both would give it the same point.
                if of_a >= -1e-9 && of_b >= -1e-9 && of_a + of_b <= 1.0 + 1e-9 && !taken[texel] {
                    taken[texel] = true;
                    places.push(Place { texel, triangle: triangle as u32, weights: [of_a, of_b], past: None });
                }
            }
        }
    }

    // Round the faces, the texels within `padding` of one go on from it. A
    // renderer reads a texture between its texels: on the edge of an island,
    // between one inside and one outside, and the two islands that meet
    // along a seam must both give there the height of the edge they share,
    // or the surface cracks. So a texel past an edge is given what makes
    // that so: the height at the nearest point of the edge, which is on the
    // surface and the same seen from either island, and from it as much
    // again as the height changes over the same way back inside the face.
    // (Not the height where the face would be if it went on past its edge:
    // where the surface bends that is nowhere near it, and what is found
    // from there belongs to another part of the sculpt.)
    let inside = places.len();
    if padding > 0 {
        let reach_in_texels = padding as f64;
        let mut beyond: std::collections::HashMap<usize, (f64, Place)> = std::collections::HashMap::new();
        for (triangle, corners) in target.uvs.iter().enumerate() {
            let [a, b, c] = corners.map(|[u, v]| [u * width as f64, v * height as f64]);
            let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if !area.is_finite() || area.abs() < 1e-12 {
                continue;
            }
            let low = [a[0].min(b[0]).min(c[0]) - reach_in_texels, a[1].min(b[1]).min(c[1]) - reach_in_texels];
            let high = [a[0].max(b[0]).max(c[0]) + reach_in_texels, a[1].max(b[1]).max(c[1]) + reach_in_texels];
            let columns = (low[0].floor().max(0.0) as usize)..(high[0].ceil().min(width as f64) as usize);
            let rows = (low[1].floor().max(0.0) as usize)..(high[1].ceil().min(height as f64) as usize);
            for row in rows {
                for column in columns.clone() {
                    let texel = row * width + column;
                    if taken[texel] {
                        continue;
                    }
                    let p = [column as f64 + 0.5, row as f64 + 0.5];
                    // The nearest point of the outline of the triangle.
                    let (away, on_edge) = [(a, b), (b, c), (c, a)]
                        .into_iter()
                        .map(|(from, to)| {
                            let (along, to_p) = ([to[0] - from[0], to[1] - from[1]], [p[0] - from[0], p[1] - from[1]]);
                            let length = along[0] * along[0] + along[1] * along[1];
                            let t = ((to_p[0] * along[0] + to_p[1] * along[1]) / length.max(1e-300)).clamp(0.0, 1.0);
                            let nearest = [from[0] + t * along[0], from[1] + t * along[1]];
                            ((p[0] - nearest[0]).hypot(p[1] - nearest[1]), nearest)
                        })
                        .fold((f64::INFINITY, p), |best, found| if found.0 < best.0 { found } else { best });
                    if away > reach_in_texels || beyond.get(&texel).is_some_and(|&(nearest, _)| nearest <= away) {
                        continue;
                    }
                    let shares = |q: [f64; 2]| {
                        let of_a = ((b[0] - q[0]) * (c[1] - q[1]) - (b[1] - q[1]) * (c[0] - q[0])) / area;
                        let of_b = ((c[0] - q[0]) * (a[1] - q[1]) - (c[1] - q[1]) * (a[0] - q[0])) / area;
                        [of_a, of_b]
                    };
                    let place =
                        Place { texel, triangle: triangle as u32, weights: shares(p), past: Some((shares(on_edge), away)) };
                    beyond.insert(texel, (away, place));
                }
            }
        }
        let mut beyond: Vec<Place> = beyond.into_values().map(|(_, place)| place).collect();
        beyond.sort_by_key(|place| place.texel);
        places.extend(beyond);
    }

    let surface = Bvh::build(source);
    let at = |triangle: u32, [of_a, of_b]: [f64; 2]| -> (f32, bool) {
        let [a, b, c] = target.triangles[triangle as usize].map(|v| v as usize);
        let of_c = 1.0 - of_a - of_b;
        let point = target.positions[a] * of_a + target.positions[b] * of_b + target.positions[c] * of_c;
        let normal = (target.normals[a] * of_a + target.normals[b] * of_b + target.normals[c] * of_c).normalize_or_zero();
        if normal == Vec3::ZERO {
            return (0.0, true);
        }
        // Out through the back of a face of the source that looks the way
        // the target does, or in through the front of one: never the far
        // side of a thin part, which looks the other way.
        let above = surface.first_hit(point, normal, reach, |_, front| !front).map(|(distance, _)| distance);
        let below = surface.first_hit(point, -normal, reach, |_, front| front).map(|(distance, _)| -distance);
        match (above, below) {
            (Some(up), Some(down)) => (if up <= -down { up } else { down } as f32, false),
            (Some(found), None) | (None, Some(found)) => (found as f32, false),
            (None, None) => {
                let nearest = surface.closest_point(point).map_or(0.0, |hit| (hit.point - point).dot(normal));
                (nearest.clamp(-reach, reach) as f32, true)
            }
        }
    };

    let measure = |place: &Place| -> (f32, bool) {
        let Some((on_edge, away)) = place.past else { return at(place.triangle, place.weights) };
        // As far back inside the face as the texel is past its edge, a
        // texel at most.
        let back = away.min(1.0) / away.max(1e-12);
        let inside = [0, 1].map(|k| on_edge[k] + (on_edge[k] - place.weights[k]) * back);
        let ((edge, lost), (within, _)) = (at(place.triangle, on_edge), at(place.triangle, inside));
        // Farther out than a texel the height goes on no more: nothing
        // reads there but a filter, and a slope carried on over several
        // texels stretches the range of the whole map.
        (edge + (edge - within), lost)
    };

    // The texels are measured several at a time: they do not depend on one
    // another.
    let threads = crate::remesher::threads().min(places.len().max(1));
    let share = places.len().div_ceil(threads).max(1);
    let mut measured: Vec<(f32, bool)> = Vec::with_capacity(places.len());
    std::thread::scope(|scope| {
        let workers: Vec<_> =
            places.chunks(share).map(|chunk| scope.spawn(|| chunk.iter().map(&measure).collect::<Vec<_>>())).collect();
        for worker in workers {
            measured.extend(worker.join().expect("a texel measured without a panic"));
        }
    });

    let mut heights = vec![f32::NAN; width * height];
    let (mut low, mut high, mut astray) = (f32::INFINITY, f32::NEG_INFINITY, 0);
    for (index, (place, &(found, lost))) in places.iter().zip(&measured).enumerate() {
        heights[place.texel] = found;
        // The least and the greatest are of every texel written, for an
        // image scaled on them to hold them all; what went astray is
        // counted under the faces only.
        (low, high) = (low.min(found), high.max(found));
        astray += (lost && index < inside) as usize;
    }
    if places.is_empty() {
        (low, high) = (0.0, 0.0);
    }

    Ok(HeightMap { width, height, heights, covered: inside, astray, low, high })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A square of side 1 in the plane z = `z`, facing up, or down if
    /// `flipped`.
    fn sheet(z: f64, flipped: bool) -> TriMesh {
        let positions = vec![Vec3::new(0.0, 0.0, z), Vec3::new(1.0, 0.0, z), Vec3::new(1.0, 1.0, z), Vec3::new(0.0, 1.0, z)];
        let triangles = if flipped { vec![[0, 2, 1], [0, 3, 2]] } else { vec![[0, 1, 2], [0, 2, 3]] };
        TriMesh { positions, triangles }
    }

    /// Both sheets as one mesh.
    fn together(one: &TriMesh, other: &TriMesh) -> TriMesh {
        let first = one.positions.len() as u32;
        let mut mesh = one.clone();
        mesh.positions.extend(&other.positions);
        mesh.triangles.extend(other.triangles.iter().map(|tri| tri.map(|v| v + first)));
        mesh
    }

    /// The unit square at z = 0 with the texture laid on the part of it
    /// from 0 to `share` of the width.
    fn bake(source: &TriMesh, share: f64, padding: usize) -> HeightMap {
        let flat = sheet(0.0, false);
        let normals = vec![Vec3::Z; 4];
        let uv = |v: u32| [flat.positions[v as usize].x * share, flat.positions[v as usize].y];
        let uvs: Vec<[[f64; 2]; 3]> = flat.triangles.iter().map(|tri| tri.map(uv)).collect();
        let target = Target { positions: &flat.positions, normals: &normals, triangles: &flat.triangles, uvs: &uvs };
        height_map(&target, source, 16, 16, 0.5, padding).unwrap()
    }

    #[test]
    fn the_height_is_how_far_the_source_is_along_the_normal_above_or_below() {
        let above = bake(&sheet(0.2, false), 1.0, 0);
        assert_eq!((above.covered, above.astray), (256, 0));
        assert!(above.heights.iter().all(|height| (height - 0.2).abs() < 1e-6));
        assert!((above.low - 0.2).abs() < 1e-6 && (above.high - 0.2).abs() < 1e-6);
        let below = bake(&sheet(-0.1, false), 1.0, 0);
        assert!(below.heights.iter().all(|height| (height + 0.1).abs() < 1e-6));
    }

    #[test]
    fn a_surface_that_looks_the_other_way_is_not_the_one() {
        // The far side of a thin part, nearer than the surface looked for.
        let source = together(&sheet(0.05, true), &sheet(0.2, false));
        let baked = bake(&source, 1.0, 0);
        assert!(baked.heights.iter().all(|height| (height - 0.2).abs() < 1e-6));
    }

    #[test]
    fn beyond_the_reach_the_nearest_point_is_taken_and_counted() {
        let baked = bake(&sheet(2.0, false), 1.0, 0);
        assert_eq!((baked.covered, baked.astray), (256, 256));
        // No farther than the reach, whatever is found.
        assert!(baked.heights.iter().all(|height| (height - 0.5).abs() < 1e-6));
    }

    #[test]
    fn texels_under_no_face_are_left_out_but_for_the_padding() {
        // The texture laid over the left half of the image only.
        let bare = bake(&sheet(0.2, false), 0.5, 0);
        assert_eq!(bare.covered, 128);
        assert_eq!(bare.heights.iter().filter(|height| height.is_nan()).count(), 128);
        let padded = bake(&sheet(0.2, false), 0.5, 2);
        // Two columns more, at the height of their neighbours.
        assert_eq!(padded.heights.iter().filter(|height| height.is_nan()).count(), 128 - 2 * 16);
        assert!(padded.heights.iter().filter(|height| !height.is_nan()).all(|height| (height - 0.2).abs() < 1e-6));
    }

    #[test]
    fn round_a_face_the_heights_go_on_as_the_face_does() {
        // A slope over the target, wider than it: 0.1 above its left edge,
        // 0.3 above its right one. The texture is laid on its left half.
        let mut slope = sheet(0.0, false);
        for p in &mut slope.positions {
            p.x = 3.0 * p.x - 1.0;
            p.z = 0.1 + 0.2 * p.x;
        }
        let baked = bake(&slope, 0.5, 2);
        let expected = |column: usize| 0.1 + 0.2 * (column as f64 + 0.5) / 8.0;
        for row in 0..16 {
            // Under the face, and the column past its right edge: the
            // height the slope has there, not a copy of the last column.
            for column in 0..9 {
                let found = baked.heights[row * 16 + column] as f64;
                assert!((found - expected(column)).abs() < 1e-6, "column {column}: {found}");
            }
            // The next one out goes on from the edge by a texel, no more.
            let farther = baked.heights[row * 16 + 9] as f64;
            assert!((farther - (0.3 + 0.2 / 8.0)).abs() < 1e-6, "{farther}");
            assert!(baked.heights[row * 16 + 10].is_nan());
        }
        // Read half way between the last texel inside and the first one
        // outside, as a renderer does on the edge: the height of the edge.
        let on_the_edge = 0.5 * (baked.heights[7] + baked.heights[8]);
        assert!((on_the_edge - 0.3).abs() < 1e-6, "{on_the_edge} on the edge");
        assert_eq!(baked.covered, 128);
        assert!((baked.high as f64 - 0.325).abs() < 1e-6);
    }

    /// The unit square at z = 0 with the texture laid on the texels from
    /// `low` to `high` of an image of 16 by 16, with normals all `normal`.
    fn bake_in_the_middle(source: &TriMesh, low: f64, high: f64, normal: Vec3, padding: usize) -> HeightMap {
        let flat = sheet(0.0, false);
        let normals = vec![normal; 4];
        let across = (high - low) / 16.0;
        let uv = |v: u32| [low / 16.0 + flat.positions[v as usize].x * across, low / 16.0 + flat.positions[v as usize].y * across];
        let uvs: Vec<[[f64; 2]; 3]> = flat.triangles.iter().map(|tri| tri.map(uv)).collect();
        let target = Target { positions: &flat.positions, normals: &normals, triangles: &flat.triangles, uvs: &uvs };
        height_map(&target, source, 16, 16, 0.5, padding).unwrap()
    }

    /// A plane much wider than the unit square, at `height(x, y)` above it.
    fn wide(height: impl Fn(f64, f64) -> f64) -> TriMesh {
        let mut plane = sheet(0.0, false);
        for p in &mut plane.positions {
            (p.x, p.y) = (5.0 * p.x - 2.0, 5.0 * p.y - 2.0);
            p.z = height(p.x, p.y);
        }
        plane
    }

    #[test]
    fn of_a_surface_above_and_one_below_the_nearer_is_taken() {
        let below = bake(&together(&sheet(0.3, false), &sheet(-0.1, false)), 1.0, 0);
        assert_eq!(below.astray, 0);
        assert!(below.heights.iter().all(|height| (height + 0.1).abs() < 1e-6), "{:?}", &below.heights[..4]);
        let above = bake(&together(&sheet(0.1, false), &sheet(-0.3, false)), 1.0, 0);
        assert!(above.heights.iter().all(|height| (height - 0.1).abs() < 1e-6), "{:?}", &above.heights[..4]);
    }

    #[test]
    fn below_too_a_surface_that_looks_the_other_way_is_not_the_one() {
        let source = together(&sheet(-0.05, true), &sheet(-0.2, false));
        let baked = bake(&source, 1.0, 0);
        assert_eq!(baked.astray, 0);
        assert!(baked.heights.iter().all(|height| (height + 0.2).abs() < 1e-6), "{:?}", &baked.heights[..4]);
    }

    #[test]
    fn below_as_above_nothing_is_taken_farther_than_the_reach() {
        let baked = bake(&sheet(-2.0, false), 1.0, 0);
        assert_eq!((baked.covered, baked.astray), (256, 256));
        assert!(baked.heights.iter().all(|height| (height + 0.5).abs() < 1e-6));
        assert!((baked.low + 0.5).abs() < 1e-6 && (baked.high + 0.5).abs() < 1e-6);
    }

    #[test]
    fn the_height_is_measured_along_the_normal_given_whatever_its_length() {
        // Normals leaning at 45 degrees: the plane 0.2 above is 0.2 times
        // the square root of two away that way.
        let leaning = Vec3::new(2.0, 0.0, 2.0);
        let baked = bake_in_the_middle(&wide(|_, _| 0.2), 0.0, 16.0, leaning, 0);
        assert_eq!((baked.covered, baked.astray), (256, 0));
        let expected = 0.2 * std::f64::consts::SQRT_2;
        assert!(baked.heights.iter().all(|&height| (height as f64 - expected).abs() < 1e-6), "{:?}", &baked.heights[..4]);
        // Without a normal there is no way to look: no height, and counted.
        let none = bake_in_the_middle(&wide(|_, _| 0.2), 0.0, 16.0, Vec3::ZERO, 0);
        assert_eq!((none.covered, none.astray), (256, 256));
        assert!(none.heights.iter().all(|&height| height == 0.0));
    }

    #[test]
    fn an_island_in_the_middle_goes_on_from_its_four_edges_and_its_corners() {
        // The texture on the texels 4 to 12 both ways; a slope both ways
        // over it. A texel is an eighth of the square.
        let slope = |x: f64, y: f64| 0.1 + 0.2 * x + 0.1 * y;
        // And on the texels 1 to 15: what goes on from it is the first and
        // the last row and column of the image.
        for (low, high, padding) in [(4.0, 12.0, 2), (1.0, 15.0, 1)] {
            let baked = bake_in_the_middle(&wide(slope), low, high, Vec3::Z, padding);
            let side = (high - low) as usize;
            assert_eq!((baked.covered, baked.astray), (side * side, 0));
            // Where a texel centre is on the square, texels as the unit.
            let on_the_square = |texel: f64| (texel - low) / (high - low);
            let mut round = 0;
            for row in 0..16 {
                for column in 0..16 {
                    let found = baked.heights[row * 16 + column] as f64;
                    let (x, y) = (column as f64 + 0.5, row as f64 + 0.5);
                    // How far past the square the texel is, each way.
                    let past = |t: f64| (low - t).max(t - high).max(0.0);
                    let away = past(x).hypot(past(y));
                    if away > padding as f64 {
                        assert!(found.is_nan(), "column {column}, row {row}: {found} so far from the island");
                        continue;
                    }
                    // Under the square and one texel out, the slope itself;
                    // farther out it goes on by one texel from the edge, no
                    // more: the height one texel past the nearest point of
                    // the outline, away from the square.
                    let nearest = (x.clamp(low, high), y.clamp(low, high));
                    let onward = if away > 1.0 { 1.0 / away } else { 1.0 };
                    let (at_x, at_y) = (nearest.0 + (x - nearest.0) * onward, nearest.1 + (y - nearest.1) * onward);
                    let expected = slope(on_the_square(at_x), on_the_square(at_y));
                    assert!((found - expected).abs() < 1e-6, "column {column}, row {row}: {found} for {expected}");
                    round += (away > 0.0) as usize;
                }
            }
            // All round: the rows and columns past each side, and the
            // corners but for what is farther than the padding.
            assert_eq!(round, if padding == 2 { 4 * 2 * 8 + 4 * 3 } else { 4 * 14 + 4 });
        }
    }

    #[test]
    fn an_island_larger_than_the_image_is_read_where_the_image_is() {
        // The texture laid from 4 texels before the image to 4 past it.
        let slope = |x: f64, y: f64| 0.1 + 0.2 * x + 0.1 * y;
        let baked = bake_in_the_middle(&wide(slope), -4.0, 20.0, Vec3::Z, 2);
        assert_eq!((baked.covered, baked.astray), (256, 0));
        for row in 0..16 {
            for column in 0..16 {
                let expected = slope((column as f64 + 4.5) / 24.0, (row as f64 + 4.5) / 24.0);
                let found = baked.heights[row * 16 + column] as f64;
                assert!((found - expected).abs() < 1e-6, "column {column}, row {row}: {found} for {expected}");
            }
        }
    }

    #[test]
    fn each_texel_is_read_from_the_face_it_is_under() {
        // The square laid as a trapezium: its two triangles are not laid
        // the same way, and a texel read from the wrong one is read from
        // another point of the square.
        let corners = [[2.0, 2.0], [14.0, 2.0], [10.0, 14.0], [2.0, 14.0]];
        let slope = |x: f64, y: f64| 0.1 + 0.2 * x + 0.1 * y;
        let flat = sheet(0.0, false);
        let normals = vec![Vec3::Z; 4];
        let uvs: Vec<[[f64; 2]; 3]> =
            flat.triangles.iter().map(|tri| tri.map(|v| corners[v as usize].map(|c| c / 16.0))).collect();
        let target = Target { positions: &flat.positions, normals: &normals, triangles: &flat.triangles, uvs: &uvs };
        let baked = height_map(&target, &wide(slope), 16, 16, 0.5, 0).unwrap();
        // How much of each corner of a triangle a point of the texture is
        // made of, by the areas of the triangles it makes with the others.
        let twice = |p: [f64; 2], q: [f64; 2], r: [f64; 2]| (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0]);
        let mut under = 0;
        for row in 0..16 {
            for column in 0..16 {
                let p = [column as f64 + 0.5, row as f64 + 0.5];
                let found = baked.heights[row * 16 + column] as f64;
                let mut expected = None;
                for tri in &flat.triangles {
                    let [a, b, c] = tri.map(|v| corners[v as usize]);
                    let whole = twice(a, b, c);
                    let shares = [twice(p, b, c) / whole, twice(a, p, c) / whole, twice(a, b, p) / whole];
                    if shares.iter().all(|&share| share >= 0.0) {
                        let point: Vec3 = (0..3).map(|k| flat.positions[tri[k] as usize] * shares[k]).sum();
                        expected = Some(slope(point.x, point.y));
                    }
                }
                match expected {
                    Some(expected) => {
                        assert!((found - expected).abs() < 1e-6, "column {column}, row {row}: {found} for {expected}");
                        under += 1;
                    }
                    None => assert!(found.is_nan(), "column {column}, row {row}: {found} under no face"),
                }
            }
        }
        assert_eq!(baked.covered, under);
        assert!(under > 100, "{under} texels under the trapezium");
    }

    #[test]
    fn a_face_with_no_area_in_the_texture_writes_nothing() {
        // Its three corners on one line of the texture.
        let flat = sheet(0.0, false);
        let normals = vec![Vec3::Z; 4];
        let uvs = vec![[[0.25, 0.25], [0.5, 0.5], [0.75, 0.75]]; 2];
        let target = Target { positions: &flat.positions, normals: &normals, triangles: &flat.triangles, uvs: &uvs };
        for padding in [0, 2] {
            let baked = height_map(&target, &sheet(0.2, false), 16, 16, 0.5, padding).unwrap();
            assert_eq!(baked.covered, 0);
            assert!(baked.heights.iter().all(|height| height.is_nan()), "padding {padding}");
        }
    }

    #[test]
    fn where_the_normal_misses_the_height_is_to_the_nearest_point_along_the_normal() {
        // Normals leaning at 45 degrees towards x, and a source 0.2 above
        // that stops half way across: its left half only.
        let mut source = sheet(0.2, false);
        for p in &mut source.positions {
            p.x *= 0.5;
        }
        let leaning = Vec3::new(1.0, 0.0, 1.0);
        let baked = bake_in_the_middle(&source, 0.0, 16.0, leaning, 0);
        let root = std::f64::consts::SQRT_2;
        let mut astray = 0;
        for row in 0..16 {
            for column in 0..16 {
                let x = (column as f64 + 0.5) / 16.0;
                // The normal meets the source 0.2 farther along x.
                let expected = if x + 0.2 <= 0.5 {
                    0.2 * root
                } else {
                    astray += 1;
                    // The nearest point is straight above, or on the edge
                    // of the source, behind.
                    ((x.min(0.5) - x) + 0.2) / root
                };
                let found = baked.heights[row * 16 + column] as f64;
                assert!((found - expected).abs() < 1e-6, "column {column}, row {row}: {found} for {expected}");
            }
        }
        assert_eq!((baked.covered, baked.astray), (256, astray));
        assert_eq!(astray, 11 * 16);
    }

    #[test]
    fn an_island_laid_askew_is_read_as_it_lies() {
        // The square laid on the texture turned and sheared: its corner at
        // the texel `origin`, its sides `along` and `up`. A slope both ways
        // over it.
        let (origin, along, up) = ([5.0, 3.0], [8.0, 3.0], [-2.0, 7.0]);
        let slope = |x: f64, y: f64| 0.1 + 0.2 * x + 0.1 * y;
        let flat = sheet(0.0, false);
        let normals = vec![Vec3::Z; 4];
        let uv = |v: u32| {
            let p = flat.positions[v as usize];
            [(origin[0] + along[0] * p.x + up[0] * p.y) / 16.0, (origin[1] + along[1] * p.x + up[1] * p.y) / 16.0]
        };
        let uvs: Vec<[[f64; 2]; 3]> = flat.triangles.iter().map(|tri| tri.map(uv)).collect();
        let target = Target { positions: &flat.positions, normals: &normals, triangles: &flat.triangles, uvs: &uvs };
        let baked = height_map(&target, &wide(slope), 16, 16, 0.5, 1).unwrap();

        // Where a point of the texture is on the square, and how far it is
        // from the outline of the island, in texels.
        let det = along[0] * up[1] - along[1] * up[0];
        let on_the_square = |x: f64, y: f64| {
            let (dx, dy) = (x - origin[0], y - origin[1]);
            ((dx * up[1] - dy * up[0]) / det, (along[0] * dy - along[1] * dx) / det)
        };
        let corner = |k: usize| uv([0, 1, 2, 3][k % 4]).map(|c| c * 16.0);
        let from_the_outline = |x: f64, y: f64| {
            (0..4)
                .map(|k| {
                    let (from, to) = (corner(k), corner(k + 1));
                    let (side, to_p) = ([to[0] - from[0], to[1] - from[1]], [x - from[0], y - from[1]]);
                    let t = ((to_p[0] * side[0] + to_p[1] * side[1]) / (side[0] * side[0] + side[1] * side[1])).clamp(0.0, 1.0);
                    (x - from[0] - t * side[0]).hypot(y - from[1] - t * side[1])
                })
                .fold(f64::INFINITY, f64::min)
        };
        let (mut under, mut round) = (0, 0);
        for row in 0..16 {
            for column in 0..16 {
                let (x, y) = (column as f64 + 0.5, row as f64 + 0.5);
                let (on_x, on_y) = on_the_square(x, y);
                let inside = (0.0..=1.0).contains(&on_x) && (0.0..=1.0).contains(&on_y);
                let found = baked.heights[row * 16 + column] as f64;
                if inside || from_the_outline(x, y) < 1.0 {
                    // The slope where the texel is, going on past the edge.
                    let expected = slope(on_x, on_y);
                    assert!((found - expected).abs() < 1e-6, "column {column}, row {row}: {found} for {expected}");
                    under += inside as usize;
                    round += !inside as usize;
                } else {
                    assert!(found.is_nan(), "column {column}, row {row}: {found} away from the island");
                }
            }
        }
        assert_eq!(baked.covered, under);
        assert!(under > 50 && round > 25, "{under} texels under the island, {round} round it");
    }

    #[test]
    fn between_two_islands_a_texel_goes_on_from_the_nearer_one() {
        // Two squares, one 0.2 under the source and one 0.1 under it, each
        // with its island: columns 0 to 6 and 10 to 16. The four columns
        // between them are shared out, two each, whichever comes first.
        let source = wide(|_, _| 0.2);
        for swapped in [false, true] {
            let (first, second) = (sheet(0.0, false), sheet(0.1, false));
            let both = together(&first, &second);
            let normals = vec![Vec3::Z; 8];
            let left = |v: u32| [both.positions[v as usize].x * 6.0 / 16.0, both.positions[v as usize].y];
            let right = |v: u32| [(10.0 + both.positions[v as usize].x * 6.0) / 16.0, both.positions[v as usize].y];
            let mut triangles = both.triangles.clone();
            let mut uvs: Vec<[[f64; 2]; 3]> =
                triangles.iter().enumerate().map(|(k, tri)| if k < 2 { tri.map(left) } else { tri.map(right) }).collect();
            if swapped {
                triangles.reverse();
                uvs.reverse();
            }
            let target = Target { positions: &both.positions, normals: &normals, triangles: &triangles, uvs: &uvs };
            let baked = height_map(&target, &source, 16, 16, 0.5, 3).unwrap();
            assert_eq!(baked.covered, 2 * 6 * 16);
            for row in 0..16 {
                for column in 0..16 {
                    let expected = if column < 8 { 0.2 } else { 0.1 };
                    let found = baked.heights[row * 16 + column];
                    assert!((found - expected).abs() < 1e-6, "column {column}, row {row}, swapped {swapped}: {found}");
                }
            }
            assert!((baked.low - 0.1).abs() < 1e-6 && (baked.high - 0.2).abs() < 1e-6);
        }
    }

    #[test]
    fn a_target_without_a_face_gives_an_empty_map() {
        let flat = sheet(0.0, false);
        let target = Target { positions: &flat.positions, normals: &[Vec3::Z; 4], triangles: &[], uvs: &[] };
        let baked = height_map(&target, &flat, 4, 4, 0.5, 2).unwrap();
        assert_eq!((baked.covered, baked.astray, baked.low, baked.high), (0, 0, 0.0, 0.0));
        assert!(baked.heights.len() == 16 && baked.heights.iter().all(|height| height.is_nan()));
    }

    #[test]
    fn what_cannot_be_baked_is_refused() {
        let flat = sheet(0.0, false);
        let normals = vec![Vec3::Z; 4];
        let uvs = vec![[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]; 2];
        let target = Target { positions: &flat.positions, normals: &normals, triangles: &flat.triangles, uvs: &uvs };
        assert!(height_map(&target, &flat, 0, 16, 0.5, 0).is_err());
        assert!(height_map(&target, &flat, 16, 16, 0.0, 0).is_err());
        let short = Target { normals: &normals[..3], ..target };
        assert!(height_map(&short, &flat, 16, 16, 0.5, 0).is_err());
    }
}
