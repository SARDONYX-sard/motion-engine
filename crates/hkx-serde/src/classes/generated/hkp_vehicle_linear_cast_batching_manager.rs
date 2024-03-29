//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleLinearCastBatchingManager`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleLinearCastBatchingManager`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkpVehicleCastBatchingManager`/`0x53340a9`
/// - signature: `0xed529f13`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleLinearCastBatchingManager<'a> {
    /// # C++ Parent class(`hkpVehicleCastBatchingManager` => parent: `hkpVehicleManager`) field Info
    /// -   name:`"totalNumWheels"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "totalNumWheels")]
    TotalNumWheels(Primitive<u16>),

    /// # C++ Parent class(`hkpVehicleManager` => parent: `hkReferencedObject`) field Info
    /// -   name:`"registeredVehicles"`
    /// -   type: `hkArray<hkpVehicleInstance*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "registeredVehicles")]
    RegisteredVehicles(HkArrayRef<Cow<'a, str>>),

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastBatchingManager<'de>, "@name",
    ("totalNumWheels" => TotalNumWheels(Primitive<u16>)),
    ("registeredVehicles" => RegisteredVehicles(HkArrayRef<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
}

impl ByteDeSerialize for HkpVehicleLinearCastBatchingManager<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
