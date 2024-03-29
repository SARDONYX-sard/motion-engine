//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProxyModifierProxyInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbProxyModifierProxyInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x39de637e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProxyModifierProxyInfo {
    /// # C++ Class Fields Info
    /// -   name:`"dynamicFriction"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dynamicFriction")]
    DynamicFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"staticFriction"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticFriction")]
    StaticFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"keepContactTolerance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepContactTolerance")]
    KeepContactTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"keepDistance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepDistance")]
    KeepDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"contactAngleSensitivity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactAngleSensitivity")]
    ContactAngleSensitivity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"userPlanes"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userPlanes")]
    UserPlanes(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxCharacterSpeedForSolver"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCharacterSpeedForSolver")]
    MaxCharacterSpeedForSolver(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterStrength"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterStrength")]
    CharacterStrength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"characterMass"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterMass")]
    CharacterMass(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"penetrationRecoverySpeed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "penetrationRecoverySpeed")]
    PenetrationRecoverySpeed(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxCastIterations"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCastIterations")]
    MaxCastIterations(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"refreshManifoldInCheckSupport"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refreshManifoldInCheckSupport")]
    RefreshManifoldInCheckSupport(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProxyModifierProxyInfo, "@name",
    ("dynamicFriction" => DynamicFriction(Primitive<f32>)),
    ("staticFriction" => StaticFriction(Primitive<f32>)),
    ("keepContactTolerance" => KeepContactTolerance(Primitive<f32>)),
    ("up" => Up(Primitive<Vector4<f32>>)),
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

impl ByteDeSerialize for HkbProxyModifierProxyInfo {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
