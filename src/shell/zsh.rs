use crate::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Zsh;

impl Shell for Zsh {
    fn path(&self, path: &Path) -> String {
        format!("export PATH={:?}:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::MamimiConfig) -> String {
        indoc!(
            r#"
                autoload -U add-zsh-hook
                _mamimi_autoload_hook () {
                    mamimi --log-level quiet local
                }

                add-zsh-hook chpwd _mamimi_autoload_hook \
                    && _mamimi_autoload_hook
            "#
        )
        .into()
    }

    // fn as_clap_shell(&self) -> clap::Shell {
    //     clap::Shell::Zsh
    // }
}
