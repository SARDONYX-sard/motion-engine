mod c_style_array;
mod hk_array;
mod matrix3;
mod matrix4;
mod primitive;
mod qs_transform;
mod quaternion;
mod transform;
mod vector3;
mod vector4;

pub use c_style_array::*;
pub use hk_array::*;
pub use matrix3::Matrix3;
pub use matrix4::Matrix4;
pub use primitive::Primitive;
pub use qs_transform::QsTransform;
pub use quaternion::Quaternion;
pub use transform::Transform;
pub use vector3::Vector3;
pub use vector4::Vector4;

/// Representing a rotation
pub type Rotation<S> = Matrix3<S>;
