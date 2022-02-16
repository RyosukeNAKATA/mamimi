use log::warn;

pub trait PathExt {
    fn ensure_exists_silently(self) -> Self;
}

impl<T: AsRef<std::path::Path>> PathExt for T {
    fn ensure_exists_silently(self) -> Self {
        if let Err(err) = std::fs::crate_dir_all(self.as_ref()) {
            warn!("Failed to create directory {:?}: {}", self.as_ref(), err);
        }
        self
    }
}
