//! Field-aligned automatic quad retopology.
//!
//! The pipeline follows the "field-aligned" family of remeshers (Instant
//! Meshes, QuadriFlow, QuadWild): a smooth 4-RoSy *orientation field* decides
//! where edges point, a *position field* decides where they sit, and the quad
//! mesh is read off the integer lattice that the two fields describe.
//!
//! ```text
//! TriMesh ─► clean / subdivide ─► Hierarchy ─► orientation ─► position ─► extract ─► relax
//!            (mesh)               (hierarchy)   (field)        (field)     (extract)   (relax)
//! ```
//!
//! [`remesh`] runs the whole thing; every stage is also public so it can be
//! swapped or driven on its own.

// Index loops over parallel per-vertex arrays are the clearest form here.
#![allow(clippy::needless_range_loop)]

pub mod bake;
pub mod config;
pub mod creases;
pub mod crossfield;
pub mod curvature;
pub mod error;
pub mod extract;
pub mod field;
pub mod fitting;
pub mod geometry;
pub mod hierarchy;
pub mod linalg;
pub mod math;
pub mod mesh;
pub mod meshing;
pub mod metrics;
pub mod parametrization;
pub mod pipeline;
pub mod preprocess;
pub mod quantization;
pub mod recorder;
pub mod relax;
pub mod remesher;
pub mod singularity;
pub mod sizing;
pub mod spatial;
pub mod thickness;
pub mod tmesh;
pub mod uv;

pub use config::RemeshConfig;
pub use error::{Error, Result};
pub use mesh::{PolyMesh, TriMesh};
pub use pipeline::{Params, Report, Target, remesh};
pub use preprocess::GuideCurve;
pub use recorder::{Recorder, Silent};
pub use remesher::{Analysis, Guides, Method, Remesher};
