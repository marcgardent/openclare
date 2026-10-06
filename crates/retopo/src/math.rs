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

/// Complex number, for fields with rotational symmetry: a cross of angle `θ`
/// is stored as `e^{4iθ}`, which is the same for its four arms.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const ZERO: Complex = Complex { re: 0.0, im: 0.0 };

    #[inline]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// `e^{i angle}`.
    #[inline]
    pub fn from_angle(angle: f64) -> Self {
        let (im, re) = angle.sin_cos();
        Self { re, im }
    }

    #[inline]
    pub fn conj(self) -> Self {
        Self { re: self.re, im: -self.im }
    }

    #[inline]
    pub fn norm_squared(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline]
    pub fn abs(self) -> f64 {
        self.norm_squared().sqrt()
    }

    #[inline]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;
    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex::new(self.re * other.re - self.im * other.im, self.re * other.im + self.im * other.re)
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Complex;
    #[inline]
    fn mul(self, scale: f64) -> Complex {
        Complex::new(self.re * scale, self.im * scale)
    }
}

impl std::ops::AddAssign for Complex {
    #[inline]
    fn add_assign(&mut self, other: Complex) {
        *self = *self + other;
    }
}
