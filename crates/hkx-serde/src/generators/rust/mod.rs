pub mod cpp_type_parser;
mod gen_class_params;
mod generate_code;
mod generated_types;

pub use gen_class_params::generate_class_params;
pub use generate_code::{generate_code, has_life_time};
