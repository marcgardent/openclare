use crate::error::{Error, Result};
use crate::math::{INVALID, Vec3};

/// Indexed triangle mesh: the input representation of the remesher.
#[derive(Clone, Debug, Default)]
pub struct TriMesh {
    pub positions: Vec<Vec3>,
    pub triangles: Vec<[u32; 3]>,
}

impl TriMesh {
    #[inline]
    pub fn corners(&self, f: usize) -> [Vec3; 3] {
        let [a, b, c] = self.triangles[f];
        [self.positions[a as usize], self.positions[b as usize], self.positions[c as usize]]
    }

    /// Face normal scaled by the face area.
    #[inline]
    pub fn area_vector(&self, f: usize) -> Vec3 {
        let [a, b, c] = self.corners(f);
        0.5 * (b - a).cross(c - a)
    }

    pub fn surface_area(&self) -> f64 {
        (0..self.triangles.len()).map(|f| self.area_vector(f).length()).sum()
    }

    /// Signed enclosed volume; positive when a closed mesh is wound outward.
    pub fn signed_volume(&self) -> f64 {
        (0..self.triangles.len())
            .map(|f| {
                let [a, b, c] = self.corners(f);
                a.dot(b.cross(c)) / 6.0
            })
            .sum()
    }

    /// `(min, mean, max)` over all triangle sides.
    pub fn edge_length_stats(&self) -> (f64, f64, f64) {
        let (mut min, mut sum, mut max) = (f64::INFINITY, 0.0, 0.0f64);
        for f in 0..self.triangles.len() {
            let c = self.corners(f);
            for k in 0..3 {
                let len = c[k].distance(c[(k + 1) % 3]);
                min = min.min(len);
                max = max.max(len);
                sum += len;
            }
        }
        let count = (3 * self.triangles.len()).max(1) as f64;
        (min, sum / count, max)
    }

    pub fn validate(&self) -> Result<()> {
        if self.triangles.is_empty() {
            return Err(Error::InvalidMesh("mesh has no triangles".into()));
        }
        if let Some(i) = self.positions.iter().position(|p| !p.is_finite()) {
            return Err(Error::InvalidMesh(format!("vertex {i} is not finite")));
        }
        let n = self.positions.len() as u32;
        for (f, tri) in self.triangles.iter().enumerate() {
            if tri.iter().any(|&v| v >= n) {
                return Err(Error::InvalidMesh(format!("triangle {f} references a missing vertex")));
            }
        }
        Ok(())
    }

    /// Drops degenerate triangles (repeated index or zero area) and the
    /// vertices nothing references any more.
    pub fn cleaned(&self) -> TriMesh {
        let mut remap = vec![INVALID; self.positions.len()];
        let mut out = TriMesh::default();
        for (f, tri) in self.triangles.iter().enumerate() {
            let [a, b, c] = *tri;
            if a == b || b == c || c == a || self.area_vector(f).length_squared() == 0.0 {
                continue;
            }
            out.triangles.push(tri.map(|v| {
                if remap[v as usize] == INVALID {
                    remap[v as usize] = out.positions.len() as u32;
                    out.positions.push(self.positions[v as usize]);
                }
                remap[v as usize]
            }));
        }
        out
    }
}
