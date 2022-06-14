use crate::config::MamimiConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::python_version::PythonVersion;
use crate::symlink::remove_symlink_dir;
use colored::Colorize;
use log::debug;
use std::ffi::OsStr;
use std::path::Component;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't find the number of cores.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Can't find version: {version}")]
    VersionNotFound { version: InputVersion },
    #[error("The reqwested version is not installable: {version}")]
    NotInstallableVersion { version: PythonVersion },
    #[error("We can't find the necessary envitonment to replace the Python version.")]
    MamimipathNotFound,
}

#[derive(clap::Parser, Debug)]
pub struct Uninstall {
    version: InputVersion,
}

impl crate::commands::command::Command for Uninstall {
    type Error = MamimiError;

    fn apply(self, config: &MamimiConfig) -> Result<(), Self::Error> {
        let current_version = self.version.clone();
        let version = match current_version.clone() {
            InputVersion::Full(PythonVersion::Semver(v)) => PythonVersion::Semver(v),
            InputVersion::Full(PythonVersion::System) => {
                return Err(MamimiError::NotInstallableVersion {
                    version: PythonVersion::System,
                })
            }
            _ => unreachable!(),
        };
        let installation_dir = PathBuf::from(&config.versions_dir()).join(version.to_string());
        if !installation_dir.exists() {
            return Err(MamimiError::VersionNotFound {
                version: current_version,
            });
        }
        outln!(
            config,
            Error,
            "{} Uninstalling {}",
            "==>".green(),
            format!("Python {}", current_version).green()
        );
        if symlink_exists(
            config
                .mamimi_path
                .clone()
                .ok_or(MamimiError::MamimipathNotFound)?,
            &version,
        )? {
            debug!("remove mamimi path symlink");
            remove_symlink_dir(
                &config
                    .mamimi_path
                    .clone()
                    .ok_or(MamimiError::MamimipathNotFound)?,
            )?;
        }
        if symlink_exists(config.default_python_version_dir(), &version)? {
            debug!("rmeove default alias symlink");
            remove_symlink_dir(&config.default_python_version_dir())?;
        }
        debug!("remove dir");
        std::fs::remove_dir_all(&installation_dir)?;
        Ok(())
    }
}

fn symlink_exists(to: PathBuf, version: &PythonVersion) -> Result<bool, MamimiError> {
    debug!("symlink exists?");
    Ok(std::fs::read_link(to)?.components().last()
        == Some(Component::Normal(OsStr::new(&version.to_string()))))
}
