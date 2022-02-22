use crate::arch::Arch;
use crate::log_level::LogLevel;
use crate::path_ext::PathExt;
use crate::version_file_strategy::VersionFileStrategy;
use dirs::{data_dir, home_dir};
use std::path::PathBuf;
use url::Url;

#[derive(clap::Parser, Debug)]
pub struct MamimiConfig {
    /// https://www.python.org/ftp/python/ mirror
    #[clap(
        long,
        env = "MAMIMI_PYTHON_FTP_MIRROR",
        default_value = "https://www.python.org/ftp/python/",
        global = true,
        hide_env_values = true
    )]
    pub python_ftp_mirror: Url,

    /// The root directory of mamimi installations.
    #[clap(
        long = "mamimi-dir",
        env = "MAMIMI_DIR",
        global = true,
        hide_env_values = true
    )]
    pub base_dir: Option<PathBuf>,

    /// This value will be automatically populated.
    /// 'mamimi env' in your shell profile. Read more about it using 'mamimi help env'
    #[clap(
        long,
        env = "MAMIMI_MULTISHELL_PATH",
        hide_env_values = true,
        hide = true
    )]
    multishell_path: Option<PathBuf>,

    /// The log level of mamimi commands
    #[clap(
        long,
        env = "MAMIMI_LOGLEVEL",
        default_value = "info",
        global = true,
        hide_env_values = true,
        possible_values = LogLevel::possible_values()
    )]
    log_level: LogLevel,

    /// Override the architecture of the installed Python binary.
    /// Default to arch of mamimi binary.
    #[clap(
        long,
        env = "MAMIMI_ARCH",
        default_value_t,
        global = true,
        hide_env_values = true,
        hide_default_value = true
    )]
    pub arch: Arch,

    /// A strategy for how to resolve the Python version.
    /// - `local`: use the local version of Python defined within the current directory
    #[clap(
        long,
        env = "MAMIMI_VERSION_FILE_STRATEGY",
        possible_values = VersionFileStrategy::possible_values(),
        default_value = "local",
        global = true,
        hide_env_values = true,
    )]
    version_file_strategy: VersionFileStrategy,
}

impl Default for MamimiConfig {
    fn default() -> Self {
        Self {
            python_ftp_mirror: Url::parse("https://www.python.org/ftp/python/").unwrap(),
            base_dir: None,
            multishell_path: None,
            log_level: LogLevel::default(),
            arch: Arch::default(),
            version_file_strategy: VersionFileStrategy::default(),
        }
    }
}

impl MamimiConfig {
    pub fn version_file_strategy(&self) -> &VersionFileStrategy {
        &self.version_file_strategy
    }

    pub fn multishell_path(&self) -> Option<&std::path::Path> {
        match &self.multishell_path {
            None => None,
            Some(v) => Some(v.as_path()),
        }
    }

    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    pub fn base_dir_with_default(&self) -> PathBuf {
        let user_pref = self.base_dir.clone();
        if let Some(dir) = user_pref {
            return dir;
        }

        let legacy = home_dir()
            .map(|dir| dir.join(".mamimi"))
            .filter(|dir| dir.exists());

        let modern = data_dir().map(|dir| dir.join(".mamimi"));

        if let Some(dir) = legacy {
            return dir;
        }

        modern
            .expect("Can't get data directory")
            .ensure_exists_silently()
    }

    pub fn installations_dir(&self) -> PathBuf {
        self.base_dir_with_default()
            .join("python-versinos")
            .ensure_exists_silently()
    }

    pub fn default_python_version_dir(&self) -> PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> PathBuf {
        self.base_dir_with_default()
            .join("aliases")
            .ensure_exists_silently()
    }

    #[cfg(test)]
    pub fn with_base_dir(mut self, base_dir: Option<PathBuf>) -> Self {
        self.base_dir = base_dir;
        self
    }
}
