//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorInfoIdToNamePair`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBehaviorInfoIdToNamePair`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x35a0439a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorInfoIdToNamePair<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"behaviorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorName", default)]
    BehaviorName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeName", default)]
    NodeName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"toolType"`
    /// -   type: `enum ToolNodeType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toolType", default)]
    ToolType(Primitive<ToolNodeType>),
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "id", default)]
    Id(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorInfoIdToNamePair<'de>, "@name",
    ("behaviorName" => BehaviorName(Primitive<Cow<'de, str>>)),
    ("nodeName" => NodeName(Primitive<Cow<'de, str>>)),
    ("toolType" => ToolType(Primitive<ToolNodeType>)),
    ("id" => Id(Primitive<i16>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolNodeType {
    #[serde(rename = "NODE_TYPE_UNKNOWN")]
    NodeTypeUnknown = 0,
    #[serde(rename = "NODE_TYPE_STATE_MACHINE")]
    NodeTypeStateMachine = 1,
    #[serde(rename = "NODE_TYPE_CLIP")]
    NodeTypeClip = 2,
    #[serde(rename = "NODE_TYPE_BLEND")]
    NodeTypeBlend = 3,
    #[serde(rename = "NODE_TYPE_MODIFIER")]
    NodeTypeModifier = 4,
    #[serde(rename = "NODE_TYPE_GENERATOR")]
    NodeTypeGenerator = 5,
    #[serde(rename = "NODE_TYPE_MODIFIER_GENERATOR")]
    NodeTypeModifierGenerator = 6,
    #[serde(rename = "NODE_TYPE_TRANSITION_EFFECT")]
    NodeTypeTransitionEffect = 7,
    #[serde(rename = "NODE_TYPE_BEHAVIOR_FILE_REFERENCE")]
    NodeTypeBehaviorFileReference = 8,
}
