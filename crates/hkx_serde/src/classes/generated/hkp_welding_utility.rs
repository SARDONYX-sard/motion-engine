//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWeldingUtility`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpWeldingUtility`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0xb2b41feb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@name")]
pub enum HkpWeldingUtility {
}

impl ByteDeSerialize for HkpWeldingUtility {
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
pub enum WeldingType {
    #[serde(rename = "WELDING_TYPE_ANTICLOCKWISE")]
    WeldingTypeAnticlockwise = 0,
    #[serde(rename = "WELDING_TYPE_CLOCKWISE")]
    WeldingTypeClockwise = 4,
    #[serde(rename = "WELDING_TYPE_TWO_SIDED")]
    WeldingTypeTwoSided = 5,
    #[serde(rename = "WELDING_TYPE_NONE")]
    WeldingTypeNone = 6,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SectorType {
    #[serde(rename = "ACCEPT_0")]
    Accept0 = 1,
    #[serde(rename = "SNAP_0")]
    Snap0 = 0,
    #[serde(rename = "REJECT")]
    Reject = 2,
    #[serde(rename = "SNAP_1")]
    Snap1 = 4,
    #[serde(rename = "ACCEPT_1")]
    Accept1 = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum NumAngles {
    #[serde(rename = "NUM_ANGLES")]
    NumAngles = 31,
}
