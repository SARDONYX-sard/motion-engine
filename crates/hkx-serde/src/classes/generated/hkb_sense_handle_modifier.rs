//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSenseHandleModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbSenseHandleModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x2a064d99`
/// -   version: 2
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSenseHandleModifier<'a> {
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
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "handle", skip_serializing)]
    Handle(HkbHandle<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"sensorLocalOffset"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorLocalOffset")]
    SensorLocalOffset(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ranges"`
    /// -   type: `hkArray&lt;struct hkbSenseHandleModifierRange&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ranges")]
    Ranges(HkArrayClass<HkbSenseHandleModifierRange>),
    /// # C++ Class Fields Info
    /// -   name:`"handleOut"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleOut")]
    HandleOut(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"handleIn"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "handleIn")]
    HandleIn(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"sensorLocalFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorLocalFrameName")]
    SensorLocalFrameName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"minDistance"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDistance")]
    MinDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxDistance"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxDistance")]
    MaxDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"distanceOut"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distanceOut")]
    DistanceOut(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"sensorRagdollBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorRagdollBoneIndex")]
    SensorRagdollBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"sensorAnimationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 142
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensorAnimationBoneIndex")]
    SensorAnimationBoneIndex(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"sensingMode"`
    /// -   type: `enum SensingMode`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sensingMode")]
    SensingMode(Primitive<SensingMode>),
    /// # C++ Class Fields Info
    /// -   name:`"extrapolateSensorPosition"`
    /// -   type: `hkBool`
    /// - offset: 145
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolateSensorPosition")]
    ExtrapolateSensorPosition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"keepFirstSensedHandle"`
    /// -   type: `hkBool`
    /// - offset: 146
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepFirstSensedHandle")]
    KeepFirstSensedHandle(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"foundHandleOut"`
    /// -   type: `hkBool`
    /// - offset: 147
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "foundHandleOut")]
    FoundHandleOut(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rangeIndexForEventToSendNextUpdate"`
    /// -   type: `hkInt32`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rangeIndexForEventToSendNextUpdate", skip_serializing)]
    RangeIndexForEventToSendNextUpdate(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSenseHandleModifier<'de>, "@name",
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
    ("handle" => Handle(HkbHandle<'de>)),
    ("sensorLocalOffset" => SensorLocalOffset(Vector4<f32>)),
    ("ranges" => Ranges(HkArrayClass<HkbSenseHandleModifierRange>)),
    ("handleOut" => HandleOut(Primitive<Cow<'de, str>>)),
    ("handleIn" => HandleIn(Primitive<Cow<'de, str>>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'de, str>>)),
    ("sensorLocalFrameName" => SensorLocalFrameName(Primitive<Cow<'de, str>>)),
    ("minDistance" => MinDistance(Primitive<f32>)),
    ("maxDistance" => MaxDistance(Primitive<f32>)),
    ("distanceOut" => DistanceOut(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("sensorRagdollBoneIndex" => SensorRagdollBoneIndex(Primitive<i16>)),
    ("sensorAnimationBoneIndex" => SensorAnimationBoneIndex(Primitive<i16>)),
    ("sensingMode" => SensingMode(Primitive<SensingMode>)),
    ("extrapolateSensorPosition" => ExtrapolateSensorPosition(Primitive<bool>)),
    ("keepFirstSensedHandle" => KeepFirstSensedHandle(Primitive<bool>)),
    ("foundHandleOut" => FoundHandleOut(Primitive<bool>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
    ("rangeIndexForEventToSendNextUpdate" => RangeIndexForEventToSendNextUpdate(Primitive<i32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SensingMode {
    #[serde(rename = "SENSE_IN_NEARBY_RIGID_BODIES")]
    SenseInNearbyRigidBodies = 0,
    #[serde(rename = "SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER")]
    SenseInRigidBodiesOutsideThisCharacter = 1,
    #[serde(rename = "SENSE_IN_OTHER_CHARACTER_RIGID_BODIES")]
    SenseInOtherCharacterRigidBodies = 2,
    #[serde(rename = "SENSE_IN_THIS_CHARACTER_RIGID_BODIES")]
    SenseInThisCharacterRigidBodies = 3,
    #[serde(rename = "SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES")]
    SenseInGivenCharacterRigidBodies = 4,
    #[serde(rename = "SENSE_IN_GIVEN_RIGID_BODY")]
    SenseInGivenRigidBody = 5,
    #[serde(rename = "SENSE_IN_OTHER_CHARACTER_SKELETON")]
    SenseInOtherCharacterSkeleton = 6,
    #[serde(rename = "SENSE_IN_THIS_CHARACTER_SKELETON")]
    SenseInThisCharacterSkeleton = 7,
    #[serde(rename = "SENSE_IN_GIVEN_CHARACTER_SKELETON")]
    SenseInGivenCharacterSkeleton = 8,
    #[serde(rename = "SENSE_IN_GIVEN_LOCAL_FRAME_GROUP")]
    SenseInGivenLocalFrameGroup = 9,
}
