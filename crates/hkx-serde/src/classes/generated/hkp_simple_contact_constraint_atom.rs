//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSimpleContactConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpSimpleContactConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x920df11a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleContactConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// # C++ Class Fields Info
    /// -   name:`"sizeOfAllAtoms"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfAllAtoms")]
    SizeOfAllAtoms(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"numContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numContactPoints")]
    NumContactPoints(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"numReservedContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numReservedContactPoints")]
    NumReservedContactPoints(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"numUserDatasForBodyA"`
    /// -   type: `hkUint8`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numUserDatasForBodyA")]
    NumUserDatasForBodyA(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"numUserDatasForBodyB"`
    /// -   type: `hkUint8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numUserDatasForBodyB")]
    NumUserDatasForBodyB(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"contactPointPropertiesStriding"`
    /// -   type: `hkUint8`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointPropertiesStriding")]
    ContactPointPropertiesStriding(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"maxNumContactPoints"`
    /// -   type: `hkUint16`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumContactPoints")]
    MaxNumContactPoints(Primitive<u16>),
    /// # C++ Class Fields Info
    /// -   name:`"info"`
    /// -   type: `struct hkpSimpleContactConstraintDataInfo`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    #[serde(rename = "info")]
    Info(SingleClass<HkpSimpleContactConstraintDataInfo>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleContactConstraintAtom, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("sizeOfAllAtoms" => SizeOfAllAtoms(Primitive<u16>)),
    ("numContactPoints" => NumContactPoints(Primitive<u16>)),
    ("numReservedContactPoints" => NumReservedContactPoints(Primitive<u16>)),
    ("numUserDatasForBodyA" => NumUserDatasForBodyA(Primitive<u8>)),
    ("numUserDatasForBodyB" => NumUserDatasForBodyB(Primitive<u8>)),
    ("contactPointPropertiesStriding" => ContactPointPropertiesStriding(Primitive<u8>)),
    ("maxNumContactPoints" => MaxNumContactPoints(Primitive<u16>)),
    ("info" => Info(SingleClass<HkpSimpleContactConstraintDataInfo>)),
}
