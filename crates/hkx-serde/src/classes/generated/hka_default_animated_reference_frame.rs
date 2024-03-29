//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaDefaultAnimatedReferenceFrame`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkaDefaultAnimatedReferenceFrame`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkaAnimatedReferenceFrame`/`0xda8c7d7d`
/// - signature: `0x6d85e445`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaDefaultAnimatedReferenceFrame {
    // C++ Parent class(`hkaAnimatedReferenceFrame` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"forward"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forward")]
    Forward(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"referenceFrameSamples"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceFrameSamples")]
    ReferenceFrameSamples(HkArrayVector<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaDefaultAnimatedReferenceFrame, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("up" => Up(Primitive<Vector4<f32>>)),
    ("forward" => Forward(Primitive<Vector4<f32>>)),
    ("duration" => Duration(Primitive<f32>)),
    ("referenceFrameSamples" => ReferenceFrameSamples(HkArrayVector<Vector4<f32>>)),
}

impl ByteDeSerialize for HkaDefaultAnimatedReferenceFrame {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
