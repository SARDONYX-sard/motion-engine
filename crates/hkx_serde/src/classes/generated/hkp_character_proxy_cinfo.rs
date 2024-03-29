//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCharacterProxyCinfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpCharacterProxyCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkpCharacterControllerCinfo`/`0xda8c7d7d`
/// - signature: `0x586d97b2`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCharacterProxyCinfo<'a> {
    // C++ Parent class(`hkpCharacterControllerCinfo` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"velocity"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocity")]
    Velocity(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"dynamicFriction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dynamicFriction")]
    DynamicFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticFriction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticFriction")]
    StaticFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"keepContactTolerance"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepContactTolerance")]
    KeepContactTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"extraUpStaticFriction"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraUpStaticFriction")]
    ExtraUpStaticFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extraDownStaticFriction"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraDownStaticFriction")]
    ExtraDownStaticFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"shapePhantom"`
    /// -   type: `struct hkpShapePhantom*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapePhantom")]
    ShapePhantom(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"keepDistance"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepDistance")]
    KeepDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"contactAngleSensitivity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactAngleSensitivity")]
    ContactAngleSensitivity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"userPlanes"`
    /// -   type: `hkUint32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userPlanes")]
    UserPlanes(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxCharacterSpeedForSolver"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCharacterSpeedForSolver")]
    MaxCharacterSpeedForSolver(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterStrength"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterStrength")]
    CharacterStrength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterMass"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterMass")]
    CharacterMass(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"penetrationRecoverySpeed"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "penetrationRecoverySpeed")]
    PenetrationRecoverySpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxCastIterations"`
    /// -   type: `hkInt32`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCastIterations")]
    MaxCastIterations(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"refreshManifoldInCheckSupport"`
    /// -   type: `hkBool`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refreshManifoldInCheckSupport")]
    RefreshManifoldInCheckSupport(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCharacterProxyCinfo<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("position" => Position(Primitive<Vector4<f32>>)),
    ("velocity" => Velocity(Primitive<Vector4<f32>>)),
    ("dynamicFriction" => DynamicFriction(Primitive<f32>)),
    ("staticFriction" => StaticFriction(Primitive<f32>)),
    ("keepContactTolerance" => KeepContactTolerance(Primitive<f32>)),
    ("up" => Up(Primitive<Vector4<f32>>)),
    ("extraUpStaticFriction" => ExtraUpStaticFriction(Primitive<f32>)),
    ("extraDownStaticFriction" => ExtraDownStaticFriction(Primitive<f32>)),
    ("shapePhantom" => ShapePhantom(Primitive<Cow<'de, str>>)),
    ("keepDistance" => KeepDistance(Primitive<f32>)),
    ("contactAngleSensitivity" => ContactAngleSensitivity(Primitive<f32>)),
    ("userPlanes" => UserPlanes(Primitive<u32>)),
    ("maxCharacterSpeedForSolver" => MaxCharacterSpeedForSolver(Primitive<f32>)),
    ("characterStrength" => CharacterStrength(Primitive<f32>)),
    ("characterMass" => CharacterMass(Primitive<f32>)),
    ("maxSlope" => MaxSlope(Primitive<f32>)),
    ("penetrationRecoverySpeed" => PenetrationRecoverySpeed(Primitive<f32>)),
    ("maxCastIterations" => MaxCastIterations(Primitive<i32>)),
    ("refreshManifoldInCheckSupport" => RefreshManifoldInCheckSupport(Primitive<bool>)),
}

impl ByteDeSerialize for HkpCharacterProxyCinfo<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
