//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWorld`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpWorld`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 864
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xaadcec37`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWorld<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"simulation"`
    /// -   type: `struct hkpSimulation*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulation")]
    Simulation(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fixedIsland"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fixedIsland", skip_serializing)]
    FixedIsland(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"fixedRigidBody"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixedRigidBody")]
    FixedRigidBody(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"activeSimulationIslands"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "activeSimulationIslands", skip_serializing)]
    ActiveSimulationIslands(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"inactiveSimulationIslands"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "inactiveSimulationIslands", skip_serializing)]
    InactiveSimulationIslands(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"dirtySimulationIslands"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "dirtySimulationIslands", skip_serializing)]
    DirtySimulationIslands(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"maintenanceMgr"`
    /// -   type: `void*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "maintenanceMgr", skip_serializing)]
    MaintenanceMgr(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"memoryWatchDog"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memoryWatchDog", skip_serializing)]
    MemoryWatchDog(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"assertOnRunningOutOfSolverMemory"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "assertOnRunningOutOfSolverMemory", skip_serializing)]
    AssertOnRunningOutOfSolverMemory(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhase"`
    /// -   type: `void*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhase", skip_serializing)]
    BroadPhase(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"kdTreeManager"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "kdTreeManager", skip_serializing)]
    KdTreeManager(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"autoUpdateTree"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoUpdateTree")]
    AutoUpdateTree(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseDispatcher"`
    /// -   type: `void*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhaseDispatcher", skip_serializing)]
    BroadPhaseDispatcher(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"phantomBroadPhaseListener"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomBroadPhaseListener", skip_serializing)]
    PhantomBroadPhaseListener(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"entityEntityBroadPhaseListener"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "entityEntityBroadPhaseListener", skip_serializing)]
    EntityEntityBroadPhaseListener(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseBorderListener"`
    /// -   type: `void*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhaseBorderListener", skip_serializing)]
    BroadPhaseBorderListener(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"multithreadedSimulationJobData"`
    /// -   type: `void*`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "multithreadedSimulationJobData", skip_serializing)]
    MultithreadedSimulationJobData(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionInput"`
    /// -   type: `void*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionInput", skip_serializing)]
    CollisionInput(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilter"`
    /// -   type: `void*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionFilter", skip_serializing)]
    CollisionFilter(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionDispatcher"`
    /// -   type: `void*`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionDispatcher", skip_serializing)]
    CollisionDispatcher(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"convexListFilter"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "convexListFilter", skip_serializing)]
    ConvexListFilter(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperations"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pendingOperations", skip_serializing)]
    PendingOperations(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperationsCount"`
    /// -   type: `hkInt32`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pendingOperationsCount")]
    PendingOperationsCount(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"pendingBodyOperationsCount"`
    /// -   type: `hkInt32`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pendingBodyOperationsCount", skip_serializing)]
    PendingBodyOperationsCount(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"criticalOperationsLockCount"`
    /// -   type: `hkInt32`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "criticalOperationsLockCount")]
    CriticalOperationsLockCount(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"criticalOperationsLockCountForPhantoms"`
    /// -   type: `hkInt32`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "criticalOperationsLockCountForPhantoms")]
    CriticalOperationsLockCountForPhantoms(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"blockExecutingPendingOperations"`
    /// -   type: `hkBool`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockExecutingPendingOperations")]
    BlockExecutingPendingOperations(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"criticalOperationsAllowed"`
    /// -   type: `hkBool`
    /// - offset: 157
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "criticalOperationsAllowed")]
    CriticalOperationsAllowed(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperationQueues"`
    /// -   type: `void*`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pendingOperationQueues", skip_serializing)]
    PendingOperationQueues(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"pendingOperationQueueCount"`
    /// -   type: `hkInt32`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pendingOperationQueueCount")]
    PendingOperationQueueCount(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "multiThreadCheck", skip_serializing)]
    MultiThreadCheck(HkMultiThreadCheck),
    /// # C++ Class Fields Info
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 181
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"modifyConstraintCriticalSection"`
    /// -   type: `void*`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "modifyConstraintCriticalSection", skip_serializing)]
    ModifyConstraintCriticalSection(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"isLocked"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isLocked")]
    IsLocked(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"islandDirtyListCriticalSection"`
    /// -   type: `void*`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandDirtyListCriticalSection", skip_serializing)]
    IslandDirtyListCriticalSection(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"propertyMasterLock"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "propertyMasterLock", skip_serializing)]
    PropertyMasterLock(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"wantSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantSimulationIslands")]
    WantSimulationIslands(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 205
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "useHybridBroadphase", skip_serializing)]
    UseHybridBroadphase(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConvexEdgeThreshold")]
    SnapCollisionToConvexEdgeThreshold(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConcaveEdgeThreshold")]
    SnapCollisionToConcaveEdgeThreshold(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableToiWeldRejection")]
    EnableToiWeldRejection(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"wantDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wantDeactivation")]
    WantDeactivation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shouldActivateOnRigidBodyTransformChange")]
    ShouldActivateOnRigidBodyTransformChange(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationReferenceDistance")]
    DeactivationReferenceDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toiCollisionResponseRotateNormal")]
    ToiCollisionResponseRotateNormal(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerMidphaseCollideTask")]
    MaxSectorsPerMidphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerNarrowphaseCollideTask")]
    MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processToisMultithreaded")]
    ProcessToisMultithreaded(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiMidphaseCollideTask")]
    MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 244
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiNarrowphaseCollideTask")]
    MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 248
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumToiCollisionPairsSinglethreaded")]
    MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"simulationType"`
    /// -   type: `enum unknown`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "simulationType", skip_serializing)]
    SimulationType(Primitive<Unknown>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 256
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationSimplifiedToi")]
    NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 260
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToi")]
    NumToisTillAllowedPenetrationToi(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 264
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiHigher")]
    NumToisTillAllowedPenetrationToiHigher(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 268
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiForced")]
    NumToisTillAllowedPenetrationToiForced(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"lastEntityUid"`
    /// -   type: `hkUint32`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastEntityUid")]
    LastEntityUid(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"lastIslandUid"`
    /// -   type: `hkUint32`
    /// - offset: 276
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastIslandUid")]
    LastIslandUid(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"lastConstraintUid"`
    /// -   type: `hkUint32`
    /// - offset: 280
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastConstraintUid")]
    LastConstraintUid(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"phantoms"`
    /// -   type: `hkArray&lt;hkpPhantom*&gt;`
    /// - offset: 284
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantoms")]
    Phantoms(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"actionListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 296
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "actionListeners", skip_serializing)]
    ActionListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"entityListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 308
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "entityListeners", skip_serializing)]
    EntityListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"phantomListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomListeners", skip_serializing)]
    PhantomListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"constraintListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 332
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "constraintListeners", skip_serializing)]
    ConstraintListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldDeletionListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 344
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldDeletionListeners", skip_serializing)]
    WorldDeletionListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"islandActivationListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 356
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandActivationListeners", skip_serializing)]
    IslandActivationListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldPostSimulationListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 368
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldPostSimulationListeners", skip_serializing)]
    WorldPostSimulationListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldPostIntegrateListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 380
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldPostIntegrateListeners", skip_serializing)]
    WorldPostIntegrateListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldPostCollideListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 392
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldPostCollideListeners", skip_serializing)]
    WorldPostCollideListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"islandPostIntegrateListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 404
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandPostIntegrateListeners", skip_serializing)]
    IslandPostIntegrateListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"islandPostCollideListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 416
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "islandPostCollideListeners", skip_serializing)]
    IslandPostCollideListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"contactListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 428
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactListeners", skip_serializing)]
    ContactListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"contactImpulseLimitBreachedListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 440
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactImpulseLimitBreachedListeners", skip_serializing)]
    ContactImpulseLimitBreachedListeners(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"worldExtensions"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 452
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "worldExtensions", skip_serializing)]
    WorldExtensions(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"violatedConstraintArray"`
    /// -   type: `void*`
    /// - offset: 464
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "violatedConstraintArray", skip_serializing)]
    ViolatedConstraintArray(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseBorder"`
    /// -   type: `void*`
    /// - offset: 468
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "broadPhaseBorder", skip_serializing)]
    BroadPhaseBorder(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"destructionWorld"`
    /// -   type: `void*`
    /// - offset: 472
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "destructionWorld", skip_serializing)]
    DestructionWorld(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"npWorld"`
    /// -   type: `void*`
    /// - offset: 476
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "npWorld", skip_serializing)]
    NpWorld(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseExtents"`
    /// -   type: `hkVector4[2]`
    /// - offset: 800
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseExtents")]
    BroadPhaseExtents([Vector4<f32>; 2]),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 832
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 836
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 840
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseUpdateSize"`
    /// -   type: `hkInt32`
    /// - offset: 844
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseUpdateSize")]
    BroadPhaseUpdateSize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum unknown`
    /// - offset: 848
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "contactPointGeneration", skip_serializing)]
    ContactPointGeneration(Primitive<Unknown>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorld<'de>, "@name",
    ("simulation" => Simulation(Cow<'de, str>)),
    ("gravity" => Gravity(Vector4<f32>)),
    ("fixedIsland" => FixedIsland(Cow<'de, str>)),
    ("fixedRigidBody" => FixedRigidBody(Cow<'de, str>)),
    ("activeSimulationIslands" => ActiveSimulationIslands(HkArrayRef<Cow<'de, str>>)),
    ("inactiveSimulationIslands" => InactiveSimulationIslands(HkArrayRef<Cow<'de, str>>)),
    ("dirtySimulationIslands" => DirtySimulationIslands(HkArrayRef<Cow<'de, str>>)),
    ("maintenanceMgr" => MaintenanceMgr(Cow<'de, str>)),
    ("memoryWatchDog" => MemoryWatchDog(Cow<'de, str>)),
    ("assertOnRunningOutOfSolverMemory" => AssertOnRunningOutOfSolverMemory(Primitive<bool>)),
    ("broadPhase" => BroadPhase(Cow<'de, str>)),
    ("kdTreeManager" => KdTreeManager(Cow<'de, str>)),
    ("autoUpdateTree" => AutoUpdateTree(Primitive<bool>)),
    ("broadPhaseDispatcher" => BroadPhaseDispatcher(Cow<'de, str>)),
    ("phantomBroadPhaseListener" => PhantomBroadPhaseListener(Cow<'de, str>)),
    ("entityEntityBroadPhaseListener" => EntityEntityBroadPhaseListener(Cow<'de, str>)),
    ("broadPhaseBorderListener" => BroadPhaseBorderListener(Cow<'de, str>)),
    ("multithreadedSimulationJobData" => MultithreadedSimulationJobData(Cow<'de, str>)),
    ("collisionInput" => CollisionInput(Cow<'de, str>)),
    ("collisionFilter" => CollisionFilter(Cow<'de, str>)),
    ("collisionDispatcher" => CollisionDispatcher(Cow<'de, str>)),
    ("convexListFilter" => ConvexListFilter(Cow<'de, str>)),
    ("pendingOperations" => PendingOperations(Cow<'de, str>)),
    ("pendingOperationsCount" => PendingOperationsCount(Primitive<i32>)),
    ("pendingBodyOperationsCount" => PendingBodyOperationsCount(Primitive<i32>)),
    ("criticalOperationsLockCount" => CriticalOperationsLockCount(Primitive<i32>)),
    ("criticalOperationsLockCountForPhantoms" => CriticalOperationsLockCountForPhantoms(Primitive<i32>)),
    ("blockExecutingPendingOperations" => BlockExecutingPendingOperations(Primitive<bool>)),
    ("criticalOperationsAllowed" => CriticalOperationsAllowed(Primitive<bool>)),
    ("pendingOperationQueues" => PendingOperationQueues(Cow<'de, str>)),
    ("pendingOperationQueueCount" => PendingOperationQueueCount(Primitive<i32>)),
    ("multiThreadCheck" => MultiThreadCheck(HkMultiThreadCheck)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(Primitive<bool>)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(Primitive<u32>)),
    ("modifyConstraintCriticalSection" => ModifyConstraintCriticalSection(Cow<'de, str>)),
    ("isLocked" => IsLocked(Primitive<i32>)),
    ("islandDirtyListCriticalSection" => IslandDirtyListCriticalSection(Cow<'de, str>)),
    ("propertyMasterLock" => PropertyMasterLock(Cow<'de, str>)),
    ("wantSimulationIslands" => WantSimulationIslands(Primitive<bool>)),
    ("useHybridBroadphase" => UseHybridBroadphase(Primitive<bool>)),
    ("snapCollisionToConvexEdgeThreshold" => SnapCollisionToConvexEdgeThreshold(Primitive<f32>)),
    ("snapCollisionToConcaveEdgeThreshold" => SnapCollisionToConcaveEdgeThreshold(Primitive<f32>)),
    ("enableToiWeldRejection" => EnableToiWeldRejection(Primitive<bool>)),
    ("wantDeactivation" => WantDeactivation(Primitive<bool>)),
    ("shouldActivateOnRigidBodyTransformChange" => ShouldActivateOnRigidBodyTransformChange(Primitive<bool>)),
    ("deactivationReferenceDistance" => DeactivationReferenceDistance(Primitive<f32>)),
    ("toiCollisionResponseRotateNormal" => ToiCollisionResponseRotateNormal(Primitive<f32>)),
    ("maxSectorsPerMidphaseCollideTask" => MaxSectorsPerMidphaseCollideTask(Primitive<i32>)),
    ("maxSectorsPerNarrowphaseCollideTask" => MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>)),
    ("processToisMultithreaded" => ProcessToisMultithreaded(Primitive<bool>)),
    ("maxEntriesPerToiMidphaseCollideTask" => MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>)),
    ("maxEntriesPerToiNarrowphaseCollideTask" => MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>)),
    ("maxNumToiCollisionPairsSinglethreaded" => MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>)),
    ("simulationType" => SimulationType(Primitive<Unknown>)),
    ("numToisTillAllowedPenetrationSimplifiedToi" => NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToi" => NumToisTillAllowedPenetrationToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiHigher" => NumToisTillAllowedPenetrationToiHigher(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiForced" => NumToisTillAllowedPenetrationToiForced(Primitive<f32>)),
    ("lastEntityUid" => LastEntityUid(Primitive<u32>)),
    ("lastIslandUid" => LastIslandUid(Primitive<u32>)),
    ("lastConstraintUid" => LastConstraintUid(Primitive<u32>)),
    ("phantoms" => Phantoms(HkArrayRef<Cow<'de, str>>)),
    ("actionListeners" => ActionListeners(HkArrayRef<Cow<'de, str>>)),
    ("entityListeners" => EntityListeners(HkArrayRef<Cow<'de, str>>)),
    ("phantomListeners" => PhantomListeners(HkArrayRef<Cow<'de, str>>)),
    ("constraintListeners" => ConstraintListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldDeletionListeners" => WorldDeletionListeners(HkArrayRef<Cow<'de, str>>)),
    ("islandActivationListeners" => IslandActivationListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldPostSimulationListeners" => WorldPostSimulationListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldPostIntegrateListeners" => WorldPostIntegrateListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldPostCollideListeners" => WorldPostCollideListeners(HkArrayRef<Cow<'de, str>>)),
    ("islandPostIntegrateListeners" => IslandPostIntegrateListeners(HkArrayRef<Cow<'de, str>>)),
    ("islandPostCollideListeners" => IslandPostCollideListeners(HkArrayRef<Cow<'de, str>>)),
    ("contactListeners" => ContactListeners(HkArrayRef<Cow<'de, str>>)),
    ("contactImpulseLimitBreachedListeners" => ContactImpulseLimitBreachedListeners(HkArrayRef<Cow<'de, str>>)),
    ("worldExtensions" => WorldExtensions(HkArrayRef<Cow<'de, str>>)),
    ("violatedConstraintArray" => ViolatedConstraintArray(Cow<'de, str>)),
    ("broadPhaseBorder" => BroadPhaseBorder(Cow<'de, str>)),
    ("destructionWorld" => DestructionWorld(Cow<'de, str>)),
    ("npWorld" => NpWorld(Cow<'de, str>)),
    ("broadPhaseExtents" => BroadPhaseExtents([Vector4<f32>; 2])),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(Primitive<i32>)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(Primitive<i32>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(Primitive<i32>)),
    ("broadPhaseUpdateSize" => BroadPhaseUpdateSize(Primitive<i32>)),
    ("contactPointGeneration" => ContactPointGeneration(Primitive<Unknown>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReintegrationRecollideMode {
    #[serde(rename = "RR_MODE_REINTEGRATE")]
    RrModeReintegrate = 1,
    #[serde(rename = "RR_MODE_RECOLLIDE_BROADPHASE")]
    RrModeRecollideBroadphase = 2,
    #[serde(rename = "RR_MODE_RECOLLIDE_NARROWPHASE")]
    RrModeRecollideNarrowphase = 4,
    #[serde(rename = "RR_MODE_ALL")]
    RrModeAll = 7,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MtAccessChecking {
    #[serde(rename = "MT_ACCESS_CHECKING_ENABLED")]
    MtAccessCheckingEnabled = 0,
    #[serde(rename = "MT_ACCESS_CHECKING_DISABLED")]
    MtAccessCheckingDisabled = 1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CachedAabbUpdate {
    #[serde(rename = "SHIFT_BROADPHASE_UPDATE_ENTITY_AABBS")]
    ShiftBroadphaseUpdateEntityAabbs = 0,
    #[serde(rename = "SHIFT_BROADPHASE_IGNORE_ENTITY_AABBS")]
    ShiftBroadphaseIgnoreEntityAabbs = 1,
}
