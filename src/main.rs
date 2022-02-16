#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::module_name_repetitions
    clippy::similar_names
)]

fn main() {
    env_logger::init();
    let value = crate::cli::parse();
    value.subcmd.call(value.config);
}
