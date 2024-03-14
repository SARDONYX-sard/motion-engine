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
    /// # C++ Class Fields Info
    /// -   name:`"variableMode"`
    /// -   type: `enum VariableMode`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableMode")]
    VariableMode(Primitive<VariableMode>),
    /// # C++ Class Fields Info
    /// -   name:`"uniqueIdPool"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "uniqueIdPool", skip_serializing)]
    UniqueIdPool(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"idToStateMachineTemplateMap"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "idToStateMachineTemplateMap", skip_serializing)]
    IdToStateMachineTemplateMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"mirroredExternalIdMap"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredExternalIdMap", skip_serializing)]
    MirroredExternalIdMap(HkArrayRef<()>),
    /// # C++ Class Fields Info
    /// -   name:`"pseudoRandomGenerator"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pseudoRandomGenerator", skip_serializing)]
    PseudoRandomGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"rootGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootGenerator")]
    RootGenerator(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"rootGeneratorClone"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rootGeneratorClone", skip_serializing)]
    RootGeneratorClone(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"activeNodes"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodes", skip_serializing)]
    ActiveNodes(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"activeNodeTemplateToIndexMap"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodeTemplateToIndexMap", skip_serializing)]
    ActiveNodeTemplateToIndexMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"activeNodesChildrenIndices"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeNodesChildrenIndices", skip_serializing)]
    ActiveNodesChildrenIndices(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"globalTransitionData"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "globalTransitionData", skip_serializing)]
    GlobalTransitionData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"eventIdMap"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventIdMap", skip_serializing)]
    EventIdMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"attributeIdMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attributeIdMap", skip_serializing)]
    AttributeIdMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"variableIdMap"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableIdMap", skip_serializing)]
    VariableIdMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPropertyIdMap", skip_serializing)]
    CharacterPropertyIdMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"variableValueSet"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "variableValueSet", skip_serializing)]
    VariableValueSet(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeTemplateToCloneMap", skip_serializing)]
    NodeTemplateToCloneMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeCloneToTemplateMap"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeCloneToTemplateMap", skip_serializing)]
    NodeCloneToTemplateMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"stateListenerTemplateToCloneMap"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateListenerTemplateToCloneMap", skip_serializing)]
    StateListenerTemplateToCloneMap(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"nodePartitionInfo"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodePartitionInfo", skip_serializing)]
    NodePartitionInfo(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"numIntermediateOutputs"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numIntermediateOutputs", skip_serializing)]
    NumIntermediateOutputs(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"jobs"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "jobs", skip_serializing)]
    Jobs(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"allPartitionMemory"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "allPartitionMemory", skip_serializing)]
    AllPartitionMemory(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"numStaticNodes"`
    /// -   type: `hkInt16`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "numStaticNodes", skip_serializing)]
    NumStaticNodes(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"nextUniqueId"`
    /// -   type: `hkInt16`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nextUniqueId", skip_serializing)]
    NextUniqueId(Primitive<i16>),
    /// # C++ Class Fields Info
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"isLinked"`
    /// -   type: `hkBool`
    /// - offset: 173
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isLinked", skip_serializing)]
    IsLinked(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"updateActiveNodes"`
    /// -   type: `hkBool`
    /// - offset: 174
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "updateActiveNodes", skip_serializing)]
    UpdateActiveNodes(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"stateOrTransitionChanged"`
    /// -   type: `hkBool`
    /// - offset: 175
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stateOrTransitionChanged", skip_serializing)]
    StateOrTransitionChanged(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraph<'de>, "@name",
    ("variableMode" => VariableMode(Primitive<VariableMode>)),
    ("uniqueIdPool" => UniqueIdPool(HkArrayRef<()>)),
    ("idToStateMachineTemplateMap" => IdToStateMachineTemplateMap(Cow<'de, str>)),
    ("mirroredExternalIdMap" => MirroredExternalIdMap(HkArrayRef<()>)),
    ("pseudoRandomGenerator" => PseudoRandomGenerator(Cow<'de, str>)),
    ("rootGenerator" => RootGenerator(Cow<'de, str>)),
    ("data" => Data(Cow<'de, str>)),
    ("rootGeneratorClone" => RootGeneratorClone(Cow<'de, str>)),
    ("activeNodes" => ActiveNodes(Cow<'de, str>)),
    ("activeNodeTemplateToIndexMap" => ActiveNodeTemplateToIndexMap(Cow<'de, str>)),
    ("activeNodesChildrenIndices" => ActiveNodesChildrenIndices(Cow<'de, str>)),
    ("globalTransitionData" => GlobalTransitionData(Cow<'de, str>)),
    ("eventIdMap" => EventIdMap(Cow<'de, str>)),
    ("attributeIdMap" => AttributeIdMap(Cow<'de, str>)),
    ("variableIdMap" => VariableIdMap(Cow<'de, str>)),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(Cow<'de, str>)),
    ("variableValueSet" => VariableValueSet(Cow<'de, str>)),
    ("nodeTemplateToCloneMap" => NodeTemplateToCloneMap(Cow<'de, str>)),
    ("nodeCloneToTemplateMap" => NodeCloneToTemplateMap(Cow<'de, str>)),
    ("stateListenerTemplateToCloneMap" => StateListenerTemplateToCloneMap(Cow<'de, str>)),
    ("nodePartitionInfo" => NodePartitionInfo(Cow<'de, str>)),
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
