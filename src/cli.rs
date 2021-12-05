use crate::input_version::InputVersion;
use anyhow::Result;
use clap::{AppSettings, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "Mamimi",
    version = "0.0.1",
    author = "Ryosuke",
    about = "A cool Python version manager written in Rust"
)]
pub struct Opts {
    #[clap(short, long)]
    verbose: bool,
    /// The root directory of mamimi  installations [default: $HOME/.mamimi]
    #[clap(name = "base-dir", long = "mamimi-dir")]
    take_vale: bool,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// Sets environment variables for initializing mamimi
    #[clap(name = "init")]
    Init,
    /// Installs a specific Python version
    #[clap(name = "install")]
    Install {
        /// Lists Python versions avalable to install
        #[clap(short, long)]
        list: bool,
        #[clap(name = "version")]
        version: Option<InputVersion>,
    },
}

#[derive(Parser)]
struct Test {
    #[clap(short)]
    debug: bool,
}
