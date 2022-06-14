use crate::config::MamimiConfig;
use crate::current_python_version::current_python_version;
use crate::outln;
use crate::python_version::PythonVersion;
use colored::Colorize;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SemverError(#[from] semver::Error),
}

pub struct Versions {}

impl crate::commands::command::Command for Versions {
    type Error = MamimiError;

    fn apply(&self, config: &MamimiConfig) -> Result<(), Self::Error> {
        for entry in config.versions().read_dir().map_err(MamimiError::IoError)? {
            let entry = entry.map_err(MamimiError::IoError)?;
            if crate::python_version::is_dotfile(&entry) {
                continue;
            }

            let path = entry.path();
            let filename = path
                .file_name()
                .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                .map_err(MamimiError::IoError)?
                .to_str()
                .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                .map_err(MamimiError::IoError)?;
            let version = PythonVersion::parse(filename).map_err(MamimiError::SemverError)?;
            let current_python_version = current_python_version(&config).ok().flatten();
            debug!(
                "Current Python Version: {}",
                current_python_version.clone().unwrap()
            );
            if let Some(current_python_version) = current_python_version {
                if current_python_version == version {
                    outln!(
                        config,
                        Error,
                        "{} {}",
                        "*".green(),
                        version.to_string().green()
                    );
                } else {
                    outln!(config, Error, "{} {}", " ", version);
                }
            } else {
                outln!(config, Error, "{} {}", " ", version);
            };
        }
        Ok(())
    }
}
