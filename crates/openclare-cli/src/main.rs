use std::process::ExitCode;

use openclare::config::{PRESETS, RemeshConfig};
use openclare::crossfield;
use openclare::math::{Vec3, tangent_frame};
use openclare::mesh::{Topology, obj, primitives};
use openclare::metrics::measure;
use openclare::parametrization;
use openclare::preprocess;
use openclare::quantization;
use openclare::singularity;
use openclare::tmesh::TMesh;
use openclare::{Guides, PolyMesh, Target, TriMesh};

const USAGE: &str = "\
openclare - field-aligned automatic quad retopology

USAGE:
    openclare remesh <input.obj> <output.obj> [options]
    openclare presets
    openclare measure <source.obj> <quads.obj> [--target <N>] [--json]
    openclare preprocess <input.obj> <output.obj> [options]
    openclare field <input.obj> [options]
    openclare parametrize <input.obj> --quads <N> [options]
    openclare tmesh <input.obj> --quads <N> [options]
    openclare quantize <input.obj> --quads <N> [options]
    openclare primitive <torus|sphere|cube|plane|lblock|cylinder> <output.obj> [detail]

OPTIONS (remesh):
    --preset <NAME>              start from a shipped preset (see `openclare presets`; default organic)
    --config <FILE>              start from a saved configuration instead
    --save-config <FILE>         write the configuration used, to reuse it as a preset
    --quads <N>                  target number of quads (default 2000)
    --edge-length <L>            target edge length, instead of a number of quads
    --method <NAME>              remeshing chain: quad-layout, integer-grid or instant-meshes
    --alignment <A>              pull of the edges towards curvature directions
    --singularity-reach <R>      cancel opposite singularities closer than R quad edges
    --singularity-tolerance <T>  what such a cancellation may cost
    --crease-angle <DEG|none>    follow input edges sharper than this dihedral angle
    --adaptivity <A>             0 for quads of one size, up to 1 for quads as small as the shape bends
    --quads-around <N>           fewest quads round a thin part, so that it keeps its volume (0: do not look at thickness)
    --volume-fit <F>             0 leaves every vertex on the surface; 1 moves them off it for the mesh to keep the volume
    --density <FILE>             painted density: one weight per input vertex, one per line, from 0 (least dense) to 1 (most)
    --density-contrast <F>       how many times denser a weight of 1 is than a weight of 0 (default 4)
    --curves <FILE>              guide curves, lines for the quad mesh to have edges along: one point per line
                                 as x y z; a blank line starts another curve; a line saying closed closes one,
                                 a line saying sharp makes the edges along one hard edges of the result
    --skip-pieces-under <Q>      leave out the separate pieces whose share of the quads is below Q (default 0: keep all)
    --record <FILE.rrd>          record what each stage makes, to open in the Rerun viewer
                                 (needs a build with `--features rerun`)
    --seed <N>                   random seed

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

OPTIONS (parametrize):
    --quads <N>            number of quads the map is scaled for (required)
    --triangles <N>        size of the work mesh (default 50000)
    --alignment <A>        pull of the field towards curvature (default 0.05)
    --simplify <R>         cancel opposite singularities closer than R quad edges
    --crease-angle <DEG>   treat input edges sharper than this as hard constraints
    --rounds <N>           most stiffening rounds against inverted triangles (default 40)
    --kind <NAME>          seamless (default) or integer-grid

OPTIONS (tmesh):
    --quads <N>            number of quads the map is scaled for (required)
    --triangles <N>        size of the work mesh (default 50000)
    --alignment <A>        pull of the field towards curvature (default 0.05)
    --simplify <R>         cancel opposite singularities closer than R quad edges
    --crease-angle <DEG>   treat input edges sharper than this as hard constraints
    --rounds <N>           most stiffening rounds against inverted triangles (default 40)
    --traces <out.obj>     write the cuts as lines on the surface
    --json                 print the report as one JSON object

OPTIONS (quantize): those of tmesh up to --rounds, and
    --tolerance <T>        how close to the number of quads is close enough (default 0.02)
    --keep-scale           keep the scale of the T-mesh instead of aiming for the number of quads
    --json                 print the report as one JSON object

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
        Some("presets") => {
            for preset in PRESETS {
                println!("{:14} {}", preset.name, preset.summary);
            }
            Ok(())
        }
        Some("measure") => measure_command(&args[1..]),
        Some("preprocess") => preprocess_command(&args[1..]),
        Some("field") => field_command(&args[1..]),
        Some("parametrize") => parametrize_command(&args[1..]),
        Some("tmesh") => tmesh_command(&args[1..]),
        Some("quantize") => quantize_command(&args[1..]),
        Some("primitive") => primitive(&args[1..]),
        Some("-h" | "--help") => {
            print!("{USAGE}");
            Ok(())
        }
        _ => Err("expected a subcommand".into()),
    }
}

fn primitive(args: &[String]) -> Result<(), String> {
    let (shape, output, detail) = match args {
        [shape, output] => (shape, output, None),
        [shape, output, detail] => (shape, output, Some(parse::<u32>("detail", detail)?)),
        _ => return Err("primitive takes a shape, an output path and optionally a level of detail".into()),
    };
    let mesh: TriMesh = match shape.as_str() {
        "torus" => primitives::torus(1.0, 0.4, 2 * detail.unwrap_or(48), detail.unwrap_or(48)),
        "sphere" => primitives::icosphere(detail.unwrap_or(4)),
        "cube" => primitives::cube(detail.unwrap_or(16)),
        "plane" => primitives::plane(detail.unwrap_or(24)),
        "lblock" => primitives::l_block(detail.unwrap_or(12)),
        "cylinder" => primitives::cylinder(detail.unwrap_or(16)),
        other => return Err(format!("unknown primitive '{other}'")),
    };
    obj::write_obj_tri(output, &mesh).map_err(|e| e.to_string())
}

/// Guide curves from text: one point per line as `x y z`; a blank line ends
/// a curve; a line `closed` says the curve it is in joins its first point,
/// a line `sharp` that the edges along it are hard edges of the result.
fn read_curves(text: &str) -> Result<Vec<openclare::GuideCurve>, String> {
    let mut curves = vec![openclare::GuideCurve::default()];
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        let current = curves.last_mut().expect("never empty");
        if line.is_empty() {
            if !current.points.is_empty() {
                curves.push(openclare::GuideCurve::default());
            }
        } else if line == "closed" {
            current.closed = true;
        } else if line == "sharp" {
            current.sharp = true;
        } else if !line.starts_with('#') {
            let numbers: Vec<f64> = line
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map_err(|_| format!("line {}: '{line}' is not a point", number + 1))?;
            let [x, y, z] = numbers[..] else {
                return Err(format!("line {}: a point is three numbers", number + 1));
            };
            current.points.push(openclare::math::Vec3::new(x, y, z));
        }
    }
    curves.retain(|curve| curve.points.len() >= 2);
    Ok(curves)
}

/// Runs the chain while recording what its stages make to the file at
/// `path`. The recording is kept even if the chain fails: that is when it
/// is most worth a look.
#[cfg(feature = "rerun")]
fn remesh_recorded(
    path: &str,
    config: &RemeshConfig,
    mesh: &openclare::TriMesh,
    guides: &Guides,
) -> Result<(openclare::PolyMesh, openclare::Report), String> {
    let mut recorder = openclare_rerun::RerunRecorder::create(std::path::Path::new(path))?;
    let done = config.remesh_recorded(mesh, guides, &mut recorder).map_err(|e| e.to_string());
    recorder.finish()?;
    println!("recorded {path}");
    done
}

#[cfg(not(feature = "rerun"))]
fn remesh_recorded(
    _: &str,
    _: &RemeshConfig,
    _: &openclare::TriMesh,
    _: &Guides,
) -> Result<(openclare::PolyMesh, openclare::Report), String> {
    Err("this build cannot record".into())
}

fn parse<T: std::str::FromStr>(name: &str, text: &str) -> Result<T, String> {
    text.parse().map_err(|_| format!("invalid value '{text}' for {name}"))
}

fn remesh(args: &[String]) -> Result<(), String> {
    // A preset or a saved file is the starting point, whatever its place on
    // the command line; the other options then adjust it.
    let named = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1));
    let mut config = match (named("--config"), named("--preset")) {
        (Some(_), Some(_)) => return Err("--config and --preset are two starting points: give one".into()),
        (Some(path), None) => {
            let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
            RemeshConfig::from_text(&text).map_err(|e| format!("{path}: {e}"))?
        }
        (None, Some(name)) => RemeshConfig::preset(name, RemeshConfig::default().target).map_err(|e| e.to_string())?,
        (None, None) => RemeshConfig::default(),
    };

    let (mut save, mut density_path, mut record_path) = (None, None, None::<String>);
    let mut curves_path = None::<String>;
    let mut paths = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} expects a value"));
        match arg.as_str() {
            "--config" | "--preset" => drop(value(arg)?),
            "--save-config" => save = Some(value(arg)?.clone()),
            "--method" => config.method = value(arg)?.parse().map_err(|e: openclare::Error| e.to_string())?,
            "--quads" => config.target = Target::QuadCount(parse(arg, value(arg)?)?),
            "--edge-length" => config.target = Target::EdgeLength(parse(arg, value(arg)?)?),
            "--alignment" => config.alignment = parse(arg, value(arg)?)?,
            "--singularity-reach" => config.singularity_reach = parse(arg, value(arg)?)?,
            "--singularity-tolerance" => config.singularity_tolerance = parse(arg, value(arg)?)?,
            "--crease-angle" => {
                let text = value(arg)?;
                config.crease_angle = if text == "none" { None } else { Some(parse(arg, text)?) };
            }
            "--adaptivity" => config.adaptivity = parse(arg, value(arg)?)?,
            "--quads-around" => config.quads_around = parse(arg, value(arg)?)?,
            "--volume-fit" => config.volume_fit = parse(arg, value(arg)?)?,
            "--density" => density_path = Some(value(arg)?.clone()),
            "--curves" => curves_path = Some(value(arg)?.clone()),
            "--density-contrast" => config.density_contrast = parse(arg, value(arg)?)?,
            "--skip-pieces-under" => config.skip_pieces_under = parse(arg, value(arg)?)?,
            "--record" => record_path = Some(value(arg)?.clone()),
            "--seed" => config.seed = parse(arg, value(arg)?)?,
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => paths.push(path),
        }
    }
    config.validate().map_err(|e| e.to_string())?;
    if record_path.is_some() && !cfg!(feature = "rerun") {
        return Err("this build cannot record: build it with `cargo build --release --features rerun`".into());
    }
    if let Some(path) = save {
        std::fs::write(&path, config.to_text()).map_err(|e| format!("{path}: {e}"))?;
    }
    let [input, output] = paths[..] else {
        return Err("remesh takes an input and an output path".into());
    };

    let mesh = obj::read_obj(input).map_err(|e| format!("{input}: {e}"))?;
    println!("input    {} vertices, {} triangles", mesh.positions.len(), mesh.triangles.len());

    let density = match &density_path {
        Some(path) => {
            let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
            let lines = text.lines().map(str::trim).filter(|line| !line.is_empty() && !line.starts_with('#'));
            let values: Vec<f64> = lines
                .map(|line| parse("--density", line))
                .collect::<Result<_, _>>()
                .map_err(|e| format!("{path}: {e}"))?;
            println!("density  {} painted weights, contrast {}", values.len(), config.density_contrast);
            Some(values)
        }
        None => None,
    };
    let curves = match &curves_path {
        Some(path) => {
            let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
            let curves = read_curves(&text).map_err(|e| format!("{path}: {e}"))?;
            println!("curves   {} guide curves", curves.len());
            curves
        }
        None => Vec::new(),
    };
    let guides = Guides { density: density.as_deref(), curves: &curves };
    let (result, report) = match &record_path {
        Some(path) => remesh_recorded(path, &config, &mesh, &guides)?,
        None => config.remesh_guided(&mesh, &guides).map_err(|e| e.to_string())?,
    };
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
        "quality  {} irregular vertices, {} boundary edges, {} non-manifold edges, {} open loops, {} map inversions",
        stats.irregular_vertices,
        stats.boundary_edges,
        stats.non_manifold_edges,
        report.open_loops,
        report.map_inversions,
    );
    if !curves.is_empty() {
        println!("guides   {} edges along the guide curves, {} breaks", report.guide_edges.len(), report.guide_gaps);
        if curves.iter().any(|curve| curve.sharp) {
            println!("         {} hard edges along the curves drawn as sharp", report.sharp_curve_edges.len());
        }
        if report.guides_let_go > 0 {
            println!("         could not be kept as lines of edges on {} piece(s)", report.guides_let_go);
        }
    }
    if report.pieces > 1 {
        println!(
            "pieces   {} separate pieces, {} through the integer-grid chain, {} not meshed, {} left out as too small",
            report.pieces, report.pieces_fell_back, report.pieces_failed, report.pieces_skipped
        );
    } else if report.fell_back {
        println!("note     the integer-grid chain did the job");
    }
    for (cause, count) in &report.fallback_causes {
        println!("         {count} x {cause}");
    }
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
    println!(
        "volume       {} of the source; thin parts ({:.1} % of the source): {}",
        optional(m.volume_ratio.map(|ratio| format!("{:.1} %", 100.0 * ratio))),
        100.0 * m.thin_share,
        optional(m.thin_distance.map(|distance| format!("{distance:.3} quad edges away on average"))),
    );
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
            "--kind" => kind = value(arg)?.parse().map_err(|e: openclare::Error| e.to_string())?,
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
            "--kind" => kind = value(arg)?.parse().map_err(|e: openclare::Error| e.to_string())?,
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

fn parametrize_command(args: &[String]) -> Result<(), String> {
    let mut work_request = preprocess::Request { triangles: 50_000, ..Default::default() };
    let mut field_request = crossfield::Request::default();
    let (mut quads, mut simplify, mut rounds, mut paths) = (None::<usize>, None::<f64>, 40usize, Vec::new());
    let mut kind = parametrization::Kind::Seamless;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} expects a value"));
        match arg.as_str() {
            "--quads" => quads = Some(parse(arg, value(arg)?)?),
            "--triangles" => work_request.triangles = parse(arg, value(arg)?)?,
            "--alignment" => field_request.alignment = parse(arg, value(arg)?)?,
            "--simplify" => simplify = Some(parse(arg, value(arg)?)?),
            "--crease-angle" => work_request.crease_angle = Some(parse(arg, value(arg)?)?),
            "--rounds" => rounds = parse(arg, value(arg)?)?,
            "--kind" => kind = value(arg)?.parse().map_err(|e: openclare::Error| e.to_string())?,
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => paths.push(path),
        }
    }
    let [input] = paths[..] else {
        return Err("parametrize takes one input path".into());
    };
    let quads = quads.filter(|&n| n > 0).ok_or("parametrize needs --quads")?;

    let source = obj::read_obj(input).map_err(|e| format!("{input}: {e}"))?;
    let edge_length = (source.surface_area() / quads as f64).sqrt();
    work_request.curvature_scale = Some(edge_length);
    let work = preprocess::create(preprocess::Kind::Isotropic)
        .preprocess(&source, &work_request)
        .map_err(|e| e.to_string())?;
    let mut field =
        crossfield::create(crossfield::Kind::Power).solve(&work, &field_request).map_err(|e| e.to_string())?;
    if let Some(quad_edges) = simplify {
        let options = singularity::Request {
            alignment: field_request.alignment,
            reach: Some(quad_edges * edge_length),
            ..Default::default()
        };
        field = singularity::create(singularity::Kind::PairCancellation)
            .optimize(&work, &field, &options)
            .map_err(|e| e.to_string())?
            .0;
    }
    let (positive, negative, other) = field.singularities();

    let start = std::time::Instant::now();
    let request = parametrization::Request { edge_length, stiffening_rounds: rounds };
    let map = parametrization::create(kind).parametrize(&work, &field, &request).map_err(|e| e.to_string())?;
    let seconds = start.elapsed().as_secs_f64();

    let report = &map.report;
    println!("work mesh      {} triangles, {} singularities", work.mesh.triangles.len(), positive + negative + other);
    println!("cut            {} seam edges, {} vertices once cut open", report.seam_edges, report.wedges);
    println!(
        "inverted       {} after {} stiffening rounds ({} at first, {} before the last series)",
        report.flipped, report.rounds, report.flipped_first, report.flipped_before_stiffening
    );
    println!("distortion     mean {:.4}, worst stretch {:.2}", report.distortion_mean, report.stretch_max);
    println!("conditions     worst mismatch {:.2e}, {} whole numbers imposed", report.seam_error, report.integers);
    println!("seconds        {seconds:.3}");
    Ok(())
}

/// What the stages up to the T-mesh are run with.
struct LayoutOptions {
    input: Option<String>,
    quads: Option<usize>,
    work: preprocess::Request<'static>,
    field: crossfield::Request,
    simplify: Option<f64>,
    rounds: usize,
}

impl LayoutOptions {
    fn new() -> Self {
        Self {
            input: None,
            quads: None,
            work: preprocess::Request { triangles: 50_000, ..Default::default() },
            field: crossfield::Request::default(),
            simplify: None,
            rounds: 40,
        }
    }

    /// Takes `arg` if it is one of these options. Returns whether it was.
    fn take(&mut self, arg: &str, args: &mut std::slice::Iter<String>) -> Result<bool, String> {
        let mut value = || args.next().ok_or(format!("{arg} expects a value"));
        match arg {
            "--quads" => self.quads = Some(parse(arg, value()?)?),
            "--triangles" => self.work.triangles = parse(arg, value()?)?,
            "--alignment" => self.field.alignment = parse(arg, value()?)?,
            "--simplify" => self.simplify = Some(parse(arg, value()?)?),
            "--crease-angle" => self.work.crease_angle = Some(parse(arg, value()?)?),
            "--rounds" => self.rounds = parse(arg, value()?)?,
            flag if flag.starts_with("--") => return Ok(false),
            path if self.input.is_none() => self.input = Some(path.to_string()),
            _ => return Err("expected a single input path".into()),
        }
        Ok(true)
    }

    /// Runs the chain up to the T-mesh. Returns it with the size of the
    /// work mesh, whether the map kept feature edges on grid lines, the
    /// number of quads asked for, and the seconds the tracing took.
    fn build(mut self) -> Result<Layout, String> {
        let input = self.input.ok_or("expected an input path")?;
        let quads = self.quads.filter(|&n| n > 0).ok_or("--quads is required")?;
        let source = obj::read_obj(&input).map_err(|e| format!("{input}: {e}"))?;
        let edge_length = (source.surface_area() / quads as f64).sqrt();
        self.work.curvature_scale = Some(edge_length);
        let work = preprocess::create(preprocess::Kind::Isotropic)
            .preprocess(&source, &self.work)
            .map_err(|e| e.to_string())?;
        let mut field =
            crossfield::create(crossfield::Kind::Power).solve(&work, &self.field).map_err(|e| e.to_string())?;
        if let Some(quad_edges) = self.simplify {
            let options = singularity::Request {
                alignment: self.field.alignment,
                reach: Some(quad_edges * edge_length),
                ..Default::default()
            };
            field = singularity::create(singularity::Kind::PairCancellation)
                .optimize(&work, &field, &options)
                .map_err(|e| e.to_string())?
                .0;
        }
        let request = parametrization::Request { edge_length, stiffening_rounds: self.rounds };
        let map = parametrization::create(parametrization::Kind::Seamless)
            .parametrize(&work, &field, &request)
            .map_err(|e| e.to_string())?;

        let start = std::time::Instant::now();
        let tmesh = TMesh::trace(&work, &field, &map, 20.0 * (quads as f64).sqrt());
        Ok(Layout {
            tmesh,
            triangles: work.mesh.triangles.len(),
            features_off_grid: map.report.features_off_grid,
            quads,
            seconds: start.elapsed().as_secs_f64(),
        })
    }
}

struct Layout {
    tmesh: TMesh,
    triangles: usize,
    features_off_grid: bool,
    quads: usize,
    seconds: f64,
}

fn tmesh_command(args: &[String]) -> Result<(), String> {
    let mut options = LayoutOptions::new();
    let (mut traces_path, mut json) = (None, false);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if options.take(arg, &mut args)? {
            continue;
        }
        match arg.as_str() {
            "--traces" => traces_path = Some(args.next().ok_or("--traces expects a value")?.clone()),
            "--json" => json = true,
            flag => return Err(format!("unknown option '{flag}'")),
        }
    }
    let Layout { tmesh, triangles, features_off_grid, seconds, .. } = options.build()?;

    let report = &tmesh.report;
    if json {
        println!(
            "{{\"triangles\": {}, \"map_inversions\": {}, \"features_off_grid\": {}, \"off_grid_edges\": {}, \
             \"singularities\": {}, \"traces\": {}, \"lost_traces\": {}, \"overlaps\": {}, \"nodes\": {}, \
             \"t_junctions\": {}, \"arcs\": {}, \"patches\": {}, \"rectangles\": {}, \"side_mismatch\": {:e}, \
             \"area_error\": {:e}, \"euler\": {}, \"surface_euler\": {}, \"longest_trace\": {}, \
             \"short_arcs\": {}, \"thin_patches\": {}, \"valid\": {}, \"seconds\": {}}}",
            triangles,
            report.map_inversions,
            features_off_grid,
            report.off_grid_edges,
            report.singularities,
            report.traces,
            report.lost_traces,
            report.overlaps,
            report.nodes,
            report.t_junctions,
            report.arcs,
            report.patches,
            report.rectangles,
            report.side_mismatch,
            report.area_error,
            report.euler,
            report.surface_euler,
            report.longest_trace,
            report.short_arcs,
            report.thin_patches,
            report.is_valid(),
            seconds
        );
    } else {
        println!(
            "work mesh      {} triangles, {} inverted in the map, {} border or hard edges off the grid lines",
            triangles, report.map_inversions, report.off_grid_edges
        );
        println!(
            "traces         {} from {} singularities, {} lost, {} overlapping, longest {:.1} quad edges",
            report.traces, report.singularities, report.lost_traces, report.overlaps, report.longest_trace
        );
        println!("graph          {} nodes ({} T-junctions), {} arcs", report.nodes, report.t_junctions, report.arcs);
        println!(
            "patches        {} of which {} rectangles; opposite sides differ by {:.2e} at most",
            report.patches, report.rectangles, report.side_mismatch
        );
        println!(
            "checks         area off by {:.2e}; nodes - arcs + patches = {} (surface: {}); {}",
            report.area_error,
            report.euler,
            report.surface_euler,
            if report.is_valid() { "valid" } else { "NOT VALID" }
        );
        println!(
            "to quantize    {} arcs and {} patches under half a quad edge",
            report.short_arcs, report.thin_patches
        );
        // What is wrong with the patches that are no rectangles: the angles
        // round their rim, in quarter turns, the kind of node at each, and a
        // star where the rim runs along a border or a hard edge.
        for patch in tmesh.patches.iter().filter(|patch| patch.sides.is_none()).take(5) {
            let rim: Vec<String> = patch
                .boundary
                .iter()
                .zip(&patch.angles)
                .map(|(&half, angle)| {
                    let arc = &tmesh.arcs[half as usize / 2];
                    let kind = match tmesh.nodes[arc.nodes[1 - half as usize % 2] as usize].kind {
                        openclare::tmesh::NodeKind::Singularity { .. } => "s",
                        openclare::tmesh::NodeKind::Corner => "c",
                        openclare::tmesh::NodeKind::Junction => "j",
                    };
                    format!("{angle}{kind}{}", if arc.feature { "*" } else { "" })
                })
                .collect();
            println!("not a rectangle {}", rim.join(" "));
        }
        println!("seconds        {seconds:.3}");
    }

    if let Some(path) = traces_path {
        let mut text = String::new();
        let mut count = 0;
        for trace in tmesh.traces.iter().map(|trace| &trace.points) {
            for p in trace {
                text.push_str(&format!("v {} {} {}\n", p.x, p.y, p.z));
            }
            let indices: Vec<String> = (count + 1..=count + trace.len()).map(|i| i.to_string()).collect();
            text.push_str(&format!("l {}\n", indices.join(" ")));
            count += trace.len();
        }
        std::fs::write(&path, text).map_err(|e| format!("{path}: {e}"))?;
    }
    Ok(())
}

fn quantize_command(args: &[String]) -> Result<(), String> {
    let mut options = LayoutOptions::new();
    let mut request = quantization::Request::default();
    let (mut json, mut exact_scale) = (false, false);
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if options.take(arg, &mut args)? {
            continue;
        }
        match arg.as_str() {
            "--tolerance" => request.tolerance = parse(arg, args.next().ok_or("--tolerance expects a value")?)?,
            "--keep-scale" => exact_scale = true,
            "--json" => json = true,
            flag => return Err(format!("unknown option '{flag}'")),
        }
    }
    let Layout { tmesh, quads, .. } = options.build()?;
    if !tmesh.report.is_valid() {
        return Err("the T-mesh is not a valid partition into rectangles".into());
    }
    request.quads = (!exact_scale).then_some(quads);

    let start = std::time::Instant::now();
    let quantized = quantization::create(quantization::Kind::WalkCancelling)
        .quantize(&tmesh, &request)
        .map_err(|e| e.to_string())?;
    let seconds = start.elapsed().as_secs_f64();

    let report = &quantized.report;
    if json {
        println!(
            "{{\"arcs\": {}, \"patches\": {}, \"target\": {}, \"quads\": {}, \"budget_error\": {}, \"scale\": {}, \
             \"zero_arcs\": {}, \"collapsed_patches\": {}, \"held_arcs\": {}, \"deviation_mean\": {}, \
             \"deviation_max\": {}, \"balanced\": {}, \"separated\": {}, \"moves\": {}, \"seconds\": {}}}",
            report.arcs,
            tmesh.patches.len(),
            quads,
            report.quads,
            report.budget_error.map_or("null".to_string(), |error| error.to_string()),
            report.scale,
            report.zero_arcs,
            report.collapsed_patches,
            report.held_arcs,
            report.deviation_mean,
            report.deviation_max,
            report.balanced,
            report.separated,
            report.moves,
            seconds
        );
    } else {
        println!("t-mesh         {} arcs, {} patches", report.arcs, tmesh.patches.len());
        match report.budget_error {
            Some(error) => println!(
                "quads          {} for {} asked ({:+.1} %), real lengths scaled by {:.3}",
                report.quads,
                quads,
                100.0 * error * if report.quads < quads { -1.0 } else { 1.0 },
                report.scale
            ),
            None => println!("quads          {} at the scale of the T-mesh ({} asked)", report.quads, quads),
        }
        println!(
            "closed up      {} arcs of length zero, {} patches without area; {} arcs held open",
            report.zero_arcs, report.collapsed_patches, report.held_arcs
        );
        println!(
            "deviation      mean {:.3}, worst {:.2} quad edges from the scaled real lengths",
            report.deviation_mean, report.deviation_max
        );
        println!(
            "checks         opposite sides {}; structure {}",
            if report.balanced { "equal" } else { "NOT EQUAL" },
            if report.separated { "intact" } else { "NOT INTACT" }
        );
        println!("seconds        {seconds:.3} ({} improving moves)", report.moves);
    }
    Ok(())
}
