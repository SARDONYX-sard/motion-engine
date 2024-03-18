//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbVariableBindingSetBinding`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbVariableBindingSetBinding`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x4d592f72`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableBindingSetBinding<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"memberPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memberPath")]
    MemberPath(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"memberClass"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memberClass", skip_serializing)]
    MemberClass(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"offsetInObjectPlusOne"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "offsetInObjectPlusOne", skip_serializing)]
    OffsetInObjectPlusOne(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"offsetInArrayPlusOne"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "offsetInArrayPlusOne", skip_serializing)]
    OffsetInArrayPlusOne(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"rootVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "rootVariableIndex", skip_serializing)]
    RootVariableIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"variableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableIndex")]
    VariableIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"bitIndex"`
    /// -   type: `hkInt8`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitIndex")]
    BitIndex(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"bindingType"`
    /// -   type: `enum BindingType`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindingType")]
    BindingType(Primitive<BindingType>),
    /// # C++ Class Fields Info
    /// -   name:`"memberType"`
    /// -   type: `enum unknown`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "memberType", skip_serializing)]
    MemberType(Primitive<()>),
    /// # C++ Class Fields Info
    /// -   name:`"variableType"`
    /// -   type: `hkInt8`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "variableType", skip_serializing)]
    VariableType(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags unknown`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "flags", skip_serializing)]
    Flags(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableBindingSetBinding<'de>, "@name",
    ("memberPath" => MemberPath(Primitive<Cow<'de, str>>)),
    ("memberClass" => MemberClass(Primitive<Cow<'de, str>>)),
    ("offsetInObjectPlusOne" => OffsetInObjectPlusOne(Primitive<i32>)),
    ("offsetInArrayPlusOne" => OffsetInArrayPlusOne(Primitive<i32>)),
    ("rootVariableIndex" => RootVariableIndex(Primitive<i32>)),
    ("variableIndex" => VariableIndex(Primitive<i32>)),
    ("bitIndex" => BitIndex(Primitive<i8>)),
    ("bindingType" => BindingType(Primitive<BindingType>)),
    ("memberType" => MemberType(Primitive<()>)),
    ("variableType" => VariableType(Primitive<i8>)),
    ("flags" => Flags(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BindingType {
    #[serde(rename = "BINDING_TYPE_VARIABLE")]
    BindingTypeVariable = 0,
    #[serde(rename = "BINDING_TYPE_CHARACTER_PROPERTY")]
    BindingTypeCharacterProperty = 1,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InternalBindingFlags {
    #[serde(rename = "FLAG_NONE")]
    FlagNone = 0,
    #[serde(rename = "FLAG_OUTPUT")]
    FlagOutput = 1,
}
