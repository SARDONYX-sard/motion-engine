//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClientCharacterState`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbClientCharacterState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa2624c97`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbClientCharacterState<'a> {
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

    /// # C++ Class Fields Info
    /// -   name:`"deformableSkinIds"`
    /// -   type: `hkArray<hkUint64>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deformableSkinIds")]
    DeformableSkinIds(HkArrayRef<Primitive<u64>>),
    /// # C++ Class Fields Info
    /// -   name:`"rigidSkinIds"`
    /// -   type: `hkArray<hkUint64>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidSkinIds")]
    RigidSkinIds(HkArrayRef<Primitive<u64>>),
    /// # C++ Class Fields Info
    /// -   name:`"externalEventIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "externalEventIds")]
    ExternalEventIds(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"auxiliaryInfo"`
    /// -   type: `hkArray<hkbAuxiliaryNodeInfo*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "auxiliaryInfo")]
    AuxiliaryInfo(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"activeEventIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeEventIds")]
    ActiveEventIds(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"activeVariableIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeVariableIds")]
    ActiveVariableIds(HkArrayRef<Primitive<i16>>),
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # C++ Class Fields Info
    /// -   name:`"instanceName"`
    /// -   type: `hkStringPtr`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "instanceName")]
    InstanceName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"templateName"`
    /// -   type: `hkStringPtr`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "templateName")]
    TemplateName(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToProject"`
    /// -   type: `hkStringPtr`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fullPathToProject")]
    FullPathToProject(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorData"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorData")]
    BehaviorData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"behaviorInternalState"`
    /// -   type: `struct hkbBehaviorGraphInternalState*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorInternalState")]
    BehaviorInternalState(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"nodeIdToInternalStateMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "nodeIdToInternalStateMap", skip_serializing)]
    NodeIdToInternalStateMap(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"elapsedSimulationTime"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elapsedSimulationTime")]
    ElapsedSimulationTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeleton")]
    Skeleton(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModel"`
    /// -   type: `hkQsTransform`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModel")]
    WorldFromModel(QsTransform<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"poseModelSpace"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "poseModelSpace")]
    PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>),
    /// # C++ Class Fields Info
    /// -   name:`"rigidAttachmentTransforms"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rigidAttachmentTransforms")]
    RigidAttachmentTransforms(HkArrayMatrix3<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClientCharacterState<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("deformableSkinIds" => DeformableSkinIds(HkArrayRef<Primitive<u64>>)),
    ("rigidSkinIds" => RigidSkinIds(HkArrayRef<Primitive<u64>>)),
    ("externalEventIds" => ExternalEventIds(HkArrayRef<Primitive<i16>>)),
    ("auxiliaryInfo" => AuxiliaryInfo(HkArrayRef<Cow<'de, str>>)),
    ("activeEventIds" => ActiveEventIds(HkArrayRef<Primitive<i16>>)),
    ("activeVariableIds" => ActiveVariableIds(HkArrayRef<Primitive<i16>>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("instanceName" => InstanceName(Primitive<Cow<'de, str>>)),
    ("templateName" => TemplateName(Primitive<Cow<'de, str>>)),
    ("fullPathToProject" => FullPathToProject(Primitive<Cow<'de, str>>)),
    ("behaviorData" => BehaviorData(Primitive<Cow<'de, str>>)),
    ("behaviorInternalState" => BehaviorInternalState(Primitive<Cow<'de, str>>)),
    ("nodeIdToInternalStateMap" => NodeIdToInternalStateMap(Primitive<Cow<'de, str>>)),
    ("visible" => Visible(Primitive<bool>)),
    ("elapsedSimulationTime" => ElapsedSimulationTime(Primitive<f32>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
    ("worldFromModel" => WorldFromModel(QsTransform<f32>)),
    ("poseModelSpace" => PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>)),
    ("rigidAttachmentTransforms" => RigidAttachmentTransforms(HkArrayMatrix3<QsTransform<f32>>)),
}
