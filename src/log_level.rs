#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum LogLevel {
    Quiet,
    Error,
    Info,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl LogLevel {
    pub fn is_writable(&self, logging: &Self) -> bool {
        use std::cmp::Ordering;
        matches!(self.cmp(logging), Ordering::Greater | Ordering::Equal)
    }

    pub fn write_for(&self, logging: &Self) -> Box<dyn std::io::Write> {
        if self.is_writable(logging) {
            match logging {
                Self::Error => Box::from(std::io::stderr()),
                _ => Box::from(std::io::stdout()),
            }
        } else {
            Box::from(std::io::sink())
        }
    }

    pub fn passible_values() -> &'static [&'static str; 4] {
        &["quiet", "info", "all", "error"]
    }
}

impl From<LogLevel> for &'static str {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Quiet => "quiet",
            LogLevel::Info => "info",
            LogLevel::Error => "error",
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        match s {
            "quiet" => Ok(Self::Quiet),
            "info" | "all" => Ok(Self::Info),
            "error" => Ok(Self::Error),
            _ => Err("Unsupported log level"),
        }
    }
}

#[macro_export]
macro_rules! outln {
    ($config:ident, $level:path, $($expr:expr),+) => {{
        use $crate::log::LogLevel::*;
        writeln!($config.log_level().write_for(&$level), $($expr),+).expect("Can't write output");
    }}
}