//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMassProperties`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpMassProperties`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x68a56834`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMassProperties {
    /// # C++ Class Fields Info
    /// -   name:`"volume"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "volume")]
    Volume(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"centerOfMass"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "centerOfMass")]
    CenterOfMass(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"inertiaTensor"`
    /// -   type: `hkMatrix3`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inertiaTensor")]
    InertiaTensor(Primitive<Matrix3<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMassProperties, "@name",
    ("volume" => Volume(Primitive<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("centerOfMass" => CenterOfMass(Primitive<Vector4<f32>>)),
    ("inertiaTensor" => InertiaTensor(Primitive<Matrix3<f32>>)),
}

impl ByteDeSerialize for HkpMassProperties {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
