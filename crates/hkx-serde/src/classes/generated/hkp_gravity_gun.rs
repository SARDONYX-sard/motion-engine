//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGravityGun`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpGravityGun`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpFirstPersonGun`/`0x852ab70b`
/// - signature: `0x5e2754cd`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGravityGun<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"grabbedBodies"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "grabbedBodies", skip_serializing)]
    GrabbedBodies(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"maxNumObjectsPicked"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumObjectsPicked")]
    MaxNumObjectsPicked(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxMassOfObjectPicked"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxMassOfObjectPicked")]
    MaxMassOfObjectPicked(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxDistOfObjectPicked"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxDistOfObjectPicked")]
    MaxDistOfObjectPicked(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"impulseAppliedWhenObjectNotPicked"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "impulseAppliedWhenObjectNotPicked")]
    ImpulseAppliedWhenObjectNotPicked(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"throwVelocity"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "throwVelocity")]
    ThrowVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"capturedObjectPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capturedObjectPosition")]
    CapturedObjectPosition(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"capturedObjectsOffset"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capturedObjectsOffset")]
    CapturedObjectsOffset(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpGravityGun<'de>, "@name",
    ("grabbedBodies" => GrabbedBodies(HkArrayRef<Cow<'de, str>>)),
    ("maxNumObjectsPicked" => MaxNumObjectsPicked(Primitive<i32>)),
    ("maxMassOfObjectPicked" => MaxMassOfObjectPicked(Primitive<f32>)),
    ("maxDistOfObjectPicked" => MaxDistOfObjectPicked(Primitive<f32>)),
    ("impulseAppliedWhenObjectNotPicked" => ImpulseAppliedWhenObjectNotPicked(Primitive<f32>)),
    ("throwVelocity" => ThrowVelocity(Primitive<f32>)),
    ("capturedObjectPosition" => CapturedObjectPosition(Vector4<f32>)),
    ("capturedObjectsOffset" => CapturedObjectsOffset(Vector4<f32>)),
}
