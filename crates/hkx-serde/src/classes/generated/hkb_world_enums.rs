//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbWorldEnums`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbWorldEnums`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x25640b46`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbWorldEnums {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbWorldEnums, "@name",
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulationState {
    #[serde(rename = "SIMULATION_STATE_PLAY")]
    SimulationStatePlay = 0,
    #[serde(rename = "SIMULATION_STATE_PAUSE")]
    SimulationStatePause = 1,
    #[serde(rename = "SIMULATION_STATE_STEP")]
    SimulationStateStep = 2,
    #[serde(rename = "SIMULATION_STATE_STOP")]
    SimulationStateStop = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccumulateMotionState {
    #[serde(rename = "ACCUMULATE_MOTION")]
    AccumulateMotion = 0,
    #[serde(rename = "DO_NOT_ACCUMULATE_MOTION")]
    DoNotAccumulateMotion = 1,
}
