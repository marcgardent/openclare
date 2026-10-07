use std::process::ExitCode;

use retopo::crossfield;
use retopo::math::{Vec3, tangent_frame};
use retopo::mesh::{Topology, obj, primitives};
use retopo::metrics::measure;
use retopo::preprocess;
use retopo::remesher::{self, Method, Request};
use retopo::singularity;
use retopo::{PolyMesh, Target, TriMesh};

const USAGE: &str = "\
retopo - field-aligned automatic quad retopology

USAGE:
    retopo remesh <input.obj> <output.obj> [options]
    retopo measure <source.obj> <quads.obj> [--target <N>] [--json]
    retopo preprocess <input.obj> <output.obj> [options]
    retopo field <input.obj> [options]
    retopo primitive <torus|sphere|cube|plane> <output.obj>

OPTIONS (remesh):
    --method <NAME>        remeshing chain (default instant-meshes)
    --quads <N>            target number of quads (default 2000)
    --edge-length <L>      target edge length, overrides --quads
    --crease-angle <DEG>   follow input edges sharper than this dihedral angle
    --no-boundary-align    do not align the output with open boundaries
    --seed <N>             random seed (default 1)

OPTIONS (preprocess):
    --kind <NAME>          isotropic (default) or passthrough
    --triangles <N>        size of the work mesh (default 100000)
    --crease-angle <DEG>   keep input edges sharper than this dihedral angle

OPTIONS (field):
    --kind <NAME>          power (default) or extrinsic
    --triangles <N>        size of the work mesh (default 50000)
    --alignment <A>        pull towards curvature directions (default 0.05, 0 to ignore)
    --quads <N>            read curvature at the scale of a mesh of N quads
    --scale <L>            length over which curvature is read (default: work mesh edge)
    --crease-angle <DEG>   treat input edges sharper than this as hard constraints
    --simplify <R>         cancel opposite singularities closer than R quad edges (needs --quads)
    --tolerance <T>        what a cancellation may cost (default 1)
    --crosses <out.obj>    write the field as small crosses, singularities as markers

OPTIONS (measure):
    --target <N>           face count that was asked for
    --json                 print the metrics as one JSON object
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some("remesh") => remesh(&args[1..]),
        Some("measure") => measure_command(&args[1..]),
        Some("preprocess") => preprocess_command(&args[1..]),
        Some("field") => field_command(&args[1..]),
        Some("primitive") => primitive(&args[1..]),
        Some("-h" | "--help") => {
            print!("{USAGE}");
            Ok(())
        }
        _ => Err("expected a subcommand".into()),
    }
}

fn primitive(args: &[String]) -> Result<(), String> {
    let [shape, output] = args else {
        return Err("primitive takes a shape and an output path".into());
    };
    let mesh: TriMesh = match shape.as_str() {
        "torus" => primitives::torus(1.0, 0.4, 96, 48),
        "sphere" => primitives::icosphere(4),
        "cube" => primitives::cube(16),
        "plane" => primitives::plane(24),
        other => return Err(format!("unknown primitive '{other}'")),
    };
    obj::write_obj_tri(output, &mesh).map_err(|e| e.to_string())
}

fn parse<T: std::str::FromStr>(name: &str, text: &str) -> Result<T, String> {
    text.parse().map_err(|_| format!("invalid value '{text}' for {name}"))
}

fn remesh(args: &[String]) -> Result<(), String> {
    let mut request = Request::default();
    let mut method = Method::InstantMeshes;
    let mut paths = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} expects a value"));
        match arg.as_str() {
            "--method" => method = value(arg)?.parse().map_err(|e: retopo::Error| e.to_string())?,
            "--quads" => request.target = Target::QuadCount(parse(arg, value(arg)?)?),
            "--edge-length" => request.target = Target::EdgeLength(parse(arg, value(arg)?)?),
            "--crease-angle" => request.crease_angle = Some(parse(arg, value(arg)?)?),
            "--no-boundary-align" => request.align_boundaries = false,
            "--seed" => request.seed = parse(arg, value(arg)?)?,
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => paths.push(path),
        }
    }
    let [input, output] = paths[..] else {
        return Err("remesh takes an input and an output path".into());
    };

    let mesh = obj::read_obj(input).map_err(|e| format!("{input}: {e}"))?;
    println!("input    {} vertices, {} triangles", mesh.positions.len(), mesh.triangles.len());

    let (result, report) = remesher::create(method).remesh(&mesh, &request).map_err(|e| e.to_string())?;
    obj::write_obj_poly(output, &result).map_err(|e| format!("{output}: {e}"))?;

    let stats = &report.stats;
    println!(
        "fields   {} samples, {} levels, {} feature edges, singularities +{} / -{}",
        report.working_vertices,
        report.hierarchy_levels,
        report.feature_edges,
        report.singularities.0,
        report.singularities.1,
    );
    println!(
        "output   {} vertices, {} faces ({} quads, {} triangles, {} n-gons), edge length {:.4}",
        stats.vertices, stats.faces, stats.quads, stats.triangles, stats.ngons, report.edge_length,
    );
    println!(
        "quality  {} irregular vertices, {} boundary edges, {} non-manifold edges, {} open loops",
        stats.irregular_vertices, stats.boundary_edges, stats.non_manifold_edges, report.open_loops,
    );
    let valences: Vec<String> = stats.valences.iter().map(|(v, n)| format!("{v}:{n}")).collect();
    println!("valence  {}", valences.join("  "));
    let timings: Vec<String> =
        report.timings.iter().map(|(stage, t)| format!("{stage} {:.0} ms", t.as_secs_f64() * 1e3)).collect();
    println!("timing   {}", timings.join(", "));
    let total: f64 = report.timings.iter().map(|(_, t)| t.as_secs_f64()).sum();
    println!("seconds  {total:.3}");
    Ok(())
}

fn measure_command(args: &[String]) -> Result<(), String> {
    let (mut target, mut json, mut paths) = (None, false, Vec::new());
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--target" => target = Some(parse(arg, args.next().ok_or("--target expects a value")?)?),
            "--json" => json = true,
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => paths.push(path),
        }
    }
    let [source, quads] = paths[..] else {
        return Err("measure takes the source mesh and the mesh to measure".into());
    };
    let source = obj::read_obj(source).map_err(|e| format!("{source}: {e}"))?;
    let mesh = obj::read_obj_poly(quads).map_err(|e| format!("{quads}: {e}"))?;
    let m = measure(&source, &mesh, target);
    if json {
        println!("{}", m.to_json());
        return Ok(());
    }

    let optional = |value: Option<String>| value.unwrap_or_else(|| "n/a".into());
    println!("validity     {}", if m.is_valid() { "ok" } else { "FAILED" });
    println!(
        "             {} faces ({} quads), {} non-manifold edges, {} misoriented, {} inverted, {} degenerate",
        m.faces, m.quads, m.non_manifold_edges, m.misoriented_edges, m.inverted_faces, m.degenerate_faces,
    );
    println!(
        "             {} boundary edges (source {}), {} components (source {})",
        m.boundary_edges, m.source_boundary_edges, m.components, m.source_components,
    );
    println!(
        "irregular    {} vertices, topological minimum {}, excess {}",
        m.irregular_vertices,
        optional(m.topological_minimum.map(|n| n.to_string())),
        optional(m.irregular_excess().map(|n| n.to_string())),
    );
    println!("structure    {} base complex patches", optional(m.base_complex_patches.map(|n| n.to_string())));
    println!("alignment    {}", optional(m.alignment.map(|a| format!("{a:.2} deg (0 perfect, 22.5 random)"))));
    println!(
        "quad shape   angle deviation {:.2} deg mean, {:.2} p95; aspect {:.3} mean, {:.3} p95; area variation {:.3}",
        m.angle_deviation_mean, m.angle_deviation_p95, m.aspect_mean, m.aspect_p95, m.area_variation,
    );
    println!("fidelity     Hausdorff {:.5}, mean {:.6} (of the bounding diagonal)", m.hausdorff, m.mean_distance);
    println!("budget       {}", optional(m.budget_error.map(|e| format!("{:.1} % off target", 100.0 * e))));
    Ok(())
}

fn preprocess_command(args: &[String]) -> Result<(), String> {
    let mut request = preprocess::Request::default();
    let mut kind = preprocess::Kind::Isotropic;
    let mut paths = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} expects a value"));
        match arg.as_str() {
            "--kind" => kind = value(arg)?.parse().map_err(|e: retopo::Error| e.to_string())?,
            "--triangles" => request.triangles = parse(arg, value(arg)?)?,
            "--crease-angle" => request.crease_angle = Some(parse(arg, value(arg)?)?),
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => paths.push(path),
        }
    }
    let [input, output] = paths[..] else {
        return Err("preprocess takes an input and an output path".into());
    };

    let source = obj::read_obj(input).map_err(|e| format!("{input}: {e}"))?;
    let before = preprocess::quality(&source);
    let start = std::time::Instant::now();
    let work = preprocess::create(kind).preprocess(&source, &request).map_err(|e| e.to_string())?;
    let seconds = start.elapsed().as_secs_f64();
    obj::write_obj_tri(output, &work.mesh).map_err(|e| format!("{output}: {e}"))?;

    let after = preprocess::quality(&work.mesh);
    let as_polygons = PolyMesh {
        positions: work.mesh.positions.clone(),
        faces: work.mesh.triangles.iter().map(|t| t.to_vec()).collect(),
    };
    let m = measure(&source, &as_polygons, None);
    println!("triangles    {} -> {}", source.triangles.len(), work.mesh.triangles.len());
    println!(
        "min angle    mean {:.1} -> {:.1} deg, 1st percentile {:.1} -> {:.1} deg",
        before.min_angle_mean, after.min_angle_mean, before.min_angle_p01, after.min_angle_p01,
    );
    println!("edge ratio   {:.1} -> {:.1}", before.edge_ratio, after.edge_ratio);
    println!(
        "topology     {} boundary edges (source {}), {} components (source {}), {} non-manifold edges",
        m.boundary_edges, m.source_boundary_edges, m.components, m.source_components, m.non_manifold_edges,
    );
    println!("fidelity     Hausdorff {:.5}, mean {:.6} (of the bounding diagonal)", m.hausdorff, m.mean_distance);
    let confident = work.confidence.iter().filter(|&&c| c > 0.4).count();
    println!(
        "features     {} feature edges, curvature directions trusted on {:.0} % of the vertices",
        work.feature_edges.len(),
        100.0 * confident as f64 / work.confidence.len().max(1) as f64,
    );
    println!("seconds      {seconds:.3}");
    Ok(())
}

fn field_command(args: &[String]) -> Result<(), String> {
    let mut work_request = preprocess::Request { triangles: 50_000, ..Default::default() };
    let mut request = crossfield::Request::default();
    let mut kind = crossfield::Kind::Power;
    let (mut crosses, mut quads, mut paths) = (None, None::<usize>, Vec::new());
    let (mut simplify, mut tolerance) = (None::<f64>, singularity::Request::default().tolerance);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} expects a value"));
        match arg.as_str() {
            "--kind" => kind = value(arg)?.parse().map_err(|e: retopo::Error| e.to_string())?,
            "--triangles" => work_request.triangles = parse(arg, value(arg)?)?,
            "--alignment" => request.alignment = parse(arg, value(arg)?)?,
            "--scale" => work_request.curvature_scale = Some(parse(arg, value(arg)?)?),
            "--quads" => quads = Some(parse(arg, value(arg)?)?),
            "--crease-angle" => work_request.crease_angle = Some(parse(arg, value(arg)?)?),
            "--crosses" => crosses = Some(value(arg)?.clone()),
            "--simplify" => simplify = Some(parse(arg, value(arg)?)?),
            "--tolerance" => tolerance = parse(arg, value(arg)?)?,
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => paths.push(path),
        }
    }
    let [input] = paths[..] else {
        return Err("field takes one input path".into());
    };

    let source = obj::read_obj(input).map_err(|e| format!("{input}: {e}"))?;
    if let Some(quads) = quads.filter(|&n| n > 0) {
        work_request.curvature_scale = Some((source.surface_area() / quads as f64).sqrt());
    }
    let work = preprocess::create(preprocess::Kind::Isotropic)
        .preprocess(&source, &work_request)
        .map_err(|e| e.to_string())?;
    let start = std::time::Instant::now();
    let mut field = crossfield::create(kind).solve(&work, &request).map_err(|e| e.to_string())?;
    let seconds = start.elapsed().as_secs_f64();

    let mut simplification = None;
    if let Some(quad_edges) = simplify {
        let quad_edge = work_request.curvature_scale.ok_or("--simplify needs --quads or --scale")?;
        let optimizer = singularity::create(singularity::Kind::PairCancellation);
        let options =
            singularity::Request { alignment: request.alignment, tolerance, reach: Some(quad_edges * quad_edge) };
        let start = std::time::Instant::now();
        let (simplified, report) = optimizer.optimize(&work, &field, &options).map_err(|e| e.to_string())?;
        let frames = crossfield::FaceFrames::build(&work.mesh, &Topology::build(&work.mesh));
        let before = crossfield::alignment_error(&work, &frames, &field);
        simplification = Some((report, before, start.elapsed().as_secs_f64()));
        field = simplified;
    }

    let frames = crossfield::FaceFrames::build(&work.mesh, &Topology::build(&work.mesh));
    let (positive, negative, other) = field.singularities();
    println!("work mesh      {} triangles, {} feature edges", work.mesh.triangles.len(), work.feature_edges.len());
    println!("singularities  {} (+1/4: {positive}, -1/4: {negative}, other: {other})", positive + negative + other);
    println!("total index    {} quarter turns", field.total_index());
    match crossfield::alignment_error(&work, &frames, &field) {
        Some(error) => println!("alignment      {error:.2} deg (0 perfect, 22.5 random)"),
        None => println!("alignment      n/a"),
    }
    println!("seconds        {seconds:.3}");
    if let Some((report, alignment_before, seconds)) = simplification {
        println!(
            "simplified     {} -> {} singularities ({} pairs cancelled of {} tried, {} split), alignment was {}, {seconds:.3} s",
            report.singularities_before,
            report.singularities_after,
            report.cancelled,
            report.tried,
            report.split,
            alignment_before.map_or("n/a".into(), |a| format!("{a:.2} deg")),
        );
    }

    if let Some(path) = crosses {
        obj::write_obj_poly(&path, &field_as_crosses(&work, &frames, &field)).map_err(|e| format!("{path}: {e}"))?;
    }
    Ok(())
}

/// The field drawn as geometry, for a viewer: a small cross on a sample of
/// the faces, a triangle on every +1/4 singularity and a pentagon on every
/// other one.
fn field_as_crosses(
    work: &preprocess::WorkMesh,
    frames: &crossfield::FaceFrames,
    field: &crossfield::CrossField,
) -> PolyMesh {
    let mesh = &work.mesh;
    let step = (mesh.triangles.len() / 6000).max(1);
    let arm = 0.45 * (step as f64 * mesh.surface_area() / mesh.triangles.len() as f64).sqrt();
    let mut out = PolyMesh::default();
    let mut polygon = |corners: &[Vec3]| {
        let first = out.positions.len() as u32;
        out.positions.extend_from_slice(corners);
        out.faces.push((first..first + corners.len() as u32).collect());
    };

    for f in (0..mesh.triangles.len()).step_by(step) {
        let [a, b, c] = mesh.corners(f);
        let center = (a + b + c) / 3.0 + frames.normal[f] * (0.02 * arm);
        let along = field.direction(frames, f) * arm;
        let across = frames.normal[f].cross(along);
        // Both arms wound the same way, facing out of the surface.
        for (long, thin) in [(along, across * 0.06), (across, along * -0.06)] {
            polygon(&[center - long - thin, center + long - thin, center + long + thin, center - long + thin]);
        }
    }
    for (v, &index) in field.indices.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let sides = if index == 1 { 3 } else { 5 };
        let (s, t) = tangent_frame(work.normals[v]);
        let center = mesh.positions[v] + work.normals[v] * (0.05 * arm);
        let corners: Vec<Vec3> = (0..sides)
            .map(|k| {
                let angle = std::f64::consts::TAU * k as f64 / sides as f64;
                center + (s * angle.cos() + t * angle.sin()) * (1.6 * arm)
            })
            .collect();
        polygon(&corners);
    }
    out
}
