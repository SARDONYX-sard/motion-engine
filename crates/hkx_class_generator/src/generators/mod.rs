//! # Havok Classes => Rust Code
//!
//! - It may be called in `lib` or `test`, but not elsewhere.
//! - Not used except to generate Rust code.
// subs
mod bitflags;
mod class_params;
mod cpp_type_parser;
mod generated_types;

mod one_class; // one class generator

mod all_class; // main all generators

pub use all_class::generate_classes;
