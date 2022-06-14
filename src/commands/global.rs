use super::command::Command;
use crate::alias::create_alias;
use crate::commands::versions;
use crate::input_version::InputVersion;
use crate::python_version::PythonVersion;
use log::debug;
use reqwest::Version;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Requested version {version} is not currently installed")]
    VersionNotFound { version: InputVersion },
}

#[derive(clap::Parser, Debug)]
pub struct Global {
    pub version: InputVersion,
}

impl Command for Global {
    type Error = MamimiError;
    fn apply(&self, config: &crate::config::MamimiConfig) -> Result<(), Self::Error> {
        debug!("Use {} as the default version", &self.version);
        let version = match self.version.clone() {
            InputVersion::Full(PythonVersion::Semver(v)) => PythonVersion::Semver(v),
            version => return Err(MamimiError::VersionNotFound { version }),
        };
        if !&config
            .versions_dir()
            .join(self.version.to_string())
            .exists()
        {
            return Err(MamimiError::VersionNotFound {
                version: self.version.clone(),
            });
        }
        create_alias(&config, "default", &version).map_err(MamimiError::IoError)?;
        Ok(())
    }
}
