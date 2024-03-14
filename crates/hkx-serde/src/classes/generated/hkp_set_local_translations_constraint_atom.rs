//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpSetLocalTranslationsConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpSetLocalTranslationsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x5cbfcf4a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSetLocalTranslationsConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"translationA"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationA")]
    TranslationA(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"translationB"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationB")]
    TranslationB(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalTranslationsConstraintAtom, "@name",
    ("translationA" => TranslationA(Vector4<f32>)),
    ("translationB" => TranslationB(Vector4<f32>)),
}
