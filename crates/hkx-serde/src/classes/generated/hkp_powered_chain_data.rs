//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpPoweredChainData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkpConstraintChainData`/`0x5facc7ff`
/// - signature: `0x38aeafc3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainData<'a> {
    // C++ Parent class(`hkpConstraintChainData` => parent: `hkpConstraintData`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"infos"`
    /// -   type: `hkArray<struct hkpPoweredChainDataConstraintInfo>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "infos")]
    Infos(HkArrayClass<HkpPoweredChainDataConstraintInfo<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cfmLinAdd"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmLinAdd")]
    CfmLinAdd(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cfmLinMul"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmLinMul")]
    CfmLinMul(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cfmAngAdd"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmAngAdd")]
    CfmAngAdd(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"cfmAngMul"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cfmAngMul")]
    CfmAngMul(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxErrorDistance"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxErrorDistance")]
    MaxErrorDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainData<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("infos" => Infos(HkArrayClass<HkpPoweredChainDataConstraintInfo<'de>>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
    ("cfmLinAdd" => CfmLinAdd(Primitive<f32>)),
    ("cfmLinMul" => CfmLinMul(Primitive<f32>)),
    ("cfmAngAdd" => CfmAngAdd(Primitive<f32>)),
    ("cfmAngMul" => CfmAngMul(Primitive<f32>)),
    ("maxErrorDistance" => MaxErrorDistance(Primitive<f32>)),
}
