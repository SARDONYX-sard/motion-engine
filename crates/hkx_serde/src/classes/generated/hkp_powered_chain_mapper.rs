//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPoweredChainMapper`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpPoweredChainMapper`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 44
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x7a77ef5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPoweredChainMapper<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"links"`
    /// -   type: `hkArray<struct hkpPoweredChainMapperLinkInfo>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub links: HkArrayClass<HkpPoweredChainMapperLinkInfo<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"targets"`
    /// -   type: `hkArray<struct hkpPoweredChainMapperTarget>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub targets: HkArrayClass<HkpPoweredChainMapperTarget<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"chains"`
    /// -   type: `hkArray<hkpConstraintChainInstance*>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub chains: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpPoweredChainMapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPoweredChainMapperVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPoweredChainMapper<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPoweredChainMapperVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPoweredChainMapperVisitor<'a>>> for HkpPoweredChainMapper<'a> {
    fn from(_values: Vec<HkpPoweredChainMapperVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut links = None;
            let mut targets = None;
            let mut chains = None;


        for _value in _values {
            match _value {
                HkpPoweredChainMapperVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpPoweredChainMapperVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpPoweredChainMapperVisitor::Links(m) => links = Some(m),
                HkpPoweredChainMapperVisitor::Targets(m) => targets = Some(m),
                HkpPoweredChainMapperVisitor::Chains(m) => chains = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            links: links.unwrap_or_default(),
            targets: targets.unwrap_or_default(),
            chains: chains.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPoweredChainMapper<'a>> for Vec<HkpPoweredChainMapperVisitor<'a>> {
    fn from(data: &HkpPoweredChainMapper<'a>) -> Self {
        vec![
            HkpPoweredChainMapperVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpPoweredChainMapperVisitor::ReferenceCount(data.reference_count.into()),
            HkpPoweredChainMapperVisitor::Links(data.links.clone()),
            HkpPoweredChainMapperVisitor::Targets(data.targets.clone()),
            HkpPoweredChainMapperVisitor::Chains(data.chains.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPoweredChainMapper<'de> {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkpPoweredChainMapperVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "links")]
    Links(HkArrayClass<HkpPoweredChainMapperLinkInfo<'a>>),
    /// Visitor fields
    #[serde(rename = "targets")]
    Targets(HkArrayClass<HkpPoweredChainMapperTarget<'a>>),
    /// Visitor fields
    #[serde(rename = "chains")]
    Chains(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainMapperVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("links" => Links(HkArrayClass<HkpPoweredChainMapperLinkInfo<'de>>)),
    ("targets" => Targets(HkArrayClass<HkpPoweredChainMapperTarget<'de>>)),
    ("chains" => Chains(HkArrayRef<Cow<'de, str>>)),
}