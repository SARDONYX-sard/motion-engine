//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProxyModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbProxyModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x8a41554f`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProxyModifier<'a> {
    /// # C++ Parent class(`hkbModifier`, parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// # C++ Parent class(`hkbModifier`, parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier([Primitive<bool>; 3]),

    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode([Primitive<bool>; 1]),

    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"proxyInfo"`
    /// -   type: `struct hkbProxyModifierProxyInfo`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "proxyInfo")]
    ProxyInfo(HkbProxyModifierProxyInfo),
    /// # C++ Class Fields Info
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocity")]
    LinearVelocity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"horizontalGain"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "horizontalGain")]
    HorizontalGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalGain")]
    VerticalGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxHorizontalSeparation")]
    MaxHorizontalSeparation(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalSeparation")]
    MaxVerticalSeparation(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalDisplacementError"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalDisplacementError")]
    VerticalDisplacementError(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalDisplacementErrorGain"`
    /// -   type: `hkReal`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalDisplacementErrorGain")]
    VerticalDisplacementErrorGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalDisplacement")]
    MaxVerticalDisplacement(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"minVerticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minVerticalDisplacement")]
    MinVerticalDisplacement(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"capsuleHeight"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleHeight")]
    CapsuleHeight(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"capsuleRadius"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleRadius")]
    CapsuleRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSlopeForRotation"`
    /// -   type: `hkReal`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlopeForRotation")]
    MaxSlopeForRotation(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"phantomType"`
    /// -   type: `enum PhantomType`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantomType")]
    PhantomType(Primitive<PhantomType>),
    /// # C++ Class Fields Info
    /// -   name:`"linearVelocityMode"`
    /// -   type: `enum LinearVelocityMode`
    /// - offset: 193
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocityMode")]
    LinearVelocityMode(Primitive<LinearVelocityMode>),
    /// # C++ Class Fields Info
    /// -   name:`"ignoreIncomingRotation"`
    /// -   type: `hkBool`
    /// - offset: 194
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreIncomingRotation")]
    IgnoreIncomingRotation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"ignoreCollisionDuringRotation"`
    /// -   type: `hkBool`
    /// - offset: 195
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreCollisionDuringRotation")]
    IgnoreCollisionDuringRotation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"ignoreIncomingTranslation"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreIncomingTranslation")]
    IgnoreIncomingTranslation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"includeDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 197
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "includeDownwardMomentum")]
    IncludeDownwardMomentum(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"followWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 198
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "followWorldFromModel")]
    FollowWorldFromModel(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 199
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"characterProxy"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterProxy", skip_serializing)]
    CharacterProxy(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantom", skip_serializing)]
    Phantom(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"phantomShape"`
    /// -   type: `void*`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomShape", skip_serializing)]
    PhantomShape(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"horizontalDisplacement"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "horizontalDisplacement", skip_serializing)]
    HorizontalDisplacement(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "verticalDisplacement", skip_serializing)]
    VerticalDisplacement(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 244
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timestep", skip_serializing)]
    Timestep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousFrameFollowWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 248
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "previousFrameFollowWorldFromModel", skip_serializing)]
    PreviousFrameFollowWorldFromModel(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProxyModifier<'de>, "@name",
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
    ("proxyInfo" => ProxyInfo(HkbProxyModifierProxyInfo)),
    ("linearVelocity" => LinearVelocity(Vector4<f32>)),
    ("horizontalGain" => HorizontalGain(Primitive<f32>)),
    ("verticalGain" => VerticalGain(Primitive<f32>)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(Primitive<f32>)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(Primitive<f32>)),
    ("verticalDisplacementError" => VerticalDisplacementError(Primitive<f32>)),
    ("verticalDisplacementErrorGain" => VerticalDisplacementErrorGain(Primitive<f32>)),
    ("maxVerticalDisplacement" => MaxVerticalDisplacement(Primitive<f32>)),
    ("minVerticalDisplacement" => MinVerticalDisplacement(Primitive<f32>)),
    ("capsuleHeight" => CapsuleHeight(Primitive<f32>)),
    ("capsuleRadius" => CapsuleRadius(Primitive<f32>)),
    ("maxSlopeForRotation" => MaxSlopeForRotation(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("phantomType" => PhantomType(Primitive<PhantomType>)),
    ("linearVelocityMode" => LinearVelocityMode(Primitive<LinearVelocityMode>)),
    ("ignoreIncomingRotation" => IgnoreIncomingRotation(Primitive<bool>)),
    ("ignoreCollisionDuringRotation" => IgnoreCollisionDuringRotation(Primitive<bool>)),
    ("ignoreIncomingTranslation" => IgnoreIncomingTranslation(Primitive<bool>)),
    ("includeDownwardMomentum" => IncludeDownwardMomentum(Primitive<bool>)),
    ("followWorldFromModel" => FollowWorldFromModel(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
    ("characterProxy" => CharacterProxy(Primitive<Cow<'de, str>>)),
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("phantomShape" => PhantomShape(Primitive<Cow<'de, str>>)),
    ("horizontalDisplacement" => HorizontalDisplacement(Vector4<f32>)),
    ("verticalDisplacement" => VerticalDisplacement(Primitive<f32>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("previousFrameFollowWorldFromModel" => PreviousFrameFollowWorldFromModel(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhantomType {
    #[serde(rename = "PHANTOM_TYPE_SIMPLE")]
    PhantomTypeSimple = 0,
    #[serde(rename = "PHANTOM_TYPE_CACHING")]
    PhantomTypeCaching = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinearVelocityMode {
    #[serde(rename = "LINEAR_VELOCITY_MODE_WORLD")]
    LinearVelocityModeWorld = 0,
    #[serde(rename = "LINEAR_VELOCITY_MODE_MODEL")]
    LinearVelocityModeModel = 1,
}
