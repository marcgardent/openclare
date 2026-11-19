//! Placing a subdivision level on its source: what a Multires level needs
//! to carry the detail of a sculpt (PRO issue #6, milestone 3 of the
//! Multires specification).
//!
//! Blender's Multires subdivides the cage once; the new level is smooth and
//! runs between the faces of the level below and the source. Each of its
//! vertices is to be moved onto the source, as a Shrinkwrap would, without
//! the faults #6 measured on a Shrinkwrap: vertices thrown onto another part
//! (spikes from the hands), vertices that find nothing and stay while their
//! neighbours move (crumpled patches), creases rounded off.
//!
//! The guards are those of relaxation and fitting:
//!
//! - **same-side sheet**: a vertex only looks for triangles of the source
//!   that face the way the level faces there, so a vertex of a finger never
//!   lands on the other side of the finger, or on the body;
//! - **reach**: no vertex goes farther than [`Settings::reach`] times the
//!   shortest edge it is on, unless straight along its normal, which may go
//!   [`Settings::far`] times: the subdivision leaves a fingertip or an ear
//!   short of the source by more than an edge (by 1.9 % of the diagonal on
//!   the ogre of #6, at the first level), and with one edge of reach a level
//!   never caught it up; the tip stayed short, with a rim where the vertices
//!   of its sides had reached the source;
//! - **no folded face, no stretched edge**: a face that comes out folded, or
//!   with an edge more than [`Settings::stretch`] times as long as it was,
//!   has its vertices taken back, half their move at a pass, pass after
//!   pass. A tip brought out stretches the edges along it about three times;
//!   a vertex thrown far while its neighbour found nothing, seven to eleven.
//!   Fitting also takes back a face turned by more than 20°; here that is
//!   [`Settings::tilt`], off by default: on the ogre of #6 it took back a
//!   quarter of the vertices of the first level, which then missed more of
//!   the source than the cage.
//!
//! Per vertex: the first hit of the ray along the normal of the level, both
//! ways, on a same-side triangle, as far as `far`; failing that, the closest
//! same-side point within the reach; failing that, the closest one farther
//! out, the move along the normal cut to the reach (the closest points of
//! two neighbours that far off may lie apart, which stretched their edge on
//! the ogre; their normals do not). A vertex inside a creased line of the
//! level goes to the closest point of the source's hard edges, if there is
//! one within the reach. A vertex that finds nothing stays. Moving it as its
//! neighbours moved, against the crumpling #6 saw, left the ogre's missing
//! detail at the fourth level as it was, and was dropped.

use std::fmt::Write as _;
use std::str::FromStr;
use std::time::Instant;

use rayon::prelude::*;

use crate::creases::Crease;
use crate::error::{Error, Result};
use crate::fitting::{facing, turns, vertex_normals};
use crate::math::Vec3;
use crate::mesh::{PolyMesh, TriMesh};
use crate::metrics::{DETAIL_SAMPLES, LevelDetail, level_detail, level_detail_by_face};
use crate::preprocess::{self, Request};
use crate::spatial::Bvh;

/// What became of a vertex of the level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Status {
    /// Moved onto the source, along its normal or to the closest point.
    Moved = 0,
    /// Moved onto a hard edge of the source, being inside a creased line.
    OnLine = 1,
    /// The source is farther than the reach: moved towards it along its
    /// normal as far as the reach allows.
    Reached = 2,
    /// Nothing of the source facing its way anywhere near: left where it was.
    NotFound = 3,
    /// Taken back towards where the subdivision left it, a face round it
    /// having come out folded or tilted.
    PutBack = 4,
    /// Left where the subdivision put it: on a border, or at a corner or an
    /// end of a creased line.
    Held = 5,
}

impl Status {
    pub const ALL: [Status; 6] =
        [Status::Moved, Status::OnLine, Status::Reached, Status::NotFound, Status::PutBack, Status::Held];

    /// The name of its count in the report.
    pub fn name(self) -> &'static str {
        match self {
            Status::Moved => "moved",
            Status::OnLine => "on_line",
            Status::Reached => "reached",
            Status::NotFound => "not_found",
            Status::PutBack => "put_back",
            Status::Held => "held",
        }
    }
}

/// The tuning of [`place`].
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// Farthest a vertex may go, as a share of the shortest edge it is on.
    /// One edge is what limited Project used in #6.
    pub reach: f64,
    /// Farthest a vertex may go straight along its normal, as a share of the
    /// same shortest edge: the subdivision leaves a fingertip short of the
    /// source by more than an edge, and only a move along the normal brings
    /// it out without throwing it sideways.
    pub far: f64,
    /// Longest an edge may come out, as a multiple of its length before:
    /// longer, the moves of its ends are taken back as a fold's are.
    pub stretch: f64,
    /// Most a face may turn, in degrees, before its vertices are taken
    /// back; 180, the default, never.
    pub tilt: f64,
    /// Dihedral angle, in degrees, above which an edge of the source is a
    /// hard edge that creased lines of the level go to.
    pub crease_angle: f64,
    /// Points spread on the source to measure the missing detail of the
    /// level once placed; 0 measures nothing.
    pub samples: usize,
    /// Where those points are drawn from.
    pub seed: u64,
    /// Missing detail, 99th percentile in percent of the diagonal, below
    /// which a level is done (decision 2 of the specification: 0.05).
    pub target: f64,
    /// A cage face above the target and above the level's own missing
    /// detail, whose missing detail fell by less than this share since the
    /// level below, no longer gains from another level: it wants more quads
    /// round it.
    pub stall: f64,
    /// A cage face above the target and above the level's own missing
    /// detail, with more than this share of its corners in the level not
    /// found or taken back, wants more quads too.
    pub stuck: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            reach: 1.0,
            far: 8.0,
            stretch: 4.0,
            tilt: 180.0,
            crease_angle: 30.0,
            samples: DETAIL_SAMPLES,
            seed: 1,
            target: 0.05,
            stall: 0.25,
            stuck: 0.25,
        }
    }
}

impl FromStr for Settings {
    type Err = Error;

    /// `key = value` lines, or `;` between them; blank lines and `#` comments
    /// ignored; what is not said keeps its default.
    fn from_str(text: &str) -> Result<Self> {
        let mut settings = Settings::default();
        for line in text.split(['\n', ';']).map(|line| line.split('#').next().unwrap_or("").trim()) {
            if line.is_empty() {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(Error::InvalidParams(format!("'{line}' is not 'key = value'")));
            };
            let (key, value) = (key.trim(), value.trim());
            let number = || -> Result<f64> {
                value
                    .parse::<f64>()
                    .ok()
                    .filter(|n| n.is_finite() && *n >= 0.0)
                    .ok_or_else(|| Error::InvalidParams(format!("{key} wants a number not below 0, not '{value}'")))
            };
            match key {
                "reach" => settings.reach = number()?,
                "far" => settings.far = number()?,
                "stretch" => settings.stretch = number()?,
                "tilt" => settings.tilt = number()?.min(180.0),
                "crease_angle" => settings.crease_angle = number()?.min(180.0),
                "samples" => settings.samples = number()? as usize,
                "seed" => settings.seed = number()? as u64,
                "target" => settings.target = number()?,
                "stall" => settings.stall = number()?.min(1.0),
                "stuck" => settings.stuck = number()?.min(1.0),
                _ => return Err(Error::InvalidParams(format!("unknown setting '{key}'"))),
            }
        }
        Ok(settings)
    }
}

/// A level placed on its source.
#[derive(Clone, Debug)]
pub struct Placed {
    /// Where each vertex of the level now is.
    pub positions: Vec<Vec3>,
    /// What became of each.
    pub status: Vec<Status>,
    /// What the level now misses of the source, if it was measured.
    pub detail: Option<LevelDetail>,
    /// What to do next, if the level was measured.
    pub advice: Option<Advice>,
    /// Cage face by cage face, if the level was measured and its faces'
    /// cage faces were given.
    pub faces: Option<CageDetail>,
    /// Seconds the placing took, the measure left out.
    pub seconds: f64,
}

impl Placed {
    /// How many vertices are in that status.
    pub fn count(&self, status: Status) -> usize {
        self.status.iter().filter(|&&s| s == status).count()
    }

    /// What happened, as `key = value` lines: the count of each status, the
    /// missing detail and the stretched edges if measured, seconds.
    pub fn report(&self) -> String {
        let mut text = String::new();
        let _ = writeln!(text, "vertices = {}", self.positions.len());
        for status in Status::ALL {
            let _ = writeln!(text, "{} = {}", status.name(), self.count(status));
        }
        if let Some(detail) = &self.detail {
            let _ = writeln!(text, "missing_p99 = {}", detail.missing_p99);
            let _ = writeln!(text, "missing_p50 = {}", detail.missing_p50);
            let _ = writeln!(text, "missing_p999 = {}", detail.missing_p999);
            let _ = writeln!(text, "stretched = {}", detail.stretched);
            let _ = writeln!(text, "longest_over_median = {}", detail.longest_over_median);
        }
        if let Some(advice) = self.advice {
            let _ = writeln!(text, "advice = {}", advice.name());
        }
        if let Some(faces) = &self.faces {
            let _ = writeln!(text, "denser_faces = {}", faces.count(Verdict::Denser));
            let _ =
                writeln!(text, "above_target_faces = {}", faces.count(Verdict::More) + faces.count(Verdict::Denser));
        }
        let _ = writeln!(text, "seconds = {:.3}", self.seconds);
        text
    }

    /// The same as one JSON object.
    pub fn to_json(&self) -> String {
        let mut fields = vec![format!("\"vertices\": {}", self.positions.len())];
        fields.extend(Status::ALL.iter().map(|&s| format!("\"{}\": {}", s.name(), self.count(s))));
        if let Some(detail) = &self.detail {
            fields.push(format!("\"detail\": {}", detail.to_json()));
        }
        if let Some(advice) = self.advice {
            fields.push(format!("\"advice\": \"{}\"", advice.name()));
        }
        if let Some(faces) = &self.faces {
            fields.push(format!("\"denser_faces\": {}", faces.count(Verdict::Denser)));
        }
        fields.push(format!("\"seconds\": {:.3}", self.seconds));
        format!("{{{}}}", fields.join(", "))
    }
}

/// What a level placed calls for next (Section 3.3 of the specification).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advice {
    /// The level misses less than the target: no other level is needed.
    Done,
    /// Above the target, and some cage faces above it still gain: one more
    /// level. Those that no longer gain are counted beside it
    /// (`denser_faces`): on the ogre of #6, at the first three levels, a
    /// few faces stall while all the rest gains.
    More,
    /// Every cage face above the target no longer gains from a level, or
    /// its vertices found little of the source: more quads round them in
    /// the cage, not another level.
    Denser,
}

impl Advice {
    pub fn name(self) -> &'static str {
        match self {
            Advice::Done => "done",
            Advice::More => "more",
            Advice::Denser => "denser",
        }
    }
}

/// What a cage face of a level calls for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Verdict {
    /// Its part of the level misses less than the target.
    Fine = 0,
    /// Above the target, and still gaining: another level helps.
    More = 1,
    /// Above the target and no longer gaining, or stuck: more quads round it.
    Denser = 2,
}

/// The cage a level was subdivided from, as [`place_on_cage`] takes it.
#[derive(Clone, Copy, Debug)]
pub struct Cage<'a> {
    /// For each face of the level, the cage face it comes from.
    pub face_of: &'a [u32],
    /// How many faces the cage has.
    pub faces: usize,
    /// What the level below missed over each cage face, as
    /// [`CageDetail::missing`] gave it; `None` for the first level.
    pub previous: Option<&'a [f64]>,
}

/// A level placed, cage face by cage face.
#[derive(Clone, Debug, PartialEq)]
pub struct CageDetail {
    /// What the level misses of the source over each cage face: the 99th
    /// percentile of the distances of the points closest to it, in percent
    /// of the diagonal, 0 where no point is.
    pub missing: Vec<f64>,
    /// What each cage face calls for.
    pub verdict: Vec<Verdict>,
}

impl CageDetail {
    pub fn count(&self, verdict: Verdict) -> usize {
        self.verdict.iter().filter(|&&v| v == verdict).count()
    }
}

/// Passes of taking back the moves that fold or tilt a face: halving a
/// move eleven times leaves a two-thousandth of it, then the last takes all.
const BACK_PASSES: usize = 12;

/// How much farther than the reach a vertex looks for the source before it
/// gives up.
const LOOK_BEYOND: f64 = 4.0;

/// Places `level`, a subdivision level, on `source`. `creases` are the
/// creased edges of the level, as the subdivision carried them from the cage.
pub fn place(level: &PolyMesh, creases: &[Crease], source: &TriMesh, settings: &Settings) -> Result<Placed> {
    place_on_cage(level, creases, source, settings, None)
}

/// [`place`], with the cage the level was subdivided from: the missing
/// detail is then also measured over each cage face, and each told what it
/// calls for.
pub fn place_on_cage(
    level: &PolyMesh,
    creases: &[Crease],
    source: &TriMesh,
    settings: &Settings,
    cage: Option<&Cage>,
) -> Result<Placed> {
    let vertices = level.positions.len();
    if let Some(beyond) = level.faces.iter().flatten().find(|&&v| v as usize >= vertices) {
        return Err(Error::InvalidMesh(format!("a face of the level names vertex {beyond}, of {vertices}")));
    }
    if let Some(([a, b], _)) = creases.iter().find(|([a, b], _)| *a as usize >= vertices || *b as usize >= vertices) {
        return Err(Error::InvalidMesh(format!("a crease names vertices {a} and {b}, of {vertices}")));
    }
    if source.triangles.is_empty() {
        return Err(Error::InvalidMesh("the source has no triangle".into()));
    }
    if let Some(cage) = cage {
        if cage.face_of.len() != level.faces.len() {
            return Err(Error::InvalidMesh(format!(
                "{} cage faces named for the {} faces of the level",
                cage.face_of.len(),
                level.faces.len()
            )));
        }
        if let Some(beyond) = cage.face_of.iter().find(|&&f| f as usize >= cage.faces) {
            return Err(Error::InvalidMesh(format!("a face of the level names cage face {beyond}, of {}", cage.faces)));
        }
        if let Some(previous) = cage.previous
            && previous.len() != cage.faces
        {
            return Err(Error::InvalidMesh(format!(
                "the level below is measured over {} cage faces, not {}",
                previous.len(),
                cage.faces
            )));
        }
    }
    let clock = Instant::now();
    let surface = Bvh::build(source);
    let start = &level.positions;

    // Neighbours, and the vertices of a border.
    let uses = level.edge_uses();
    let mut neighbours = vec![Vec::new(); vertices];
    let mut border = vec![false; vertices];
    for (&(a, b), &faces) in &uses {
        neighbours[a as usize].push(b);
        neighbours[b as usize].push(a);
        if faces != 2 {
            border[a as usize] = true;
            border[b as usize] = true;
        }
    }
    // Creased edges at each vertex: two make it a point inside a line, one
    // an end, three or more a corner.
    let mut creased = vec![0usize; vertices];
    for &([a, b], weight) in creases {
        if weight > 0.0 && a != b {
            creased[a as usize] += 1;
            creased[b as usize] += 1;
        }
    }
    let held: Vec<bool> = (0..vertices).map(|v| border[v] || creased[v] == 1 || creased[v] >= 3).collect();
    let lines = if creased.contains(&2) { hard_lines(source, settings.crease_angle)? } else { None };

    let normals = vertex_normals(level);
    let shortest: Vec<f64> = (0..vertices)
        .map(|v| {
            let shortest =
                neighbours[v].iter().map(|&n| start[v].distance(start[n as usize])).fold(f64::INFINITY, f64::min);
            if shortest.is_finite() { shortest } else { 0.0 }
        })
        .collect();

    let (mut positions, mut status): (Vec<Vec3>, Vec<Status>) = (0..vertices)
        .into_par_iter()
        .map(|v| {
            let (p, reach) = (start[v], settings.reach * shortest[v]);
            if held[v] {
                return (p, Status::Held);
            }
            if creased[v] == 2
                && let Some(lines) = &lines
                && let Some(hit) = lines.closest_point_where(p, reach, |_| true)
            {
                return (hit.point, Status::OnLine);
            }
            let normal = normals[v];
            if normal == Vec3::ZERO {
                return (p, Status::NotFound);
            }
            let same_side = |t: u32| surface.area_vector(t).dot(normal) > 0.0;
            let far = settings.far.max(settings.reach) * shortest[v];
            let out = surface.first_hit(p, normal, far, |t, _| same_side(t));
            let back = surface.first_hit(p, -normal, far, |t, _| same_side(t));
            match (out, back) {
                (Some((o, _)), Some((b, _))) if b < o => return (p - normal * b, Status::Moved),
                (Some((o, _)), _) => return (p + normal * o, Status::Moved),
                (None, Some((b, _))) => return (p - normal * b, Status::Moved),
                (None, None) => {}
            }
            if let Some(hit) = surface.closest_point_where(p, reach, same_side) {
                return (hit.point, Status::Moved);
            }
            if let Some(hit) = surface.closest_point_where(p, LOOK_BEYOND * reach, same_side) {
                // Along the normal only: the closest points of two
                // neighbours that far off may lie apart, their normals not.
                let along = (hit.point - p).dot(normal);
                return (p + normal * along.clamp(-reach, reach), Status::Reached);
            }
            (p, Status::NotFound)
        })
        .unzip();

    put_back_folds(level, start, &mut positions, &mut status, settings.tilt.to_radians(), settings.stretch);
    let seconds = clock.elapsed().as_secs_f64();

    if settings.samples == 0 {
        return Ok(Placed { positions, status, detail: None, advice: None, faces: None, seconds });
    }
    let placed = PolyMesh { positions, faces: level.faces.clone() };
    let (detail, faces) = match cage {
        None => (level_detail(source, &placed, settings.samples, settings.seed), None),
        Some(cage) => {
            let (detail, missing) =
                level_detail_by_face(source, &placed, settings.samples, settings.seed, cage.face_of, cage.faces);
            let verdict = verdicts(level, &status, cage, &missing, detail.missing_p99, settings);
            (detail, Some(CageDetail { missing, verdict }))
        }
    };
    let advice = if detail.missing_p99 < settings.target {
        Advice::Done
    } else if faces.as_ref().is_some_and(|faces| faces.count(Verdict::Denser) > 0 && faces.count(Verdict::More) == 0) {
        Advice::Denser
    } else {
        Advice::More
    };
    Ok(Placed { positions: placed.positions, status, detail: Some(detail), advice: Some(advice), faces, seconds })
}

/// What each cage face calls for: fine below the target; above it, more
/// quads round it when it misses more than the level as a whole (`p99`, the
/// level's missing detail) and either its missing detail fell by less than
/// [`Settings::stall`] since the level below, or more than
/// [`Settings::stuck`] of its corners in the level found nothing or were
/// taken back; another level otherwise. Without the first condition, a part
/// that gains slowly but misses less than the rest of the level was told to
/// want more quads: on the ogre of #6, at the second level, 71 cage faces,
/// a tenth of its area, against 19 with it.
fn verdicts(
    level: &PolyMesh,
    status: &[Status],
    cage: &Cage,
    missing: &[f64],
    p99: f64,
    settings: &Settings,
) -> Vec<Verdict> {
    let (mut stuck, mut corners) = (vec![0usize; cage.faces], vec![0usize; cage.faces]);
    for (face, &of) in level.faces.iter().zip(cage.face_of) {
        for &v in face {
            corners[of as usize] += 1;
            if matches!(status[v as usize], Status::NotFound | Status::PutBack) {
                stuck[of as usize] += 1;
            }
        }
    }
    (0..cage.faces)
        .map(|f| {
            if missing[f] <= settings.target {
                return Verdict::Fine;
            }
            let stalled = cage.previous.is_some_and(|previous| missing[f] > (1.0 - settings.stall) * previous[f]);
            let stuck = stuck[f] as f64 > settings.stuck * corners[f] as f64;
            if missing[f] > p99 && (stalled || stuck) { Verdict::Denser } else { Verdict::More }
        })
        .collect()
}

/// The hard edges of `source` sharper than `angle` degrees, as a hierarchy
/// over their segments; `None` if it has none.
fn hard_lines(source: &TriMesh, angle: f64) -> Result<Option<Bvh>> {
    let request = Request { crease_angle: Some(angle), ..Request::default() };
    let hard = preprocess::hard_edges(source, &request)?;
    if hard.kept.is_empty() {
        return Ok(None);
    }
    let segments = hard.kept.iter().map(|&[a, b]| {
        let (a, b) = (source.positions[a as usize], source.positions[b as usize]);
        [a, b, b]
    });
    Ok(Some(Bvh::from_triangles(segments.collect())))
}

/// Takes back the vertices of every face that came out folded or turned by
/// more than `tilt` radians, pass after pass while that upsets another face:
/// half their move at each pass, all of it at the last, so that a face that
/// only needed its vertices drawn back a little keeps the rest of the move.
fn put_back_folds(
    level: &PolyMesh,
    start: &[Vec3],
    positions: &mut [Vec3],
    status: &mut [Status],
    tilt: f64,
    stretch: f64,
) {
    let stretched = |positions: &[Vec3], face: &[u32]| {
        (0..face.len()).any(|i| {
            let (a, b) = (face[i] as usize, face[(i + 1) % face.len()] as usize);
            positions[a].distance(positions[b]) > stretch * start[a].distance(start[b])
        })
    };
    for pass in 0..BACK_PASSES {
        let back: Vec<u32> = level
            .faces
            .par_iter()
            .filter(|face| {
                let (was, now) = (facing(start, face), facing(positions, face));
                let tilted = was.dot(now) < tilt.cos() * was.length() * now.length();
                tilted
                    || stretched(positions, face)
                    || turns(start, face).iter().zip(turns(positions, face)).any(|(&was, now)| was && !now)
            })
            .flat_map_iter(|face| face.iter().copied())
            .filter(|&v| positions[v as usize] != start[v as usize])
            .collect();
        let mut back = back;
        back.sort_unstable();
        back.dedup();
        if back.is_empty() {
            break;
        }
        let share = if pass + 1 == BACK_PASSES { 0.0 } else { 0.5 };
        for v in back {
            let v = v as usize;
            positions[v] = start[v] + (positions[v] - start[v]) * share;
            status[v] = Status::PutBack;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_read_what_they_are_told_and_keep_the_rest() {
        let settings: Settings =
            "reach = 0.5\n# a comment\ntilt = 30 ; seed = 4; far = 2; stretch = 3".parse().unwrap();
        assert_eq!(
            settings,
            Settings { reach: 0.5, far: 2.0, stretch: 3.0, tilt: 30.0, seed: 4, ..Settings::default() }
        );
        assert_eq!(
            "target = 0.1; stall = 0.5; stuck = 2".parse::<Settings>().unwrap(),
            Settings { target: 0.1, stall: 0.5, stuck: 1.0, ..Settings::default() }
        );
        assert_eq!("".parse::<Settings>().unwrap(), Settings::default());
        assert!("reach = -1".parse::<Settings>().is_err());
        assert!("speed = 3".parse::<Settings>().is_err());
    }
}
