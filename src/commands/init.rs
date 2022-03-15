use super::command::Command;
use crate::config::MamimiConfig;
use crate::directories;
use crate::outln;
use crate::path_ext::PathExt;
use crate::shell::{infer_shell, Shell, AVAILABLE_SHELLS};
use crate::symlink::create_symlink_dir;
use colored::Colorize;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MamimiError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't infer shell !")]
    CantInferShell,
}

#[derive(clap::Parser, Debug, Default)]
pub struct Init {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    #[clap(possible_values=AVAILABLE_SHELLS)]
    shell: Option<Box<dyn Shell>>,
    /// Deprecated. This is the default now.
    #[clap(long, hide = true)]
    multi: bool,
    /// Print the script to change Node versions every directory change
    #[clap(long)]
    use_on_cd: bool,
}

impl Command for Init {
    type Error = MamimiError;

    fn apply(&self, config: &crate::config::MamimiConfig) -> Result<(), Self::Error> {
        let shell: Box<dyn Shell> = infer_shell().ok_or(MamimiError::CantInferShell)?;
        let mamimi_path = create_symlink(&config);
        let binary_path = if cfg!(windows) {
            mamimi_path.clone()
        } else {
            mamimi_path.join("bin")
        };
        println!("{}", shell.path(&binary_path));
        println!(
            "{}",
            shell.set_env_var("MAMIMI_MULTISHELL_PATH", mamimi_path.to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("MAMIMI_DIR", config.base_dir().to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("MAMIMI_LOGLEVEL", config.log_level.clone().into())
        );
        //println!(
        //    "{}",
        //    shell.set_env_var("MAMIMI_PYTHON_BUILD_MIRROR", config.python_mirror.as_str())
        //);
        println!("{}", shell.use_on_cd(&config));
        Ok(())
    }
}

fn generate_symlink_path(root: &std::path::Path) -> std::path::PathBuf {
    let temp_dir_name = format!(
        "mamimi_{}_{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis()
    );
    root.join(temp_dir_name)
}

fn create_symlink(config: &crate::config::MamimiConfig) -> std::path::PathBuf {
    let system_temp_dir = std::env::temp_dir();
    let mut temp_dir = generate_symlink_path(&system_temp_dir);

    while temp_dir.exists() {
        temp_dir = generate_symlink_path(&system_temp_dir);
    }

    create_symlink_dir(config.default_python_version_dir(), &temp_dir)
        .expect("Can't create symlink");
    temp_dir
}
