//! `settings.toml` in the config folder (SPEC.md, section 12): what Marc
//! tunes, kept on this machine. A missing file is written with the defaults;
//! a missing key takes its default.

use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    /// The repository the mirror is fetched from: `owner/name` on GitHub, or a URL.
    pub remote: String,
    /// The name of this machine (SPEC.md, §4b): its branch `lab/runs-<machine>`
    /// in the repository. The hostname by default (Q17).
    pub machine: String,
    /// When the live memory is pushed to that branch (§4b).
    pub push: Push,
    /// The global thread setting of the top bar.
    pub threads: u32,
    /// Above it the queue warns, never blocks (SPEC.md, section 7.1).
    pub comfort_threads: u32,
    pub sample_seconds: u32,
    /// Between SIGTERM and SIGKILL.
    pub stop_grace_seconds: u32,
    pub on_stop: Policy,
    pub on_failure: Policy,
    /// The hardware line of the exports, checked against what is detected.
    pub machine_label: String,
    pub calm: CalmSettings,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CalmSettings {
    pub window_seconds: u32,
    /// Percent of the whole machine.
    pub foreign_cpu_mean_max: f64,
    pub foreign_cpu_peak_max: f64,
    pub load1_max: f64,
    /// After a run of ours ends, load1 is not looked at for this long.
    pub load1_ignored_minutes: u32,
    /// The processors the jobs run on, pinned, as `1-3,5-7` (SPEC.md, §7.5):
    /// the calm is then judged on them alone. Empty: every processor, no pinning.
    pub job_cpus: String,
    /// Whether lab keeps the desktop off the job processors while it runs
    /// something, and gives it every processor back after (SPEC.md, §7.5).
    pub pin_desktop: bool,
    /// Whether a job whose timing matters waits at the head of the queue for a
    /// calm machine. Off by default: it starts at once, and its time does not
    /// count when the machine was not calm (SPEC.md, §7.2).
    pub wait: bool,
}

/// When the live memory is pushed to the machine's branch (SPEC.md, §4b):
/// on Marc's click, after every run, or never.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Push {
    #[default]
    Ask,
    Auto,
    Never,
}

/// What happens to the rest of the queue after a stop or a failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Policy {
    PauseAndAsk,
    Continue,
    CancelTheRest,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            remote: crate::mirror::DEFAULT_REMOTE.to_string(),
            machine: crate::machine::hostname(),
            push: Push::Ask,
            threads: 4,
            comfort_threads: 6,
            sample_seconds: 2,
            stop_grace_seconds: 10,
            on_stop: Policy::PauseAndAsk,
            on_failure: Policy::PauseAndAsk,
            machine_label: "i7-7700K 4c/8t 2017, 62 GB".to_string(),
            calm: CalmSettings::default(),
        }
    }
}

impl Default for CalmSettings {
    fn default() -> Self {
        CalmSettings {
            window_seconds: 60,
            foreign_cpu_mean_max: 5.0,
            foreign_cpu_peak_max: 25.0,
            load1_max: 1.0,
            load1_ignored_minutes: 5,
            job_cpus: String::new(),
            pin_desktop: true,
            wait: false,
        }
    }
}

impl Settings {
    /// The settings in `path`, written there with the defaults if missing.
    pub fn load(path: &Path) -> Result<Settings, String> {
        match std::fs::read_to_string(path) {
            Ok(text) => toml::from_str(&text).map_err(|error| format!("{}: {error}", path.display())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let settings = Settings::default();
                settings.save(path)?;
                Ok(settings)
            }
            Err(error) => Err(format!("{}: {error}", path.display())),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let text = toml::to_string_pretty(self).map_err(|error| error.to_string())?;
        crate::store::write_atomically(path, text.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_key_takes_its_default() {
        let settings: Settings = toml::from_str("threads = 6\n[calm]\nload1_max = 0.5\n").unwrap();
        assert_eq!(settings.threads, 6);
        assert_eq!(settings.comfort_threads, 6);
        assert_eq!(settings.remote, "marcgardent/openclare");
        assert!(!settings.machine.is_empty(), "the hostname");
        assert_eq!(settings.push, Push::Ask);
        assert_eq!(toml::from_str::<Settings>("push = \"auto\"\nmachine = \"night\"\n").unwrap().push, Push::Auto);
        assert_eq!(settings.calm.load1_max, 0.5);
        assert_eq!(settings.calm.window_seconds, 60);
        assert!(!settings.calm.wait, "a timed job starts at once unless asked to wait");
        assert!(toml::from_str::<Settings>("[calm]\nwait = true\n").unwrap().calm.wait);
        assert!(toml::from_str::<Settings>("thread = 6\n").is_err());
    }

    #[test]
    fn settings_read_back_as_written() {
        let settings = Settings { on_failure: Policy::Continue, ..Settings::default() };
        let text = toml::to_string_pretty(&settings).unwrap();
        assert!(text.contains("on_failure = \"continue\""), "{text}");
        assert_eq!(toml::from_str::<Settings>(&text).unwrap(), settings);
    }
}
