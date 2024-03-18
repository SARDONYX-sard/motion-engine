//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonMapperData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaSkeletonMapperData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0x95687ea0`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonMapperData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"skeletonA"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeletonA")]
    SkeletonA(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"skeletonB"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeletonB")]
    SkeletonB(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"simpleMappings"`
    /// -   type: `hkArray<struct hkaSkeletonMapperDataSimpleMapping>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simpleMappings")]
    SimpleMappings(HkArrayClass<HkaSkeletonMapperDataSimpleMapping>),
    /// # C++ Class Fields Info
    /// -   name:`"chainMappings"`
    /// -   type: `hkArray<struct hkaSkeletonMapperDataChainMapping>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chainMappings")]
    ChainMappings(HkArrayClass<HkaSkeletonMapperDataChainMapping>),
    /// # C++ Class Fields Info
    /// -   name:`"unmappedBones"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "unmappedBones")]
    UnmappedBones(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"extractedMotionMapping"`
    /// -   type: `hkQsTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotionMapping")]
    ExtractedMotionMapping(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"keepUnmappedLocal"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepUnmappedLocal")]
    KeepUnmappedLocal(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"mappingType"`
    /// -   type: `enum MappingType`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mappingType")]
    MappingType(Primitive<MappingType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperData<'de>, "@name",
    ("skeletonA" => SkeletonA(Primitive<Cow<'de, str>>)),
    ("skeletonB" => SkeletonB(Primitive<Cow<'de, str>>)),
    ("simpleMappings" => SimpleMappings(HkArrayClass<HkaSkeletonMapperDataSimpleMapping>)),
    ("chainMappings" => ChainMappings(HkArrayClass<HkaSkeletonMapperDataChainMapping>)),
    ("unmappedBones" => UnmappedBones(HkArrayRef<Primitive<i16>>)),
    ("extractedMotionMapping" => ExtractedMotionMapping(QsTransform<f32>)),
    ("keepUnmappedLocal" => KeepUnmappedLocal(Primitive<bool>)),
    ("mappingType" => MappingType(Primitive<MappingType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MappingType {
    #[serde(rename = "HK_RAGDOLL_MAPPING")]
    HkRagdollMapping = 0,
    #[serde(rename = "HK_RETARGETING_MAPPING")]
    HkRetargetingMapping = 1,
}
