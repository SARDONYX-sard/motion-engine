//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpBvTreeShape`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBvTreeShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0xa823d623`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBvTreeShape {
    /// # C++ Class Fields Info
    /// -   name:`"bvTreeType"`
    /// -   type: `enum BvTreeType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bvTreeType")]
    BvTreeType(BvTreeType),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBvTreeShape, "@name",
    ("bvTreeType" => BvTreeType(BvTreeType)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BvTreeType {
    #[serde(rename = "BVTREE_MOPP")]
    BvtreeMopp = 0,
    #[serde(rename = "BVTREE_TRISAMPLED_HEIGHTFIELD")]
    BvtreeTrisampledHeightfield = 1,
    #[serde(rename = "BVTREE_USER")]
    BvtreeUser = 2,
    #[serde(rename = "BVTREE_MAX")]
    BvtreeMax = 3,
}
