//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterControllerModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacterControllerModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xf675d6fb`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerModifier {
    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbCharacterControllerControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbCharacterControllerControlData),
    /// # C++ Class Fields Info
    /// -   name:`"initialVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialVelocity")]
    InitialVelocity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"initialVelocityCoordinates"`
    /// -   type: `enum InitialVelocityCoordinates`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialVelocityCoordinates")]
    InitialVelocityCoordinates(Primitive<InitialVelocityCoordinates>),
    /// # C++ Class Fields Info
    /// -   name:`"motionMode"`
    /// -   type: `enum MotionMode`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionMode")]
    MotionMode(Primitive<MotionMode>),
    /// # C++ Class Fields Info
    /// -   name:`"forceDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceDownwardMomentum")]
    ForceDownwardMomentum(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"applyGravity"`
    /// -   type: `hkBool`
    /// - offset: 99
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "applyGravity")]
    ApplyGravity(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"setInitialVelocity"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setInitialVelocity")]
    SetInitialVelocity(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 101
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "gravity", skip_serializing)]
    Gravity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timestep", skip_serializing)]
    Timestep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isInitialVelocityAdded", skip_serializing)]
    IsInitialVelocityAdded(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifier, "@name",
    ("controlData" => ControlData(HkbCharacterControllerControlData)),
    ("initialVelocity" => InitialVelocity(Vector4<f32>)),
    ("initialVelocityCoordinates" => InitialVelocityCoordinates(Primitive<InitialVelocityCoordinates>)),
    ("motionMode" => MotionMode(Primitive<MotionMode>)),
    ("forceDownwardMomentum" => ForceDownwardMomentum(Primitive<bool>)),
    ("applyGravity" => ApplyGravity(Primitive<bool>)),
    ("setInitialVelocity" => SetInitialVelocity(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
    ("gravity" => Gravity(Vector4<f32>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("isInitialVelocityAdded" => IsInitialVelocityAdded(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InitialVelocityCoordinates {
    #[serde(rename = "INITIAL_VELOCITY_IN_WORLD_COORDINATES")]
    InitialVelocityInWorldCoordinates = 0,
    #[serde(rename = "INITIAL_VELOCITY_IN_MODEL_COORDINATES")]
    InitialVelocityInModelCoordinates = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MotionMode {
    #[serde(rename = "MOTION_MODE_FOLLOW_ANIMATION")]
    MotionModeFollowAnimation = 0,
    #[serde(rename = "MOTION_MODE_DYNAMIC")]
    MotionModeDynamic = 1,
}
