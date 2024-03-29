//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbParticleSystemEventPayload`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbParticleSystemEventPayload`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: true
/// -    parent: `hkbEventPayload`/`0xda8c7d7d`
/// - signature: `0x9df46cd6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbParticleSystemEventPayload {
    // C++ Parent class(`hkbEventPayload` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"type"`
    /// -   type: `enum SystemType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<SystemType>),
    /// # C++ Class Fields Info
    /// -   name:`"emitBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "emitBoneIndex")]
    EmitBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"direction"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "direction")]
    Direction(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"numParticles"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numParticles")]
    NumParticles(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"speed"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "speed")]
    Speed(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbParticleSystemEventPayload, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<SystemType>)),
    ("emitBoneIndex" => EmitBoneIndex(Primitive<i16>)),
    ("offset" => Offset(Primitive<Vector4<f32>>)),
    ("direction" => Direction(Primitive<Vector4<f32>>)),
    ("numParticles" => NumParticles(Primitive<i32>)),
    ("speed" => Speed(Primitive<f32>)),
}

impl ByteDeSerialize for HkbParticleSystemEventPayload {
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
pub enum SystemType {
    #[serde(rename = "DEBRIS")]
    Debris = 0,
    #[serde(rename = "DUST")]
    Dust = 1,
    #[serde(rename = "EXPLOSION")]
    Explosion = 2,
    #[serde(rename = "SMOKE")]
    Smoke = 3,
    #[serde(rename = "SPARKS")]
    Sparks = 4,
}
