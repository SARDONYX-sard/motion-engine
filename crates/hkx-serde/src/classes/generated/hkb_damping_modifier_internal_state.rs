//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDampingModifierInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbDampingModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x508d3b36`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDampingModifierInternalState {
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
    /// # C++ Class Fields Info
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedVector")]
    DampedVector(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifierInternalState, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("dampedVector" => DampedVector(Primitive<Vector4<f32>>)),
    ("vecErrorSum" => VecErrorSum(Primitive<Vector4<f32>>)),
    ("vecPreviousError" => VecPreviousError(Primitive<Vector4<f32>>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}

impl ByteDeSerialize for HkbDampingModifierInternalState {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
