pub mod bash;
pub mod fish;
pub mod infer;
pub mod windows_command;
pub mod zsh;

use std::fmt::Debug;
use std::path::Path;

pub use bash::Bash;
pub use fish::Fish;
pub use windows_command::WindowsCommand;
pub use zsh::Zsh;
pub trait Shell: Debug {
    fn path(&self, path: &Path) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::MamimiConfig) -> String;
    // fn as_clap_shell(&self) -> clap::Shell;
}

#[cfg(windows)]
pub const AVALABLE_SHELLS: &[&str; 5] = &["cmd", "powershell", "bash", "zsh", "fish"];

#[cfg(unix)]
pub const AVALABLE_SHELLS: &[&str; 4] = &["powershell", "bash", "zsh", "fish"];

#[cfg(windows)]
pub fn infer_shell() -> Option<Box<dyn Shell>> {
    self::infer::windows::infer_shell()
}

#[cfg(unix)]
pub fn infer_shell() -> Option<Box<dyn Shell>> {
    infer::unix::infer_shell()
}

// impl From<Box<dyn Shell>> for clap::Shell {
//     fn from(shell: Box<dyn Shell>) -> clap::Shell {
//         shell.as_clap_shell()
//     }
// }
