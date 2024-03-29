//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMotionState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkMotionState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: false
/// - signature: `0x5797386e`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMotionState {
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"sweptTransform"`
    /// -   type: `struct hkSweptTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sweptTransform")]
    SweptTransform(SingleClass<HkSweptTransform>),
    /// # C++ Class Fields Info
    /// -   name:`"deltaAngle"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deltaAngle")]
    DeltaAngle(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"objectRadius"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectRadius")]
    ObjectRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"linearDamping"`
    /// -   type: `hkHalf`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearDamping")]
    LinearDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"angularDamping"`
    /// -   type: `hkHalf`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularDamping")]
    AngularDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeFactor"`
    /// -   type: `hkHalf`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeFactor")]
    TimeFactor(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkUint8`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAngularVelocity"`
    /// -   type: `hkUint8`
    /// - offset: 171
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngularVelocity")]
    MaxAngularVelocity(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationClass"`
    /// -   type: `hkUint8`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationClass")]
    DeactivationClass(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMotionState, "@name",
    ("transform" => Transform(Primitive<Transform<f32>>)),
    ("sweptTransform" => SweptTransform(SingleClass<HkSweptTransform>)),
    ("deltaAngle" => DeltaAngle(Primitive<Vector4<f32>>)),
    ("objectRadius" => ObjectRadius(Primitive<f32>)),
    ("linearDamping" => LinearDamping(Primitive<f32>)),
    ("angularDamping" => AngularDamping(Primitive<f32>)),
    ("timeFactor" => TimeFactor(Primitive<f32>)),
    ("maxLinearVelocity" => MaxLinearVelocity(Primitive<u8>)),
    ("maxAngularVelocity" => MaxAngularVelocity(Primitive<u8>)),
    ("deactivationClass" => DeactivationClass(Primitive<u8>)),
}

impl ByteDeSerialize for HkMotionState {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
