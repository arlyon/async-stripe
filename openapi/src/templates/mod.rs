//! This module holds methods relating to the generation of rust code.
pub mod cargo_toml;
pub mod enums;

#[doc(hidden)]
mod miniserde;
pub mod object_trait;
pub mod object_writer;
pub mod requests;
pub mod structs;
pub mod utils;

pub use object_writer::ObjectWriter;
