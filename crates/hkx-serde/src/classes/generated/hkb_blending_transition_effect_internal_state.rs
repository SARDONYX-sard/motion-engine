//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbBlendingTransitionEffectInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBlendingTransitionEffectInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xb18c70c2`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendingTransitionEffectInternalState {
    /// # C++ Class Fields Info
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray&lt;hkQsTransform&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPoseAtBeginningOfTransition")]
    CharacterPoseAtBeginningOfTransition(HkArrayVector<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeRemaining")]
    TimeRemaining(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeInTransition")]
    TimeInTransition(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "applySelfTransition")]
    ApplySelfTransition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initializeCharacterPose")]
    InitializeCharacterPose(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffectInternalState, "@name",
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(HkArrayVector<QsTransform<f32>>)),
    ("timeRemaining" => TimeRemaining(Primitive<f32>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("applySelfTransition" => ApplySelfTransition(Primitive<bool>)),
    ("initializeCharacterPose" => InitializeCharacterPose(Primitive<bool>)),
}
