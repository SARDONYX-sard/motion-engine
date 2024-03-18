//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventProperty`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbEventProperty`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `hkbEventBase`/`0x76bddb31`
/// - signature: `0xdb38a15`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventProperty<'a> {
    /// # C++ Parent class(`hkbEventBase` => parent: `None`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "id")]
    Id(Primitive<i32>),
    /// # C++ Parent class(`hkbEventBase` => parent: `None`) field Info
    /// -   name:`"payload"`
    /// -   type: `struct hkbEventPayload*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "payload")]
    Payload(Primitive<Cow<'a, str>>),

}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventProperty<'de>, "@name",
    ("id" => Id(Primitive<i32>)),
    ("payload" => Payload(Primitive<Cow<'de, str>>)),
}
