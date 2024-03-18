//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraphData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbBehaviorGraphData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x95aca5d`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphData<'a> {
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
    /// -   name:`"attributeDefaults"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeDefaults")]
    AttributeDefaults(HkArrayRef<Primitive<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"variableInfos"`
    /// -   type: `hkArray<struct hkbVariableInfo>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableInfos")]
    VariableInfos(HkArrayClass<HkbVariableInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyInfos"`
    /// -   type: `hkArray<struct hkbVariableInfo>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyInfos")]
    CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"eventInfos"`
    /// -   type: `hkArray<struct hkbEventInfo>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventInfos")]
    EventInfos(HkArrayClass<HkbEventInfo>),
    /// # C++ Class Fields Info
    /// -   name:`"wordMinVariableValues"`
    /// -   type: `hkArray<struct hkbVariableValue>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordMinVariableValues")]
    WordMinVariableValues(HkArrayClass<HkbVariableValue>),
    /// # C++ Class Fields Info
    /// -   name:`"wordMaxVariableValues"`
    /// -   type: `hkArray<struct hkbVariableValue>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordMaxVariableValues")]
    WordMaxVariableValues(HkArrayClass<HkbVariableValue>),
    /// # C++ Class Fields Info
    /// -   name:`"variableInitialValues"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableInitialValues")]
    VariableInitialValues(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbBehaviorGraphStringData*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphData<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("attributeDefaults" => AttributeDefaults(HkArrayRef<Primitive<f32>>)),
    ("variableInfos" => VariableInfos(HkArrayClass<HkbVariableInfo>)),
    ("characterPropertyInfos" => CharacterPropertyInfos(HkArrayClass<HkbVariableInfo>)),
    ("eventInfos" => EventInfos(HkArrayClass<HkbEventInfo>)),
    ("wordMinVariableValues" => WordMinVariableValues(HkArrayClass<HkbVariableValue>)),
    ("wordMaxVariableValues" => WordMaxVariableValues(HkArrayClass<HkbVariableValue>)),
    ("variableInitialValues" => VariableInitialValues(Primitive<Cow<'de, str>>)),
    ("stringData" => StringData(Primitive<Cow<'de, str>>)),
}
