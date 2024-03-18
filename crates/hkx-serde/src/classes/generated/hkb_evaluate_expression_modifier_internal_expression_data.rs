//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEvaluateExpressionModifierInternalExpressionData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbEvaluateExpressionModifierInternalExpressionData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// - signature: `0xb8686f6b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateExpressionModifierInternalExpressionData {
    /// # C++ Class Fields Info
    /// -   name:`"raisedEvent"`
    /// -   type: `hkBool`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "raisedEvent")]
    RaisedEvent(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"wasTrueInPreviousFrame"`
    /// -   type: `hkBool`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wasTrueInPreviousFrame")]
    WasTrueInPreviousFrame(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateExpressionModifierInternalExpressionData, "@name",
    ("raisedEvent" => RaisedEvent(Primitive<bool>)),
    ("wasTrueInPreviousFrame" => WasTrueInPreviousFrame(Primitive<bool>)),
}
