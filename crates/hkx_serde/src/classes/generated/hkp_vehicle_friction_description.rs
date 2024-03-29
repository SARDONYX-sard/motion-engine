//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionDescription`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleFrictionDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: false
/// - signature: `0x1034549a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionDescription {
    /// # C++ Class Fields Info
    /// -   name:`"wheelDistance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wheelDistance")]
    WheelDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"chassisMassInv"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chassisMassInv")]
    ChassisMassInv(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"axleDescr"`
    /// -   type: `struct hkpVehicleFrictionDescriptionAxisDescription[2]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "axleDescr")]
    AxleDescr(CStyleArrayClass<HkpVehicleFrictionDescriptionAxisDescription, 2>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescription, "@name",
    ("wheelDistance" => WheelDistance(Primitive<f32>)),
    ("chassisMassInv" => ChassisMassInv(Primitive<f32>)),
    ("axleDescr" => AxleDescr(CStyleArrayClass<HkpVehicleFrictionDescriptionAxisDescription, 2>)),
}

impl ByteDeSerialize for HkpVehicleFrictionDescription {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
