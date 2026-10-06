//! Orientation and position fields.
//!
//! Both are solved by Gauss-Seidel sweeps of the *extrinsic* smoothness
//! energies of Jakob et al., "Instant Field-Aligned Meshes" (2015): each
//! vertex is repeatedly replaced by the average of its neighbours, after
//! picking for every neighbour the symmetric copy (one of 4 rotations, or one
//! of the lattice translations) that agrees best in 3D.

pub mod constraints;
pub mod orientation;
pub mod position;
pub mod rosy;

pub use constraints::Constraints;
pub use orientation::Singularity;
