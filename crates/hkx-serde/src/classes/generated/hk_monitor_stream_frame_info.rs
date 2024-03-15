//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMonitorStreamFrameInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkMonitorStreamFrameInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: false
/// - signature: `0x7798b7db`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMonitorStreamFrameInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"heading"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "heading")]
    Heading(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"indexOfTimer0"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfTimer0")]
    IndexOfTimer0(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"indexOfTimer1"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfTimer1")]
    IndexOfTimer1(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"absoluteTimeCounter"`
    /// -   type: `enum AbsoluteTimeCounter`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absoluteTimeCounter")]
    AbsoluteTimeCounter(Primitive<AbsoluteTimeCounter>),
    /// # C++ Class Fields Info
    /// -   name:`"timerFactor0"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timerFactor0")]
    TimerFactor0(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timerFactor1"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timerFactor1")]
    TimerFactor1(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"threadId"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "threadId")]
    ThreadId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"frameStreamStart"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameStreamStart")]
    FrameStreamStart(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"frameStreamEnd"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameStreamEnd")]
    FrameStreamEnd(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamFrameInfo<'de>, "@name",
    ("heading" => Heading(Primitive<Cow<'de, str>>)),
    ("indexOfTimer0" => IndexOfTimer0(Primitive<i32>)),
    ("indexOfTimer1" => IndexOfTimer1(Primitive<i32>)),
    ("absoluteTimeCounter" => AbsoluteTimeCounter(Primitive<AbsoluteTimeCounter>)),
    ("timerFactor0" => TimerFactor0(Primitive<f32>)),
    ("timerFactor1" => TimerFactor1(Primitive<f32>)),
    ("threadId" => ThreadId(Primitive<i32>)),
    ("frameStreamStart" => FrameStreamStart(Primitive<i32>)),
    ("frameStreamEnd" => FrameStreamEnd(Primitive<i32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AbsoluteTimeCounter {
    #[serde(rename = "ABSOLUTE_TIME_TIMER_0")]
    AbsoluteTimeTimer0 = 0,
    #[serde(rename = "ABSOLUTE_TIME_TIMER_1")]
    AbsoluteTimeTimer1 = 1,
    #[serde(rename = "ABSOLUTE_TIME_NOT_TIMED")]
    AbsoluteTimeNotTimed = -1,
}
