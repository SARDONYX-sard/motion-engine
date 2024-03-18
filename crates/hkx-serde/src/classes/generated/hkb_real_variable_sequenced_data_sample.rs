//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRealVariableSequencedDataSample`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbRealVariableSequencedDataSample`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xbb708bbd`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRealVariableSequencedDataSample {
    /// # C++ Class Fields Info
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "time")]
    Time(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRealVariableSequencedDataSample, "@name",
    ("time" => Time(Primitive<f32>)),
    ("value" => Value(Primitive<f32>)),
}
