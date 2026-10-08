//! Sparse linear algebra: a thin layer over `faer`, so that the rest of the
//! crate deals in plain slices and never names the library.

use faer::Side;
use faer::linalg::solvers::Solve;
use faer::sparse::linalg::solvers::Llt;
use faer::sparse::{SparseColMat, Triplet};

use crate::error::{Error, Result};

/// A symmetric positive definite matrix being assembled entry by entry.
/// Entries added to the same place accumulate.
pub struct SymmetricBuilder {
    size: usize,
    entries: Vec<Triplet<usize, usize, f64>>,
}

impl SymmetricBuilder {
    pub fn new(size: usize) -> Self {
        Self { size, entries: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds `value` at `(row, col)`. Both triangles must be filled: adding
    /// an off-diagonal entry does not add its mirror image.
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: f64) {
        if value != 0.0 {
            self.entries.push(Triplet::new(row, col, value));
        }
    }

    /// Adds `weight * row^T row` for a sparse row given as `(column, value)`
    /// pairs: the contribution of one squared linear term.
    pub fn add_squared(&mut self, row: &[(usize, f64)], weight: f64) {
        for &(i, a) in row {
            for &(j, b) in row {
                self.add(i, j, weight * a * b);
            }
        }
    }

    /// Cholesky factorization, ready to solve against any number of
    /// right-hand sides.
    pub fn factorize(&self) -> Result<Factorization> {
        let matrix = SparseColMat::<usize, f64>::try_new_from_triplets(self.size, self.size, &self.entries)
            .map_err(|e| Error::Numerical(format!("cannot assemble the sparse system: {e:?}")))?;
        let llt = matrix
            .sp_cholesky(Side::Lower)
            .map_err(|e| Error::Numerical(format!("the system is not positive definite: {e:?}")))?;
        Ok(Factorization { llt, size: self.size })
    }
}

pub struct Factorization {
    llt: Llt<usize, f64>,
    size: usize,
}

impl Factorization {
    pub fn solve(&self, rhs: &[f64]) -> Vec<f64> {
        assert_eq!(rhs.len(), self.size);
        let mut column = faer::Mat::<f64>::from_fn(self.size, 1, |i, _| rhs[i]);
        self.llt.solve_in_place(&mut column);
        (0..self.size).map(|i| column[(i, 0)]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_a_small_laplacian() {
        // 1D chain with both ends tied down: -x[i-1] + 2 x[i] - x[i+1] = 1.
        let n = 50;
        let mut builder = SymmetricBuilder::new(n);
        for i in 0..n {
            builder.add(i, i, 1.0);
            builder.add(i, i, 1.0);
            if i + 1 < n {
                builder.add(i, i + 1, -1.0);
                builder.add(i + 1, i, -1.0);
            }
        }
        let x = builder.factorize().unwrap().solve(&vec![1.0; n]);
        // The exact solution is the parabola i (n + 1 - i) / 2, counting from 1.
        for (i, value) in x.iter().enumerate() {
            let expected = (i + 1) as f64 * (n - i) as f64 / 2.0;
            assert!((value - expected).abs() < 1e-9, "x[{i}] = {value}, expected {expected}");
        }
    }

    #[test]
    fn squared_rows_assemble_a_least_squares_system() {
        // Minimise (x0 - 1)^2 + (x1 - 2)^2 + 100 (x0 - x1)^2.
        let mut builder = SymmetricBuilder::new(2);
        builder.add_squared(&[(0, 1.0)], 1.0);
        builder.add_squared(&[(1, 1.0)], 1.0);
        builder.add_squared(&[(0, 1.0), (1, -1.0)], 100.0);
        let x = builder.factorize().unwrap().solve(&[1.0, 2.0]);
        assert!((x[0] + x[1] - 3.0).abs() < 1e-12);
        assert!((x[1] - x[0] - 1.0 / 201.0).abs() < 1e-12);
    }

    #[test]
    fn an_indefinite_system_is_reported() {
        let mut builder = SymmetricBuilder::new(2);
        builder.add(0, 0, 1.0);
        builder.add(1, 1, -1.0);
        assert!(builder.factorize().is_err());
    }
}
