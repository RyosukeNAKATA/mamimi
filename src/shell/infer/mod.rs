mod unix;
mod windows;

#[cfg(unix)]
pub use self::unix::infer_shell;
#[cfg(windows)]
pub use self::windows::infer_shell;

pub(self) fn shell_from_string(shell: &str) -> Option<Box<dyn super::Shell>> {
    use super::{Bash, Fish, PowerShell, WindowsCommand, Zsh};
    match shell {
        "sh" | "bash" => return Some(Box::from(Bash)),
        "zsh" => return Some(Box::from(Zsh)),
        "fish" => return Some(Box::from(Fish)),
        "pwsh" => return Some(Box::from(PowerShell)),
        "cmd" => return Some(Box::from(WindowsCommand)),
        cmd_name => log::debug!("binary is not a supported shell: {:?}", cmd_name),
    };
    None
}
