//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBallGun`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpBallGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpFirstPersonGun`/`0x852ab70b`
/// - signature: `0x57b06d35`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallGun<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"bulletRadius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletRadius")]
    BulletRadius(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bulletVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletVelocity")]
    BulletVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"bulletMass"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletMass")]
    BulletMass(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damageMultiplier")]
    DamageMultiplier(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxBulletsInWorld"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxBulletsInWorld")]
    MaxBulletsInWorld(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"bulletOffsetFromCenter"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletOffsetFromCenter")]
    BulletOffsetFromCenter(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"addedBodies"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "addedBodies", skip_serializing)]
    AddedBodies(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallGun<'de>, "@name",
    ("bulletRadius" => BulletRadius(Primitive<f32>)),
    ("bulletVelocity" => BulletVelocity(Primitive<f32>)),
    ("bulletMass" => BulletMass(Primitive<f32>)),
    ("damageMultiplier" => DamageMultiplier(Primitive<f32>)),
    ("maxBulletsInWorld" => MaxBulletsInWorld(Primitive<i32>)),
    ("bulletOffsetFromCenter" => BulletOffsetFromCenter(Vector4<f32>)),
    ("addedBodies" => AddedBodies(Cow<'de, str>)),
}
