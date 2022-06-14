use itertools::Itertools;
use thiserror::Error;

use crate::config;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct InstallList {}

impl crate::commands::command::Command for InstallList {
    type Error = MamimiError;

    fn apply(self, config: &crate::config::MamimiConfig) -> Result<(), MamimiError> {
        let versions = crate::remote_python_index::list()?;
        let versions = versions
            .into_iter()
            .map(|v| v.python_version)
            .sorted()
            .dedup();
        for version in versions {
            crate::outln!(config, Error, "{}", version);
        }
        Ok(())
    }
}
