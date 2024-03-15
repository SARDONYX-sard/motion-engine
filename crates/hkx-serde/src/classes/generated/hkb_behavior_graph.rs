//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraph`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbBehaviorGraph`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbGenerator`/`0xd68aefc`
/// - signature: `0xb1218f86`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraph<'a> {
    // `hkbGenerator`(Parent class) has no fields

    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData", default)]
    UserData(Primitive<usize>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"id"`
    /// -   type: `hkInt16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "id", default, skip_serializing)]
    Id(Primitive<i16>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"cloneState"`
    /// -   type: `enum unknown`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cloneState", default, skip_serializing)]
    CloneState(Primitive<Unknown>),
    /// # C++ Parent class(`hkbNode`, parent: `hkbBindable`) field Info
    /// -   name:`"padNode"`
    /// -   type: `hkBool[1]`
    /// - offset: 39
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "padNode", default, skip_serializing)]
    PadNode([Primitive<bool>; 1]),

    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"variableBindingSet"`
    /// -   type: `struct hkbVariableBindingSet*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableBindingSet", default)]
    VariableBindingSet(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"cachedBindables"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "cachedBindables", default, skip_serializing)]
    CachedBindables(HkArrayRef<Primitive<()>>),
    /// # C++ Parent class(`hkbBindable`, parent: `hkReferencedObject`) field Info
    /// -   name:`"areBindablesCached"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "areBindablesCached", default, skip_serializing)]
    AreBindablesCached(Primitive<bool>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"variableMode"`
    /// -   type: `enum VariableMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableMode", default)]
    VariableMode(Primitive<VariableMode>),
    /// # C++ Class Fields Info
    /// -   name:`"uniqueIdPool"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "uniqueIdPool", default, skip_serializing)]
    UniqueIdPool(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"idToStateMachineTemplateMap"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "idToStateMachineTemplateMap", default, skip_serializing)]
    IdToStateMachineTemplateMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredExternalIdMap"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredExternalIdMap", default, skip_serializing)]
    MirroredExternalIdMap(HkArrayRef<Primitive<()>>),
    /// # C++ Class Fields Info
    /// -   name:`"pseudoRandomGenerator"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pseudoRandomGenerator", default, skip_serializing)]
    PseudoRandomGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootGenerator", default)]
    RootGenerator(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data", default)]
    Data(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rootGeneratorClone"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rootGeneratorClone", default, skip_serializing)]
    RootGeneratorClone(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"activeNodes"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodes", default, skip_serializing)]
    ActiveNodes(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"activeNodeTemplateToIndexMap"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodeTemplateToIndexMap", default, skip_serializing)]
    ActiveNodeTemplateToIndexMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"activeNodesChildrenIndices"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodesChildrenIndices", default, skip_serializing)]
    ActiveNodesChildrenIndices(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"globalTransitionData"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "globalTransitionData", default, skip_serializing)]
    GlobalTransitionData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", default, skip_serializing)]
    EventIdMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"attributeIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributeIdMap", default, skip_serializing)]
    AttributeIdMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableIdMap", default, skip_serializing)]
    VariableIdMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPropertyIdMap", default, skip_serializing)]
    CharacterPropertyIdMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"variableValueSet"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableValueSet", default, skip_serializing)]
    VariableValueSet(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeTemplateToCloneMap", default, skip_serializing)]
    NodeTemplateToCloneMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeCloneToTemplateMap"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeCloneToTemplateMap", default, skip_serializing)]
    NodeCloneToTemplateMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"stateListenerTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateListenerTemplateToCloneMap", default, skip_serializing)]
    StateListenerTemplateToCloneMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodePartitionInfo"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodePartitionInfo", default, skip_serializing)]
    NodePartitionInfo(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"numIntermediateOutputs"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numIntermediateOutputs", default, skip_serializing)]
    NumIntermediateOutputs(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"jobs"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "jobs", default, skip_serializing)]
    Jobs(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"allPartitionMemory"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "allPartitionMemory", default, skip_serializing)]
    AllPartitionMemory(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"numStaticNodes"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numStaticNodes", default, skip_serializing)]
    NumStaticNodes(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"nextUniqueId"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextUniqueId", default, skip_serializing)]
    NextUniqueId(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", default, skip_serializing)]
    IsActive(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isLinked"`
    /// -   type: `hkBool`
    /// - offset: 173
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isLinked", default, skip_serializing)]
    IsLinked(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "updateActiveNodes", default, skip_serializing)]
    UpdateActiveNodes(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 175
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateOrTransitionChanged", default, skip_serializing)]
    StateOrTransitionChanged(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraph<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("id" => Id(Primitive<i16>)),
    ("cloneState" => CloneState(Primitive<Unknown>)),
    ("padNode" => PadNode([Primitive<bool>; 1])),
    ("variableBindingSet" => VariableBindingSet(Primitive<Cow<'de, str>>)),
    ("cachedBindables" => CachedBindables(HkArrayRef<Primitive<()>>)),
    ("areBindablesCached" => AreBindablesCached(Primitive<bool>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("variableMode" => VariableMode(Primitive<VariableMode>)),
    ("uniqueIdPool" => UniqueIdPool(HkArrayRef<Primitive<()>>)),
    ("idToStateMachineTemplateMap" => IdToStateMachineTemplateMap(Primitive<Cow<'de, str>>)),
    ("mirroredExternalIdMap" => MirroredExternalIdMap(HkArrayRef<Primitive<()>>)),
    ("pseudoRandomGenerator" => PseudoRandomGenerator(Primitive<Cow<'de, str>>)),
    ("rootGenerator" => RootGenerator(Primitive<Cow<'de, str>>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("rootGeneratorClone" => RootGeneratorClone(Primitive<Cow<'de, str>>)),
    ("activeNodes" => ActiveNodes(Primitive<Cow<'de, str>>)),
    ("activeNodeTemplateToIndexMap" => ActiveNodeTemplateToIndexMap(Primitive<Cow<'de, str>>)),
    ("activeNodesChildrenIndices" => ActiveNodesChildrenIndices(Primitive<Cow<'de, str>>)),
    ("globalTransitionData" => GlobalTransitionData(Primitive<Cow<'de, str>>)),
    ("eventIdMap" => EventIdMap(Primitive<Cow<'de, str>>)),
    ("attributeIdMap" => AttributeIdMap(Primitive<Cow<'de, str>>)),
    ("variableIdMap" => VariableIdMap(Primitive<Cow<'de, str>>)),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(Primitive<Cow<'de, str>>)),
    ("variableValueSet" => VariableValueSet(Primitive<Cow<'de, str>>)),
    ("nodeTemplateToCloneMap" => NodeTemplateToCloneMap(Primitive<Cow<'de, str>>)),
    ("nodeCloneToTemplateMap" => NodeCloneToTemplateMap(Primitive<Cow<'de, str>>)),
    ("stateListenerTemplateToCloneMap" => StateListenerTemplateToCloneMap(Primitive<Cow<'de, str>>)),
    ("nodePartitionInfo" => NodePartitionInfo(Primitive<Cow<'de, str>>)),
    ("numIntermediateOutputs" => NumIntermediateOutputs(Primitive<i32>)),
    ("jobs" => Jobs(HkArrayRef<Cow<'de, str>>)),
    ("allPartitionMemory" => AllPartitionMemory(HkArrayRef<Cow<'de, str>>)),
    ("numStaticNodes" => NumStaticNodes(Primitive<i16>)),
    ("nextUniqueId" => NextUniqueId(Primitive<i16>)),
    ("isActive" => IsActive(Primitive<bool>)),
    ("isLinked" => IsLinked(Primitive<bool>)),
    ("updateActiveNodes" => UpdateActiveNodes(Primitive<bool>)),
    ("stateOrTransitionChanged" => StateOrTransitionChanged(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VariableMode {
    #[serde(rename = "VARIABLE_MODE_DISCARD_WHEN_INACTIVE")]
    VariableModeDiscardWhenInactive = 0,
    #[serde(rename = "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE")]
    VariableModeMaintainValuesWhenInactive = 1,
}
