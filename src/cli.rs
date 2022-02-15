use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(clap::Parser)]
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
pub enum SubCommand {
    /// Sets environment variables for initializing mamimi
    #[clap(name = "init")]
    Init,
    /// Installs a specific Python version
    #[clap(name = "install")]
    Install {
        /// Lists Python versions avalable to install
        #[clap(short, long)]
        list: bool,
        /// Options passed to ./configure
        #[clap(name = "version")]
        python_version: String,
    },
    /// Uninstall a specific Python version
    #[clap(name = "uninstall")]
    Uninstall {
        #[clap(name = "version")]
        python_version: String,
    },
    /// Lists installed Python version
    #[clap(name = "versions")]
    Versions,
    /// Sets the current Python version
    #[clap(name = "local")]
    Local,
    /// Sets the global Python version
    #[clap(name = "global")]
    Global,
    /// Print shell completions to stdout
    #[clap(name = "completions")]
    Completions {
        /// The shell syntax to use
        #[clap(short, long)]
        shell: String,
        /// Lists installed Python versions
        #[clap(short, long)]
        list: bool,
    },
}

#[derive(Parser)]
struct Test {
    #[clap(short)]
    debug: bool,
}
