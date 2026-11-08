//! The catalogue of jobs, `lab/jobs.toml` (SPEC.md, section 6).
//!
//! A placeholder is `{name}` with a name of lower-case letters, digits, `_`
//! and `-`: `{threads}`, `{repo}`, `{commit}`, `{ref}` and the job's own
//! parameters.
//! `${...}` is the shell's and `{a,b}` is not a name: both are left as they
//! are.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;

use crate::model::Paper;

/// The placeholders every job has.
pub const BUILT_IN: [&str; 4] = ["threads", "repo", "commit", "ref"];

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
}

/// The catalogue as read, with what is wrong in it. A file that does not
/// parse is an error; a job that parses but breaks a rule is a problem,
/// listed on the screen, and the rest of the catalogue stays usable.
#[derive(Clone, Debug, Default)]
pub struct Catalogue {
    pub jobs: Vec<Job>,
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
        let mut catalogue = Catalogue { jobs: file.jobs, sequences: file.sequences, problems: Vec::new() };
        catalogue.problems = catalogue.check();
        Ok(catalogue)
    }

    pub fn job(&self, name: &str) -> Option<&Job> {
        self.jobs.iter().find(|job| job.name == name)
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
                if !BUILT_IN.contains(&name.as_str()) && !self.parameters.contains_key(&name) {
                    problems.push(format!("{field}: unknown placeholder {{{name}}}"));
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
    }
}
