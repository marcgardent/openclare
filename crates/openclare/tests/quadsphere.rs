//! The academic quad sphere: a sphere gets the layout of a cube, eight
//! poles of valence three and six patches, every loop closed.

use openclare::mesh::primitives;
use openclare::metrics::measure;
use openclare::{RemeshConfig, Target};

#[test]
fn a_sphere_gets_the_layout_of_a_cube() {
    // An icosphere: its facets have a pattern of their own, twenty and
    // twelve, which a field pulled towards curvature directions copies.
    let sphere = primitives::icosphere(4);
    for quads in [1000, 3000] {
        let (mesh, _) = RemeshConfig::for_organic(Target::QuadCount(quads)).remesh(&sphere).unwrap();
        let metrics = measure(&sphere, &mesh, Some(quads));
        assert!(metrics.is_valid(), "{metrics:?}");
        assert_eq!(metrics.irregular_vertices, 8, "{quads}: {metrics:?}");
        assert_eq!(metrics.base_complex_patches, Some(6), "{quads}");
        let loops = metrics.edge_loops.expect("a closed quad mesh");
        assert_eq!(loops.spiral, 0, "{quads}: {loops:?}");
    }
}
