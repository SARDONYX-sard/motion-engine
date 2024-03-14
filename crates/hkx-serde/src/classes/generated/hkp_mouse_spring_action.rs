//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpMouseSpringAction`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpMouseSpringAction`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpUnaryAction`/`0x895532c0`
/// - signature: `0x6e087fd6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMouseSpringAction<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"positionInRbLocal"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "positionInRbLocal")]
    PositionInRbLocal(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"mousePositionInWorld"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mousePositionInWorld")]
    MousePositionInWorld(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"springDamping"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springDamping")]
    SpringDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"springElasticity"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "springElasticity")]
    SpringElasticity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxRelativeForce"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxRelativeForce")]
    MaxRelativeForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"objectDamping"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectDamping")]
    ObjectDamping(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapeKey")]
    ShapeKey(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"applyCallbacks"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "applyCallbacks", skip_serializing)]
    ApplyCallbacks(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMouseSpringAction<'de>, "@name",
    ("positionInRbLocal" => PositionInRbLocal(Vector4<f32>)),
    ("mousePositionInWorld" => MousePositionInWorld(Vector4<f32>)),
    ("springDamping" => SpringDamping(Primitive<f32>)),
    ("springElasticity" => SpringElasticity(Primitive<f32>)),
    ("maxRelativeForce" => MaxRelativeForce(Primitive<f32>)),
    ("objectDamping" => ObjectDamping(Primitive<f32>)),
    ("shapeKey" => ShapeKey(Primitive<u32>)),
    ("applyCallbacks" => ApplyCallbacks(HkArrayRef<Cow<'de, str>>)),
}
