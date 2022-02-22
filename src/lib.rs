#![warn(rust_2021_idioms, clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::module_name_repetitions,
    clippy::similar_names
)]

mod alias;
mod archive;
mod cli;
mod commands;
mod config;
mod current_python_version;
mod input_version;
mod log_level;
mod lts;
mod path_ext;
mod python_version;
mod remote_python_index;
mod shell;
mod symlink;
mod system_info;
mod system_version;
mod version_file_strategy;
mod version_files;

#[macro_use]
mod directories;
