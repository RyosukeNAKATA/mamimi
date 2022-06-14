#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::module_name_repetitions,
    clippy::similar_names
)]

pub mod alias;
pub mod archive;
pub mod cli;
pub mod commands;
pub mod config;
pub mod current_python_version;
pub mod input_version;
pub mod log_level;
pub mod path_ext;
pub mod python_version;
pub mod remote_python_index;
pub mod shell;
pub mod symlink;
pub mod system_info;
pub mod system_version;
pub mod version_file_strategy;
pub mod version_files;

#[macro_use]
pub mod directories;