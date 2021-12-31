use crate::alias::create_alias;
use crate::archive::{self, extract::Error as ExtractError, extract::Extract};
use crate::config::MamimiConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::python_version::{current_python_version, PythonVersion};
use crate::version_file::get_user_version_for_directory;
use anyhow::Result;
use colored::Colorize;
use dirs::config_dir;
use log::debug;
use reqwest::Url;
use std::error;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't find the number of cores")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Can't extract the file: {source:?}")]
    ExtractError { source: ExtractError },
    #[error("The downloaded archive is empty")]
    TarIsEmpty,
    #[error("Can't find version")]
    VersionNotFound { version: InputVersion },
    #[error("Can't list the remote versions: {source:?}")]
    CannotListRemoteVersions { source: reqwest::Error },
    #[error("Version already installed at {path:?}")]
    VersionAlreadyInstalled { path: PathBuf },
    #[error(
        "Cannnot find the version in dotfiles. Please provide a version manually to the command."
    )]
    CannotInferVersion,
    #[error("The requested version is not installable: {version}")]
    NotInstallableVerison { version: PythonVersion },
    #[error("Cannot build Python: {stderr}")]
    CannotBuildPython { stderr: String },
}

pub struct Install {
    pub version: Option<InputVersion>,
    pub configure_opts: Vec<String>,
}
impl crate::command::Command for Install {
    type Error = MamimiError;
    fn apply(&self, config: &MamimiConfig) -> Result<(), Self::Error> {
        let current_version = self
            .version
            .clone()
            .or_else(|| get_user_version_for_directory(std::env::current_dir().unwrap()))
            .ok_or(MamimiError::CannotInferVersion)?;
        let version = match current_version.clone() {
            InputVersion::Full(PythonVersion::Semver(v)) => PythonVersion::Semver(v),
            InputVersion::Full(PythonVersion::System) => {
                return Err(MamimiError::NotInstallableVerison {
                    version: PythonVersion::System,
                })
            }
            current_version => {
                let avalable_versions = crate::remote_python_index::list()
                    .map_err(|source| MamimiError::CannotListRemoteVersions { source })?
                    .drain(..)
                    .map(|x| x.python_version)
                    .collect::<Vec<_>>();
                current_version
                    .to_version(&avalable_versions)
                    .ok_or(MamimiError::VersionNotFound {
                        version: current_version,
                    })?
                    .clone()
            }
        };
        let installations_dir = config.python_versions_dir();
        let installation_dir = PathBuf::from(&installations_dir).join(version.to_string());
    }
}

fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: reqwest::blocking::Response,
) -> Result<(), MamimiError> {
    #[cfg(unix)]
    let extractor = archive::tar_xz::TarXz::new(response);
    #[cfg(windows)]
    let extractor = archive::tar_xz::TarXz::new(response);
    extractor
        .extract_into(path)
        .map_err(|source| MamimiError::ExtractError { source })?;
    Ok(())
}
