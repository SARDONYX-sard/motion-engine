//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSIStateManagerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `BSIStateManagerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x6cb24f2e`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsiStateManagerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"iStateVar"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iStateVar")]
    IStateVar(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"stateData"`
    /// -   type: `hkArray&lt;struct BSIStateManagerModifierBSiStateData&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateData")]
    StateData(HkArrayClass<BsiStateManagerModifierBSiStateData>),
    /// # C++ Class Fields Info
    /// -   name:`"myStateListener"`
    /// -   type: `struct BSIStateManagerModifierBSIStateManagerStateListener`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "myStateListener", skip_serializing)]
    MyStateListener(BsiStateManagerModifierBsiStateManagerStateListener),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifier, "@name",
    ("iStateVar" => IStateVar(Primitive<i32>)),
    ("stateData" => StateData(HkArrayClass<BsiStateManagerModifierBSiStateData>)),
    ("myStateListener" => MyStateListener(BsiStateManagerModifierBsiStateManagerStateListener)),
}
