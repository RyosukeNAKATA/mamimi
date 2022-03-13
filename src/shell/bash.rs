use super::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Bash
    }
    fn path(&self, path: &Path) -> String {
        format!("export PATH={:?}:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::MamimiConfig) -> String {
        indoc!(
            r#"
                _mamimicd() {
                \cd "$@" || return $?
                mamimi --log-level quiet local
                }

                alias cd=_mamimicd
            "#
        )
        .into()
    }
}
