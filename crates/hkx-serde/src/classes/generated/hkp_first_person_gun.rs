//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpFirstPersonGun`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpFirstPersonGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x852ab70b`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpFirstPersonGun<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"keyboardKey"`
    /// -   type: `enum KeyboardKey`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyboardKey")]
    KeyboardKey(Primitive<KeyboardKey>),
    /// # C++ Class Fields Info
    /// -   name:`"listeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpFirstPersonGun<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("type" => Type(Primitive<()>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("keyboardKey" => KeyboardKey(Primitive<KeyboardKey>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WEAPON_TYPE_INVALID")]
    WeaponTypeInvalid = 0,
    #[serde(rename = "WEAPON_TYPE_BALLGUN")]
    WeaponTypeBallgun = 1,
    #[serde(rename = "WEAPON_TYPE_GRENADEGUN")]
    WeaponTypeGrenadegun = 2,
    #[serde(rename = "WEAPON_TYPE_GRAVITYGUN")]
    WeaponTypeGravitygun = 3,
    #[serde(rename = "WEAPON_TYPE_MOUNTEDBALLGUN")]
    WeaponTypeMountedballgun = 4,
    #[serde(rename = "WEAPON_TYPE_TWEAKERGUN")]
    WeaponTypeTweakergun = 5,
    #[serde(rename = "WEAPON_TYPE_MISSILEGUN")]
    WeaponTypeMissilegun = 6,
    #[serde(rename = "WEAPON_TYPE_RAYCASTGUN")]
    WeaponTypeRaycastgun = 7,
    #[serde(rename = "WEAPON_TYPE_SPHEREGUN")]
    WeaponTypeSpheregun = 8,
    #[serde(rename = "WEAPON_TYPE_STICKYGUN")]
    WeaponTypeStickygun = 9,
    #[serde(rename = "WEAPON_TYPE_NUM_TYPES")]
    WeaponTypeNumTypes = 10,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyboardKey {
    #[serde(rename = "KEY_F1")]
    KeyF1 = 112,
    #[serde(rename = "KEY_F2")]
    KeyF2 = 113,
    #[serde(rename = "KEY_F3")]
    KeyF3 = 114,
    #[serde(rename = "KEY_F4")]
    KeyF4 = 115,
    #[serde(rename = "KEY_F5")]
    KeyF5 = 116,
    #[serde(rename = "KEY_F6")]
    KeyF6 = 117,
    #[serde(rename = "KEY_F7")]
    KeyF7 = 118,
    #[serde(rename = "KEY_F8")]
    KeyF8 = 119,
    #[serde(rename = "KEY_F9")]
    KeyF9 = 120,
    #[serde(rename = "KEY_F10")]
    KeyF10 = 121,
    #[serde(rename = "KEY_F11")]
    KeyF11 = 122,
    #[serde(rename = "KEY_F12")]
    KeyF12 = 123,
}
