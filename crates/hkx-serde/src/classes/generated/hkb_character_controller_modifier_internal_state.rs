//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkbCharacterControllerModifierInternalState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCharacterControllerModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xf8dfec0d`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerModifierInternalState {
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timestep")]
    Timestep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isInitialVelocityAdded")]
    IsInitialVelocityAdded(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 37
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifierInternalState, "@name",
    ("gravity" => Gravity(Vector4<f32>)),
    ("timestep" => Timestep(Primitive<f32>)),
    ("isInitialVelocityAdded" => IsInitialVelocityAdded(Primitive<bool>)),
    ("isTouchingGround" => IsTouchingGround(Primitive<bool>)),
}
