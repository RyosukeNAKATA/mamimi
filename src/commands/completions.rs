use super::command::Command;
use clap::{IntoApp, Parser};
use clap_complete::Shell;

#[derive(Parser, Debug)]
pub struct Completions {
    #[clap(long)]
    shell: Option<Shell>,
}

impl Command for Completions {}
