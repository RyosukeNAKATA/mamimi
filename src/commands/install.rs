use crate::alias::create_alias;
use crate::archive::{self, extract::Error as ExtractError, extract::Extract};
use crate::config::MamimiConfig;
use crate::current_python_version::current_python_version;
use crate::input_version::InputVersion;
use crate::outln;
use crate::python_version::PythonVersion;
use crate::version_files::get_user_version_for_directory;
use anyhow::Result;
use colored::Colorize;
use dirs::config_dir;
use log::debug;
use num_cpus;
use reqwest::Url;
use std::env::current_dir;
use std::error;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile;
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

#[derive(clap::Parser, Debug, Default)]
pub struct Install {
    pub version: Option<InputVersion>,
    pub configure_opts: Vec<String>,
}

impl crate::commands::command::Command for Install {
    type Error = MamimiError;

    fn apply(self, config: &MamimiConfig) -> Result<(), Self::Error> {
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
        let installations_dir = config.versions_dir();
        let installation_dir = PathBuf::from(&installations_dir).join(version.to_string());

        if installation_dir.exists() {
            return Err(MamimiError::VersionAlreadyInstalled {
                path: installation_dir,
            });
        }

        let url = package_url(&version);
        outln!(
            config,
            Error,
            "{} Downloading {}",
            "==>".green(),
            format!("{}", url).green()
        );
        let response = reqwest::blocking::get(url.clone())?;
        if response.status() == 404 {
            return Err(MamimiError::VersionNotFound {
                version: current_version,
            });
        }

        outln!(
            config,
            Error,
            "{} Extracting {}",
            "==>".green(),
            format!("{}", url).green()
        );
        let tmp_installations_dir = installations_dir.join(".downloads");
        std::fs::create_dir_all(&tmp_installations_dir).map_err(MamimiError::IoError)?;
        let tmp_dir = tempfile::TempDir::new_in(&tmp_installations_dir)
            .expect("Cannot generate a temp directory");
        extract_archive_into(&tmp_dir, response)?;

        outln!(
            config,
            Error,
            "{} Building {}",
            "==>".green(),
            format!("Python {}", current_version).green()
        );
        let installed_directory = std::fs::read_dir(&tmp_dir)
            .map_err(MamimiError::IoError)?
            .next()
            .ok_or(MamimiError::TarIsEmpty)?
            .map_err(MamimiError::IoError)?;
        let installed_directory = installed_directory.path();
        build_package(
            &installed_directory,
            &installation_dir,
            &self.configure_opts,
        )?;

        if !config.default_python_version_dir().exists() {
            debug!("Use {} as the default Python version", current_version);
            create_alias(&config, "default", &version).map_err(MamimiError::IoError)?;
        }
        Ok(())
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

#[cfg(unix)]
fn package_url(version: &PythonVersion) -> Url {
    debug!("package url");
    Url::parse(&format!(
        "https://www.python.org/ftp/python/{}/Python-{}.tar.xz",
        version, version
    ))
    .unwrap()
}

#[cfg(windows)]
fn package_url(version: &PythonVersion) -> Url {
    debug!("package url");
    Url::parse(&format!(
        "https://www.python.org/ftp/python/{}/Python-{}-embed-amd64.zip",
        version, version
    ))
    .unwrap()
}

#[cfg(unix)]
fn archive(version: &PythonVersion) -> String {
    format!("python-{}.tar.xz", version)
}

#[cfg(windows)]
fn archive(version: &PythonVersion) -> String {
    format!("python-{}.zip", version)
}

#[allow(clippy::unnecessary_wraps)]
fn openssl_dir() -> Result<String, MamimiError> {
    #[cfg(target_os = "macos")]
    return Ok(String::from_utf8_lossy(
        &Command::new("brew")
            .arg("--prefix")
            .arg("openssl@1.1")
            .output()
            .map_err(MamimiError::IoError)?
            .stdout,
    )
    .trim()
    .to_string());
    #[cfg(not(target_os = "macos"))]
    return Ok("/url/local".to_string());
}

fn build_package(
    current_dir: &Path,
    installed_dir: &Path,
    configure_opts: &[String],
) -> Result<(), MamimiError> {
    debug!("./configure {}", configure_opts.join(" "));
    let mut command = Command::new("sh");
    command
        .arg("configure")
        .arg(format!("--prefix={}", installed_dir.to_str().unwrap()))
        .args(configure_opts);

    // Provide a default value for --with-openssl-dir
    if !configure_opts
        .iter()
        .any(|opt| opt.starts_with("--with-openssl-dir"))
    {
        command.arg(format!("--with-openssl-dir={}", openssl_dir()?));
    }

    let configure = command
        .current_dir(&current_dir)
        .output()
        .map_err(MamimiError::IoError)?;
    if !configure.status.success() {
        return Err(MamimiError::CannotBuildPython {
            stderr: format!(
                "configure failed: {}",
                String::from_utf8_lossy(&configure.stderr).to_string()
            ),
        });
    };
    debug!("make -j {}", num_cpus::get().to_string());
    let make = Command::new("make")
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .current_dir(&current_dir)
        .output()
        .map_err(MamimiError::IoError)?;
    if !make.status.success() {
        return Err(MamimiError::CannotBuildPython {
            stderr: format!(
                "make failed: {}",
                String::from_utf8_lossy(&make.stderr).to_string()
            ),
        });
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::command::Command;
    use crate::config::MamimiConfig;
    use crate::python_version::PythonVersion;
    use itertools::Itertools;
    use tempfile::tempdir;

    #[test]
    fn test_install_second_version() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = MamimiConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));

        Install {
            version: Some(InputVersion::Full(PythonVersion::Semver(
                semver::Version::parse("3.9.6").unwrap(),
            ))),
            configure_opts: vec![],
        }
        .apply(&config)
        .expect("Can't install Python3.9.6");

        assert_eq!(
            std::fs::read_link(&config.default_python_version_dir())
                .unwrap()
                .components()
                .last(),
            Some(std::path::Component::Normal(std::ffi::OsStr::new("3.9.6")))
        );
    }

    #[test]
    fn test_install_default_python_version() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = MamimiConfig::default().with_base_dir(Some(base_dir.path().to_path_buf()));

        Install {
            version: Some(InputVersion::Full(PythonVersion::Semver(
                semver::Version::parse("3.9.6").unwrap(),
            ))),
            configure_opts: vec![],
        }
        .apply(&config)
        .expect("Can't insatll");

        assert!(config.installations_dir().join("3.9.6").exists());
        assert!(config
            .installations_dir()
            .join("3.9.6")
            .join("bin")
            .join("python3")
            .exists());
        assert!(config.default_python_version_dir().exists());
    }
}
