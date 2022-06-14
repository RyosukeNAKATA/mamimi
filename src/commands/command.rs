use crate::config::MamimiConfig;
use crate::outln;
use colored::Colorize;

pub trait Command: Sized {
    type Error: std::error::Error;

    fn apply(self, config: &MamimiConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, config: &MamimiConfig) {
        let err_s = format!("{}", err);
        outln!(config, Error, "{} {}", "error:".red().bold(), err_s.red());
        std::process::exit(1);
    }

    fn call(self, config: &MamimiConfig) {
        if let Err(err) = self.apply(&config) {
            Self::handle_error(err, &config)
        }
    }
}
