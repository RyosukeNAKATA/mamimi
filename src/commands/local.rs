use crate::alias::create_alias;
use crate::input_version::InputVersion;
use crate::python_version::PythonVersion;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}
