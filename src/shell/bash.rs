use crate::shell::Shell;
use anyhow::Ok;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn path(&self, path: &Path) -> anyhow::Result<String> {
        Ok(format!("export PATH={:?}:$PATH", path.to_str().unwrap()))
    }
    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }
    fn rehash(&self) -> Option<String> {
        Some("rehash".to_string())
    }
    fn to_clap_shell(&self) -> clap_complete::Shell {
        clap_complete::Shell::Bash
    }
    fn use_on_cd(&self, config: &crate::config::MamimiConfig) -> String {
        indoc!(
            r#"
                __mamimicd() {
                    \cd "$@" || return $?
                    mamimi --log-level quiet local
                }
                alias cd=__mamimicd
            "#
        )
        .into()
    }
}
