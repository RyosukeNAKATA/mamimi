use crate::commands;
use crate::commands::command::Command;
use crate::config::MamimiConfig;
use clap::Parser;

/// Blazingly falt python manager
#[derive(clap::Parser, Debug, Parser, Debug)]
#[clap(name="mamimi",version=env!("CARGO_PKG_VERSION"),bin_name="mamimi")]
pub struct Cli {
    #[clap(flatten)]
    pub config: MamimiConfig,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
    /// Sets environment variables for initializing mamimi
    #[clap(name = "init")]
    Init(commands::init::Init),
    /// Installs a specific Python version
    #[clap(name = "install")]
    Install(commands::install::Install),
    /// Uninstall a specific Python version
    #[clap(name = "uninstall", bin_name = "uninstall")]
    Uninstall(commands::uninstall::Uninstall),
    /// Lists installed Python version
    #[clap(name = "versions")]
    Versions(commands::versions::Versions),
    /// Sets the current Python version
    #[clap(name = "local")]
    Local(commands::local::Local),
    /// Sets the global Python version
    #[clap(name = "global")]
    Global(commands::global::Global),
    /// Print shell completions to stdout
    #[clap(name = "completions")]
    Completions(commands::completions::Completions),
}

impl SubCommand {
    pub fn call(self, config: MamimiConfig) {
        match self {
            Self::Init(cmd) => cmd.call(&config),
            Self::Install(cmd) => cmd.call(config),
            Self::Uninstall(cmd) => cmd.call(config),
            Self::Versions(cmd) => cmd.call(config),
            Self::Local(cmd) => cmd.call(config),
            Self::Global(cmd) => cmd.call(config),
            Self::Completions(cmd) => cmd.call(&config),
        }
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
