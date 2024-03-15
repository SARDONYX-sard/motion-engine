//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWorldCinfo`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpWorldCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 240
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa5255445`
/// -   version: 11
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWorldCinfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gravity")]
    Gravity(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseQuerySize"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseQuerySize")]
    BroadPhaseQuerySize(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contactRestingVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactRestingVelocity")]
    ContactRestingVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseBorderBehaviour"`
    /// -   type: `enum BroadPhaseBorderBehaviour`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseBorderBehaviour")]
    BroadPhaseBorderBehaviour(Primitive<BroadPhaseBorderBehaviour>),
    /// # C++ Class Fields Info
    /// -   name:`"mtPostponeAndSortBroadPhaseBorderCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 41
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mtPostponeAndSortBroadPhaseBorderCallbacks")]
    MtPostponeAndSortBroadPhaseBorderCallbacks(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseWorldAabb"`
    /// -   type: `struct hkAabb`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseWorldAabb")]
    BroadPhaseWorldAabb(HkAabb),
    /// # C++ Class Fields Info
    /// -   name:`"useKdTree"`
    /// -   type: `hkBool`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useKdTree")]
    UseKdTree(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"useMultipleTree"`
    /// -   type: `hkBool`
    /// - offset: 81
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useMultipleTree")]
    UseMultipleTree(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"treeUpdateType"`
    /// -   type: `enum TreeUpdateType`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "treeUpdateType")]
    TreeUpdateType(Primitive<TreeUpdateType>),
    /// # C++ Class Fields Info
    /// -   name:`"autoUpdateKdTree"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "autoUpdateKdTree")]
    AutoUpdateKdTree(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionTolerance")]
    CollisionTolerance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilter"`
    /// -   type: `struct hkpCollisionFilter*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilter")]
    CollisionFilter(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"convexListFilter"`
    /// -   type: `struct hkpConvexListFilter*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "convexListFilter")]
    ConvexListFilter(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"expectedMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expectedMaxLinearVelocity")]
    ExpectedMaxLinearVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"sizeOfToiEventQueue"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sizeOfToiEventQueue")]
    SizeOfToiEventQueue(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"expectedMinPsiDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expectedMinPsiDeltaTime")]
    ExpectedMinPsiDeltaTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"memoryWatchDog"`
    /// -   type: `struct hkWorldMemoryAvailableWatchDog*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memoryWatchDog")]
    MemoryWatchDog(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"broadPhaseNumMarkers"`
    /// -   type: `hkInt32`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "broadPhaseNumMarkers")]
    BroadPhaseNumMarkers(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"contactPointGeneration"`
    /// -   type: `enum ContactPointGeneration`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactPointGeneration")]
    ContactPointGeneration(Primitive<ContactPointGeneration>),
    /// # C++ Class Fields Info
    /// -   name:`"allowToSkipConfirmedCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 117
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowToSkipConfirmedCallbacks")]
    AllowToSkipConfirmedCallbacks(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"useHybridBroadphase"`
    /// -   type: `hkBool`
    /// - offset: 118
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useHybridBroadphase")]
    UseHybridBroadphase(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"solverTau"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverTau")]
    SolverTau(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"solverDamp"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverDamp")]
    SolverDamp(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"solverIterations"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverIterations")]
    SolverIterations(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"solverMicrosteps"`
    /// -   type: `hkInt32`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "solverMicrosteps")]
    SolverMicrosteps(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxConstraintViolation"`
    /// -   type: `hkReal`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxConstraintViolation")]
    MaxConstraintViolation(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"forceCoherentConstraintOrderingInSolver"`
    /// -   type: `hkBool`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceCoherentConstraintOrderingInSolver")]
    ForceCoherentConstraintOrderingInSolver(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConvexEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConvexEdgeThreshold")]
    SnapCollisionToConvexEdgeThreshold(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"snapCollisionToConcaveEdgeThreshold"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "snapCollisionToConcaveEdgeThreshold")]
    SnapCollisionToConcaveEdgeThreshold(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enableToiWeldRejection"`
    /// -   type: `hkBool`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableToiWeldRejection")]
    EnableToiWeldRejection(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"enableDeprecatedWelding"`
    /// -   type: `hkBool`
    /// - offset: 153
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableDeprecatedWelding")]
    EnableDeprecatedWelding(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"iterativeLinearCastEarlyOutDistance"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iterativeLinearCastEarlyOutDistance")]
    IterativeLinearCastEarlyOutDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"iterativeLinearCastMaxIterations"`
    /// -   type: `hkInt32`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iterativeLinearCastMaxIterations")]
    IterativeLinearCastMaxIterations(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationNumInactiveFramesSelectFlag0"`
    /// -   type: `hkUint8`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag0")]
    DeactivationNumInactiveFramesSelectFlag0(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationNumInactiveFramesSelectFlag1"`
    /// -   type: `hkUint8`
    /// - offset: 165
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationNumInactiveFramesSelectFlag1")]
    DeactivationNumInactiveFramesSelectFlag1(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationIntegrateCounter"`
    /// -   type: `hkUint8`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationIntegrateCounter")]
    DeactivationIntegrateCounter(Primitive<u8>),
    /// # C++ Class Fields Info
    /// -   name:`"shouldActivateOnRigidBodyTransformChange"`
    /// -   type: `hkBool`
    /// - offset: 167
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shouldActivateOnRigidBodyTransformChange")]
    ShouldActivateOnRigidBodyTransformChange(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"deactivationReferenceDistance"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationReferenceDistance")]
    DeactivationReferenceDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"toiCollisionResponseRotateNormal"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "toiCollisionResponseRotateNormal")]
    ToiCollisionResponseRotateNormal(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerMidphaseCollideTask")]
    MaxSectorsPerMidphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxSectorsPerNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSectorsPerNarrowphaseCollideTask")]
    MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"processToisMultithreaded"`
    /// -   type: `hkBool`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processToisMultithreaded")]
    ProcessToisMultithreaded(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiMidphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiMidphaseCollideTask")]
    MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxEntriesPerToiNarrowphaseCollideTask"`
    /// -   type: `hkInt32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxEntriesPerToiNarrowphaseCollideTask")]
    MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxNumToiCollisionPairsSinglethreaded"`
    /// -   type: `hkInt32`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxNumToiCollisionPairsSinglethreaded")]
    MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationSimplifiedToi"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationSimplifiedToi")]
    NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToi"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToi")]
    NumToisTillAllowedPenetrationToi(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiHigher"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiHigher")]
    NumToisTillAllowedPenetrationToiHigher(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"numToisTillAllowedPenetrationToiForced"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numToisTillAllowedPenetrationToiForced")]
    NumToisTillAllowedPenetrationToiForced(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"enableDeactivation"`
    /// -   type: `hkBool`
    /// - offset: 216
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableDeactivation")]
    EnableDeactivation(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"simulationType"`
    /// -   type: `enum SimulationType`
    /// - offset: 217
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulationType")]
    SimulationType(Primitive<SimulationType>),
    /// # C++ Class Fields Info
    /// -   name:`"enableSimulationIslands"`
    /// -   type: `hkBool`
    /// - offset: 218
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableSimulationIslands")]
    EnableSimulationIslands(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"minDesiredIslandSize"`
    /// -   type: `hkUint32`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDesiredIslandSize")]
    MinDesiredIslandSize(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"processActionsInSingleThread"`
    /// -   type: `hkBool`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "processActionsInSingleThread")]
    ProcessActionsInSingleThread(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"allowIntegrationOfIslandsWithoutConstraintsInASeparateJob"`
    /// -   type: `hkBool`
    /// - offset: 225
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob")]
    AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>),
    /// # C++ Class Fields Info
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameMarkerPsiSnap")]
    FrameMarkerPsiSnap(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fireCollisionCallbacks"`
    /// -   type: `hkBool`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fireCollisionCallbacks")]
    FireCollisionCallbacks(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldCinfo<'de>, "@name",
    ("gravity" => Gravity(Vector4<f32>)),
    ("broadPhaseQuerySize" => BroadPhaseQuerySize(Primitive<i32>)),
    ("contactRestingVelocity" => ContactRestingVelocity(Primitive<f32>)),
    ("broadPhaseBorderBehaviour" => BroadPhaseBorderBehaviour(Primitive<BroadPhaseBorderBehaviour>)),
    ("mtPostponeAndSortBroadPhaseBorderCallbacks" => MtPostponeAndSortBroadPhaseBorderCallbacks(Primitive<bool>)),
    ("broadPhaseWorldAabb" => BroadPhaseWorldAabb(HkAabb)),
    ("useKdTree" => UseKdTree(Primitive<bool>)),
    ("useMultipleTree" => UseMultipleTree(Primitive<bool>)),
    ("treeUpdateType" => TreeUpdateType(Primitive<TreeUpdateType>)),
    ("autoUpdateKdTree" => AutoUpdateKdTree(Primitive<bool>)),
    ("collisionTolerance" => CollisionTolerance(Primitive<f32>)),
    ("collisionFilter" => CollisionFilter(Primitive<Cow<'de, str>>)),
    ("convexListFilter" => ConvexListFilter(Primitive<Cow<'de, str>>)),
    ("expectedMaxLinearVelocity" => ExpectedMaxLinearVelocity(Primitive<f32>)),
    ("sizeOfToiEventQueue" => SizeOfToiEventQueue(Primitive<i32>)),
    ("expectedMinPsiDeltaTime" => ExpectedMinPsiDeltaTime(Primitive<f32>)),
    ("memoryWatchDog" => MemoryWatchDog(Primitive<Cow<'de, str>>)),
    ("broadPhaseNumMarkers" => BroadPhaseNumMarkers(Primitive<i32>)),
    ("contactPointGeneration" => ContactPointGeneration(Primitive<ContactPointGeneration>)),
    ("allowToSkipConfirmedCallbacks" => AllowToSkipConfirmedCallbacks(Primitive<bool>)),
    ("useHybridBroadphase" => UseHybridBroadphase(Primitive<bool>)),
    ("solverTau" => SolverTau(Primitive<f32>)),
    ("solverDamp" => SolverDamp(Primitive<f32>)),
    ("solverIterations" => SolverIterations(Primitive<i32>)),
    ("solverMicrosteps" => SolverMicrosteps(Primitive<i32>)),
    ("maxConstraintViolation" => MaxConstraintViolation(Primitive<f32>)),
    ("forceCoherentConstraintOrderingInSolver" => ForceCoherentConstraintOrderingInSolver(Primitive<bool>)),
    ("snapCollisionToConvexEdgeThreshold" => SnapCollisionToConvexEdgeThreshold(Primitive<f32>)),
    ("snapCollisionToConcaveEdgeThreshold" => SnapCollisionToConcaveEdgeThreshold(Primitive<f32>)),
    ("enableToiWeldRejection" => EnableToiWeldRejection(Primitive<bool>)),
    ("enableDeprecatedWelding" => EnableDeprecatedWelding(Primitive<bool>)),
    ("iterativeLinearCastEarlyOutDistance" => IterativeLinearCastEarlyOutDistance(Primitive<f32>)),
    ("iterativeLinearCastMaxIterations" => IterativeLinearCastMaxIterations(Primitive<i32>)),
    ("deactivationNumInactiveFramesSelectFlag0" => DeactivationNumInactiveFramesSelectFlag0(Primitive<u8>)),
    ("deactivationNumInactiveFramesSelectFlag1" => DeactivationNumInactiveFramesSelectFlag1(Primitive<u8>)),
    ("deactivationIntegrateCounter" => DeactivationIntegrateCounter(Primitive<u8>)),
    ("shouldActivateOnRigidBodyTransformChange" => ShouldActivateOnRigidBodyTransformChange(Primitive<bool>)),
    ("deactivationReferenceDistance" => DeactivationReferenceDistance(Primitive<f32>)),
    ("toiCollisionResponseRotateNormal" => ToiCollisionResponseRotateNormal(Primitive<f32>)),
    ("maxSectorsPerMidphaseCollideTask" => MaxSectorsPerMidphaseCollideTask(Primitive<i32>)),
    ("maxSectorsPerNarrowphaseCollideTask" => MaxSectorsPerNarrowphaseCollideTask(Primitive<i32>)),
    ("processToisMultithreaded" => ProcessToisMultithreaded(Primitive<bool>)),
    ("maxEntriesPerToiMidphaseCollideTask" => MaxEntriesPerToiMidphaseCollideTask(Primitive<i32>)),
    ("maxEntriesPerToiNarrowphaseCollideTask" => MaxEntriesPerToiNarrowphaseCollideTask(Primitive<i32>)),
    ("maxNumToiCollisionPairsSinglethreaded" => MaxNumToiCollisionPairsSinglethreaded(Primitive<i32>)),
    ("numToisTillAllowedPenetrationSimplifiedToi" => NumToisTillAllowedPenetrationSimplifiedToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToi" => NumToisTillAllowedPenetrationToi(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiHigher" => NumToisTillAllowedPenetrationToiHigher(Primitive<f32>)),
    ("numToisTillAllowedPenetrationToiForced" => NumToisTillAllowedPenetrationToiForced(Primitive<f32>)),
    ("enableDeactivation" => EnableDeactivation(Primitive<bool>)),
    ("simulationType" => SimulationType(Primitive<SimulationType>)),
    ("enableSimulationIslands" => EnableSimulationIslands(Primitive<bool>)),
    ("minDesiredIslandSize" => MinDesiredIslandSize(Primitive<u32>)),
    ("processActionsInSingleThread" => ProcessActionsInSingleThread(Primitive<bool>)),
    ("allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => AllowIntegrationOfIslandsWithoutConstraintsInASeparateJob(Primitive<bool>)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(Primitive<f32>)),
    ("fireCollisionCallbacks" => FireCollisionCallbacks(Primitive<bool>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SolverType {
    #[serde(rename = "SOLVER_TYPE_INVALID")]
    SolverTypeInvalid = 0,
    #[serde(rename = "SOLVER_TYPE_2ITERS_SOFT")]
    SolverType2ItersSoft = 1,
    #[serde(rename = "SOLVER_TYPE_2ITERS_MEDIUM")]
    SolverType2ItersMedium = 2,
    #[serde(rename = "SOLVER_TYPE_2ITERS_HARD")]
    SolverType2ItersHard = 3,
    #[serde(rename = "SOLVER_TYPE_4ITERS_SOFT")]
    SolverType4ItersSoft = 4,
    #[serde(rename = "SOLVER_TYPE_4ITERS_MEDIUM")]
    SolverType4ItersMedium = 5,
    #[serde(rename = "SOLVER_TYPE_4ITERS_HARD")]
    SolverType4ItersHard = 6,
    #[serde(rename = "SOLVER_TYPE_8ITERS_SOFT")]
    SolverType8ItersSoft = 7,
    #[serde(rename = "SOLVER_TYPE_8ITERS_MEDIUM")]
    SolverType8ItersMedium = 8,
    #[serde(rename = "SOLVER_TYPE_8ITERS_HARD")]
    SolverType8ItersHard = 9,
    #[serde(rename = "SOLVER_TYPE_MAX_ID")]
    SolverTypeMaxId = 10,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulationType {
    #[serde(rename = "SIMULATION_TYPE_INVALID")]
    SimulationTypeInvalid = 0,
    #[serde(rename = "SIMULATION_TYPE_DISCRETE")]
    SimulationTypeDiscrete = 1,
    #[serde(rename = "SIMULATION_TYPE_CONTINUOUS")]
    SimulationTypeContinuous = 2,
    #[serde(rename = "SIMULATION_TYPE_MULTITHREADED")]
    SimulationTypeMultithreaded = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContactPointGeneration {
    #[serde(rename = "CONTACT_POINT_ACCEPT_ALWAYS")]
    ContactPointAcceptAlways = 0,
    #[serde(rename = "CONTACT_POINT_REJECT_DUBIOUS")]
    ContactPointRejectDubious = 1,
    #[serde(rename = "CONTACT_POINT_REJECT_MANY")]
    ContactPointRejectMany = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BroadPhaseBorderBehaviour {
    #[serde(rename = "BROADPHASE_BORDER_ASSERT")]
    BroadphaseBorderAssert = 0,
    #[serde(rename = "BROADPHASE_BORDER_FIX_ENTITY")]
    BroadphaseBorderFixEntity = 1,
    #[serde(rename = "BROADPHASE_BORDER_REMOVE_ENTITY")]
    BroadphaseBorderRemoveEntity = 2,
    #[serde(rename = "BROADPHASE_BORDER_DO_NOTHING")]
    BroadphaseBorderDoNothing = 3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TreeUpdateType {
    #[serde(rename = "REBUILD_ACTIVE")]
    RebuildActive = 0,
    #[serde(rename = "REBUILD_ALL")]
    RebuildAll = 1,
}
