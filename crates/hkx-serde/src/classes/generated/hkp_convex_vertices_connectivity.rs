//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexVerticesConnectivity`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConvexVerticesConnectivity`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x63d38e9c`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexVerticesConnectivity {
    /// # C++ Class Fields Info
    /// -   name:`"vertexIndices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexIndices")]
    VertexIndices(HkArrayRef<Primitive<u16>>),
    /// # C++ Class Fields Info
    /// -   name:`"numVerticesPerFace"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVerticesPerFace")]
    NumVerticesPerFace(HkArrayRef<Primitive<u8>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesConnectivity, "@name",
    ("vertexIndices" => VertexIndices(HkArrayRef<Primitive<u16>>)),
    ("numVerticesPerFace" => NumVerticesPerFace(HkArrayRef<Primitive<u8>>)),
}
