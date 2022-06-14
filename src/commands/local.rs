use crate::input_version::InputVersion;
use crate::symlink::{create_symlink_dir, remove_symlink_dir};
use crate::version_files::get_user_version_for_directory;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("We can't find the necessary environment variables to replace the Ruby version.")]
    MamimiPathNotFound,
    #[error("Requested version {version} is not currently installed")]
    VersionNotFound { version: InputVersion },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CannotInferVersion,
}

#[derive(clap::Parser, Debug)]
pub struct Local {
    pub version: Option<InputVersion>,
}

impl crate::commands::command::Command for Local {
    type Error = MamimiError;

    fn apply(self, config: &crate::config::MamimiConfig) -> Result<(), Self::Error> {
        debug!("log level {:?}", config.log_level());
        let current_python_version = match self.version.clone().ok_or_else(|| {
            match get_user_version_for_directory(std::env::current_dir().unwrap()) {
                Some(version) => Ok(version),
                None => {
                    replace_symlink(
                        &config.default_python_version_dir(),
                        &config
                            .mamimi_path
                            .clone()
                            .ok_or(MamimiError::MamimiPathNotFound)?,
                    )?;
                    Err(MamimiError::CannotInferVersion)
                }
            }
        }) {
            Ok(version) => version,
            Err(result) => result?,
        };
        debug!("Use {} as the current version", current_python_version);
        if !&config
            .versions_dir()
            .join(current_python_version.to_string())
            .exists()
        {
            return Err(MamimiError::VersionNotFound {
                version: current_python_version,
            });
        }
        replace_symlink(
            &config
                .versions_dir()
                .join(current_python_version.to_string()),
            &config
                .mamimi_path
                .clone()
                .ok_or(MamimiError::MamimiPathNotFound)?,
        )
        .map_err(MamimiError::IoError)?;
        Ok(())
    }
}

fn replace_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    let symlink_deletion_result = remove_symlink_dir(&to);
    match create_symlink_dir(&from, &to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
    }
}
