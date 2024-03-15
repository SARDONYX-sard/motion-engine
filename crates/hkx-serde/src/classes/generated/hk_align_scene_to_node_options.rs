//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkAlignSceneToNodeOptions`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkAlignSceneToNodeOptions`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x207cb01`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAlignSceneToNodeOptions<'a> {
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"invert"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invert")]
    Invert(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"transformPositionX"`
    /// -   type: `hkBool`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformPositionX")]
    TransformPositionX(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"transformPositionY"`
    /// -   type: `hkBool`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformPositionY")]
    TransformPositionY(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"transformPositionZ"`
    /// -   type: `hkBool`
    /// - offset: 11
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformPositionZ")]
    TransformPositionZ(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"transformRotation"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformRotation")]
    TransformRotation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"transformScale"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformScale")]
    TransformScale(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"transformSkew"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformSkew")]
    TransformSkew(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"keyframe"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframe")]
    Keyframe(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAlignSceneToNodeOptions<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("invert" => Invert(Primitive<bool>)),
    ("transformPositionX" => TransformPositionX(Primitive<bool>)),
    ("transformPositionY" => TransformPositionY(Primitive<bool>)),
    ("transformPositionZ" => TransformPositionZ(Primitive<bool>)),
    ("transformRotation" => TransformRotation(Primitive<bool>)),
    ("transformScale" => TransformScale(Primitive<bool>)),
    ("transformSkew" => TransformSkew(Primitive<bool>)),
    ("keyframe" => Keyframe(Primitive<i32>)),
    ("nodeName" => NodeName(Primitive<Cow<'de, str>>)),
}
