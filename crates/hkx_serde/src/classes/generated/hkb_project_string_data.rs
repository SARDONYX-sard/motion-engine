//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProjectStringData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkbProjectStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x76ad60a`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProjectStringData<'a> {
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
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    animation_filenames: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    behavior_filenames: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"characterFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    character_filenames: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"eventNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    event_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"animationPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    animation_path: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    behavior_path: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    character_path: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToSource"`
    /// -   type: `hkStringPtr`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    full_path_to_source: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rootPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    root_path: Cow<'a, str>,
}

impl Serialize for HkbProjectStringData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbProjectStringDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbProjectStringData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbProjectStringDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbProjectStringDataVisitor<'a>>> for HkbProjectStringData<'a> {
    fn from(_values: Vec<HkbProjectStringDataVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut animation_filenames = None;
            let mut behavior_filenames = None;
            let mut character_filenames = None;
            let mut event_names = None;
            let mut animation_path = None;
            let mut behavior_path = None;
            let mut character_path = None;
            let mut full_path_to_source = None;
            let mut root_path = None;


        for _value in _values {
            match _value {
                HkbProjectStringDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbProjectStringDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbProjectStringDataVisitor::AnimationFilenames(m) => animation_filenames = Some(m),
                HkbProjectStringDataVisitor::BehaviorFilenames(m) => behavior_filenames = Some(m),
                HkbProjectStringDataVisitor::CharacterFilenames(m) => character_filenames = Some(m),
                HkbProjectStringDataVisitor::EventNames(m) => event_names = Some(m),
                HkbProjectStringDataVisitor::AnimationPath(m) => animation_path = Some(m),
                HkbProjectStringDataVisitor::BehaviorPath(m) => behavior_path = Some(m),
                HkbProjectStringDataVisitor::CharacterPath(m) => character_path = Some(m),
                HkbProjectStringDataVisitor::FullPathToSource(m) => full_path_to_source = Some(m),
                HkbProjectStringDataVisitor::RootPath(m) => root_path = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            animation_filenames: animation_filenames.unwrap_or_default(),
            behavior_filenames: behavior_filenames.unwrap_or_default(),
            character_filenames: character_filenames.unwrap_or_default(),
            event_names: event_names.unwrap_or_default(),
            animation_path: animation_path.unwrap_or_default().into_inner(),
            behavior_path: behavior_path.unwrap_or_default().into_inner(),
            character_path: character_path.unwrap_or_default().into_inner(),
            full_path_to_source: full_path_to_source.unwrap_or_default().into_inner(),
            root_path: root_path.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbProjectStringData<'a>> for Vec<HkbProjectStringDataVisitor<'a>> {
    fn from(data: &HkbProjectStringData<'a>) -> Self {
        vec![
            HkbProjectStringDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbProjectStringDataVisitor::ReferenceCount(data.reference_count.into()),
            HkbProjectStringDataVisitor::AnimationFilenames(data.animation_filenames.clone()),
            HkbProjectStringDataVisitor::BehaviorFilenames(data.behavior_filenames.clone()),
            HkbProjectStringDataVisitor::CharacterFilenames(data.character_filenames.clone()),
            HkbProjectStringDataVisitor::EventNames(data.event_names.clone()),
            HkbProjectStringDataVisitor::AnimationPath(data.animation_path.clone().into()),
            HkbProjectStringDataVisitor::BehaviorPath(data.behavior_path.clone().into()),
            HkbProjectStringDataVisitor::CharacterPath(data.character_path.clone().into()),
            HkbProjectStringDataVisitor::FullPathToSource(data.full_path_to_source.clone().into()),
            HkbProjectStringDataVisitor::RootPath(data.root_path.clone().into()),

        ]
    }
}

impl ByteDeSerialize for HkbProjectStringData<'_> {
    fn from_bytes<B>(
        _bytes: &[u8],
        _de: &mut packfile_deserializer::PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized,
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
pub enum HkbProjectStringDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "animationFilenames")]
    AnimationFilenames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "behaviorFilenames")]
    BehaviorFilenames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "characterFilenames")]
    CharacterFilenames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "eventNames")]
    EventNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "animationPath")]
    AnimationPath(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "behaviorPath")]
    BehaviorPath(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "characterPath")]
    CharacterPath(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "fullPathToSource")]
    FullPathToSource(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rootPath", skip_serializing)]
    RootPath(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProjectStringDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("animationFilenames" => AnimationFilenames(HkArrayStringPtr<'de>)),
    ("behaviorFilenames" => BehaviorFilenames(HkArrayStringPtr<'de>)),
    ("characterFilenames" => CharacterFilenames(HkArrayStringPtr<'de>)),
    ("eventNames" => EventNames(HkArrayStringPtr<'de>)),
    ("animationPath" => AnimationPath(Primitive<Cow<'de, str>>)),
    ("behaviorPath" => BehaviorPath(Primitive<Cow<'de, str>>)),
    ("characterPath" => CharacterPath(Primitive<Cow<'de, str>>)),
    ("fullPathToSource" => FullPathToSource(Primitive<Cow<'de, str>>)),
    ("rootPath" => RootPath(Primitive<Cow<'de, str>>)),
}
