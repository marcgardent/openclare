//! The integer-grid chain end to end: the map takes whole numbers where it
//! must, and its grid lines come out as a quad mesh.

use openclare::crossfield::{self, CrossField};
use openclare::extract::grid::graph_from_parametrization;
use openclare::geometry::vertex_normals;
use openclare::mesh::{Topology, primitives};
use openclare::metrics::measure;
use openclare::parametrization::{Kind, Parametrization, Request, create};
use openclare::preprocess::{self, WorkMesh};
use openclare::{Method, RemeshConfig, Target, TriMesh};

fn work_mesh(source: &TriMesh, triangles: usize, crease_angle: Option<f64>) -> WorkMesh {
    let request = preprocess::Request { triangles, crease_angle, ..Default::default() };
    preprocess::create(preprocess::Kind::Isotropic).preprocess(source, &request).unwrap()
}

fn field(work: &WorkMesh, alignment: f64) -> CrossField {
    let request = crossfield::Request { alignment, ..Default::default() };
    crossfield::create(crossfield::Kind::Power).solve(work, &request).unwrap()
}

fn integer_map(work: &WorkMesh, field: &CrossField, edge_length: f64) -> Parametrization {
    let request = Request { edge_length, stiffening_rounds: 40 };
    create(Kind::IntegerGrid).parametrize(work, field, &request).unwrap()
}

fn off_grid(value: f64) -> f64 {
    (value - value.round()).abs()
}

fn config(target: usize) -> RemeshConfig {
    RemeshConfig { method: Method::IntegerGrid, ..RemeshConfig::for_organic(Target::QuadCount(target)) }
}

#[test]
fn singularities_land_on_grid_points() {
    let work = work_mesh(&primitives::icosphere(4), 8000, None);
    let cross = field(&work, 0.0);
    let map = integer_map(&work, &cross, 0.15);
    assert_eq!(map.report.flipped, 0, "{:?}", map.report);
    assert!(map.report.integers > 0);

    let mut singular_corners = 0;
    for (corner, &vertex) in work.mesh.triangles.iter().flatten().enumerate() {
        if cross.indices[vertex as usize] != 0 {
            singular_corners += 1;
            let [u, v] = map.uv[corner];
            assert!(off_grid(u) < 1e-6 && off_grid(v) < 1e-6, "singular vertex {vertex} at ({u}, {v})");
        }
    }
    assert!(singular_corners > 0);
}

#[test]
fn crossing_a_seam_shifts_by_whole_numbers() {
    let work = work_mesh(&primitives::torus(1.0, 0.4, 96, 48), 8000, None);
    let cross = field(&work, 0.05);
    let map = integer_map(&work, &cross, 0.1);
    assert_eq!(map.report.flipped, 0);

    let topology = Topology::build(&work.mesh);
    let mut seams = 0;
    for h in 0..map.cut.seam.len() {
        if map.cut.seam[h] {
            seams += 1;
            let t = topology.opposite[h] as usize;
            let (here, there) = (map.uv[h], map.uv[3 * (t / 3) + (t % 3 + 1) % 3]);
            assert_eq!(map.cut.transition[h], 0);
            assert!(off_grid(there[0] - here[0]) < 1e-6 && off_grid(there[1] - here[1]) < 1e-6);
        }
    }
    assert!(seams > 0);
}

#[test]
fn an_open_border_lies_on_grid_lines() {
    let work = work_mesh(&primitives::plane(6), 3000, None);
    let map = integer_map(&work, &field(&work, 0.0), 0.1);
    assert!(!map.report.features_off_grid);
    for corner in 0..map.uv.len() {
        let p = work.mesh.positions[work.mesh.triangles[corner / 3][corner % 3] as usize];
        let on_border = p.x.abs() == 1.0 || p.y.abs() == 1.0;
        if on_border {
            let [u, v] = map.uv[corner];
            assert!(off_grid(u).min(off_grid(v)) < 1e-6, "border corner at ({u}, {v})");
        }
    }
    // 2 units wide at 0.1 per quad: the border lines are 20 apart.
    for axis in 0..2 {
        let (low, high) =
            map.uv.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| (lo.min(p[axis]), hi.max(p[axis])));
        assert!((high - low - 20.0).abs() < 1e-6, "extent {}", high - low);
    }
}

#[test]
fn the_grid_lines_of_a_plain_map_are_read_as_a_grid() {
    // No solver involved: (u, v) is just (x, y) in units of 0.25.
    let mesh = primitives::plane(40);
    let normals = vertex_normals(&mesh);
    let uv: Vec<[f64; 2]> = mesh
        .triangles
        .iter()
        .flatten()
        .map(|&v| {
            let p = mesh.positions[v as usize];
            [p.x / 0.25, p.y / 0.25]
        })
        .collect();
    let graph = graph_from_parametrization(&mesh, &normals, &uv);

    // 2 units wide: grid points from -4 to 4 along each axis.
    let live: Vec<usize> = (0..graph.positions.len()).filter(|&c| graph.members[c] > 0).collect();
    assert_eq!(live.len(), 81);
    let (mut corners, mut sides, mut inner) = (0, 0, 0);
    for &c in &live {
        let p = graph.positions[c];
        assert!(off_grid(p.x / 0.25) < 1e-9 && off_grid(p.y / 0.25) < 1e-9, "vertex off its grid point: {p}");
        match graph.adjacency[c].len() {
            2 => corners += 1,
            3 => sides += 1,
            4 => inner += 1,
            other => panic!("a grid vertex with {other} neighbours"),
        }
    }
    assert_eq!((corners, sides, inner), (4, 28, 49));
}

#[test]
fn a_torus_comes_out_as_one_regular_grid() {
    let source = primitives::torus(1.0, 0.4, 96, 48);
    let (mesh, report) = config(2000).remesh(&source).unwrap();
    let metrics = measure(&source, &mesh, Some(2000));
    assert!(metrics.is_valid(), "{metrics:?}");
    assert_eq!(metrics.irregular_vertices, 0);
    assert_eq!(metrics.base_complex_patches, Some(1));
    assert!(metrics.alignment.unwrap() < 2.0);
    assert!(metrics.budget_error.unwrap() < 0.2, "{} faces", metrics.faces);
    assert_eq!(report.map_inversions, 0);
}

#[test]
fn a_sphere_comes_out_closed_and_all_quads() {
    let source = primitives::icosphere(4);
    let (mesh, _) = config(1500).remesh(&source).unwrap();
    let metrics = measure(&source, &mesh, Some(1500));
    assert!(metrics.is_valid(), "{metrics:?}");
    // Close to the eight irregular vertices a sphere cannot do without.
    assert!((0..=16).contains(&metrics.irregular_excess().unwrap()), "{metrics:?}");
    assert!(metrics.budget_error.unwrap() < 0.2, "{} faces", metrics.faces);
    assert!(metrics.hausdorff < 0.01);
}

#[test]
fn a_flat_sheet_comes_out_as_a_grid_up_to_its_border() {
    let source = primitives::plane(24);
    let (mesh, _) = config(400).remesh(&source).unwrap();
    let stats = mesh.stats();
    assert_eq!(stats.quads, stats.faces);
    assert_eq!(stats.faces, 400);
    assert_eq!(stats.irregular_vertices, 0);
    let (min, max) = mesh
        .positions
        .iter()
        .fold((openclare::math::Vec3::INFINITY, openclare::math::Vec3::NEG_INFINITY), |(lo, hi), &p| {
            (lo.min(p), hi.max(p))
        });
    assert!(min.distance(openclare::math::Vec3::new(-1.0, -1.0, 0.0)) < 1e-6, "{min}");
    assert!(max.distance(openclare::math::Vec3::new(1.0, 1.0, 0.0)) < 1e-6, "{max}");
}

#[test]
fn every_preset_gives_a_mesh_and_the_chain_is_deterministic() {
    let sphere = primitives::icosphere(4);
    for preset in openclare::config::PRESETS {
        let settings = preset.config(Target::QuadCount(800));
        let (mesh, _) = settings.remesh(&sphere).unwrap();
        assert!(mesh.faces.len() > 400, "preset {}", preset.name);
    }
    let cube = primitives::cube(8);
    let hard = RemeshConfig { method: Method::IntegerGrid, ..RemeshConfig::for_hard_surface(Target::QuadCount(600)) };
    let (mesh, _) = hard.remesh(&cube).unwrap();
    assert!(measure(&cube, &mesh, None).is_valid());

    let settings = config(800);
    assert_eq!(settings.method, Method::IntegerGrid);
    let (first, second) = (settings.remesh(&sphere).unwrap().0, settings.remesh(&sphere).unwrap().0);
    assert_eq!(first.faces, second.faces);
    assert_eq!(first.positions, second.positions);

    assert!(RemeshConfig { alignment: -1.0, ..settings.clone() }.remesh(&sphere).is_err());
    assert!(RemeshConfig { target: Target::QuadCount(0), ..settings }.remesh(&sphere).is_err());
}
