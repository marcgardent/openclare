//! Turning the two fields into a mesh.
//!
//! [`LatticeGraph::from_fields`] collapses the input vertices onto the
//! lattice points they snapped to and keeps the lattice edges between them;
//! [`extract_faces`] then reads the faces off that embedded graph.

pub mod faces;
pub mod graph;
pub mod grid;

pub use faces::{FaceExtraction, extract_faces};
pub use graph::LatticeGraph;
