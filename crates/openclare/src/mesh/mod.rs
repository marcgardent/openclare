//! Mesh containers, connectivity and I/O.

pub mod intersections;
pub mod obj;
pub mod poly_mesh;
pub mod primitives;
pub mod subdivide;
pub mod topology;
pub mod tri_mesh;

pub use poly_mesh::{MeshStats, PolyMesh};
pub use topology::Topology;
pub use tri_mesh::{Mended, TriMesh};
