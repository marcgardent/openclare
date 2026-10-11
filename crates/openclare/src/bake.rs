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
    /// The least and the greatest height among the texels covered.
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
}

/// Bakes the height of `source` above `target` into a texture of `width` by
/// `height` texels, looking for the source no farther than `reach` from the
/// target along its normal. The texels round the faces are filled from
/// their neighbours over `padding` texels, so that a renderer that reads
/// between two texels at the rim of an island reads no hole.
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
                    places.push(Place { texel, triangle: triangle as u32, weights: [of_a, of_b] });
                }
            }
        }
    }

    let surface = Bvh::build(source);
    let measure = |place: &Place| -> (f32, bool) {
        let [a, b, c] = target.triangles[place.triangle as usize].map(|v| v as usize);
        let [of_a, of_b] = place.weights;
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
    for (place, &(found, lost)) in places.iter().zip(&measured) {
        heights[place.texel] = found;
        (low, high) = (low.min(found), high.max(found));
        astray += lost as usize;
    }
    if places.is_empty() {
        (low, high) = (0.0, 0.0);
    }

    // Round the faces, each texel takes the mean of its neighbours that
    // have a height, one ring at a time.
    for _ in 0..padding {
        let before = heights.clone();
        for row in 0..height {
            for column in 0..width {
                if !before[row * width + column].is_nan() {
                    continue;
                }
                let (mut sum, mut count) = (0.0, 0);
                for (across, up) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
                    let (x, y) = (column as i64 + across, row as i64 + up);
                    if x < 0 || y < 0 || x >= width as i64 || y >= height as i64 {
                        continue;
                    }
                    let near = before[y as usize * width + x as usize];
                    if !near.is_nan() {
                        sum += near;
                        count += 1;
                    }
                }
                if count > 0 {
                    heights[row * width + column] = sum / count as f32;
                }
            }
        }
    }
    Ok(HeightMap { width, height, heights, covered: places.len(), astray, low, high })
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
