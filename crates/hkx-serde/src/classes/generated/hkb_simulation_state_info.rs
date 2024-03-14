//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbSimulationStateInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSimulationStateInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa40822b4`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSimulationStateInfo {
    /// # C++ Class Fields Info
    /// -   name:`"simulationState"`
    /// -   type: `enum SimulationState`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulationState")]
    SimulationState(SimulationState),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSimulationStateInfo, "@name",
    ("simulationState" => SimulationState(SimulationState)),
}
