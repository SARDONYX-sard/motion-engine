//! Limit Size array types
mod c_style_array_class;
mod c_style_array_primitive;
mod c_style_array_vector;

pub use c_style_array_class::{CStyleArrayClass, CStyleArrayClassParam};
pub use c_style_array_primitive::HkArrayCStyle;
pub use c_style_array_vector::{HkArrayCStyleMatrix3, HkArrayCStyleMatrix4, HkArrayCStyleVector};
