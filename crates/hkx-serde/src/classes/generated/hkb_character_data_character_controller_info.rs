//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterDataCharacterControllerInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbCharacterDataCharacterControllerInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xa0f415bf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterDataCharacterControllerInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"capsuleHeight"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleHeight")]
    CapsuleHeight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"capsuleRadius"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleRadius")]
    CapsuleRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterControllerCinfo"`
    /// -   type: `struct hkpCharacterControllerCinfo*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterControllerCinfo")]
    CharacterControllerCinfo(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterDataCharacterControllerInfo<'de>, "@name",
    ("capsuleHeight" => CapsuleHeight(Primitive<f32>)),
    ("capsuleRadius" => CapsuleRadius(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("characterControllerCinfo" => CharacterControllerCinfo(Primitive<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkbCharacterDataCharacterControllerInfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
