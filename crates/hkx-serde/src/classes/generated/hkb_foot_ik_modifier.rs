//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbFootIkModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbFootIkModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xed8966c0`
/// -   version: 3
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifier {
    /// # C++ Class Fields Info
    /// -   name:`"gains"`
    /// -   type: `struct hkbFootIkGains`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gains")]
    Gains(HkbFootIkGains),
    /// # C++ Class Fields Info
    /// -   name:`"legs"`
    /// -   type: `hkArray&lt;struct hkbFootIkModifierLeg&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(HkArrayClass<HkbFootIkModifierLeg>),
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceUp"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceUp")]
    RaycastDistanceUp(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"raycastDistanceDown"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raycastDistanceDown")]
    RaycastDistanceDown(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"originalGroundHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalGroundHeightMS")]
    OriginalGroundHeightMs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"errorOut"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOut")]
    ErrorOut(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"errorOutTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOutTranslation")]
    ErrorOutTranslation(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalOffset")]
    VerticalOffset(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"forwardAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardAlignFraction")]
    ForwardAlignFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"sidewaysAlignFraction"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysAlignFraction")]
    SidewaysAlignFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"sidewaysSampleWidth"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sidewaysSampleWidth")]
    SidewaysSampleWidth(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"useTrackData"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useTrackData")]
    UseTrackData(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"lockFeetWhenPlanted"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lockFeetWhenPlanted")]
    LockFeetWhenPlanted(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"useCharacterUpVector"`
    /// -   type: `hkBool`
    /// - offset: 182
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useCharacterUpVector")]
    UseCharacterUpVector(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"alignMode"`
    /// -   type: `enum AlignMode`
    /// - offset: 183
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignMode")]
    AlignMode(AlignMode),
    /// # C++ Class Fields Info
    /// -   name:`"internalLegData"`
    /// -   type: `hkArray&lt;struct hkbFootIkModifierInternalLegData&gt;`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalLegData", skip_serializing)]
    InternalLegData(HkArrayClass<HkbFootIkModifierInternalLegData>),
    /// # C++ Class Fields Info
    /// -   name:`"prevIsFootIkEnabled"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "prevIsFootIkEnabled", skip_serializing)]
    PrevIsFootIkEnabled(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isSetUp"`
    /// -   type: `hkBool`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isSetUp", skip_serializing)]
    IsSetUp(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isGroundPositionValid"`
    /// -   type: `hkBool`
    /// - offset: 201
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isGroundPositionValid", skip_serializing)]
    IsGroundPositionValid(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifier, "@name",
    ("gains" => Gains(HkbFootIkGains)),
    ("legs" => Legs(HkArrayClass<HkbFootIkModifierLeg>)),
    ("raycastDistanceUp" => RaycastDistanceUp(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(Primitive<f32>)),
    ("errorOut" => ErrorOut(Primitive<f32>)),
    ("errorOutTranslation" => ErrorOutTranslation(Vector4<f32>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(Quaternion<f32>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("forwardAlignFraction" => ForwardAlignFraction(Primitive<f32>)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(Primitive<f32>)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(Primitive<f32>)),
    ("useTrackData" => UseTrackData(Primitive<bool>)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(Primitive<bool>)),
    ("useCharacterUpVector" => UseCharacterUpVector(Primitive<bool>)),
    ("alignMode" => AlignMode(AlignMode)),
    ("internalLegData" => InternalLegData(HkArrayClass<HkbFootIkModifierInternalLegData>)),
    ("prevIsFootIkEnabled" => PrevIsFootIkEnabled(Primitive<f32>)),
    ("isSetUp" => IsSetUp(Primitive<bool>)),
    ("isGroundPositionValid" => IsGroundPositionValid(Primitive<bool>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlignMode {
    #[serde(rename = "ALIGN_MODE_FORWARD_RIGHT")]
    AlignModeForwardRight = 0,
    #[serde(rename = "ALIGN_MODE_FORWARD")]
    AlignModeForward = 1,
}
