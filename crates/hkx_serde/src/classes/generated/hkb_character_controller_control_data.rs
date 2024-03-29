//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterControllerControlData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbCharacterControllerControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x5b6c03d9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerControlData {
    /// # C++ Class Fields Info
    /// -   name:`"desiredVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "desiredVelocity")]
    DesiredVelocity(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalGain")]
    VerticalGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"horizontalCatchUpGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "horizontalCatchUpGain")]
    HorizontalCatchUpGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalSeparation")]
    MaxVerticalSeparation(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxHorizontalSeparation")]
    MaxHorizontalSeparation(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerControlData, "@name",
    ("desiredVelocity" => DesiredVelocity(Primitive<Vector4<f32>>)),
    ("verticalGain" => VerticalGain(Primitive<f32>)),
    ("horizontalCatchUpGain" => HorizontalCatchUpGain(Primitive<f32>)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(Primitive<f32>)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(Primitive<f32>)),
}

impl ByteDeSerialize for HkbCharacterControllerControlData {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
