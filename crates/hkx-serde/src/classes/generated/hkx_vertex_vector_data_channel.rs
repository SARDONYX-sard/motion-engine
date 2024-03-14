//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkxVertexVectorDataChannel`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxVertexVectorDataChannel`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x2ea63179`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexVectorDataChannel {
    /// # C++ Class Fields Info
    /// -   name:`"perVertexVectors"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "perVertexVectors")]
    PerVertexVectors(HkArrayVector<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexVectorDataChannel, "@name",
    ("perVertexVectors" => PerVertexVectors(HkArrayVector<Vector4<f32>>)),
}
