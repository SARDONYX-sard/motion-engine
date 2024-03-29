//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkSweptTransform`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkSweptTransform`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xb4e5770`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSweptTransform {
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMass0"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMass0")]
    CenterOfMass0(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMass1"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMass1")]
    CenterOfMass1(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation0"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation0")]
    Rotation0(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation1"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation1")]
    Rotation1(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMassLocal"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMassLocal")]
    CenterOfMassLocal(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkSweptTransform, "@name",
    ("centerOfMass0" => CenterOfMass0(Primitive<Vector4<f32>>)),
    ("centerOfMass1" => CenterOfMass1(Primitive<Vector4<f32>>)),
    ("rotation0" => Rotation0(Primitive<Quaternion<f32>>)),
    ("rotation1" => Rotation1(Primitive<Quaternion<f32>>)),
    ("centerOfMassLocal" => CenterOfMassLocal(Primitive<Vector4<f32>>)),
}

impl ByteDeSerialize for HkSweptTransform {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
