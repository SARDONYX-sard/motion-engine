//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpTyremarksWheel`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpTyremarksWheel`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x1eaef041`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTyremarksWheel {
    /// # C++ Class Fields Info
    /// -   name:`"currentPosition"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPosition")]
    CurrentPosition(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numPoints"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numPoints")]
    NumPoints(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"tyremarkPoints"`
    /// -   type: `hkArray&lt;struct hkpTyremarkPoint&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tyremarkPoints")]
    TyremarkPoints(HkArrayClass<HkpTyremarkPoint>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarksWheel, "@name",
    ("currentPosition" => CurrentPosition(Primitive<i32>)),
    ("numPoints" => NumPoints(Primitive<i32>)),
    ("tyremarkPoints" => TyremarkPoints(HkArrayClass<HkpTyremarkPoint>)),
}
