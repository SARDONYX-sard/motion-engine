//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaAnimationContainer`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkaAnimationContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 68
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x8dc20333`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnimationContainer<'a> {
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

    /// # C++ Class Fields Info
    /// -   name:`"skeletons"`
    /// -   type: `hkArray<hkaSkeleton*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeletons")]
    Skeletons(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animations"`
    /// -   type: `hkArray<hkaAnimation*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animations")]
    Animations(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"bindings"`
    /// -   type: `hkArray<hkaAnimationBinding*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindings")]
    Bindings(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"attachments"`
    /// -   type: `hkArray<hkaBoneAttachment*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachments")]
    Attachments(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"skins"`
    /// -   type: `hkArray<hkaMeshBinding*>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skins")]
    Skins(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationContainer<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("skeletons" => Skeletons(HkArrayRef<Cow<'de, str>>)),
    ("animations" => Animations(HkArrayRef<Cow<'de, str>>)),
    ("bindings" => Bindings(HkArrayRef<Cow<'de, str>>)),
    ("attachments" => Attachments(HkArrayRef<Cow<'de, str>>)),
    ("skins" => Skins(HkArrayRef<Cow<'de, str>>)),
}
