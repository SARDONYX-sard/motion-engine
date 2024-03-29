//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPhysicsSystemWithContacts`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpPhysicsSystemWithContacts`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkpPhysicsSystem`/`0xff724c17`
/// - signature: `0xd0fd4bbe`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPhysicsSystemWithContacts<'a> {
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"rigidBodies"`
    /// -   type: `hkArray<hkpRigidBody*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidBodies")]
    RigidBodies(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"constraints"`
    /// -   type: `hkArray<hkpConstraintInstance*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "constraints")]
    Constraints(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"actions"`
    /// -   type: `hkArray<hkpAction*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "actions")]
    Actions(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"phantoms"`
    /// -   type: `hkArray<hkpPhantom*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantoms")]
    Phantoms(HkArrayRef<Cow<'a, str>>),
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkpPhysicsSystem` => parent: `hkReferencedObject`) field Info
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "active")]
    Active(Primitive<bool>),

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
    //
    /// # C++ Class Fields Info
    /// -   name:`"contacts"`
    /// -   type: `hkArray<hkpSerializedAgentNnEntry*>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contacts")]
    Contacts(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhysicsSystemWithContacts<'de>, "@name",
    ("rigidBodies" => RigidBodies(HkArrayRef<Cow<'de, str>>)),
    ("constraints" => Constraints(HkArrayRef<Cow<'de, str>>)),
    ("actions" => Actions(HkArrayRef<Cow<'de, str>>)),
    ("phantoms" => Phantoms(HkArrayRef<Cow<'de, str>>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("active" => Active(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("contacts" => Contacts(HkArrayRef<Cow<'de, str>>)),
}

impl ByteDeSerialize for HkpPhysicsSystemWithContacts<'_> {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
