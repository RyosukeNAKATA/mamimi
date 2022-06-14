use crate::shell::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct WindowsCommand;

impl Shell for WindowsCommand {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        panic!("Shell completion is not supported for Windows Command Promt. Could you try to use PowerShell for a better experience?");
    }
    fn path(&self, path: &Path) -> anyhow::Result<String> {
        let current_path = std::env::var_os("path").expect("Can't read Path env var");
        let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
        split_paths.insert(0, path.to_path_buf());
        let new_path = std::env::join_paths(split_paths).expect("Can't join paths");
        Ok(format!(
            "SET PATH={}",
            new_path.to_str().expect("Cant't read PATH")
        ))
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("SET {}={}", name, value)
    }

    fn use_on_cd(&self, config: &crate::config::MamimiConfig) -> String {
        let path = config.base_dir_with_default().join("cd.cmd");
        create_cd_file_at(&path).expect("Can't create cd.cmd file for use-on-cd");
        format!(
            "doskey cd={} $*",
            path.to_str().expect("Cant't read path to cd.cmd")
        )
    }
}

fn create_cd_file_at(path: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    let cmd_contents = include_bytes!("./cd.cmd");
    let mut file = std::fs::File::create(path)?;
    file.write_all(cmd_contents)?;
    Ok(())
}
