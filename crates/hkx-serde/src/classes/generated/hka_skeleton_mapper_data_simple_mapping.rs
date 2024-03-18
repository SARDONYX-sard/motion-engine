//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonMapperDataSimpleMapping`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaSkeletonMapperDataSimpleMapping`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x3405deca`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonMapperDataSimpleMapping {
    /// # C++ Class Fields Info
    /// -   name:`"boneA"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneA")]
    BoneA(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"boneB"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneB")]
    BoneB(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"aFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aFromBTransform")]
    AFromBTransform(QsTransform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataSimpleMapping, "@name",
    ("boneA" => BoneA(Primitive<i16>)),
    ("boneB" => BoneB(Primitive<i16>)),
    ("aFromBTransform" => AFromBTransform(QsTransform<f32>)),
}
