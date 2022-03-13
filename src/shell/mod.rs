pub mod bash;
pub mod fish;
pub mod infer;
pub mod powershell;
pub mod windows_command;
pub mod zsh;

#[allow(clippy::module_inception)]
mod shell;

pub use bash::Bash;
pub use fish::Fish;
pub use powershell::PowerShell;
pub use shell::{Shell, AVAILABLE_SHELLS};
pub use windows_command::WindowsCommand;
pub use zsh::Zsh;
