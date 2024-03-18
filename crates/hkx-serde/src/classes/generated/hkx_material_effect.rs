//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterialEffect`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxMaterialEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x1d39f925`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialEffect<'a> {
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
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum EffectType`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<EffectType>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkArrayRef<Primitive<u8>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialEffect<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("type" => Type(Primitive<EffectType>)),
    ("data" => Data(HkArrayRef<Primitive<u8>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "EFFECT_TYPE_INVALID")]
    EffectTypeInvalid = 0,
    #[serde(rename = "EFFECT_TYPE_UNKNOWN")]
    EffectTypeUnknown = 1,
    #[serde(rename = "EFFECT_TYPE_HLSL_FX_INLINE")]
    EffectTypeHlslFxInline = 2,
    #[serde(rename = "EFFECT_TYPE_CG_FX_INLINE")]
    EffectTypeCgFxInline = 3,
    #[serde(rename = "EFFECT_TYPE_HLSL_FX_FILENAME")]
    EffectTypeHlslFxFilename = 4,
    #[serde(rename = "EFFECT_TYPE_CG_FX_FILENAME")]
    EffectTypeCgFxFilename = 5,
    #[serde(rename = "EFFECT_TYPE_MAX_ID")]
    EffectTypeMaxId = 6,
}
