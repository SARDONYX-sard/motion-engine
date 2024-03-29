//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkLinkAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkLinkAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x255d8164`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkLinkAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum Link`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<Link>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkLinkAttribute, "@name",
    ("type" => Type(Primitive<Link>)),
}

impl ByteDeSerialize for HkLinkAttribute {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Link {
    #[serde(rename = "NONE")]
    None = 0,
    #[serde(rename = "DIRECT_LINK")]
    DirectLink = 1,
    #[serde(rename = "CHILD")]
    Child = 2,
    #[serde(rename = "MESH")]
    Mesh = 3,
    #[serde(rename = "PARENT_NAME")]
    ParentName = 4,
}
