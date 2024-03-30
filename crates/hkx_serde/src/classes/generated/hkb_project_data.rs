//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProjectData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use self::packfile_deserializer::PackFileDeserializer;

#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProjectData<'a> {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"worldUpWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    world_up_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbProjectStringData*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    string_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    default_event_mode: EventMode,
}

impl<'de> Deserialize<'de> for HkbProjectData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let de = <[HkbProjectDataVisiter; 5]>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<[HkbProjectDataVisiter<'a>; 5]> for HkbProjectData<'a> {
    fn from(values: [HkbProjectDataVisiter<'a>; 5]) -> Self {
        let mut mem_size_and_flags = None;
        let mut reference_count = None;
        let mut world_up_ws = None;
        let mut string_data = None;
        let mut default_event_mode = None;

        for value in values {
            match value {
                HkbProjectDataVisiter::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbProjectDataVisiter::ReferenceCount(r) => reference_count = Some(r),
                HkbProjectDataVisiter::WorldUpWs(w) => world_up_ws = Some(w),
                HkbProjectDataVisiter::StringData(s) => string_data = Some(s),
                HkbProjectDataVisiter::DefaultEventMode(d) => default_event_mode = Some(d),
            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            world_up_ws: world_up_ws.unwrap_or_default().into_inner(),
            string_data: string_data.unwrap_or_default().into_inner(),
            default_event_mode: default_event_mode.unwrap_or_default().into_inner(),
        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbProjectData<'a>> for [HkbProjectDataVisiter<'a>; 5] {
    fn from(data: &HkbProjectData<'a>) -> Self {
        [
            HkbProjectDataVisiter::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbProjectDataVisiter::ReferenceCount(data.reference_count.into()),
            HkbProjectDataVisiter::WorldUpWs(data.world_up_ws.into()),
            HkbProjectDataVisiter::StringData(data.string_data.clone().into()),
            HkbProjectDataVisiter::DefaultEventMode(data.default_event_mode.clone().into()),
        ]
    }
}

impl<'a> Serialize for HkbProjectData<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let visitor: [HkbProjectDataVisiter; 5] = self.into();
        visitor.serialize(serializer)
    }
}

impl HkbProjectData<'_> {
    pub fn from_bytes<B>(bytes: &[u8], de: &mut PackFileDeserializer) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}

// # Why use Visitor pattern?
// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
// this is accomplished by having the Visitor process the internally tagged enum and convert it.
// Leakage of field items may occur if Vec<enum> is left as it is.

/// `hkbProjectData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x13a39ba7`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkbProjectDataVisiter<'a> {
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
    /// -   name:`"worldUpWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldUpWS")]
    WorldUpWs(Primitive<Vector4<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"stringData"`
    /// -   type: `struct hkbProjectStringData*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stringData")]
    StringData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "defaultEventMode")]
    DefaultEventMode(Primitive<EventMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProjectDataVisiter<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("worldUpWS" => WorldUpWs(Primitive<Vector4<f32>>)),
    ("stringData" => StringData(Primitive<Cow<'de, str>>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<EventMode>)),
}
