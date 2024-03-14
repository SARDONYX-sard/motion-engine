//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbStateMachineTimeInterval`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbStateMachineTimeInterval`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x60a881e5`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachineTimeInterval {
    /// # C++ Class Fields Info
    /// -   name:`"enterEventId"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enterEventId")]
    EnterEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"exitEventId"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitEventId")]
    ExitEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"enterTime"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enterTime")]
    EnterTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"exitTime"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "exitTime")]
    ExitTime(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachineTimeInterval, "@name",
    ("enterEventId" => EnterEventId(Primitive<i32>)),
    ("exitEventId" => ExitEventId(Primitive<i32>)),
    ("enterTime" => EnterTime(Primitive<f32>)),
    ("exitTime" => ExitTime(Primitive<f32>)),
}
