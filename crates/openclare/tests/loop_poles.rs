//! A loop drawn comes out as a true edge loop: no pole on the line of edges
//! that follows it. The poles a closed loop calls for sit inside or outside
//! it, off the line.

use std::collections::HashMap;

use openclare::math::Vec3;
use openclare::mesh::obj::read_obj;
use openclare::metrics::measure;
use openclare::{GuideCurve, Guides, PolyMesh, RemeshConfig, Report, Target, TriMesh};

fn remesh(source: &TriMesh, config: RemeshConfig, curves: &[GuideCurve]) -> (PolyMesh, Report) {
    config.remesh_guided(source, &Guides { curves, ..Default::default() }).unwrap()
}

/// The valence of every vertex of `mesh`: the edges that meet there.
fn valences(mesh: &PolyMesh) -> Vec<usize> {
    let mut edges = std::collections::HashSet::new();
    for face in &mesh.faces {
        for k in 0..face.len() {
            let (a, b) = (face[k], face[(k + 1) % face.len()]);
            edges.insert((a.min(b), a.max(b)));
        }
    }
    let mut valence = vec![0; mesh.positions.len()];
    for (a, b) in edges {
        valence[a as usize] += 1;
        valence[b as usize] += 1;
    }
    valence
}

/// The vertices on the line of edges of curve `curve`, with their valence
/// in the mesh, and how many edges of the line meet at each.
fn on_the_line(mesh: &PolyMesh, report: &Report, curve: u32) -> HashMap<u32, (usize, usize)> {
    let valence = valences(mesh);
    let mut line: HashMap<u32, (usize, usize)> = HashMap::new();
    for &([a, b], c) in &report.curve_edges {
        if c == curve {
            for v in [a, b] {
                line.entry(v).or_insert((valence[v as usize], 0)).1 += 1;
            }
        }
    }
    line
}

/// What is printed with `--nocapture`: the valences along the line, and the
/// poles off it.
fn describe(mesh: &PolyMesh, report: &Report, line: &HashMap<u32, (usize, usize)>) {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for &(valence, _) in line.values() {
        *counts.entry(valence).or_default() += 1;
    }
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort();
    println!("{} quads, {} vertices on the line, valences {counts:?}", mesh.faces.len(), line.len());
    for (&v, &(valence, _)) in line {
        if valence != 4 {
            println!("  pole on the line: vertex {v} of valence {valence} at {:?}", mesh.positions[v as usize]);
        }
    }
    println!("  {} poles in all", report.poles.len());
}

/// A flat disc of radius 1 in the plane z = 0, in rings of triangles about
/// as wide as they are long.
fn disc(rings: u32) -> TriMesh {
    let mut positions = vec![Vec3::new(0.0, 0.0, 0.0)];
    let mut triangles = Vec::new();
    let mut previous: Vec<u32> = vec![0];
    for ring in 1..=rings {
        let radius = f64::from(ring) / f64::from(rings);
        let count = 6 * ring;
        let start = positions.len() as u32;
        for k in 0..count {
            let turn = std::f64::consts::TAU * f64::from(k) / f64::from(count);
            positions.push(Vec3::new(radius * turn.cos(), radius * turn.sin(), 0.0));
        }
        let current: Vec<u32> = (start..start + count).collect();
        // Join the two rings by walking round both at once.
        let (inner, outer) = (previous.len(), current.len());
        let (mut i, mut o) = (0, 0);
        while i < inner || o < outer {
            let inner_next = (i + 1) as f64 / inner as f64;
            let outer_next = (o + 1) as f64 / outer as f64;
            if o < outer && (i >= inner || outer_next <= inner_next) {
                triangles.push([previous[i % inner], current[o], current[(o + 1) % outer]]);
                o += 1;
            } else {
                triangles.push([previous[i % inner], current[o % outer], previous[(i + 1) % inner]]);
                i += 1;
            }
        }
        previous = current;
    }
    // The centre is one vertex: the first ring's triangles share it.
    triangles.retain(|t| !(t[0] == t[2] || t[0] == t[1] || t[1] == t[2]));
    TriMesh { positions, triangles }
}

#[test]
fn a_circle_on_a_disc_has_no_pole_on_it() {
    let source = disc(40);
    source.validate().unwrap();
    let points = (0..96)
        .map(|k| {
            let turn = std::f64::consts::TAU * f64::from(k) / 96.0;
            Vec3::new(0.5 * turn.cos(), 0.5 * turn.sin(), 0.0)
        })
        .collect();
    let curve = GuideCurve { points, closed: true, sharp: false };
    let config = RemeshConfig { adaptivity: 0.0, ..RemeshConfig::for_organic(Target::QuadCount(1200)) };
    let (mesh, report) = remesh(&source, config, &[curve]);
    assert!(measure(&source, &mesh, Some(1200)).is_valid());
    assert!(!report.fell_back, "{:?}", report.fallback_causes);
    let line = on_the_line(&mesh, &report, 0);
    describe(&mesh, &report, &line);
    assert!(line.len() > 30, "{} vertices on the circle", line.len());
    assert!(line.values().all(|&(_, along)| along == 2), "the line is one unbroken loop");
    let poles_on_the_line: Vec<_> = line.iter().filter(|&(_, &(valence, _))| valence != 4).collect();
    assert!(poles_on_the_line.is_empty(), "poles on the circle: {poles_on_the_line:?}");
    // Inside the circle the field turns once: four poles of valence 3,
    // and none in the ring between the circle and the border.
    let inside: Vec<_> = report.poles.iter().filter(|pole| pole.position.length() < 0.5).collect();
    let outside: Vec<_> = report.poles.iter().filter(|pole| pole.position.length() >= 0.5).collect();
    println!("  inside {:?}", inside.iter().map(|p| (p.valence, p.position.length())).collect::<Vec<_>>());
    println!("  outside {:?}", outside.iter().map(|p| (p.valence, p.position.length())).collect::<Vec<_>>());
    assert_eq!(inside.len(), 4, "{inside:?}");
    assert!(inside.iter().all(|pole| pole.valence == 3), "{inside:?}");
    assert!(outside.is_empty(), "{outside:?}");
}

/// Suzanne, Blender's monkey (see CREDITS.md), subdivided twice.
fn suzanne() -> TriMesh {
    read_obj(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/suzanne.obj")).unwrap()
}

/// A closed loop round Suzanne's mouth, a little wider than the lips, laid
/// on the surface as the add-on's Draw Loop lays it.
fn round_the_mouth() -> GuideCurve {
    let text = include_str!("data/suzanne_mouth.txt");
    let points = text
        .lines()
        .map(|line| {
            let xyz: Vec<f64> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
            Vec3::new(xyz[0], xyz[1], xyz[2])
        })
        .collect();
    GuideCurve { points, closed: true, sharp: false }
}

#[test]
fn a_loop_round_suzannes_mouth_has_no_pole_on_it() {
    let source = suzanne();
    // Before #360, two poles of valence 3 sat on the line at the sides of the
    // mouth from 2,000 quads on, and the line broke round them.
    for quads in [1000, 2000, 3000] {
        let config = RemeshConfig::for_organic(Target::QuadCount(quads));
        let (mesh, report) = remesh(&source, config, &[round_the_mouth()]);
        assert!(measure(&source, &mesh, Some(quads)).is_valid(), "{quads}");
        assert!(!report.fell_back, "{quads}: {:?}", report.fallback_causes);
        let line = on_the_line(&mesh, &report, 0);
        describe(&mesh, &report, &line);
        assert!(line.len() >= 14, "{quads}: {} vertices round the mouth", line.len());
        assert!(line.values().all(|&(_, along)| along == 2), "{quads}: the line is one unbroken loop");
        let poles_on_the_line: Vec<_> = line.iter().filter(|&(_, &(valence, _))| valence != 4).collect();
        assert!(poles_on_the_line.is_empty(), "{quads}: poles round the mouth: {poles_on_the_line:?}");
    }
}
