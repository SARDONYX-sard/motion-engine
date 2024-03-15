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
    /// # C++ Parent class(`hkbModifier`, parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable", default)]
    Enable(Primitive<bool>),
    /// # C++ Parent class(`hkbModifier`, parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padModifier", default, skip_serializing)]
    PadModifier([Primitive<bool>; 3]),

    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", default, skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", default, skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", default, skip_serializing)]
    PadNode([Primitive<bool>; 1]),

    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet", default)]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", default, skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", default, skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"controlData"`
    /// -   type: `struct hkbCharacterControllerControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData", default)]
    ControlData(HkbCharacterControllerControlData),
    /// # C++ Class Fields Info
    /// -   name:`"initialVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialVelocity", default)]
    InitialVelocity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"initialVelocityCoordinates"`
    /// -   type: `enum InitialVelocityCoordinates`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialVelocityCoordinates", default)]
    InitialVelocityCoordinates(Primitive<InitialVelocityCoordinates>),
    /// # C++ Class Fields Info
    /// -   name:`"motionMode"`
    /// -   type: `enum MotionMode`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionMode", default)]
    MotionMode(Primitive<MotionMode>),
    /// # C++ Class Fields Info
    /// -   name:`"forceDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceDownwardMomentum", default)]
    ForceDownwardMomentum(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"applyGravity"`
    /// -   type: `hkBool`
    /// - offset: 99
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "applyGravity", default)]
    ApplyGravity(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"setInitialVelocity"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setInitialVelocity", default)]
    SetInitialVelocity(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 101
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround", default)]
    IsTouchingGround(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "gravity", default, skip_serializing)]
    Gravity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timestep", default, skip_serializing)]
    Timestep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isInitialVelocityAdded", default, skip_serializing)]
    IsInitialVelocityAdded(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifier, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier([Primitive<bool>; 3])),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<Unknown>)),
    ("padNode" => PadNode([Primitive<bool>; 1])),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<Primitive<()>>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
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
