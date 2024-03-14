//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAuxiliaryNodeInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbAuxiliaryNodeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xca0888ca`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAuxiliaryNodeInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum NodeType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<NodeType>),
    /// # C++ Class Fields Info
    /// -   name:`"depth"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "depth")]
    Depth(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"referenceBehaviorName"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceBehaviorName")]
    ReferenceBehaviorName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"selfTransitionNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionNames")]
    SelfTransitionNames(HkArrayStringPtr<'a>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAuxiliaryNodeInfo<'de>, "@name",
    ("type" => Type(Primitive<NodeType>)),
    ("depth" => Depth(Primitive<u8>)),
    ("referenceBehaviorName" => ReferenceBehaviorName(Primitive<Cow<'de, str>>)),
    ("selfTransitionNames" => SelfTransitionNames(HkArrayStringPtr<'de>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    #[serde(rename = "NODE_TYPE_UNKNOWN")]
    NodeTypeUnknown = 0,
    #[serde(rename = "NODE_TYPE_NODE")]
    NodeTypeNode = 1,
    #[serde(rename = "NODE_TYPE_TRANSITION")]
    NodeTypeTransition = 2,
    #[serde(rename = "NODE_TYPE_WILDCARD_TRANSITION")]
    NodeTypeWildcardTransition = 3,
    #[serde(rename = "NODE_TYPE_STATE")]
    NodeTypeState = 4,
    #[serde(rename = "NODE_TYPE_STATE_MACHINE")]
    NodeTypeStateMachine = 5,
    #[serde(rename = "NODE_TYPE_MODIFIER_GENERATOR")]
    NodeTypeModifierGenerator = 6,
    #[serde(rename = "NODE_TYPE_MODIFIER")]
    NodeTypeModifier = 7,
    #[serde(rename = "NODE_TYPE_CLIP")]
    NodeTypeClip = 8,
    #[serde(rename = "NODE_TYPE_BLEND")]
    NodeTypeBlend = 9,
    #[serde(rename = "NODE_TYPE_TRANSITION_EFFECT")]
    NodeTypeTransitionEffect = 10,
}
