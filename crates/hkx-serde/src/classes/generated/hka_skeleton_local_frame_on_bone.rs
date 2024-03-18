//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonLocalFrameOnBone`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaSkeletonLocalFrameOnBone`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x52e8043`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSkeletonLocalFrameOnBone<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"localFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrame")]
    LocalFrame(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonLocalFrameOnBone<'de>, "@name",
    ("localFrame" => LocalFrame(Primitive<Cow<'de, str>>)),
    ("boneIndex" => BoneIndex(Primitive<i32>)),
}
