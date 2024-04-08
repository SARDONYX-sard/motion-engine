//! # Havok Classes => Rust Code
//!
//! - It may be called in `lib` or `test`, but not elsewhere.
//! - Not used except to generate Rust code.
// subs
mod aliases;
mod class_params;
mod lifetime_manager;

mod all_class;
mod one_class;

mod utils;

pub use all_class::generate_classes;
