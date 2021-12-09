use crate::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn path(&self, path: &Path) -> String {
        format!("set -gx PATH {:?} $PATH;", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, _config: &crate::config::MamimiConfig) -> String {
        indoc!(
            r#"
                function _mamimi_autoload_hook --on-valiable PWD --description 'Change Python version on directory change'
                    status --is-command-substitution; and return
                    mamimi --log-level quiet local
                end
            "#
        )
        .into()
    }
    fn as_clap_shell(&self) -> clap::Shell {
        clap::Shell::Fish
    }
}
