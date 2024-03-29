//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMoppCode`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpMoppCode`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x924c2661`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMoppCode {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"info"`
    /// -   type: `struct hkpMoppCodeCodeInfo`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "info")]
    Info(SingleClass<HkpMoppCodeCodeInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayNum<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"buildType"`
    /// -   type: `enum BuildType`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "buildType")]
    BuildType(Primitive<BuildType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppCode, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("info" => Info(SingleClass<HkpMoppCodeCodeInfo>)),
    ("data" => Data(HkArrayNum<u8>)),
    ("buildType" => BuildType(Primitive<BuildType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildType {
    #[serde(rename = "BUILT_WITH_CHUNK_SUBDIVISION")]
    BuiltWithChunkSubdivision = 0,
    #[serde(rename = "BUILT_WITHOUT_CHUNK_SUBDIVISION")]
    BuiltWithoutChunkSubdivision = 1,
    #[serde(rename = "BUILD_NOT_SET")]
    BuildNotSet = 2,
}
