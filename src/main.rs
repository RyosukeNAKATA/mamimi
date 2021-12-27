use clap::Parser;

mod cli;
mod python_version;

fn main() {
    let opts = cli::Opts::parse();
}
