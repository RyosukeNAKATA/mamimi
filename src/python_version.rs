use anyhow::Error;
use log::debug;
use std::str::FromStr;

#[derive(Dubug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum PythonVersion {
    Semver(semver::Version),
    System,
}
fn start_with_number(s: &str) -> bool {
    s.chars().next().map(|x| x.is_digit(10)).unwrap_or(false)
}

impl PythonVersion {
    pub fn parse<S: AsRef<str>>(version_str: S) -> Result<Self, semver::SemVerError> {
        let lowercased = version_str.as_ref().to_lowercase();
        let trimed_lowercased = lowercased.trim_start_matches("python-");
        debug!("{}", trimed_lowercased);
        if lowercased == "system" {
            Ok(Self::System)
        } else if start_with_number(trimed_lowercased) {
            Ok(Self::Semver(semver::Version::parse(&trimed_lowercased)?))
        } else {
            unreachable!()
        }
    }

    pub fn installation_path(
        &self,
        config: &crate::config::MamimiConfig,
    ) -> Option<std::path::PathBuf> {
        match self {
            v @ Self::Semver(_) => Some(config.version_dir().join(v.to_string())),
            Self::system => None,
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("mamimi path doesn't exist")]
    EnvNotFound,
    #[error(transparent)]
    SemverError(#[from] semver::SemVerError),
}
