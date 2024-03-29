//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSIStateManagerModifierBSiStateData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `BSIStateManagerModifierBSiStateData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x6b8a15fc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsiStateManagerModifierBSiStateData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"pStateMachine"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pStateMachine")]
    PStateMachine(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"StateID"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "StateID")]
    StateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"iStateToSetAs"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iStateToSetAs")]
    IStateToSetAs(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifierBSiStateData<'de>, "@name",
    ("pStateMachine" => PStateMachine(Primitive<Cow<'de, str>>)),
    ("StateID" => StateId(Primitive<i32>)),
    ("iStateToSetAs" => IStateToSetAs(Primitive<i32>)),
}
