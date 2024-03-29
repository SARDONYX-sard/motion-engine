//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleInstanceWheelInfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpVehicleInstanceWheelInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 224
/// -    vtable: false
/// - signature: `0x99f693f0`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleInstanceWheelInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"contactPoint"`
    /// -   type: `struct hkContactPoint`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPoint")]
    ContactPoint(SingleClass<HkContactPoint>),
    /// # C++ Class Fields Info
    /// -   name:`"contactFriction"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactFriction")]
    ContactFriction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"contactBody"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "contactBody", skip_serializing)]
    ContactBody(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"contactShapeKey"`
    /// -   type: `hkUint32[8]`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactShapeKey")]
    ContactShapeKey(CStyleArray<[u32; 8]>),
    /// # C++ Class Fields Info
    /// -   name:`"hardPointWs"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hardPointWs")]
    HardPointWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"rayEndPointWs"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rayEndPointWs")]
    RayEndPointWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"currentSuspensionLength"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentSuspensionLength")]
    CurrentSuspensionLength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"suspensionDirectionWs"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "suspensionDirectionWs")]
    SuspensionDirectionWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"spinAxisChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinAxisChassisSpace")]
    SpinAxisChassisSpace(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"spinAxisWs"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinAxisWs")]
    SpinAxisWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"steeringOrientationChassisSpace"`
    /// -   type: `hkQuaternion`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "steeringOrientationChassisSpace")]
    SteeringOrientationChassisSpace(Primitive<Quaternion<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"spinVelocity"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinVelocity")]
    SpinVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"spinAngle"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spinAngle")]
    SpinAngle(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"skidEnergyDensity"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skidEnergyDensity")]
    SkidEnergyDensity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"sideForce"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sideForce")]
    SideForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"forwardSlipVelocity"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardSlipVelocity")]
    ForwardSlipVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"sideSlipVelocity"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sideSlipVelocity")]
    SideSlipVelocity(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleInstanceWheelInfo<'de>, "@name",
    ("contactPoint" => ContactPoint(SingleClass<HkContactPoint>)),
    ("contactFriction" => ContactFriction(Primitive<f32>)),
    ("contactBody" => ContactBody(Primitive<Cow<'de, str>>)),
    ("contactShapeKey" => ContactShapeKey(CStyleArray<[u32; 8]>)),
    ("hardPointWs" => HardPointWs(Primitive<Vector4<f32>>)),
    ("rayEndPointWs" => RayEndPointWs(Primitive<Vector4<f32>>)),
    ("currentSuspensionLength" => CurrentSuspensionLength(Primitive<f32>)),
    ("suspensionDirectionWs" => SuspensionDirectionWs(Primitive<Vector4<f32>>)),
    ("spinAxisChassisSpace" => SpinAxisChassisSpace(Primitive<Vector4<f32>>)),
    ("spinAxisWs" => SpinAxisWs(Primitive<Vector4<f32>>)),
    ("steeringOrientationChassisSpace" => SteeringOrientationChassisSpace(Primitive<Quaternion<f32>>)),
    ("spinVelocity" => SpinVelocity(Primitive<f32>)),
    ("spinAngle" => SpinAngle(Primitive<f32>)),
    ("skidEnergyDensity" => SkidEnergyDensity(Primitive<f32>)),
    ("sideForce" => SideForce(Primitive<f32>)),
    ("forwardSlipVelocity" => ForwardSlipVelocity(Primitive<f32>)),
    ("sideSlipVelocity" => SideSlipVelocity(Primitive<f32>)),
}
