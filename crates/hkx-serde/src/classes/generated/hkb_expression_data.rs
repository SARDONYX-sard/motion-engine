//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbExpressionData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbExpressionData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x6740042a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExpressionData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"expression"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expression")]
    Expression(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"assignmentVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "assignmentVariableIndex")]
    AssignmentVariableIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"assignmentEventIndex"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "assignmentEventIndex")]
    AssignmentEventIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"eventMode"`
    /// -   type: `enum ExpressionEventMode`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(ExpressionEventMode),
    /// # C++ Class Fields Info
    /// -   name:`"raisedEvent"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "raisedEvent", skip_serializing)]
    RaisedEvent(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"wasTrueInPreviousFrame"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "wasTrueInPreviousFrame", skip_serializing)]
    WasTrueInPreviousFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionData<'de>, "@name",
    ("expression" => Expression(Primitive<Cow<'de, str>>)),
    ("assignmentVariableIndex" => AssignmentVariableIndex(Primitive<i32>)),
    ("assignmentEventIndex" => AssignmentEventIndex(Primitive<i32>)),
    ("eventMode" => EventMode(ExpressionEventMode)),
    ("raisedEvent" => RaisedEvent(Primitive<bool>)),
    ("wasTrueInPreviousFrame" => WasTrueInPreviousFrame(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionEventMode {
    #[serde(rename = "EVENT_MODE_SEND_ONCE")]
    EventModeSendOnce = 0,
    #[serde(rename = "EVENT_MODE_SEND_ON_TRUE")]
    EventModeSendOnTrue = 1,
    #[serde(rename = "EVENT_MODE_SEND_ON_FALSE_TO_TRUE")]
    EventModeSendOnFalseToTrue = 2,
    #[serde(rename = "EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE")]
    EventModeSendEveryFrameOnceTrue = 3,
}
