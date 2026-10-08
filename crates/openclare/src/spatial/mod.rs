//! Spatial queries against the source surface.

pub mod bvh;

pub use bvh::{Bvh, Hit, closest_point_on_triangle};
