//! The orientation field of Instant Meshes (local extrinsic smoothing on a
//! multi-resolution hierarchy), solved per vertex and read off per face.
//!
//! It is not the field the target chain uses; it is here so that the
//! power-field solver has something to be compared with.

use super::{CrossField, CrossFieldSolver, Kind, Request, finish, prepare};
use crate::error::Result;
use crate::field::rosy::compat_orientation;
use crate::field::{Constraints, orientation};
use crate::hierarchy::{Graph, Hierarchy, Level};
use crate::math::{Rng, Vec3};
use crate::preprocess::WorkMesh;

#[derive(Clone, Debug)]
pub struct Extrinsic {
    /// Gauss-Seidel sweeps per hierarchy level.
    pub iterations: usize,
}

impl Default for Extrinsic {
    fn default() -> Self {
        Self { iterations: 6 }
    }
}

impl CrossFieldSolver for Extrinsic {
    fn kind(&self) -> Kind {
        Kind::Extrinsic
    }

    fn solve(&self, work: &WorkMesh, request: &Request) -> Result<CrossField> {
        let (topology, frames) = prepare(&work.mesh)?;
        let mesh = &work.mesh;
        let constraints = Constraints::from_feature_edges(&mesh.positions, &work.normals, &work.feature_edges);
        let finest = Level::new(
            mesh.positions.clone(),
            work.normals.clone(),
            work.areas.clone(),
            Graph::uniform(&topology.neighbors),
            constraints,
        );
        let mut hierarchy = Hierarchy::build(finest);
        orientation::solve(&mut hierarchy, self.iterations, &mut Rng::new(request.seed));
        let level = hierarchy.finest();

        // On each face, bring the crosses of its corners to matching arms,
        // average them and lay the result in the plane of the face.
        let angles = mesh
            .triangles
            .iter()
            .enumerate()
            .map(|(f, tri)| {
                let first = tri[0] as usize;
                let mut sum = level.orientation[first];
                for &v in &tri[1..] {
                    let v = v as usize;
                    let (_, matched) = compat_orientation(
                        level.orientation[first],
                        level.normals[first],
                        level.orientation[v],
                        level.normals[v],
                    );
                    sum += matched;
                }
                if sum == Vec3::ZERO || frames.area[f] == 0.0 { 0.0 } else { frames.angle_of(f, sum) }
            })
            .collect();
        Ok(finish(mesh, &topology, &frames, angles))
    }
}
