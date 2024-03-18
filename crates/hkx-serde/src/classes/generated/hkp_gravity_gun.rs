//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpGravityGun`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

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
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpGravityGun<'a> {
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"keyboardKey"`
    /// -   type: `enum KeyboardKey`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyboardKey")]
    KeyboardKey(Primitive<KeyboardKey>),
    /// # C++ Parent class(`hkpFirstPersonGun` => parent: `hkReferencedObject`) field Info
    /// -   name:`"listeners"`
    /// -   type: `hkArray<void*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(HkArrayRef<Cow<'a, str>>),

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
    /// -   name:`"grabbedBodies"`
    /// -   type: `hkArray<void*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
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
    ("type" => Type(Primitive<()>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("keyboardKey" => KeyboardKey(Primitive<KeyboardKey>)),
    ("listeners" => Listeners(HkArrayRef<Cow<'de, str>>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("grabbedBodies" => GrabbedBodies(HkArrayRef<Cow<'de, str>>)),
    ("maxNumObjectsPicked" => MaxNumObjectsPicked(Primitive<i32>)),
    ("maxMassOfObjectPicked" => MaxMassOfObjectPicked(Primitive<f32>)),
    ("maxDistOfObjectPicked" => MaxDistOfObjectPicked(Primitive<f32>)),
    ("impulseAppliedWhenObjectNotPicked" => ImpulseAppliedWhenObjectNotPicked(Primitive<f32>)),
    ("throwVelocity" => ThrowVelocity(Primitive<f32>)),
    ("capturedObjectPosition" => CapturedObjectPosition(Vector4<f32>)),
    ("capturedObjectsOffset" => CapturedObjectsOffset(Vector4<f32>)),
}
