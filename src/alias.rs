use crate::config::MamimiConfig;
use crate::python_version::PythonVersion;
use crate::symlink::{create_symlink_dir, remove_symlink_dir};
use std::path::PathBuf;

pub fn create_alias(
    config: &MamimiConfig,
    common_name: &str,
    version: &PythonVersion,
) -> sid::io::Result<()> {
    let aliases_dir = config.aliases_dir();
    std::fs::create_dir_all(&aliases_dir)?;

    let version_dir = version
        .installation_path
        .ok_or_else(|| std::io::ErrorKind::from(std::io::ErrorKind::NotFound))?;
    let alias_dir = aliases_dir.join(common_name);

    if alias_dir.exists() {
        remove_symlink_dir(&alias_dir)?;
    }

    create_symlink_dir(&version_dir, &alias_dir)?;

    Ok(())
}

#[derive(Debug)]
pub struct StroredAlias {
    alias_path: PathBuf,
    destination_path: PathBuf,
}

impl std::convert::TryInto<StroredAlias> for &std::path::Path {
    type Error = std::io::Error;

    fn try_into(self) -> Result<StroredAlias, Self::Error> {
        let destination_path = std::fs::canonicalize(&self)?;
        Ok(StroredAlias {
            alias_path: PathBuf::from(self),
            destination_path,
        })
    }
}

impl StroredAlias {
    pub fn s_ver(&self) -> &str {
        self.destination_path
            .parent()
            .unwrap()
            .file_name()
            .expect("must have basename")
            .to_str()
            .unwrap()
    }

    pub fn name(&self) -> &str {
        self.alias_path
            .file_name()
            .expect("must have basename")
            .to_str()
            .unwrap()
    }

    pub fn path(&self) -> &std::path::Path {
        &self.alias_path
    }
}
