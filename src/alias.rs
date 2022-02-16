use crate::config::MamimiConfig;
use crate::python_version::PythonVersion;
use crate::symlink::{create_symlink_dir, remove_symlink_dir, shallow_read_symlink};
use crate::system_version;
use std::convert::TryInto;
use std::path::PathBuf;

pub fn create_alias(
    config: &MamimiConfig,
    common_name: &str,
    version: &PythonVersion,
) -> std::io::Result<()> {
    let aliases_dir = config.aliases_dir();
    std::fs::create_dir_all(&aliases_dir)?;

    let version_dir = version
        .installation_path(config)
        .ok_or_else(|| std::io::ErrorKind::from(std::io::ErrorKind::NotFound))?;
    let alias_dir = aliases_dir.join(common_name);

    if alias_dir.exists() {
        remove_symlink_dir(&alias_dir)?;
    }

    create_symlink_dir(&version_dir, &alias_dir)?;

    Ok(())
}

pub fn list_aliases(config: &MamimiConfig) -> std::io::Result<Vec<StroredAlias>> {
    let vec: Vec<_> = std::fs::read_dir(&config.aliases_dir())?
        .filter_map(Result::ok)
        .filter_map(|x| TryInto::<StroredAlias>::try_into(x.path().as_path()).ok())
        .collect();
    Ok(vec)
}

#[derive(Debug)]
pub struct StroredAlias {
    alias_path: PathBuf,
    destination_path: PathBuf,
}

impl std::convert::TryInto<StroredAlias> for &std::path::Path {
    type Error = std::io::Error;

    fn try_into(self) -> Result<StroredAlias, Self::Error> {
        let shallow_self = shallow_read_symlink(self)?;
        let destination_path = if shallow_self == system_version::path() {
            shallow_self
        } else {
            std::fs::canonicalize(&shallow_self)?
        };
        Ok(StroredAlias {
            alias_path: PathBuf::from(self),
            destination_path,
        })
    }
}

impl StroredAlias {
    pub fn s_ver(&self) -> &str {
        if self.destination_path == system_version::path() {
            system_version::display_name()
        } else {
            self.destination_path
                .parent()
                .unwrap()
                .file_name()
                .expect("must have basename")
                .to_str()
                .unwrap()
        }
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
