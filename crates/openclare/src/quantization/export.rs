//! The system the closing of loops works on, written out as JSON for the
//! experiments of the research papers (`OPENCLARE_EXPORT_TURNS`, see
//! [`super::Experiment`]). For each T-mesh, named as [`call_name`] says:
//!
//! - `call-<name>.json`: the T-mesh, its balances, and the lengths the
//!   quantizer ranks first, with what the engine knows of them (the bounds
//!   it keeps to, every strip of quads they make);
//! - `call-<name>.txt`: those lengths, as `OPENCLARE_LENGTHS` reads them;
//! - `call-<name>/way-<way>.json`, for each way of closing tried: the
//!   T-mesh again, every turn the way found, those it held and those the
//!   projection kept, and the whole lengths of the way;
//! - `call-<name>/way-<way>-helices.json`: every strip of quads those
//!   lengths make, with its pitch.
//!
//! ALGORITHM.md (M6, "Experiment options") gives every field.

use std::fmt::Write;
use std::path::Path;

use super::experiment::call_name;
use super::{CLOSE_WAYS, CUT_LONGEST, Kept, Lengths, Request, lattice, tubes};
use crate::tmesh::{NodeKind, TMesh};

/// Writes the files of the way of closing `way` of `tmesh`: the turns it
/// found (`every`), those it held (`held`) and those the projection kept
/// (`kept`), and its whole `lengths`.
#[allow(clippy::too_many_arguments)]
pub(super) fn way(
    directory: &Path,
    tmesh: &TMesh,
    real: &[f64],
    way: usize,
    lengths: &Lengths,
    every: &[tubes::Turn],
    held: &[tubes::Turn],
    kept: &[&tubes::Turn],
) {
    let (x, scale, quads, lower) = lengths;
    let name = call_name(tmesh);
    let (offsets, lean, longest) = CLOSE_WAYS[way];
    let mut out = String::new();
    write!(
        out,
        "{{\"call\":\"{}\",\"way\":{way},\"offsets\":{offsets:?},\"most_lean\":{lean},\"longest\":{longest},\"scale\":{scale},\"quads\":{quads},\"spirals\":{},\"real\":{real:?},\"lengths\":{x:?},\"lower\":{lower:?},",
        &name["call-".len()..],
        spirals(tmesh, x)
    )
    .unwrap();
    tmesh_fields(&mut out, tmesh);
    out.push_str(",\"turns\":[");
    for (k, turn) in every.iter().enumerate() {
        let first_arc = tmesh.patches[turn.patch].boundary[0] as usize / 2;
        let at = tmesh.nodes[tmesh.arcs[first_arc].nodes[0] as usize].position;
        let held_index = held.iter().position(|other| other.shift == turn.shift).map_or(-1, |k| k as i64);
        let kept = kept.iter().any(|other| other.shift == turn.shift);
        let crossings: Vec<String> =
            turn.crossings.iter().map(|&(patch, half, next, into)| format!("[{patch},{half},{next},{into}]")).collect();
        if k > 0 {
            out.push(',');
        }
        write!(
            out,
            "{{\"shift\":{},\"along\":{},\"around\":{},\"lean\":{},\"patch\":{},\"at\":[{},{},{}],\"held\":{},\"held_index\":{held_index},\"kept\":{kept},\"side\":{},\"offset\":{},\"crossings\":[{}]}}",
            expression(&turn.shift),
            expression(&turn.along),
            turn.around,
            tubes::value(&turn.shift, real).abs() / turn.around,
            turn.patch,
            at.x,
            at.y,
            at.z,
            held_index >= 0,
            turn.side,
            turn.offset,
            crossings.join(",")
        )
        .unwrap();
    }
    out.push_str("]}");
    let folder = directory.join(&name);
    save(&folder, &format!("way-{way}.json"), &out);

    // Every strip of quads the whole lengths make, walked through the
    // rectangles, and how far beside itself each comes back: its pitch.
    let whole: Vec<f64> = x.iter().map(|&length| length as f64).collect();
    let side = (*quads as f64).sqrt().max(1.0);
    let strips: Vec<String> = tubes::lattice_turns(tmesh, &whole, 4, CUT_LONGEST * side)
        .iter()
        .map(|strip| {
            format!(
                "{{\"shift\":{},\"around\":{},\"pitch\":{},\"field\":{}}}",
                expression(&strip.shift),
                strip.around,
                tubes::value(&strip.shift, &whole),
                scale * tubes::value(&strip.shift, real)
            )
        })
        .collect();
    save(&folder, &format!("way-{way}-helices.json"), &format!("[{}]", strips.join(",")));
}

/// Writes the files of `tmesh` for the lengths `kept`, the first ranked
/// first and the plain ones last.
pub(super) fn call(directory: &Path, tmesh: &TMesh, real: &[f64], exact: &[f64], request: &Request, kept: &[Kept]) {
    let (Some(chosen), Some(plain)) = (kept.first(), kept.last()) else { return };
    let experiment = &request.experiment;
    let (x, scale, quads, lower) = &chosen.lengths;
    let (wanted, allowed) = super::budget(request, &plain.lengths);
    let folds = super::arc_folds(tmesh);
    let plain_deviation = super::deviation(&plain.lengths, real);
    let name = call_name(tmesh);
    let mut out = String::new();
    write!(
        out,
        "{{\"call\":\"{}\",\"scale\":{scale},\"quads\":{quads},\"wanted\":{wanted},\"allowed\":{allowed},\"plain_quads\":{},\"plain_scale\":{},\"plain_deviation\":{plain_deviation},\"deviation\":{},\"most_deviation\":{},\"off_folds\":{},\"most_off_folds\":{},\"spirals\":{},\"turns_held\":{},\"from_selector\":{},\"way\":{},\"round\":{},",
        &name["call-".len()..],
        plain.lengths.2,
        plain.lengths.1,
        super::deviation(&chosen.lengths, real),
        number(super::most_deviation(plain_deviation, experiment)),
        super::off_folds(&folds, x),
        number(super::most_off_folds(&folds, &plain.lengths.0, experiment)),
        spirals(tmesh, x),
        chosen.held,
        chosen.from_selector,
        chosen.way.map_or("null".to_string(), |(way, _)| way.to_string()),
        chosen.way.map_or("null".to_string(), |(_, round)| round.to_string()),
    )
    .unwrap();
    write!(
        out,
        "\"real\":{real:?},\"exact\":{exact:?},\"lengths\":{x:?},\"plain_lengths\":{:?},\"lower\":{lower:?},\"plain_lower\":{:?},",
        plain.lengths.0, plain.lengths.3
    )
    .unwrap();
    let folds: Vec<String> = folds.iter().map(|&(arc, share, sharp)| format!("[{arc},{share},{sharp}]")).collect();
    write!(out, "\"folds\":[{}],", folds.join(",")).unwrap();
    let defects = lattice::build(tmesh, x).defects(tmesh);
    write!(out, "\"reopen\":{:?},\"unexplained\":{},", defects.reopen, defects.unexplained).unwrap();
    tmesh_fields(&mut out, tmesh);
    // Every strip of quads the lengths make, each with the conditions on the
    // lengths under which it stays the same walk.
    out.push_str(",\"strips\":[");
    let whole: Vec<f64> = x.iter().map(|&length| length as f64).collect();
    let side = (*quads as f64).sqrt().max(1.0);
    for (k, (strip, conditions)) in tubes::lattice_walks(tmesh, &whole, 4, CUT_LONGEST * side).iter().enumerate() {
        if k > 0 {
            out.push(',');
        }
        let conditions: Vec<String> =
            conditions.iter().map(|(row, constant)| format!("[{},{constant}]", expression(row))).collect();
        write!(
            out,
            "{{\"shift\":{},\"around\":{},\"pitch\":{},\"field\":{},\"conditions\":[{}]}}",
            expression(&strip.shift),
            strip.around,
            tubes::value(&strip.shift, &whole),
            scale * tubes::value(&strip.shift, real),
            conditions.join(",")
        )
        .unwrap();
    }
    out.push_str("]}");
    save(directory, &format!("{name}.json"), &out);
    let lengths: Vec<String> = x.iter().map(|length| length.to_string()).collect();
    save(directory, &format!("{name}.txt"), &format!("{scale}\n{}\n", lengths.join(" ")));
}

/// The T-mesh, as fields of an object: `stations`, the balances, a row per
/// rectangle and pair of opposite sides; `arcs_at`, the middle of each arc;
/// each node with its place, its kind, the half-arcs leaving it and the
/// angle round it in quarter turns (its index is 4 less that); each arc
/// with its nodes, its length, its patches, the trace it lies on and the
/// side of each patch it runs along; each patch's rim from its first corner,
/// as the turns read it.
fn tmesh_fields(out: &mut String, tmesh: &TMesh) {
    let mut rows = Vec::new();
    for patch in &tmesh.patches {
        let Some(side_of) = patch.side_of_each() else { continue };
        for pair in 0..2 {
            let mut row: Vec<(usize, f64)> = Vec::new();
            for (&half, &side) in patch.boundary.iter().zip(&side_of) {
                if side as usize % 2 == pair {
                    row.push((half as usize / 2, if side < 2 { 1.0 } else { -1.0 }));
                }
            }
            rows.push(expression(&row));
        }
    }
    write!(out, "\"stations\":[{}]", rows.join(",")).unwrap();
    let middles: Vec<String> = tmesh
        .arcs
        .iter()
        .map(|arc| {
            let (a, b) = (tmesh.nodes[arc.nodes[0] as usize].position, tmesh.nodes[arc.nodes[1] as usize].position);
            let middle = 0.5 * (a + b);
            format!("[{},{},{}]", middle.x, middle.y, middle.z)
        })
        .collect();
    write!(out, ",\"arcs_at\":[{}]", middles.join(",")).unwrap();
    let mut angle = vec![0u32; tmesh.nodes.len()];
    for patch in &tmesh.patches {
        for (&half, &quarter) in patch.boundary.iter().zip(&patch.angles) {
            let arc = &tmesh.arcs[half as usize / 2];
            angle[arc.nodes[1 - (half as usize % 2)] as usize] += quarter as u32;
        }
    }
    let nodes: Vec<String> = tmesh
        .nodes
        .iter()
        .zip(&angle)
        .map(|(node, angle)| {
            let (kind, vertex) = match node.kind {
                NodeKind::Singularity { vertex } => ("singularity", vertex as i64),
                NodeKind::Corner => ("corner", -1),
                NodeKind::Junction => ("junction", -1),
            };
            format!(
                "{{\"at\":[{},{},{}],\"kind\":\"{kind}\",\"vertex\":{vertex},\"halves\":{:?},\"angle\":{angle}}}",
                node.position.x, node.position.y, node.position.z, node.arcs
            )
        })
        .collect();
    write!(out, ",\"nodes\":[{}]", nodes.join(",")).unwrap();
    let rims = tubes::rim_halves(tmesh);
    let mut side_in = vec![[-1i64; 2]; tmesh.arcs.len()];
    for rim in rims.iter().flatten() {
        for &(half, side) in rim {
            side_in[half as usize / 2][half as usize % 2] = side as i64;
        }
    }
    let arcs: Vec<String> = tmesh
        .arcs
        .iter()
        .zip(&side_in)
        .map(|(arc, sides)| {
            format!(
                "{{\"nodes\":{:?},\"length\":{},\"patches\":{:?},\"border\":{},\"feature\":{},\"trace\":{},\"span\":{:?},\"sides\":{sides:?}}}",
                arc.nodes, arc.length, arc.patches, arc.border, arc.feature, arc.trace, arc.span
            )
        })
        .collect();
    write!(out, ",\"arcs\":[{}]", arcs.join(",")).unwrap();
    let patches: Vec<String> = rims
        .iter()
        .map(|rim| match rim {
            Some(rim) => {
                let halves: Vec<String> = rim.iter().map(|&(half, side)| format!("[{half},{side}]")).collect();
                format!("[{}]", halves.join(","))
            }
            None => "null".to_string(),
        })
        .collect();
    write!(out, ",\"patches\":[{}]", patches.join(",")).unwrap();
}

/// A linear expression in the lengths, as `[[arc, coefficient], ...]`.
fn expression(row: &[(usize, f64)]) -> String {
    let terms: Vec<String> = row.iter().map(|&(arc, c)| format!("[{arc},{c}]")).collect();
    format!("[{}]", terms.join(","))
}

/// A number, or `null` where there is no bound.
fn number(value: f64) -> String {
    if value.is_finite() { value.to_string() } else { "null".to_string() }
}

/// The share of spiral loop edges of the quad mesh `x` makes, `-1` where
/// its loops cannot be followed.
fn spirals(tmesh: &TMesh, x: &[i64]) -> f64 {
    let share = super::spirals(tmesh, x);
    if share.is_finite() { share } else { -1.0 }
}

fn save(directory: &Path, name: &str, text: &str) {
    let written = std::fs::create_dir_all(directory).and_then(|_| std::fs::write(directory.join(name), text));
    if let Err(error) = written {
        eprintln!("OPENCLARE_EXPORT_TURNS: could not write {name} in {}: {error}", directory.display());
    }
}
