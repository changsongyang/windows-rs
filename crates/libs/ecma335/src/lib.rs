#![doc = include_str!("../readme.md")]
#![allow(non_snake_case, non_upper_case_globals, dead_code)]

pub mod reader;
pub mod writer;

// TODO: maybe ecma335 shares the File struct only and bindgen build its own Reader/Type cache and ecma335 can have its own Reader/Type cache