//! The catalogue of jobs, `lab/jobs.toml` (SPEC.md, section 6).
//!
//! A placeholder is `{name}` with a name of lower-case letters, digits, `_`
//! and `-`: `{threads}`, `{repo}`, `{commit}`, `{ref}` and the job's own
//! parameters.
//! `${...}` is the shell's and `{a,b}` is not a name: both are left as they
//! are.
//!
//! A `[[batch]]` (SPEC.md, §6b) is a job whose command is a template over
//! axes: every field of a `[[job]]`, plus `axes`, each axis a placeholder
//! with its values, and a `timeout` of 30 minutes by default.
//!
//! A `[[dependency]]` (SPEC.md, §6b, R2) is a job made once per key and
//! shared: a batch that `needs` it names `{<its name>}` in its command, the
//! path of the file it `provides` in `<lab>/cache/<name>/<key>/`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use crate::model::Paper;

/// The placeholders every job has.
pub const BUILT_IN: [&str; 4] = ["threads", "repo", "commit", "ref"];

/// The placeholder of a batch's job for its own folder of files (SPEC.md, §6b).
pub const OUT: &str = "out";

/// The placeholder of a report for the file of its records (SPEC.md, §6b, R4).
pub const RECORDS: &str = "records";

/// The axis that names the reference each cell runs on (SPEC.md, §6b).
pub const REFERENCE_AXIS: &str = "ref";

/// The timeout of a job of a batch that sets none (decision of 2026-11-13).
pub const BATCH_TIMEOUT_SECONDS: f64 = 1800.0;

/// The variables set to the effective thread count for every job, before
/// the job's own environment (SPEC.md, section 7.1).
pub const THREAD_VARIABLES: [&str; 4] =
    ["OPENCLARE_THREADS", "RAYON_NUM_THREADS", "RUST_TEST_THREADS", "CARGO_BUILD_JOBS"];

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub category: Category,
    pub command: String,
    #[serde(default)]
    pub prepare: Option<String>,
    #[serde(default)]
    pub checkout: CheckoutChoice,
    #[serde(default = "here")]
    pub directory: String,
    /// Name and default value, editable per queue item.
    #[serde(default)]
    pub parameters: BTreeMap<String, String>,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
    #[serde(default)]
    pub threads: ThreadUse,
    #[serde(default)]
    pub memory_limit_gb: Option<u32>,
    #[serde(default)]
    pub timing_matters: bool,
    /// Globs relative to the checkout.
    #[serde(default)]
    pub outputs: Vec<String>,
    /// The summary that records a run of the job in the repository: a path
    /// relative to the checkout, `#key` for a record nested in it.
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub paper: Option<Paper>,
    #[serde(default)]
    pub tags: Vec<String>,
    /// The job is ended past it, status `TimedOut`: "90s", "30m", "2h".
    /// None for a `[[job]]` that sets none (SPEC.md, Q20); a batch's job
    /// always has one.
    #[serde(default)]
    pub timeout: Option<Timeout>,
}

/// A duration written with its unit: "90s", "30m", "1.5h".
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(try_from = "String")]
pub struct Timeout {
    pub seconds: f64,
}

impl TryFrom<String> for Timeout {
    type Error = String;

    fn try_from(text: String) -> Result<Self, String> {
        Timeout::parse(&text)
    }
}

impl Timeout {
    pub fn parse(text: &str) -> Result<Timeout, String> {
        let text = text.trim();
        let wrong = || format!("timeout = \"{text}\": a number and its unit, s, m or h (\"90s\", \"30m\", \"2h\")");
        let unit = text.chars().last().ok_or_else(wrong)?;
        let scale = match unit {
            's' => 1.0,
            'm' => 60.0,
            'h' => 3600.0,
            _ => return Err(wrong()),
        };
        let number: f64 = text[..text.len() - 1].trim().parse().map_err(|_| wrong())?;
        if number <= 0.0 || !number.is_finite() {
            return Err(wrong());
        }
        Ok(Timeout { seconds: number * scale })
    }
}

/// A grid of jobs, a `[[batch]]` table (SPEC.md, §6b).
#[derive(Clone, Debug, PartialEq)]
pub struct Batch {
    /// The template: every field of a job, its timeout set.
    pub job: Job,
    /// In the order written; the last varies fastest.
    pub axes: Vec<Axis>,
    /// The dependencies its cells use, in the order they are made (R2).
    pub needs: Vec<String>,
    /// The command that makes its table from its records (R4).
    pub report: Option<String>,
    /// A job already done is skipped (R3); false for a batch whose jobs read
    /// files that their command, commit and dependencies do not name.
    pub skip_done: bool,
}

/// A computation made once per key and shared, a `[[dependency]]` table
/// (SPEC.md, §6b, R2): the engine of a commit, the source of a scene.
#[derive(Clone, Debug, PartialEq)]
pub struct Dependency {
    /// Every field of a job but prepare, summary and outputs.
    pub job: Job,
    /// Paths of the checkout, placeholders allowed: what its result depends on.
    pub key: Vec<String>,
    /// The tools whose versions go into the key.
    pub tools: Vec<String>,
    /// The file `{<name>}` names in its folder of the cache.
    pub provides: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Axis {
    pub name: String,
    pub values: Vec<AxisValue>,
}

/// A value of an axis, and the placeholders it sets for its cells only.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisValue {
    pub value: String,
    pub placeholders: BTreeMap<String, String>,
}

/// One cell of a batch's grid: the value of each axis, and the
/// placeholders of its job (the batch's parameters, the values' own, the
/// axes but `ref`).
#[derive(Clone, Debug, PartialEq)]
pub struct CellPlan {
    /// Axis -> value, the axes in their order.
    pub values: Vec<(String, String)>,
    pub parameters: BTreeMap<String, String>,
    /// The reference it runs on, when the batch has a `ref` axis.
    pub reference: Option<String>,
}

impl CellPlan {
    /// "model=ogre budget=3000 ref=12a6675", as `lab run` prints a cell.
    pub fn text(&self) -> String {
        self.values.iter().map(|(axis, value)| format!("{axis}={value}")).collect::<Vec<_>>().join(" ")
    }
}

fn here() -> String {
    ".".to_string()
}

/// A named, ordered list of jobs: the stages of a NIGHT.md in their order.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub jobs: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Build,
    TestSuite,
    QuickBench,
    Bench,
    LeanBuild,
    LeanOracle,
    Experiment,
}

impl Category {
    pub const ALL: [Category; 7] = [
        Category::Build,
        Category::TestSuite,
        Category::QuickBench,
        Category::Bench,
        Category::LeanBuild,
        Category::LeanOracle,
        Category::Experiment,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Category::Build => "Builds",
            Category::TestSuite => "Test suites",
            Category::QuickBench => "Quick benches",
            Category::Bench => "Benches",
            Category::LeanBuild => "Lean builds",
            Category::LeanOracle => "Lean oracle",
            Category::Experiment => "Experiments",
        }
    }
}

/// How a job uses threads: `threads = "setting"` or `threads = 8`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(try_from = "ThreadText")]
pub enum ThreadUse {
    /// It gets the global setting.
    #[default]
    Setting,
    /// It uses that many whatever the setting.
    Fixed(u32),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ThreadText {
    Count(u32),
    Word(String),
}

impl TryFrom<ThreadText> for ThreadUse {
    type Error = String;

    fn try_from(text: ThreadText) -> Result<Self, String> {
        match text {
            ThreadText::Count(0) => Err("threads = 0: a job uses at least one thread".to_string()),
            ThreadText::Count(count) => Ok(ThreadUse::Fixed(count)),
            ThreadText::Word(word) if word == "setting" => Ok(ThreadUse::Setting),
            ThreadText::Word(word) => Err(format!("threads = \"{word}\": \"setting\" or a number")),
        }
    }
}

/// Where a job runs (SPEC.md, §5): `checkout = "ask"`, Marc picking the
/// reference per queue item, or a reference, `"main"` when the field is absent.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "CheckoutText")]
pub enum CheckoutChoice {
    Ask,
    /// A tag, a branch, `pull/<n>` or a hash, resolved in the mirror.
    Reference(String),
}

impl Default for CheckoutChoice {
    fn default() -> Self {
        CheckoutChoice::Reference(crate::mirror::MAIN.to_string())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum CheckoutText {
    Word(String),
    /// The form of the catalogues before M7, read as the branch's name.
    Branch {
        branch: String,
    },
}

impl TryFrom<CheckoutText> for CheckoutChoice {
    type Error = String;

    fn try_from(text: CheckoutText) -> Result<Self, String> {
        match text {
            CheckoutText::Word(word) if word == "ask" => Ok(CheckoutChoice::Ask),
            CheckoutText::Word(word) if word.trim().is_empty() => {
                Err("checkout = \"\": \"ask\" or a reference (a tag, a branch, pull/<n>, a hash)".to_string())
            }
            CheckoutText::Word(word) => Ok(CheckoutChoice::Reference(word)),
            CheckoutText::Branch { branch } => Ok(CheckoutChoice::Reference(branch)),
        }
    }
}

impl CheckoutChoice {
    pub fn text(&self) -> String {
        match self {
            CheckoutChoice::Ask => "ask".to_string(),
            CheckoutChoice::Reference(reference) => reference.clone(),
        }
    }

    /// The reference an item of the queue starts with: `main` when Marc is to pick.
    pub fn initial(&self) -> String {
        match self {
            CheckoutChoice::Ask => crate::mirror::MAIN.to_string(),
            CheckoutChoice::Reference(reference) => reference.clone(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    #[serde(default, rename = "job")]
    jobs: Vec<Job>,
    #[serde(default, rename = "sequence")]
    sequences: Vec<Sequence>,
    /// Read by hand: `axes` taken out, the rest is a job.
    #[serde(default, rename = "batch")]
    batches: Vec<toml::Table>,
    /// Read by hand: `key`, `tools` and `provides` taken out, the rest is a job.
    #[serde(default, rename = "dependency")]
    dependencies: Vec<toml::Table>,
}

/// The catalogue as read, with what is wrong in it. A file that does not
/// parse is an error; a job that parses but breaks a rule is a problem,
/// listed on the screen, and the rest of the catalogue stays usable.
#[derive(Clone, Debug, Default)]
pub struct Catalogue {
    pub jobs: Vec<Job>,
    pub batches: Vec<Batch>,
    pub dependencies: Vec<Dependency>,
    pub sequences: Vec<Sequence>,
    pub problems: Vec<String>,
}

impl Catalogue {
    pub fn read(path: &Path) -> Result<Catalogue, String> {
        let text = std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
        Catalogue::parse(&text).map_err(|error| format!("{}: {error}", path.display()))
    }

    pub fn parse(text: &str) -> Result<Catalogue, String> {
        let file: File = toml::from_str(text).map_err(|error| error.to_string())?;
        let batches = file.batches.into_iter().map(Batch::from_table).collect::<Result<Vec<_>, _>>()?;
        let dependencies = file.dependencies.into_iter().map(Dependency::from_table).collect::<Result<Vec<_>, _>>()?;
        let mut catalogue =
            Catalogue { jobs: file.jobs, batches, dependencies, sequences: file.sequences, problems: Vec::new() };
        catalogue.problems = catalogue.check();
        Ok(catalogue)
    }

    pub fn job(&self, name: &str) -> Option<&Job> {
        self.jobs.iter().find(|job| job.name == name)
    }

    /// The jobs and the batches' templates, the jobs first: what the State
    /// screen and the repository's summaries go over.
    pub fn every_job(&self) -> impl Iterator<Item = &Job> {
        self.jobs.iter().chain(self.batches.iter().map(|batch| &batch.job))
    }

    pub fn batch(&self, name: &str) -> Option<&Batch> {
        self.batches.iter().find(|batch| batch.job.name == name)
    }

    pub fn dependency(&self, name: &str) -> Option<&Dependency> {
        self.dependencies.iter().find(|dependency| dependency.job.name == name)
    }

    fn check(&self) -> Vec<String> {
        let mut problems = Vec::new();
        let mut names = BTreeSet::new();
        for job in &self.jobs {
            if !names.insert(job.name.as_str()) {
                problems.push(format!("job {}: the name is used twice", job.name));
            }
            problems.extend(job.check().into_iter().map(|problem| format!("job {}: {problem}", job.name)));
        }
        for batch in &self.batches {
            if !names.insert(batch.job.name.as_str()) {
                problems.push(format!("batch {}: the name is used twice", batch.job.name));
            }
            problems.extend(batch.check().into_iter().map(|problem| format!("batch {}: {problem}", batch.job.name)));
            for need in &batch.needs {
                match self.dependency(need) {
                    None => {
                        problems.push(format!("batch {}: needs {need}, no dependency of that name", batch.job.name))
                    }
                    Some(dependency) => {
                        let has = batch.placeholders();
                        for wanted in dependency.wanted() {
                            if !has.contains(&wanted) {
                                problems.push(format!(
                                    "batch {}: the dependency {need} names {{{wanted}}}, which its cells lack",
                                    batch.job.name
                                ));
                            }
                        }
                    }
                }
            }
        }
        let mut dependency_names = BTreeSet::new();
        for dependency in &self.dependencies {
            let name = dependency.job.name.as_str();
            if names.contains(name) || !dependency_names.insert(name) {
                problems.push(format!("dependency {name}: the name is used twice"));
            }
            problems.extend(dependency.check().into_iter().map(|problem| format!("dependency {name}: {problem}")));
        }
        let mut sequence_names = BTreeSet::new();
        for sequence in &self.sequences {
            if !sequence_names.insert(sequence.name.as_str()) {
                problems.push(format!("sequence {}: the name is used twice", sequence.name));
            }
            for name in &sequence.jobs {
                if !names.contains(name.as_str()) {
                    problems.push(format!("sequence {}: no job named {name}", sequence.name));
                }
            }
        }
        problems
    }
}

impl Job {
    fn check(&self) -> Vec<String> {
        self.check_with(&BTreeSet::new(), &BTreeSet::new())
    }

    /// The rules of a job; `command_only` are the placeholders its command,
    /// directory and environment may name besides the built-in ones and the
    /// parameters, `everywhere` those its prepare may name as well.
    fn check_with(&self, command_only: &BTreeSet<String>, everywhere: &BTreeSet<String>) -> Vec<String> {
        let mut problems = Vec::new();
        if !is_name(&self.name) {
            problems.push("a name is lower-case letters, digits and '-'".to_string());
        }
        for parameter in self.parameters.keys() {
            if BUILT_IN.contains(&parameter.as_str()) {
                problems.push(format!("parameter {parameter} hides the built-in placeholder"));
            }
            if !is_name(parameter) {
                problems.push(format!("parameter {parameter}: lower-case letters, digits, '_' and '-'"));
            }
        }
        let mut texts: Vec<(&str, &str)> = vec![("command", &self.command), ("directory", &self.directory)];
        if let Some(prepare) = &self.prepare {
            texts.push(("prepare", prepare));
        }
        for value in self.environment.values() {
            texts.push(("environment", value));
        }
        for (field, text) in texts {
            for name in placeholders(text) {
                let known = BUILT_IN.contains(&name.as_str())
                    || self.parameters.contains_key(&name)
                    || everywhere.contains(&name)
                    || (field != "prepare" && command_only.contains(&name));
                if !known {
                    let why = if field == "prepare" && command_only.contains(&name) {
                        " (a prepare runs once per checkout, not per cell)"
                    } else {
                        ""
                    };
                    problems.push(format!("{field}: unknown placeholder {{{name}}}{why}"));
                }
            }
            if let Some(path) = local_path(text) {
                problems.push(format!("{field}: a local path ({path}): write it relative to the checkout"));
            }
        }
        if self.directory.starts_with('/') {
            problems.push("directory: relative to the checkout".to_string());
        }
        if let Some(summary) = &self.summary
            && (summary.starts_with('/') || local_path(summary).is_some())
        {
            problems.push(format!("summary: {summary} is not relative to the checkout"));
        }
        for output in &self.outputs {
            if output.starts_with('/') || local_path(output).is_some() {
                problems.push(format!("outputs: {output} is not relative to the checkout"));
            }
        }
        problems
    }

    /// The threads the job uses with this setting.
    pub fn effective_threads(&self, setting: u32) -> u32 {
        match self.threads {
            ThreadUse::Setting => setting,
            ThreadUse::Fixed(count) => count,
        }
    }

    /// The values of the placeholders: the built-in ones, the parameters'
    /// defaults, then `chosen` over them. `repo` left out keeps `{repo}`
    /// as it is, as a record writes it; `reference` is what the record's
    /// `checkout` says.
    pub fn values(
        &self,
        setting: u32,
        commit: &str,
        reference: &str,
        repo: Option<&str>,
        chosen: &BTreeMap<String, String>,
    ) -> BTreeMap<String, String> {
        let mut values = self.parameters.clone();
        values.extend(chosen.iter().map(|(name, value)| (name.clone(), value.clone())));
        values.insert("threads".to_string(), self.effective_threads(setting).to_string());
        values.insert("commit".to_string(), commit.to_string());
        values.insert("ref".to_string(), reference.to_string());
        values.insert("repo".to_string(), repo.unwrap_or("{repo}").to_string());
        values
    }

    /// The variables the app sets for this job (SPEC.md, section 7.1).
    pub fn environment_for(&self, values: &BTreeMap<String, String>) -> Result<BTreeMap<String, String>, String> {
        let threads = values.get("threads").cloned().unwrap_or_default();
        let mut environment: BTreeMap<String, String> =
            THREAD_VARIABLES.iter().map(|name| (name.to_string(), threads.clone())).collect();
        for (name, value) in &self.environment {
            environment.insert(name.clone(), render(value, values)?);
        }
        Ok(environment)
    }
}

impl Batch {
    /// A `[[batch]]` table: `axes` taken out, the rest read as a job; its
    /// timeout 30 minutes when it sets none.
    fn from_table(mut table: toml::Table) -> Result<Batch, String> {
        let name = table.get("name").and_then(|name| name.as_str()).unwrap_or("?").to_string();
        let wrong = |what: String| format!("batch {name}: {what}");
        let axes = match table.remove("axes") {
            None => Vec::new(),
            Some(toml::Value::Table(axes)) => axes
                .into_iter()
                .map(|(axis, values)| Axis::read(&axis, values).map_err(|what| wrong(format!("axis {axis}: {what}"))))
                .collect::<Result<Vec<_>, _>>()?,
            Some(_) => return Err(wrong("axes is a table, an axis and its values".to_string())),
        };
        let needs = strings(&mut table, "needs").map_err(wrong)?.unwrap_or_default();
        let skip_done = match table.remove("skip_done") {
            None => true,
            Some(toml::Value::Boolean(skip)) => skip,
            Some(_) => return Err(wrong("skip_done: true or false".to_string())),
        };
        let report = match table.remove("report") {
            None => None,
            Some(toml::Value::String(command)) => Some(command),
            Some(_) => return Err(wrong("report: a command, a string".to_string())),
        };
        let mut job: Job =
            toml::Value::Table(table).try_into().map_err(|error: toml::de::Error| wrong(error.to_string()))?;
        job.timeout.get_or_insert(Timeout { seconds: BATCH_TIMEOUT_SECONDS });
        Ok(Batch { job, axes, needs, report, skip_done })
    }

    /// Every placeholder a cell of the batch has: the built-in ones, `{out}`,
    /// the parameters and the axes.
    pub fn placeholders(&self) -> BTreeSet<String> {
        let mut has: BTreeSet<String> = BUILT_IN.iter().map(|name| name.to_string()).collect();
        has.insert(OUT.to_string());
        has.extend(self.job.parameters.keys().cloned());
        has.extend(self.axes.iter().map(|axis| axis.name.clone()));
        has
    }

    fn check(&self) -> Vec<String> {
        let mut problems = Vec::new();
        let mut command_only: BTreeSet<String> = [OUT.to_string()].into();
        for axis in &self.axes {
            if !is_name(&axis.name) {
                problems.push(format!("axis {}: lower-case letters, digits, '_' and '-'", axis.name));
            }
            if axis.name != REFERENCE_AXIS
                && (BUILT_IN.contains(&axis.name.as_str())
                    || axis.name == OUT
                    || self.job.parameters.contains_key(&axis.name))
            {
                problems.push(format!("axis {} hides a placeholder of the same name", axis.name));
            }
            if axis.values.is_empty() {
                problems.push(format!("axis {}: no value", axis.name));
            }
            let mut seen = BTreeSet::new();
            for value in &axis.values {
                if !seen.insert(value.value.as_str()) {
                    problems.push(format!("axis {}: the value {} is written twice", axis.name, value.value));
                }
                for key in value.placeholders.keys() {
                    if !self.job.parameters.contains_key(key) {
                        problems.push(format!(
                            "axis {}, value {}: {key} is not a parameter of the batch (give it a default in parameters)",
                            axis.name, value.value
                        ));
                    }
                }
                if axis.name == REFERENCE_AXIS && local_path(&value.value).is_some() {
                    problems.push(format!("axis ref: {} is a local path", value.value));
                }
            }
            command_only.insert(axis.name.clone());
        }
        if self.job.parameters.contains_key(OUT) {
            problems.push(format!("parameter {OUT} hides the built-in placeholder"));
        }
        for need in &self.needs {
            if command_only.contains(need)
                || self.job.parameters.contains_key(need)
                || BUILT_IN.contains(&need.as_str())
            {
                problems.push(format!("needs {need}: a placeholder of that name exists already"));
            }
            command_only.insert(need.clone());
        }
        problems.extend(self.job.check_with(&command_only, &BTreeSet::new()));
        if let Some(report) = &self.report {
            for name in placeholders(report) {
                let known = BUILT_IN.contains(&name.as_str())
                    || self.job.parameters.contains_key(&name)
                    || name == OUT
                    || name == RECORDS;
                if !known {
                    problems
                        .push(format!("report: unknown placeholder {{{name}}} (a report reads every cell: no axis)"));
                }
            }
            if let Some(path) = local_path(report) {
                problems.push(format!("report: a local path ({path}): write it relative to the checkout"));
            }
        }
        problems
    }

    pub fn axis(&self, name: &str) -> Option<&Axis> {
        self.axes.iter().find(|axis| axis.name == name)
    }

    /// The cells of the grid, the last axis varying fastest; `only` keeps,
    /// for an axis, the values given (any reference for `ref`).
    pub fn expand(&self, only: &[(String, Vec<String>)]) -> Result<Vec<CellPlan>, String> {
        let mut axes: Vec<Axis> = self.axes.clone();
        for (name, kept) in only {
            let Some(axis) = axes.iter_mut().find(|axis| &axis.name == name) else {
                let names: Vec<&str> = self.axes.iter().map(|axis| axis.name.as_str()).collect();
                return Err(format!("{} has no axis {name} (its axes: {})", self.job.name, names.join(", ")));
            };
            let mut values = Vec::new();
            for value in kept {
                match axis.values.iter().find(|known| &known.value == value) {
                    Some(known) => values.push(known.clone()),
                    None if axis.name == REFERENCE_AXIS => {
                        values.push(AxisValue { value: value.clone(), placeholders: BTreeMap::new() })
                    }
                    None => {
                        let known: Vec<&str> = axis.values.iter().map(|value| value.value.as_str()).collect();
                        return Err(format!("axis {name} has no value {value} (its values: {})", known.join(", ")));
                    }
                }
            }
            axis.values = values;
        }
        let mut cells = vec![CellPlan { values: Vec::new(), parameters: self.job.parameters.clone(), reference: None }];
        for axis in &axes {
            let mut next = Vec::with_capacity(cells.len() * axis.values.len());
            for cell in &cells {
                for value in &axis.values {
                    let mut cell = cell.clone();
                    cell.values.push((axis.name.clone(), value.value.clone()));
                    cell.parameters.extend(value.placeholders.clone());
                    if axis.name == REFERENCE_AXIS {
                        cell.reference = Some(value.value.clone());
                    } else {
                        cell.parameters.insert(axis.name.clone(), value.value.clone());
                    }
                    next.push(cell);
                }
            }
            cells = next;
        }
        Ok(cells)
    }
}

impl Axis {
    fn read(name: &str, values: toml::Value) -> Result<Axis, String> {
        let toml::Value::Array(values) = values else {
            return Err("a list of values".to_string());
        };
        let values = values
            .into_iter()
            .map(|value| match value {
                toml::Value::Table(mut table) => {
                    let value = table
                        .remove("value")
                        .and_then(|value| scalar(&value))
                        .ok_or_else(|| "a table value has its `value`, a string or a number".to_string())?;
                    let placeholders = table
                        .into_iter()
                        .map(|(key, value)| match scalar(&value) {
                            Some(text) => Ok((key, text)),
                            None => Err(format!("{key}: a string or a number")),
                        })
                        .collect::<Result<BTreeMap<_, _>, _>>()?;
                    Ok(AxisValue { value, placeholders })
                }
                other => scalar(&other)
                    .map(|value| AxisValue { value, placeholders: BTreeMap::new() })
                    .ok_or_else(|| "a value is a string, a number or a table".to_string()),
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Axis { name: name.to_string(), values })
    }
}

impl Dependency {
    /// A `[[dependency]]` table: `key`, `tools` and `provides` taken out,
    /// the rest read as a job.
    fn from_table(mut table: toml::Table) -> Result<Dependency, String> {
        let name = table.get("name").and_then(|name| name.as_str()).unwrap_or("?").to_string();
        let wrong = |what: String| format!("dependency {name}: {what}");
        let key = strings(&mut table, "key")
            .map_err(wrong)?
            .ok_or_else(|| wrong("key: the paths it depends on".to_string()))?;
        let tools = strings(&mut table, "tools").map_err(wrong)?.unwrap_or_default();
        let provides = match table.remove("provides") {
            Some(toml::Value::String(file)) => file,
            _ => return Err(wrong("provides: the file it makes, a string".to_string())),
        };
        let job: Job =
            toml::Value::Table(table).try_into().map_err(|error: toml::de::Error| wrong(error.to_string()))?;
        Ok(Dependency { job, key, tools, provides })
    }

    /// The placeholders it takes from the cell that needs it: those it names
    /// beyond the built-in ones, `{out}` and its parameters.
    pub fn wanted(&self) -> BTreeSet<String> {
        let mut texts: Vec<&str> = vec![&self.job.command, &self.job.directory];
        texts.extend(self.key.iter().map(String::as_str));
        texts.extend(self.job.environment.values().map(String::as_str));
        texts
            .into_iter()
            .flat_map(placeholders)
            .filter(|name| !BUILT_IN.contains(&name.as_str()) && name != OUT && !self.job.parameters.contains_key(name))
            .collect()
    }

    fn check(&self) -> Vec<String> {
        let mut problems = Vec::new();
        if self.job.prepare.is_some() {
            problems.push("a dependency has no prepare: its command does it all".to_string());
        }
        if self.job.summary.is_some() || !self.job.outputs.is_empty() {
            problems.push("a dependency has no summary nor outputs: its files are those of {out}".to_string());
        }
        if self.key.is_empty() {
            problems.push("key: at least one path".to_string());
        }
        for path in &self.key {
            if path.starts_with('/') || path.contains("..") || local_path(path).is_some() {
                problems.push(format!("key: {path} is not a path of the checkout"));
            }
        }
        if self.provides.is_empty() || self.provides.starts_with('/') || self.provides.contains("..") {
            problems.push(format!("provides: {} is not a file of its folder", self.provides));
        }
        let mut command_only = self.wanted();
        command_only.insert(OUT.to_string());
        problems.extend(self.job.check_with(&command_only, &BTreeSet::new()));
        problems
    }
}

/// A list of strings taken out of a table, None when absent.
fn strings(table: &mut toml::Table, field: &str) -> Result<Option<Vec<String>>, String> {
    match table.remove(field) {
        None => Ok(None),
        Some(toml::Value::Array(values)) => values
            .into_iter()
            .map(|value| match value {
                toml::Value::String(text) => Ok(text),
                _ => Err(format!("{field}: a list of strings")),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Some),
        Some(_) => Err(format!("{field}: a list of strings")),
    }
}

/// A string, a number or a boolean, as a placeholder's text.
fn scalar(value: &toml::Value) -> Option<String> {
    match value {
        toml::Value::String(text) => Some(text.clone()),
        toml::Value::Integer(number) => Some(number.to_string()),
        toml::Value::Float(number) => Some(number.to_string()),
        toml::Value::Boolean(flag) => Some(flag.to_string()),
        _ => None,
    }
}

fn is_name(text: &str) -> bool {
    !text.is_empty()
        && text.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && !text.starts_with('-')
}

/// The placeholders of a text, in order, each once.
pub fn placeholders(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (_, name, _) in spans(text) {
        if !found.contains(&name) {
            found.push(name);
        }
    }
    found
}

/// Each placeholder as (start, name, end), in bytes.
fn spans(text: &str) -> Vec<(usize, String, usize)> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'{' && (index == 0 || bytes[index - 1] != b'$') {
            let start = index;
            let mut end = index + 1;
            while end < bytes.len()
                && (bytes[end].is_ascii_lowercase() || bytes[end].is_ascii_digit() || b"_-".contains(&bytes[end]))
            {
                end += 1;
            }
            if end > start + 1 && end < bytes.len() && bytes[end] == b'}' {
                spans.push((start, text[start + 1..end].to_string(), end + 1));
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }
    spans
}

/// The text with its placeholders filled; an unknown one is an error.
pub fn render(text: &str, values: &BTreeMap<String, String>) -> Result<String, String> {
    let mut rendered = String::with_capacity(text.len());
    let mut last = 0;
    for (start, name, end) in spans(text) {
        let value = values.get(&name).ok_or_else(|| format!("unknown placeholder {{{name}}}"))?;
        rendered.push_str(&text[last..start]);
        rendered.push_str(value);
        last = end;
    }
    rendered.push_str(&text[last..]);
    Ok(rendered)
}

/// A path that names a home folder or `/tmp`, as `hygiene.yml` refuses them.
pub fn local_path(text: &str) -> Option<String> {
    let starts = ["/home/", "/Users/", "/tmp/", "C:\\Users"];
    for start in starts {
        if let Some(at) = text.find(start) {
            let rest = &text[at + start.len()..];
            let names_someone = start == "/tmp/" || rest.chars().next().is_some_and(|c| c.is_ascii_alphabetic());
            if names_someone || start.starts_with('C') {
                return Some(text[at..].split_whitespace().next().unwrap_or(start).to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(name, value)| (name.to_string(), value.to_string())).collect()
    }

    #[test]
    fn placeholders_leave_the_shell_alone() {
        let command = "for part in {0,1}; do echo ${HOME} {export} $1; done; echo {threads} {export}";
        assert_eq!(placeholders(command), ["export", "threads"]);
        let rendered = render(command, &values(&[("export", "e"), ("threads", "4")])).unwrap();
        assert_eq!(rendered, "for part in {0,1}; do echo ${HOME} e $1; done; echo 4 e");
        assert!(render("{nothing}", &values(&[])).is_err());
    }

    #[test]
    fn a_job_reads_with_its_defaults() {
        let catalogue = Catalogue::parse(
            r#"
            [[job]]
            name = "census"
            category = "experiment"
            command = "python3 census.py {export} --jobs {threads}"
            parameters = { export = "scratch/data-package/turns-of-a-t-mesh/male-body" }
            threads = 1
            environment = { HIGHS_THREADS = "1" }

            [[job]]
            name = "doubles"
            category = "lean-oracle"
            command = "echo {ref} {commit}"
            checkout = "worktree-borders"

            [[job]]
            name = "old-form"
            category = "lean-oracle"
            command = "true"
            checkout = { branch = "worktree-borders" }
            "#,
        )
        .unwrap();
        assert!(catalogue.problems.is_empty(), "{:?}", catalogue.problems);
        let census = &catalogue.jobs[0];
        assert_eq!(census.directory, ".");
        assert_eq!(census.checkout, CheckoutChoice::Reference("main".to_string()));
        assert_eq!(census.checkout.initial(), "main");
        assert_eq!(census.effective_threads(6), 1);
        let values = census.values(6, "9683d14", "main", None, &BTreeMap::new());
        assert_eq!(
            render(&census.command, &values).unwrap(),
            "python3 census.py scratch/data-package/turns-of-a-t-mesh/male-body --jobs 1"
        );
        let environment = census.environment_for(&values).unwrap();
        assert_eq!(environment["RAYON_NUM_THREADS"], "1");
        assert_eq!(environment["HIGHS_THREADS"], "1");
        let doubles = &catalogue.jobs[1];
        assert_eq!(doubles.checkout, CheckoutChoice::Reference("worktree-borders".to_string()));
        let values = doubles.values(6, "5187ab0", "worktree-borders", None, &BTreeMap::new());
        assert_eq!(render(&doubles.command, &values).unwrap(), "echo worktree-borders 5187ab0");
        assert_eq!(catalogue.jobs[2].checkout, doubles.checkout, "the form of before M7 still reads");
        assert_eq!(CheckoutChoice::Ask.initial(), "main");
    }

    #[test]
    fn a_job_that_breaks_a_rule_is_a_problem_not_an_error() {
        // Put together here: hygiene.yml refuses a home folder written in a tracked file.
        let home = ["", "home", "someone", "repo"].join("/");
        let catalogue = Catalogue::parse(&format!(
            r#"
            [[job]]
            name = "bad"
            category = "build"
            command = "cd {home} && make {{target}}"
            [[job]]
            name = "bad"
            category = "build"
            command = "true"
            [[sequence]]
            name = "night"
            jobs = ["bad", "missing"]
            "#
        ))
        .unwrap();
        let problems = catalogue.problems.join("\n");
        assert!(problems.contains("unknown placeholder {target}"), "{problems}");
        assert!(problems.contains(&format!("a local path ({home})")), "{problems}");
        assert!(problems.contains("the name is used twice"), "{problems}");
        assert!(problems.contains("no job named missing"), "{problems}");
    }

    #[test]
    fn a_wrong_field_is_an_error() {
        assert!(
            Catalogue::parse("[[job]]\nname = \"a\"\ncategory = \"build\"\ncommand = \"x\"\nthread = 2\n").is_err()
        );
        assert!(
            Catalogue::parse("[[job]]\nname = \"a\"\ncategory = \"build\"\ncommand = \"x\"\nthreads = \"all\"\n")
                .is_err()
        );
    }

    const GRID: &str = r#"
        [[batch]]
        name = "grid"
        category = "quick-bench"
        prepare = "make {target} on {ref} with {threads}"
        command = "remesh {model} {preset} {budget} {out} on {ref}"
        parameters = { preset = "organic", target = "addon" }
        threads = 1
        [batch.axes]
        model = ["creature", { value = "cage", preset = "hard-surface" }]
        budget = [2970, 3000]
        ref = ["main", "12a6675"]
        "#;

    #[test]
    fn a_batch_expands_its_axes_the_last_fastest() {
        let catalogue = Catalogue::parse(GRID).unwrap();
        assert!(catalogue.problems.is_empty(), "{:?}", catalogue.problems);
        let batch = catalogue.batch("grid").unwrap();
        assert_eq!(batch.job.timeout, Some(Timeout { seconds: 1800.0 }), "30 minutes when it sets none");
        let names: Vec<&str> = batch.axes.iter().map(|axis| axis.name.as_str()).collect();
        assert_eq!(names, ["model", "budget", "ref"], "the order written");
        let cells = batch.expand(&[]).unwrap();
        assert_eq!(cells.len(), 8);
        let texts: Vec<String> = cells.iter().map(CellPlan::text).collect();
        assert_eq!(texts[0], "model=creature budget=2970 ref=main");
        assert_eq!(texts[1], "model=creature budget=2970 ref=12a6675");
        assert_eq!(texts[2], "model=creature budget=3000 ref=main");
        assert_eq!(texts[7], "model=cage budget=3000 ref=12a6675");
        assert_eq!(cells[1].reference.as_deref(), Some("12a6675"));
        assert!(!cells[1].parameters.contains_key("ref"), "ref is the checkout, not a parameter");
        assert_eq!(cells[0].parameters["preset"], "organic");
        assert_eq!(cells[7].parameters["preset"], "hard-surface", "the value's own placeholder");
        assert_eq!(cells[7].parameters["budget"], "3000");
        let only = batch
            .expand(&[("model".to_string(), vec!["cage".to_string()]), ("ref".to_string(), vec!["v0.7.0".to_string()])])
            .unwrap();
        let texts: Vec<String> = only.iter().map(CellPlan::text).collect();
        assert_eq!(
            texts,
            ["model=cage budget=2970 ref=v0.7.0", "model=cage budget=3000 ref=v0.7.0"],
            "any reference for ref"
        );
        assert!(batch.expand(&[("model".to_string(), vec!["ogre".to_string()])]).is_err());
        assert!(batch.expand(&[("size".to_string(), vec!["1".to_string()])]).is_err());
    }

    #[test]
    fn a_batch_that_breaks_a_rule_is_a_problem() {
        let catalogue = Catalogue::parse(
            r#"
            [[batch]]
            name = "bad"
            category = "build"
            prepare = "build {model}"
            command = "run {model} {size} {missing}"
            parameters = { model = "x" }
            timeout = "10m"
            [batch.axes]
            model = ["a", "a"]
            size = [{ value = 1, colour = "red" }]
            "#,
        )
        .unwrap();
        let problems = catalogue.problems.join("\n");
        assert!(problems.contains("axis model hides a placeholder"), "{problems}");
        assert!(problems.contains("the value a is written twice"), "{problems}");
        assert!(problems.contains("colour is not a parameter"), "{problems}");
        assert!(problems.contains("unknown placeholder {missing}"), "{problems}");
        assert_eq!(catalogue.batch("bad").unwrap().job.timeout, Some(Timeout { seconds: 600.0 }));
        let prepare = Catalogue::parse(
            "[[batch]]\nname = \"b\"\ncategory = \"build\"\nprepare = \"build {size}\"\ncommand = \"run {size}\"\n[batch.axes]\nsize = [1, 2]\n",
        )
        .unwrap();
        assert!(prepare.problems.join("\n").contains("a prepare runs once per checkout"), "{:?}", prepare.problems);
        assert!(
            Catalogue::parse("[[batch]]\nname = \"b\"\ncategory = \"build\"\ncommand = \"x\"\ntimeout = \"ten\"\n")
                .is_err()
        );
    }

    #[test]
    fn timeouts_read_with_their_units() {
        assert_eq!(Timeout::parse("90s").unwrap().seconds, 90.0);
        assert_eq!(Timeout::parse("30m").unwrap().seconds, 1800.0);
        assert_eq!(Timeout::parse("1.5h").unwrap().seconds, 5400.0);
        for wrong in ["30", "m", "-1m", "0s", "2d", ""] {
            assert!(Timeout::parse(wrong).is_err(), "{wrong}");
        }
    }

    /// The catalogue of the repository itself.
    #[test]
    fn the_seeded_catalogue_has_no_problem() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("jobs.toml");
        let catalogue = Catalogue::read(&path).unwrap();
        assert!(catalogue.problems.is_empty(), "{:#?}", catalogue.problems);
        assert!(catalogue.jobs.len() > 30);
        for job in &catalogue.jobs {
            let values = job.values(4, "9683d14", "main", None, &BTreeMap::new());
            render(&job.command, &values).unwrap();
            job.environment_for(&values).unwrap();
        }
        // Every cell of every batch renders, its prepare too.
        for batch in &catalogue.batches {
            for cell in batch.expand(&[]).unwrap() {
                let reference = cell.reference.clone().unwrap_or_else(|| batch.job.checkout.initial());
                let mut values = batch.job.values(6, "12a6675", &reference, None, &cell.parameters);
                values.insert(OUT.to_string(), "{out}".to_string());
                for need in &batch.needs {
                    values.insert(need.clone(), format!("{{{need}}}"));
                    let dependency = catalogue.dependency(need).unwrap();
                    let mut own = dependency.job.values(6, "12a6675", &reference, None, &cell.parameters);
                    own.insert(OUT.to_string(), "{out}".to_string());
                    render(&dependency.job.command, &own).unwrap();
                    for path in &dependency.key {
                        render(path, &own).unwrap();
                    }
                }
                render(&batch.job.command, &values).unwrap();
                batch.job.environment_for(&values).unwrap();
            }
            if let Some(prepare) = &batch.job.prepare {
                render(prepare, &batch.job.values(6, "12a6675", "main", None, &batch.job.parameters)).unwrap();
            }
        }
        let tmesh = catalogue.batch("pole-pairs-115-tmesh").unwrap().expand(&[]).unwrap();
        assert_eq!(tmesh.len(), 72);
        assert_eq!(tmesh[1].parameters["pairs_flag"], "--no-pairs");
        assert_eq!(catalogue.batch("lean-oracle-seed-1").unwrap().expand(&[]).unwrap().len(), 50);
        // The witness of R1 (SPEC.md, §15): ticket #115, 72 jobs.
        let batch = catalogue.batch("pole-pairs-115-remesh").unwrap();
        let cells = batch.expand(&[]).unwrap();
        assert_eq!(cells.len(), 72);
        for cell in &cells {
            let mut values = batch.job.values(6, "12a6675", cell.reference.as_deref().unwrap(), None, &cell.parameters);
            values.insert(OUT.to_string(), "{out}".to_string());
            values.insert("engine".to_string(), "{engine}".to_string());
            values.insert("source".to_string(), "{source}".to_string());
            let command = render(&batch.job.command, &values).unwrap();
            let model = &cell.parameters["model"];
            let preset = if ["cage", "part"].contains(&model.as_str()) { "hard-surface" } else { "organic" };
            assert!(command.contains(&format!("--preset {preset} --quads {}", cell.parameters["budget"])), "{command}");
        }
        // R2: the witness needs the engine and the sources, and prepares nothing.
        assert_eq!(batch.needs, ["engine", "source"]);
        assert!(batch.job.prepare.is_none());
    }

    #[test]
    fn a_dependency_that_breaks_a_rule_is_a_problem() {
        let text = r#"
            [[dependency]]
            name = "thing"
            category = "build"
            command = "make {model} > {out}/thing"
            prepare = "true"
            key = ["/abs/path", "lab/{model}"]
            provides = "thing"

            [[batch]]
            name = "uses"
            category = "build"
            needs = ["thing", "nothing"]
            command = "cat {thing}"
            [batch.axes]
            budget = [1]
            "#;
        let catalogue = Catalogue::parse(text).unwrap();
        let problems = catalogue.problems.join("\n");
        assert!(problems.contains("dependency thing: a dependency has no prepare"), "{problems}");
        assert!(problems.contains("dependency thing: key: /abs/path is not a path of the checkout"), "{problems}");
        assert!(problems.contains("batch uses: needs nothing, no dependency of that name"), "{problems}");
        assert!(
            problems.contains("batch uses: the dependency thing names {model}, which its cells lack"),
            "{problems}"
        );
        assert_eq!(catalogue.dependency("thing").unwrap().wanted(), BTreeSet::from(["model".to_string()]));
        assert!(
            Catalogue::parse(
                "[[dependency]]\nname = \"x\"\ncategory = \"build\"\ncommand = \"true\"\nprovides = \"x\"\n"
            )
            .is_err(),
            "key is required"
        );
    }
}
