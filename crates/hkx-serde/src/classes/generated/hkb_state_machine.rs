//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbStateMachine`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbStateMachine`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 180
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0x816c1dcb`
/// -   version: 4
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbStateMachine<'a> {
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
    /// -   name:`"eventToSendWhenStateOrTransitionChanges"`
    /// -   type: `struct hkbEvent`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventToSendWhenStateOrTransitionChanges")]
    EventToSendWhenStateOrTransitionChanges(SingleClass<HkbEvent<'a>>),
    /// # C++ Class Fields Info
    /// -   name:`"startStateChooser"`
    /// -   type: `struct hkbStateChooser*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateChooser")]
    StartStateChooser(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"startStateId"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateId")]
    StartStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"returnToPreviousStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "returnToPreviousStateEventId")]
    ReturnToPreviousStateEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"randomTransitionEventId"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "randomTransitionEventId")]
    RandomTransitionEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionToNextHigherStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionToNextHigherStateEventId")]
    TransitionToNextHigherStateEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionToNextLowerStateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transitionToNextLowerStateEventId")]
    TransitionToNextLowerStateEventId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"syncVariableIndex"`
    /// -   type: `hkInt32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "syncVariableIndex")]
    SyncVariableIndex(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"currentStateId"`
    /// -   type: `hkInt32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "currentStateId", skip_serializing)]
    CurrentStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"wrapAroundStateId"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wrapAroundStateId")]
    WrapAroundStateId(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSimultaneousTransitions"`
    /// -   type: `hkInt8`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSimultaneousTransitions")]
    MaxSimultaneousTransitions(Primitive<i8>),
    /// # C++ Class Fields Info
    /// -   name:`"startStateMode"`
    /// -   type: `enum StartStateMode`
    /// - offset: 86
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "startStateMode")]
    StartStateMode(Primitive<StartStateMode>),
    /// # C++ Class Fields Info
    /// -   name:`"selfTransitionMode"`
    /// -   type: `enum StateMachineSelfTransitionMode`
    /// - offset: 87
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selfTransitionMode")]
    SelfTransitionMode(Primitive<StateMachineSelfTransitionMode>),
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"states"`
    /// -   type: `hkArray<hkbStateMachineStateInfo*>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "states")]
    States(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"wildcardTransitions"`
    /// -   type: `struct hkbStateMachineTransitionInfoArray*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wildcardTransitions")]
    WildcardTransitions(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"stateIdToIndexMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "stateIdToIndexMap", skip_serializing)]
    StateIdToIndexMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"activeTransitions"`
    /// -   type: `hkArray<void>`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "activeTransitions", skip_serializing)]
    ActiveTransitions(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"transitionFlags"`
    /// -   type: `hkArray<void>`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "transitionFlags", skip_serializing)]
    TransitionFlags(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"wildcardTransitionFlags"`
    /// -   type: `hkArray<void>`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "wildcardTransitionFlags", skip_serializing)]
    WildcardTransitionFlags(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"delayedTransitions"`
    /// -   type: `hkArray<void>`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "delayedTransitions", skip_serializing)]
    DelayedTransitions(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"timeInState"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "timeInState", skip_serializing)]
    TimeInState(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"lastLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "lastLocalTime", skip_serializing)]
    LastLocalTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"previousStateId"`
    /// -   type: `hkInt32`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "previousStateId", skip_serializing)]
    PreviousStateId(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"nextStartStateIndexOverride"`
    /// -   type: `hkInt32`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "nextStartStateIndexOverride", skip_serializing)]
    NextStartStateIndexOverride(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "stateOrTransitionChanged", skip_serializing)]
    StateOrTransitionChanged(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"echoNextUpdate"`
    /// -   type: `hkBool`
    /// - offset: 177
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "echoNextUpdate", skip_serializing)]
    EchoNextUpdate(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"sCurrentStateIndexAndEntered"`
    /// -   type: `hkUint16`
    /// - offset: 178
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "sCurrentStateIndexAndEntered", skip_serializing)]
    SCurrentStateIndexAndEntered(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbStateMachine<'de>, "@name",
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
    ("eventToSendWhenStateOrTransitionChanges" => EventToSendWhenStateOrTransitionChanges(SingleClass<HkbEvent<'de>>)),
    ("startStateChooser" => StartStateChooser(Primitive<Cow<'de, str>>)),
    ("startStateId" => StartStateId(Primitive<i32>)),
    ("returnToPreviousStateEventId" => ReturnToPreviousStateEventId(Primitive<i32>)),
    ("randomTransitionEventId" => RandomTransitionEventId(Primitive<i32>)),
    ("transitionToNextHigherStateEventId" => TransitionToNextHigherStateEventId(Primitive<i32>)),
    ("transitionToNextLowerStateEventId" => TransitionToNextLowerStateEventId(Primitive<i32>)),
    ("syncVariableIndex" => SyncVariableIndex(Primitive<i32>)),
    ("currentStateId" => CurrentStateId(Primitive<i32>)),
    ("wrapAroundStateId" => WrapAroundStateId(Primitive<bool>)),
    ("maxSimultaneousTransitions" => MaxSimultaneousTransitions(Primitive<i8>)),
    ("startStateMode" => StartStateMode(Primitive<StartStateMode>)),
    ("selfTransitionMode" => SelfTransitionMode(Primitive<StateMachineSelfTransitionMode>)),
    ("isActive" => IsActive(Primitive<bool>)),
    ("states" => States(HkArrayRef<Cow<'de, str>>)),
    ("wildcardTransitions" => WildcardTransitions(Primitive<Cow<'de, str>>)),
    ("stateIdToIndexMap" => StateIdToIndexMap(Primitive<Cow<'de, str>>)),
    ("activeTransitions" => ActiveTransitions(HkArrayRef<()>)),
    ("transitionFlags" => TransitionFlags(HkArrayRef<()>)),
    ("wildcardTransitionFlags" => WildcardTransitionFlags(HkArrayRef<()>)),
    ("delayedTransitions" => DelayedTransitions(HkArrayRef<()>)),
    ("timeInState" => TimeInState(Primitive<f32>)),
    ("lastLocalTime" => LastLocalTime(Primitive<f32>)),
    ("previousStateId" => PreviousStateId(Primitive<i32>)),
    ("nextStartStateIndexOverride" => NextStartStateIndexOverride(Primitive<i32>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
    ("echoNextUpdate" => EchoNextUpdate(Primitive<bool>)),
    ("sCurrentStateIndexAndEntered" => SCurrentStateIndexAndEntered(Primitive<u16>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartStateMode {
    #[serde(rename = "START_STATE_MODE_DEFAULT")]
    StartStateModeDefault = 0,
    #[serde(rename = "START_STATE_MODE_SYNC")]
    StartStateModeSync = 1,
    #[serde(rename = "START_STATE_MODE_RANDOM")]
    StartStateModeRandom = 2,
    #[serde(rename = "START_STATE_MODE_CHOOSER")]
    StartStateModeChooser = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateMachineSelfTransitionMode {
    #[serde(rename = "SELF_TRANSITION_MODE_NO_TRANSITION")]
    SelfTransitionModeNoTransition = 0,
    #[serde(rename = "SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE")]
    SelfTransitionModeTransitionToStartState = 1,
    #[serde(rename = "SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE")]
    SelfTransitionModeForceTransitionToStartState = 2,
}
