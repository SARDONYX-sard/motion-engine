//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSLookAtModifierBoneData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `BSLookAtModifierBoneData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x29efee59`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsLookAtModifierBoneData {
    /// # C++ Class Fields Info
    /// -   name:`"index"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "index")]
    Index(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"fwdAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fwdAxisLS")]
    FwdAxisLs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enabled"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enabled")]
    Enabled(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"currentFwdAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "currentFwdAxisLS", skip_serializing)]
    CurrentFwdAxisLs(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsLookAtModifierBoneData, "@name",
    ("index" => Index(Primitive<i16>)),
    ("fwdAxisLS" => FwdAxisLs(Primitive<Vector4<f32>>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("enabled" => Enabled(Primitive<bool>)),
    ("currentFwdAxisLS" => CurrentFwdAxisLs(Primitive<Vector4<f32>>)),
}
