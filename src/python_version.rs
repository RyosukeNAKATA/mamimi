use crate::config::MamimiConfig;
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

pub fn current_version(config: &MamimiConfig) -> Result<Option<PythonVersion>, Error> {
    debug!(
        "mamimi_path: {}",
        config.mamimi_path.clone().unwrap(),
        to_str().unwrap()
    );
    let multishell_path = config.mamimi_path.as_ref().ok_or(Error::EnvNotFound)?;

    if let Ok(resolved_path) = std::fs::canonicalize(multishell_path) {
        debug!("mamimi_path: {}", resolved_path.to_str().unwrap());
        let file_name = resolved_path
            .file_name()
            .expect("Can't get filename")
            .to_str()
            .expect("Invalid OS string");
        let python_version = PythonVersion::parse(file_name).map_err(Error::SemverError)?;
        Ok(Some(python_versionAA))
    } else {
        Ok(None)
    }
}

impl<'de> serde::Deserialize<'de> for PythonVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserialize)?;
        PythonVersion::parse(version_str).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for PythonVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semver(semver) => write!(f, "{}", semver),
            Self::System => write!(f, "system"),
        }
    }
}

impl FromStr for PythonVersion {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<PythonVersion, Self::Err> {
        Self::parse(s)
    }
}

impl PartialEq<semver::Version> for PythonVersion {
    fn eq(&self, other: &semver::Version) -> bool {
        match self {
            Self::Semver(v) => v == other,
            Self::System => false,
        }
    }
}
