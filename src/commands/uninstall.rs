use crate::config::MamimiConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::python_version::PythonVersion;
use crate::symlink::remove_symlink_dir;
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

pub struct Uninstall {
    version: Option<InputVersion>,
}

impl super::command::Command for Uninstall {
    type Error = MamimiError;

    fn apply(&self, &config: MamimiConfig) -> Result<(), Self::Error> {}
}
