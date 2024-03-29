//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkModifier<'a> {
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"enable"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enable")]
    Enable(Primitive<bool>),
    /// # C++ Parent class(`hkbModifier` => parent: `hkbNode`) field Info
    /// -   name:`"padModifier"`
    /// -   type: `hkBool[3]`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "padModifier", skip_serializing)]
    PadModifier(CStyleArray<[bool; 3]>),

    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

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
    /// -   name:`"gains"`
    /// -   type: `struct hkbFootIkGains`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gains")]
    Gains(SingleClass<HkbFootIkGains>),
    /// # C++ Class Fields Info
    /// -   name:`"legs"`
    /// -   type: `hkArray<struct hkbFootIkModifierLeg>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(HkArrayClass<HkbFootIkModifierLeg<'a>>),
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
    ErrorOutTranslation(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(Primitive<Quaternion<f32>>),
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
    AlignMode(Primitive<AlignMode>),
    /// # C++ Class Fields Info
    /// -   name:`"internalLegData"`
    /// -   type: `hkArray<struct hkbFootIkModifierInternalLegData>`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "internalLegData", skip_serializing)]
    InternalLegData(HkArrayClass<HkbFootIkModifierInternalLegData<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"prevIsFootIkEnabled"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "prevIsFootIkEnabled", skip_serializing)]
    PrevIsFootIkEnabled(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isSetUp"`
    /// -   type: `hkBool`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "isSetUp", skip_serializing)]
    IsSetUp(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isGroundPositionValid"`
    /// -   type: `hkBool`
    /// - offset: 201
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "isGroundPositionValid", skip_serializing)]
    IsGroundPositionValid(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifier<'de>, "@name",
    ("enable" => Enable(Primitive<bool>)),
    ("padModifier" => PadModifier(CStyleArray<[bool; 3]>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("gains" => Gains(SingleClass<HkbFootIkGains>)),
    ("legs" => Legs(HkArrayClass<HkbFootIkModifierLeg<'de>>)),
    ("raycastDistanceUp" => RaycastDistanceUp(Primitive<f32>)),
    ("raycastDistanceDown" => RaycastDistanceDown(Primitive<f32>)),
    ("originalGroundHeightMS" => OriginalGroundHeightMs(Primitive<f32>)),
    ("errorOut" => ErrorOut(Primitive<f32>)),
    ("errorOutTranslation" => ErrorOutTranslation(Primitive<Vector4<f32>>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(Primitive<Quaternion<f32>>)),
    ("verticalOffset" => VerticalOffset(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("forwardAlignFraction" => ForwardAlignFraction(Primitive<f32>)),
    ("sidewaysAlignFraction" => SidewaysAlignFraction(Primitive<f32>)),
    ("sidewaysSampleWidth" => SidewaysSampleWidth(Primitive<f32>)),
    ("useTrackData" => UseTrackData(Primitive<bool>)),
    ("lockFeetWhenPlanted" => LockFeetWhenPlanted(Primitive<bool>)),
    ("useCharacterUpVector" => UseCharacterUpVector(Primitive<bool>)),
    ("alignMode" => AlignMode(Primitive<AlignMode>)),
    ("internalLegData" => InternalLegData(HkArrayClass<HkbFootIkModifierInternalLegData<'de>>)),
    ("prevIsFootIkEnabled" => PrevIsFootIkEnabled(Primitive<f32>)),
    ("isSetUp" => IsSetUp(Primitive<bool>)),
    ("isGroundPositionValid" => IsGroundPositionValid(Primitive<bool>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
}

impl ByteDeSerialize for HkbFootIkModifier<'_> {
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
pub enum AlignMode {
    #[serde(rename = "ALIGN_MODE_FORWARD_RIGHT")]
    AlignModeForwardRight = 0,
    #[serde(rename = "ALIGN_MODE_FORWARD")]
    AlignModeForward = 1,
}
