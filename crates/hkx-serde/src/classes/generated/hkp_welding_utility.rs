//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWeldingUtility`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWeldingUtility {
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWeldingUtility, "@name",
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumAngles {
    #[serde(rename = "NUM_ANGLES")]
    NumAngles = 31,
}
