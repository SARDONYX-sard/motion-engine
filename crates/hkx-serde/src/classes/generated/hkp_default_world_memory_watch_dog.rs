//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpDefaultWorldMemoryWatchDog`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpDefaultWorldMemoryWatchDog`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkWorldMemoryAvailableWatchDog`/`0xda8c7d7d`
/// - signature: `0x77d6b19f`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDefaultWorldMemoryWatchDog {
    /// # C++ Class Fields Info
    /// -   name:`"freeHeapMemoryRequested"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "freeHeapMemoryRequested")]
    FreeHeapMemoryRequested(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpDefaultWorldMemoryWatchDog, "@name",
    ("freeHeapMemoryRequested" => FreeHeapMemoryRequested(Primitive<i32>)),
}
