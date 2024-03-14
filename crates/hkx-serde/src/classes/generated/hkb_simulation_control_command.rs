//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSimulationControlCommand`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSimulationControlCommand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x2a241367`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSimulationControlCommand {
    /// # C++ Class Fields Info
    /// -   name:`"command"`
    /// -   type: `enum SimulationControlCommand`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "command")]
    Command(Primitive<SimulationControlCommand>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSimulationControlCommand, "@name",
    ("command" => Command(Primitive<SimulationControlCommand>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulationControlCommand {
    #[serde(rename = "COMMAND_PLAY")]
    CommandPlay = 0,
    #[serde(rename = "COMMAND_PAUSE")]
    CommandPause = 1,
    #[serde(rename = "COMMAND_STEP")]
    CommandStep = 2,
    #[serde(rename = "COMMAND_STOP")]
    CommandStop = 3,
    #[serde(rename = "COMMAND_ACCUMULATE_MOTION")]
    CommandAccumulateMotion = 4,
    #[serde(rename = "COMMAND_DO_NOT_ACCUMULATE_MOTION")]
    CommandDoNotAccumulateMotion = 5,
}
