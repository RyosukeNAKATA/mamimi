use anyhow::Error;
use itertools::Itertools;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct InstallList {}

impl crate::command::Command for InstallList {
    type Error = MamimiError;

    fn apply(&self, config: &crate::config::MamimiError) -> Result<(), MamimiError> {
        let versions = crate::remote_python_index::list(&config.python_mirror)?;
    }
}
