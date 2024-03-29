//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkContactPointMaterial`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkContactPointMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x4e32287c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkContactPointMaterial {
    /// # C++ Class Fields Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"restitution"`
    /// -   type: `hkUint8`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restitution")]
    Restitution(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"maxImpulse"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxImpulse")]
    MaxImpulse(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `hkUint8`
    /// - offset: 7
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkContactPointMaterial, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("friction" => Friction(Primitive<u8>)),
    ("restitution" => Restitution(Primitive<u8>)),
    ("maxImpulse" => MaxImpulse(Primitive<u8>)),
    ("flags" => Flags(Primitive<u8>)),
}

impl ByteDeSerialize for HkContactPointMaterial {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum FlagEnum {
    #[serde(rename = "CONTACT_IS_NEW")]
    ContactIsNew = 1,
    #[serde(rename = "CONTACT_USES_SOLVER_PATH2")]
    ContactUsesSolverPath2 = 2,
    #[serde(rename = "CONTACT_BREAKOFF_OBJECT_ID")]
    ContactBreakoffObjectId = 4,
    #[serde(rename = "CONTACT_IS_DISABLED")]
    ContactIsDisabled = 8,
}
