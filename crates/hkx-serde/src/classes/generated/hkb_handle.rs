//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandle`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbHandle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd8b6401c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandle<'a> {
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
    /// -   name:`"frame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frame")]
    Frame(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBody")]
    RigidBody(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"character"`
    /// -   type: `struct hkbCharacter*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "character")]
    Character(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationBoneIndex")]
    AnimationBoneIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandle<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("frame" => Frame(Primitive<Cow<'de, str>>)),
    ("rigidBody" => RigidBody(Primitive<Cow<'de, str>>)),
    ("character" => Character(Primitive<Cow<'de, str>>)),
    ("animationBoneIndex" => AnimationBoneIndex(Primitive<i16>)),
}

impl ByteDeSerialize for HkbHandle<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
