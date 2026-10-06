pub use glam::DVec3 as Vec3;

/// Sentinel for "no index" in the `u32` index arrays used throughout.
pub const INVALID: u32 = u32::MAX;

/// Orthonormal tangent frame `(s, t)` of the plane with unit normal `n`,
/// oriented so that `s × t = n`.
#[inline]
pub fn tangent_frame(n: Vec3) -> (Vec3, Vec3) {
    let s = n.any_orthonormal_vector();
    (s, n.cross(s))
}

/// Small deterministic PRNG (splitmix64). Remeshing must be reproducible, so
/// every random choice goes through a seeded instance of this.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            items.swap(i, self.below(i + 1));
        }
    }
}
