use crate::config::MamimiConfig;
use crate::python_version::PythonVersion;
use thiserror::Error;

pub fn current_python_version(config: &MamimiConfig) -> Result<Option<PythonVersion>, Error> {
    let multishell_path = config.multishell_path().ok_or(Error::EnvNotApplied)?;

    if let Ok(resolved_path) = std::fs::canonicalize(multishell_path) {
        let installation_path = resolved_path
            .parent()
            .expect("multishell path can't be in the root");
        let file_name = installation_path
            .file_name()
            .expect("Can't get file name")
            .to_str()
            .expect("Invalid OS string");
        let version = PythonVersion::parse(file_name).map_err(|source| Error::VersionError {
            source,
            version: file_name.to_string(),
        })?;
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "`mamimi env` was not applied in this context\nCan't find mamimi's environment variables"
    )]
    EnvNotApplied,
    #[error("Can't read the version as a valid semver")]
    VersionError {
        source: semver::Error,
        version: String,
    },
}
