//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbIntVariableSequencedData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbIntVariableSequencedData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkbSequencedData`/`0xda8c7d7d`
/// - signature: `0x7bfc518a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbIntVariableSequencedData {
    /// # C++ Class Fields Info
    /// -   name:`"samples"`
    /// -   type: `hkArray&lt;struct hkbIntVariableSequencedDataSample&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "samples")]
    Samples(HkArrayClass<HkbIntVariableSequencedDataSample>),
    /// # C++ Class Fields Info
    /// -   name:`"variableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableIndex")]
    VariableIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbIntVariableSequencedData, "@name",
    ("samples" => Samples(HkArrayClass<HkbIntVariableSequencedDataSample>)),
    ("variableIndex" => VariableIndex(Primitive<i32>)),
}
