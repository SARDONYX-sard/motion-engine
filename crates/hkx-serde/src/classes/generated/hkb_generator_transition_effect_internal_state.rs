//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGeneratorTransitionEffectInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbGeneratorTransitionEffectInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xd6692b5d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGeneratorTransitionEffectInternalState {
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

    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeInTransition")]
    TimeInTransition(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendInDuration"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "effectiveBlendInDuration")]
    EffectiveBlendInDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"effectiveBlendOutDuration"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "effectiveBlendOutDuration")]
    EffectiveBlendOutDuration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorState"`
    /// -   type: `enum ToGeneratorState`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toGeneratorState")]
    ToGeneratorState(Primitive<ToGeneratorState>),
    /// # C++ Class Fields Info
    /// -   name:`"echoTransitionGenerator"`
    /// -   type: `hkBool`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoTransitionGenerator")]
    EchoTransitionGenerator(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"echoToGenerator"`
    /// -   type: `hkBool`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "echoToGenerator")]
    EchoToGenerator(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"justActivated"`
    /// -   type: `hkBool`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "justActivated")]
    JustActivated(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "updateActiveNodes")]
    UpdateActiveNodes(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"stage"`
    /// -   type: `enum Stage`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stage")]
    Stage(Primitive<Stage>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGeneratorTransitionEffectInternalState, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("duration" => Duration(Primitive<f32>)),
    ("effectiveBlendInDuration" => EffectiveBlendInDuration(Primitive<f32>)),
    ("effectiveBlendOutDuration" => EffectiveBlendOutDuration(Primitive<f32>)),
    ("toGeneratorState" => ToGeneratorState(Primitive<ToGeneratorState>)),
    ("echoTransitionGenerator" => EchoTransitionGenerator(Primitive<bool>)),
    ("echoToGenerator" => EchoToGenerator(Primitive<bool>)),
    ("justActivated" => JustActivated(Primitive<bool>)),
    ("updateActiveNodes" => UpdateActiveNodes(Primitive<bool>)),
    ("stage" => Stage(Primitive<Stage>)),
}
