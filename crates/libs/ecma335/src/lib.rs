#![doc = include_str!("../readme.md")]
#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use std::cmp::Ordering;

mod attributes;
pub mod reader;
mod value;
pub mod writer;

pub use attributes::*;
pub use value::*;
mod bindings;
use bindings::*;
