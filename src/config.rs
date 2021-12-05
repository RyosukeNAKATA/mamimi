use std::path::PathBuf;

#[derive(Debug)]
pub struct MamimiConfig {
    pub base_dir: Option<PathBuf>,
    pub mamimi_path: Option<PathBuf>,
}

impl Default for MamimiConfig {
    fn default() -> Self {
        Self {
            base_dir: std::env::var("MAMIMI_DIR")
                .map(std::path::PathBuf::from)
                .ok(),
            mamimi_path: std::env::var("MAMIMI_MULTISHELL_PATH")
                .map(std::path::PathBuf::from)
                .ok(),
        }
    }
}

impl MamimiConfig {
    pub fn base_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists((self.base_dir.clone()).unwrap_or_else(|| {
            dirs::home_dir()
                .expect("Can't get home directory")
                .join(".mamimi")
        }))
    }

    pub fn python_version_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("versions"))
    }

    pub fn default_python_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("aliases"))
    }
}

fn ensure_dir_exists<T: AsRef<std::path::Path>>(path: T) -> T {
    std::fs::create_dir_all(path.as_ref()).ok();
    path
}
