//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintInstanceSmallArraySerializeOverrideType`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpConstraintInstanceSmallArraySerializeOverrideType`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xee3c2aec`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintInstanceSmallArraySerializeOverrideType<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "data", skip_serializing)]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"size"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "size")]
    Size(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"capacityAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capacityAndFlags")]
    CapacityAndFlags(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintInstanceSmallArraySerializeOverrideType<'de>, "@name",
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("size" => Size(Primitive<u16>)),
    ("capacityAndFlags" => CapacityAndFlags(Primitive<u16>)),
}
