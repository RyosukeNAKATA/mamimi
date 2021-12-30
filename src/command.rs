use crate::config::MamimiConfig;
use crate::outln;
use colored::Colorize;
use dirs::config_dir;

pub trait Command {
    type Error: std::error::Error;

    fn apply(&self, config: &MamimiConfig) -> Result<(), Self::Error>;
    fn handle_error(err: Self::Error, config: &MamimiConfig) {
        outln!(config #Error, "{} {}", "error:".red().bold(),format!("{}",err).red());
        std::process::exit(1);
    }

    fn call(&self, config: &MamimiConfig) {
        if let Err(err) = self.apply(&config) {
            Self::handle_error(err, &config)
        }
    }
}
