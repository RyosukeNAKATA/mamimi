use crate::shell::Shell;
use anyhow::Ok;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn path(&self, path: &Path) -> anyhow::Result<String> {
        Ok(format!("set -gx PATH {:?} $PATH;", path.to_str().unwrap()))
    }
    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }
    fn rehash(&self) -> Option<String> {
        Some("rehash".to_string())
    }
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Fish
    }
    fn use_on_cd(&self, config: &crate::config::MamimiConfig) -> String {
        indoc!(
            r#"
                function _mamimi_autoload_hook --on-variable PWD --description 'Change Python version on directory change'
                    status --is-command-substitution; and return
                    mamimi --log-level quiet local
                end
            "#
        )
        .into()
    }
}
