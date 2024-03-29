//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbContext`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbContext`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: false
/// - signature: `0xe0c4d4a7`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbContext<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"character"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "character", skip_serializing)]
    Character(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"behavior"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "behavior", skip_serializing)]
    Behavior(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeToIndexMap"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "nodeToIndexMap", skip_serializing)]
    NodeToIndexMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "eventQueue", skip_serializing)]
    EventQueue(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"sharedEventQueue"`
    /// -   type: `void*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "sharedEventQueue", skip_serializing)]
    SharedEventQueue(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"generatorOutputListener"`
    /// -   type: `struct hkbGeneratorOutputListener*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generatorOutputListener")]
    GeneratorOutputListener(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventTriggeredTransition"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "eventTriggeredTransition", skip_serializing)]
    EventTriggeredTransition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"attachmentManager"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "attachmentManager", skip_serializing)]
    AttachmentManager(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"animationCache"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "animationCache", skip_serializing)]
    AnimationCache(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbContext<'de>, "@name",
    ("character" => Character(Primitive<Cow<'de, str>>)),
    ("behavior" => Behavior(Primitive<Cow<'de, str>>)),
    ("nodeToIndexMap" => NodeToIndexMap(Primitive<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(Primitive<Cow<'de, str>>)),
    ("sharedEventQueue" => SharedEventQueue(Primitive<Cow<'de, str>>)),
    ("generatorOutputListener" => GeneratorOutputListener(Primitive<Cow<'de, str>>)),
    ("eventTriggeredTransition" => EventTriggeredTransition(Primitive<bool>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("attachmentManager" => AttachmentManager(Primitive<Cow<'de, str>>)),
    ("animationCache" => AnimationCache(Primitive<Cow<'de, str>>)),
}
