use crate::alias;
use crate::config;
use crate::system_version;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum PythonVersion {
    Semver(semver::Version),
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
            Ok(Self::Bypassed)
        } else {
            Ok(Self::Alias(lowercased))
        }
    }

    pub fn alias_name(&self) -> Option<String> {
        match self {
            l @ (&Self::Lts(_) | &Self::Alias(_)) => Some(l.v_str()),
            _ => None,
        }
    }

    pub fn find_aliases(
        &self,
        config: &config::MamimiConfig,
    ) -> std::io::Result<Vec<alias::StroredAlias>> {
        let aliases = alias::list_aliases(config)?
            .drain(..)
            .filter(|alias| alias.s_ver() == self.v_str())
            .collect();
        Ok(aliases)
    }

    pub fn v_str(&self) -> String {
        format!("{}", self)
    }

    pub fn installation_path(&self, config: &crate::config::MamimiConfig) -> std::path::PathBuf {
        match self {
            v @ Self::Semver(_) => config
                .installations_dir()
                .join(v.v_str())
                .join("installation"),
        }
    }

    pub fn root_path(&self, config: &config::MamimiConfig) -> Option<std::path::PathBuf> {
        let path = self.installation_path(config);
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
            Self::Bypassed => write!(f, "{}", system_version::display_name()),
            Self::Lts(lts) => write!(f, "lts-{}", lts),
            Self::Semver(semver) => write!(f, "v{}", semver),
            Self::Alias(alias) => write!(f, "{}", alias),
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
            Self::Bypassed | Self::Lts(_) | Self::Alias(_) => false,
            Self::Semver(v) => v == other,
        }
    }
}
