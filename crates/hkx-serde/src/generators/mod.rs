//! # Havok Classes => Rust Code generator
//!
//! - It may be called in `lib` or `test`, but not elsewhere.
//! - Not used except to generate Rust code.
pub mod cpp_type_parser;
mod gen_class_params;
mod generate_bitflags;
mod generate_code;
mod generated_types;

pub use gen_class_params::generate_class_params;
pub use generate_code::{
    generate_all_fields, generate_code, get_lifetime_from_fields, ClassMap, FieldMap, LifeTimeMap,
};
