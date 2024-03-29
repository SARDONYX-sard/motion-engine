//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbWorldEnums`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@name")]
pub enum HkbWorldEnums {
}

impl ByteDeSerialize for HkbWorldEnums {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
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

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AccumulateMotionState {
    #[serde(rename = "ACCUMULATE_MOTION")]
    AccumulateMotion = 0,
    #[serde(rename = "DO_NOT_ACCUMULATE_MOTION")]
    DoNotAccumulateMotion = 1,
}
