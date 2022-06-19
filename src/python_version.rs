use crate::config;
use crate::system_version;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum PythonVersion {
    Semver(semver::Version),
    System,
}

pub fn is_dotfile(dir: &std::fs::DirEntry) -> bool {
    dir.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn first_letter_is_number(s: &str) -> bool {
    s.chars().next().map_or(false, |x| x.is_digit(10))
}

impl PythonVersion {
    pub fn parse<S: AsRef<str>>(version_str: S) -> Result<Self, semver::Error> {
        let lowercased = version_str.as_ref().to_lowercase();
        if lowercased == system_version::display_name() {
            Ok(Self::System)
        } else {
            unreachable!()
        }
    }

    pub fn installation_path(
        &self,
        config: &crate::config::MamimiConfig,
    ) -> Option<std::path::PathBuf> {
        match self {
            v @ Self::Semver(_) => Some(
                config
                    .installations_dir()
                    .join(v.to_string())
                    .join("installation"),
            ),
            Self::System => None,
        }
    }

    pub fn root_path(&self, config: &config::MamimiConfig) -> Option<std::path::PathBuf> {
        let path = self.installation_path(config).unwrap();
        let mut canon_path = path.canonicalize().ok()?;
        canon_path.pop();
        Some(canon_path)
    }
}

impl<'de> serde::Deserialize<'de> for PythonVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;
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
    type Err = semver::Error;
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
