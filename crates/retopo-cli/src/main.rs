use std::process::ExitCode;

use retopo::mesh::{obj, primitives};
use retopo::metrics::measure;
use retopo::remesher::{self, Method, Request};
use retopo::{Target, TriMesh};

const USAGE: &str = "\
retopo - field-aligned automatic quad retopology

USAGE:
    retopo remesh <input.obj> <output.obj> [options]
    retopo measure <source.obj> <quads.obj> [--target <N>] [--json]
    retopo primitive <torus|sphere|cube|plane> <output.obj>

OPTIONS (remesh):
    --method <NAME>        remeshing chain (default instant-meshes)
    --quads <N>            target number of quads (default 2000)
    --edge-length <L>      target edge length, overrides --quads
    --crease-angle <DEG>   follow input edges sharper than this dihedral angle
    --no-boundary-align    do not align the output with open boundaries
    --seed <N>             random seed (default 1)

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
