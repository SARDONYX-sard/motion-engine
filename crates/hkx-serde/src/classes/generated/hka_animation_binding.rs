//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkaAnimationBinding`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkaAnimationBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x66eac971`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnimationBinding<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"originalSkeletonName"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalSkeletonName")]
    OriginalSkeletonName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animation"`
    /// -   type: `struct hkaAnimation*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animation")]
    Animation(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"transformTrackToBoneIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformTrackToBoneIndices")]
    TransformTrackToBoneIndices(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"floatTrackToFloatSlotIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatTrackToFloatSlotIndices")]
    FloatTrackToFloatSlotIndices(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"blendHint"`
    /// -   type: `enum BlendHint`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendHint")]
    BlendHint(BlendHint),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationBinding<'de>, "@name",
    ("originalSkeletonName" => OriginalSkeletonName(Primitive<Cow<'de, str>>)),
    ("animation" => Animation(Cow<'de, str>)),
    ("transformTrackToBoneIndices" => TransformTrackToBoneIndices(HkArrayRef<Primitive<i16>>)),
    ("floatTrackToFloatSlotIndices" => FloatTrackToFloatSlotIndices(HkArrayRef<Primitive<i16>>)),
    ("blendHint" => BlendHint(BlendHint)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlendHint {
    #[serde(rename = "NORMAL")]
    Normal = 0,
    #[serde(rename = "ADDITIVE")]
    Additive = 1,
}
