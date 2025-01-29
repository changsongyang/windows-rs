#![doc = include_str!("../readme.md")]
#![allow(
    unused_imports,
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms
)]

use std::collections::hash_map::*;

mod attributes;
mod bindings;
mod blobs;
mod codes;
mod file;
mod helpers;
mod strings;
mod tables;

pub use attributes::*;
pub use bindings::*;
pub use blobs::*;
pub use codes::*;
pub use file::*;
pub use helpers::*;
pub use strings::*;
pub use tables::*;
