//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBlendingTransitionEffect`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbBlendingTransitionEffect`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkbTransitionEffect`/`0x945da157`
/// - signature: `0xfd8584fe`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendingTransitionEffect<'a> {
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum SelfTransitionMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<SelfTransitionMode>),
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"eventMode"`
    /// -   type: `enum EventMode`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventMode")]
    EventMode(Primitive<EventMode>),
    /// # C++ Parent class(`hkbTransitionEffect` => parent: `hkbGenerator`) field Info
    /// -   name:`"defaultEventMode"`
    /// -   type: `enum unknown`
    /// - offset: 42
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "defaultEventMode", skip_serializing)]
    DefaultEventMode(Primitive<()>),

    // C++ Parent class(`hkbGenerator` => parent: `hkbNode`) has no fields
    //
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "id", skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", skip_serializing)]
    CloneState(Primitive<()>),
    /// # C++ Parent class(`hkbNode` => parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "padNode", skip_serializing)]
    PadNode(CStyleArray<[bool; 1]>),

    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet")]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray<void>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", skip_serializing)]
    CachedBindables(HkArrayRef<()>),
    /// # C++ Parent class(`hkbBindable` => parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", skip_serializing)]
    AreBindablesCached(Primitive<bool>),

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
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"toGeneratorStartTimeFraction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toGeneratorStartTimeFraction")]
    ToGeneratorStartTimeFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags FlagBits`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<FlagBits>),
    /// # C++ Class Fields Info
    /// -   name:`"endMode"`
    /// -   type: `enum EndMode`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endMode")]
    EndMode(Primitive<EndMode>),
    /// # C++ Class Fields Info
    /// -   name:`"blendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 55
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendCurve")]
    BlendCurve(Primitive<BlendCurve>),
    /// # C++ Class Fields Info
    /// -   name:`"fromGenerator"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "fromGenerator", skip_serializing)]
    FromGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"toGenerator"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "toGenerator", skip_serializing)]
    ToGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPoseAtBeginningOfTransition"`
    /// -   type: `hkArray<void>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "characterPoseAtBeginningOfTransition", skip_serializing)]
    CharacterPoseAtBeginningOfTransition(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"timeRemaining"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeRemaining", skip_serializing)]
    TimeRemaining(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"timeInTransition"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeInTransition", skip_serializing)]
    TimeInTransition(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"applySelfTransition"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "applySelfTransition", skip_serializing)]
    ApplySelfTransition(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"initializeCharacterPose"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "initializeCharacterPose", skip_serializing)]
    InitializeCharacterPose(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendingTransitionEffect<'de>, "@name",
    ("selfTransitionMode" => SelfTransitionMode(Primitive<SelfTransitionMode>)),
    ("eventMode" => EventMode(Primitive<EventMode>)),
    ("defaultEventMode" => DefaultEventMode(Primitive<()>)),
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<()>)),
    ("padNode" => PadNode(CStyleArray<[bool; 1]>)),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<()>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("duration" => Duration(Primitive<f32>)),
    ("toGeneratorStartTimeFraction" => ToGeneratorStartTimeFraction(Primitive<f32>)),
    ("flags" => Flags(Primitive<FlagBits>)),
    ("endMode" => EndMode(Primitive<EndMode>)),
    ("blendCurve" => BlendCurve(Primitive<BlendCurve>)),
    ("fromGenerator" => FromGenerator(Primitive<Cow<'de, str>>)),
    ("toGenerator" => ToGenerator(Primitive<Cow<'de, str>>)),
    ("characterPoseAtBeginningOfTransition" => CharacterPoseAtBeginningOfTransition(HkArrayRef<()>)),
    ("timeRemaining" => TimeRemaining(Primitive<f32>)),
    ("timeInTransition" => TimeInTransition(Primitive<f32>)),
    ("applySelfTransition" => ApplySelfTransition(Primitive<bool>)),
    ("initializeCharacterPose" => InitializeCharacterPose(Primitive<bool>)),
}
bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FlagBits: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
        const FLAG_NONE = 0;
        const FLAG_IGNORE_FROM_WORLD_FROM_MODEL = 1;
        const FLAG_SYNC = 2;
        const FLAG_IGNORE_TO_WORLD_FROM_MODEL = 4;
    }
}

impl Default for FlagBits {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for FlagBits {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for FlagBits {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for FlagBits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.contains(Self::NULL) {
            serializer.serialize_str("0")
        } else {
            serializer.serialize_str(&self.human_readable())
        }
    }
}

impl<'de> serde::Deserialize<'de> for FlagBits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {
            Some(s) => {
                if s.as_ref() == "0" {
                    return Ok(Self::NULL);
                };
                let mut flags = Self::empty();
                for token in s.split('|') {
                    match token.trim() {
                        "FLAG_NONE" => flags |= Self::FLAG_NONE,
                        "FLAG_IGNORE_FROM_WORLD_FROM_MODEL" => {
                            flags |= Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL
                        }
                        "FLAG_SYNC" => flags |= Self::FLAG_SYNC,
                        "FLAG_IGNORE_TO_WORLD_FROM_MODEL" => {
                            flags |= Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL
                        }
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected FlagBits flags but got \"{unknown}\"",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected FlagBits flags or integer, but got \"{unknown}\""
                                )))
                            }
                        },
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}

impl core::fmt::Display for FlagBits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl FlagBits {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAG_NONE) {
            flags.push("FLAG_NONE");
        }
        if self.contains(Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL) {
            flags.push("FLAG_IGNORE_FROM_WORLD_FROM_MODEL");
        }
        if self.contains(Self::FLAG_SYNC) {
            flags.push("FLAG_SYNC");
        }
        if self.contains(Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL) {
            flags.push("FLAG_IGNORE_TO_WORLD_FROM_MODEL");
        }

        flags.join("|").into()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndMode {
    #[serde(rename = "END_MODE_NONE")]
    EndModeNone = 0,
    #[serde(rename = "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR")]
    EndModeTransitionUntilEndOfFromGenerator = 1,
    #[serde(rename = "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR")]
    EndModeCapDurationAtEndOfFromGenerator = 2,
}
