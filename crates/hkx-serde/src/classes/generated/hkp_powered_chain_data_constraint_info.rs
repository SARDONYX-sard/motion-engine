//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainDataConstraintInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpPoweredChainDataConstraintInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0xf88aee25`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainDataConstraintInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pivotInA"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInA")]
    PivotInA(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"pivotInB"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInB")]
    PivotInB(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"aTc"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aTc")]
    ATc(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"bTc"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bTc")]
    BTc(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"motors"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"switchBodies"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "switchBodies")]
    SwitchBodies(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainDataConstraintInfo<'de>, "@name",
    ("pivotInA" => PivotInA(Primitive<Vector4<f32>>)),
    ("pivotInB" => PivotInB(Primitive<Vector4<f32>>)),
    ("aTc" => ATc(Primitive<Quaternion<f32>>)),
    ("bTc" => BTc(Primitive<Quaternion<f32>>)),
    ("motors" => Motors(Primitive<Cow<'de, str>>)),
    ("switchBodies" => SwitchBodies(Primitive<bool>)),
}

impl ByteDeSerialize for HkpPoweredChainDataConstraintInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
