//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxNode`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxNode`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0x5a218502`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxNode<'a> {
    /// # C++ Parent class(`hkxAttributeHolder`, parent: `hkReferencedObject`) field Info
    /// -   name:`"attributeGroups"`
    /// -   type: `hkArray&lt;struct hkxAttributeGroup&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeGroups")]
    AttributeGroups(HkArrayClass<HkxAttributeGroup>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"object"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "object")]
    Object(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"keyFrames"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyFrames")]
    KeyFrames(HkArrayVector<Matrix4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkxNode*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"annotations"`
    /// -   type: `hkArray&lt;struct hkxNodeAnnotationData&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotations")]
    Annotations(HkArrayClass<HkxNodeAnnotationData>),
    /// # C++ Class Fields Info
    /// -   name:`"userProperties"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userProperties")]
    UserProperties(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"selected"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selected")]
    Selected(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxNode<'de>, "@name",
    ("attributeGroups" => AttributeGroups(HkArrayClass<HkxAttributeGroup>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("object" => Object(Primitive<Cow<'de, str>>)),
    ("keyFrames" => KeyFrames(HkArrayVector<Matrix4<f32>>)),
    ("children" => Children(HkArrayRef<Cow<'de, str>>)),
    ("annotations" => Annotations(HkArrayClass<HkxNodeAnnotationData>)),
    ("userProperties" => UserProperties(Primitive<Cow<'de, str>>)),
    ("selected" => Selected(Primitive<bool>)),
}
