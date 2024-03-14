
use super::*;
use crate::classes::Class;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Pattern enumeration of all C++ havok class fields.
///
/// In XML, these are the fields of the attribute `hkparam`
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(tag = "@signature")]
pub enum ClassParams<'a> {
    #[default]
    Unknown,

    #[serde(rename = "0xc8df2d77")]
    #[serde(bound(deserialize = "Vec<BgsGamebryoSequenceGenerator<'a>>: Deserialize<'de>"))]
    BgsGamebryoSequenceGenerator(Vec<BgsGamebryoSequenceGenerator<'a>>),

    #[serde(rename = "0xc1215be6")]
    #[serde(bound(deserialize = "Vec<BsBoneSwitchGeneratorBoneData<'a>>: Deserialize<'de>"))]
    BsBoneSwitchGeneratorBoneData(Vec<BsBoneSwitchGeneratorBoneData<'a>>),

    #[serde(rename = "0xf33d3eea")]
    #[serde(bound(deserialize = "Vec<BsBoneSwitchGenerator<'a>>: Deserialize<'de>"))]
    BsBoneSwitchGenerator(Vec<BsBoneSwitchGenerator<'a>>),

    #[serde(rename = "0xa67f8c46")]
    #[serde(bound(deserialize = "Vec<BsComputeAddBoneAnimModifier<'a>>: Deserialize<'de>"))]
    BsComputeAddBoneAnimModifier(Vec<BsComputeAddBoneAnimModifier<'a>>),

    #[serde(rename = "0x5119eb06")]
    #[serde(bound(deserialize = "Vec<BsCyclicBlendTransitionGenerator<'a>>: Deserialize<'de>"))]
    BsCyclicBlendTransitionGenerator(Vec<BsCyclicBlendTransitionGenerator<'a>>),

    #[serde(rename = "0x31f6b8b6")]
    BsDecomposeVectorModifier(Vec<BsDecomposeVectorModifier>),

    #[serde(rename = "0x19a005c0")]
    #[serde(bound(deserialize = "Vec<BsDirectAtModifier<'a>>: Deserialize<'de>"))]
    BsDirectAtModifier(Vec<BsDirectAtModifier<'a>>),

    #[serde(rename = "0xb34d2bbd")]
    BsDistTriggerModifier(Vec<BsDistTriggerModifier>),

    #[serde(rename = "0x6030970c")]
    BsEventEveryNEventsModifier(Vec<BsEventEveryNEventsModifier>),

    #[serde(rename = "0x1062d993")]
    BsEventOnDeactivateModifier(Vec<BsEventOnDeactivateModifier>),

    #[serde(rename = "0x81d0777a")]
    BsEventOnFalseToTrueModifier(Vec<BsEventOnFalseToTrueModifier>),

    #[serde(rename = "0xbda33bfe")]
    BsGetTimeStepModifier(Vec<BsGetTimeStepModifier>),

    #[serde(rename = "0x29adc802")]
    BsInterpValueModifier(Vec<BsInterpValueModifier>),

    #[serde(rename = "0xb0fde45a")]
    BsIsActiveModifier(Vec<BsIsActiveModifier>),

    #[serde(rename = "0x6b8a15fc")]
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierBSiStateData<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifierBSiStateData(Vec<BsiStateManagerModifierBSiStateData<'a>>),

    #[serde(rename = "0x99463586")]
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierBsiStateManagerStateListener<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifierBsiStateManagerStateListener(Vec<BsiStateManagerModifierBsiStateManagerStateListener<'a>>),

    #[serde(rename = "0x6cb24f2e")]
    BsiStateManagerModifier(Vec<BsiStateManagerModifier>),

    #[serde(rename = "0xf0826fc1")]
    #[serde(bound(deserialize = "Vec<BSiStateTaggingGenerator<'a>>: Deserialize<'de>"))]
    BSiStateTaggingGenerator(Vec<BSiStateTaggingGenerator<'a>>),

    #[serde(rename = "0x8ea971e5")]
    #[serde(bound(deserialize = "Vec<BsLimbIkModifier<'a>>: Deserialize<'de>"))]
    BsLimbIkModifier(Vec<BsLimbIkModifier<'a>>),

    #[serde(rename = "0x29efee59")]
    BsLookAtModifierBoneData(Vec<BsLookAtModifierBoneData>),

    #[serde(rename = "0xd756fc25")]
    #[serde(bound(deserialize = "Vec<BsLookAtModifier<'a>>: Deserialize<'de>"))]
    BsLookAtModifier(Vec<BsLookAtModifier<'a>>),

    #[serde(rename = "0x1e20a97a")]
    #[serde(bound(deserialize = "Vec<BsModifyOnceModifier<'a>>: Deserialize<'de>"))]
    BsModifyOnceModifier(Vec<BsModifyOnceModifier<'a>>),

    #[serde(rename = "0xb8571122")]
    #[serde(bound(deserialize = "Vec<BsOffsetAnimationGenerator<'a>>: Deserialize<'de>"))]
    BsOffsetAnimationGenerator(Vec<BsOffsetAnimationGenerator<'a>>),

    #[serde(rename = "0x703d7b66")]
    BsPassByTargetTriggerModifier(Vec<BsPassByTargetTriggerModifier>),

    #[serde(rename = "0x8003d8ce")]
    #[serde(bound(deserialize = "Vec<BsRagdollContactListenerModifier<'a>>: Deserialize<'de>"))]
    BsRagdollContactListenerModifier(Vec<BsRagdollContactListenerModifier<'a>>),

    #[serde(rename = "0xd297fda9")]
    BsSpeedSamplerModifier(Vec<BsSpeedSamplerModifier>),

    #[serde(rename = "0xd83bea64")]
    #[serde(bound(deserialize = "Vec<BsSynchronizedClipGenerator<'a>>: Deserialize<'de>"))]
    BsSynchronizedClipGenerator(Vec<BsSynchronizedClipGenerator<'a>>),

    #[serde(rename = "0x531f3292")]
    BsTimerModifier(Vec<BsTimerModifier>),

    #[serde(rename = "0xd2d9a04")]
    BsTweenerModifier(Vec<BsTweenerModifier>),

    #[serde(rename = "0x1d716a17")]
    HkAabbHalf(Vec<HkAabbHalf>),

    #[serde(rename = "0x11e7c11")]
    HkAabbUint32(Vec<HkAabbUint32>),

    #[serde(rename = "0x4a948b16")]
    HkAabb(Vec<HkAabb>),

    #[serde(rename = "0xda8c7d7d")]
    HkWorldMemoryAvailableWatchDog(Vec<HkWorldMemoryAvailableWatchDog>),

    #[serde(rename = "0x66eac971")]
    #[serde(bound(deserialize = "Vec<HkaAnimationBinding<'a>>: Deserialize<'de>"))]
    HkaAnimationBinding(Vec<HkaAnimationBinding<'a>>),

    #[serde(rename = "0x8dc20333")]
    #[serde(bound(deserialize = "Vec<HkaAnimationContainer<'a>>: Deserialize<'de>"))]
    HkaAnimationContainer(Vec<HkaAnimationContainer<'a>>),

    #[serde(rename = "0x4bc4c3e0")]
    HkaAnimationPreviewColorContainer(Vec<HkaAnimationPreviewColorContainer>),

    #[serde(rename = "0xa6fa7e88")]
    #[serde(bound(deserialize = "Vec<HkaAnimation<'a>>: Deserialize<'de>"))]
    HkaAnimation(Vec<HkaAnimation<'a>>),

    #[serde(rename = "0x623bf34f")]
    #[serde(bound(deserialize = "Vec<HkaAnnotationTrackAnnotation<'a>>: Deserialize<'de>"))]
    HkaAnnotationTrackAnnotation(Vec<HkaAnnotationTrackAnnotation<'a>>),

    #[serde(rename = "0xd4114fdd")]
    #[serde(bound(deserialize = "Vec<HkaAnnotationTrack<'a>>: Deserialize<'de>"))]
    HkaAnnotationTrack(Vec<HkaAnnotationTrack<'a>>),

    #[serde(rename = "0xa8ccd5cf")]
    #[serde(bound(deserialize = "Vec<HkaBoneAttachment<'a>>: Deserialize<'de>"))]
    HkaBoneAttachment(Vec<HkaBoneAttachment<'a>>),

    #[serde(rename = "0x35912f8a")]
    #[serde(bound(deserialize = "Vec<HkaBone<'a>>: Deserialize<'de>"))]
    HkaBone(Vec<HkaBone<'a>>),

    #[serde(rename = "0x6d85e445")]
    HkaDefaultAnimatedReferenceFrame(Vec<HkaDefaultAnimatedReferenceFrame>),

    #[serde(rename = "0x724a7561")]
    HkaWaveletCompressedAnimationQuantizationFormat(Vec<HkaWaveletCompressedAnimationQuantizationFormat>),

    #[serde(rename = "0x90a68d40")]
    HkaDeltaCompressedAnimation(Vec<HkaDeltaCompressedAnimation>),

    #[serde(rename = "0x1d81207c")]
    #[serde(bound(deserialize = "Vec<HkaFootstepAnalysisInfoContainer<'a>>: Deserialize<'de>"))]
    HkaFootstepAnalysisInfoContainer(Vec<HkaFootstepAnalysisInfoContainer<'a>>),

    #[serde(rename = "0x824faf75")]
    HkaFootstepAnalysisInfo(Vec<HkaFootstepAnalysisInfo>),

    #[serde(rename = "0x930af031")]
    HkaInterleavedUncompressedAnimation(Vec<HkaInterleavedUncompressedAnimation>),

    #[serde(rename = "0xa3d0ac71")]
    HkaKeyFrameHierarchyUtilityControlData(Vec<HkaKeyFrameHierarchyUtilityControlData>),

    #[serde(rename = "0x7bd5c66f")]
    HkMemoryTrackerAttribute(Vec<HkMemoryTrackerAttribute>),

    #[serde(rename = "0x207cb01")]
    #[serde(bound(deserialize = "Vec<HkAlignSceneToNodeOptions<'a>>: Deserialize<'de>"))]
    HkAlignSceneToNodeOptions(Vec<HkAlignSceneToNodeOptions<'a>>),

    #[serde(rename = "0x48aceb75")]
    HkMeshBoneIndexMapping(Vec<HkMeshBoneIndexMapping>),

    #[serde(rename = "0x81d9950b")]
    #[serde(bound(deserialize = "Vec<HkaMeshBinding<'a>>: Deserialize<'de>"))]
    HkaMeshBinding(Vec<HkaMeshBinding<'a>>),

    #[serde(rename = "0xf7d64649")]
    HkaQuantizedAnimationTrackCompressionParams(Vec<HkaQuantizedAnimationTrackCompressionParams>),

    #[serde(rename = "0x3920f053")]
    #[serde(bound(deserialize = "Vec<HkaQuantizedAnimation<'a>>: Deserialize<'de>"))]
    HkaQuantizedAnimation(Vec<HkaQuantizedAnimation<'a>>),

    #[serde(rename = "0x154948e8")]
    #[serde(bound(deserialize = "Vec<HkaRagdollInstance<'a>>: Deserialize<'de>"))]
    HkaRagdollInstance(Vec<HkaRagdollInstance<'a>>),

    #[serde(rename = "0xd404a39a")]
    HkArrayTypeAttribute(Vec<HkArrayTypeAttribute>),

    #[serde(rename = "0x52e8043")]
    #[serde(bound(deserialize = "Vec<HkaSkeletonLocalFrameOnBone<'a>>: Deserialize<'de>"))]
    HkaSkeletonLocalFrameOnBone(Vec<HkaSkeletonLocalFrameOnBone<'a>>),

    #[serde(rename = "0xa528f7cf")]
    HkaSkeletonMapperDataChainMapping(Vec<HkaSkeletonMapperDataChainMapping>),

    #[serde(rename = "0x3405deca")]
    HkaSkeletonMapperDataSimpleMapping(Vec<HkaSkeletonMapperDataSimpleMapping>),

    #[serde(rename = "0x95687ea0")]
    #[serde(bound(deserialize = "Vec<HkaSkeletonMapperData<'a>>: Deserialize<'de>"))]
    HkaSkeletonMapperData(Vec<HkaSkeletonMapperData<'a>>),

    #[serde(rename = "0x12df42a5")]
    HkaSkeletonMapper(Vec<HkaSkeletonMapper>),

    #[serde(rename = "0x366e8220")]
    #[serde(bound(deserialize = "Vec<HkaSkeleton<'a>>: Deserialize<'de>"))]
    HkaSkeleton(Vec<HkaSkeleton<'a>>),

    #[serde(rename = "0xde830789")]
    HkaSplineCompressedAnimationAnimationCompressionParams(Vec<HkaSplineCompressedAnimationAnimationCompressionParams>),

    #[serde(rename = "0x42e878d3")]
    HkaSplineCompressedAnimationTrackCompressionParams(Vec<HkaSplineCompressedAnimationTrackCompressionParams>),

    #[serde(rename = "0x792ee0bb")]
    HkaSplineCompressedAnimation(Vec<HkaSplineCompressedAnimation>),

    #[serde(rename = "0x27c6cafa")]
    HkaWaveletCompressedAnimationCompressionParams(Vec<HkaWaveletCompressedAnimationCompressionParams>),

    #[serde(rename = "0x77cf0962")]
    HkaWaveletCompressedAnimation(Vec<HkaWaveletCompressedAnimation>),

    #[serde(rename = "0xe0708a00")]
    HkpShapeContainer(Vec<HkpShapeContainer>),

    #[serde(rename = "0xcc0aab32")]
    #[serde(bound(deserialize = "Vec<HkbAttachmentModifier<'a>>: Deserialize<'de>"))]
    HkbAttachmentModifier(Vec<HkbAttachmentModifier<'a>>),

    #[serde(rename = "0x774632b")]
    HkbAttachmentSetup(Vec<HkbAttachmentSetup>),

    #[serde(rename = "0x48b8ad52")]
    HkbAttributeModifierAssignment(Vec<HkbAttributeModifierAssignment>),

    #[serde(rename = "0x1245d97d")]
    HkbAttributeModifier(Vec<HkbAttributeModifier>),

    #[serde(rename = "0xca0888ca")]
    #[serde(bound(deserialize = "Vec<HkbAuxiliaryNodeInfo<'a>>: Deserialize<'de>"))]
    HkbAuxiliaryNodeInfo(Vec<HkbAuxiliaryNodeInfo<'a>>),

    #[serde(rename = "0x66840004")]
    HkbBehaviorEventsInfo(Vec<HkbBehaviorEventsInfo>),

    #[serde(rename = "0x95aca5d")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphData<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphData(Vec<HkbBehaviorGraphData<'a>>),

    #[serde(rename = "0x645f898b")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphInternalStateInfo<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphInternalStateInfo(Vec<HkbBehaviorGraphInternalStateInfo<'a>>),

    #[serde(rename = "0x8699b6eb")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphInternalState<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphInternalState(Vec<HkbBehaviorGraphInternalState<'a>>),

    #[serde(rename = "0xc713064e")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphStringData<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphStringData(Vec<HkbBehaviorGraphStringData<'a>>),

    #[serde(rename = "0xb1218f86")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraph<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraph(Vec<HkbBehaviorGraph<'a>>),

    #[serde(rename = "0x35a0439a")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorInfoIdToNamePair<'a>>: Deserialize<'de>"))]
    HkbBehaviorInfoIdToNamePair(Vec<HkbBehaviorInfoIdToNamePair<'a>>),

    #[serde(rename = "0xf7645395")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorInfo<'a>>: Deserialize<'de>"))]
    HkbBehaviorInfo(Vec<HkbBehaviorInfo<'a>>),

    #[serde(rename = "0xfcb5423")]
    #[serde(bound(deserialize = "Vec<HkbBehaviorReferenceGenerator<'a>>: Deserialize<'de>"))]
    HkbBehaviorReferenceGenerator(Vec<HkbBehaviorReferenceGenerator<'a>>),

    #[serde(rename = "0x2c1432d7")]
    #[serde(bound(deserialize = "Vec<HkbBindable<'a>>: Deserialize<'de>"))]
    HkbBindable(Vec<HkbBindable<'a>>),

    #[serde(rename = "0x23041af0")]
    HkbBlendCurveUtils(Vec<HkbBlendCurveUtils>),

    #[serde(rename = "0xff7327c0")]
    HkbBlenderGeneratorChildInternalState(Vec<HkbBlenderGeneratorChildInternalState>),

    #[serde(rename = "0xe2b384b0")]
    #[serde(bound(deserialize = "Vec<HkbBlenderGeneratorChild<'a>>: Deserialize<'de>"))]
    HkbBlenderGeneratorChild(Vec<HkbBlenderGeneratorChild<'a>>),

    #[serde(rename = "0x84717488")]
    HkbBlenderGeneratorInternalState(Vec<HkbBlenderGeneratorInternalState>),

    #[serde(rename = "0x22df7147")]
    #[serde(bound(deserialize = "Vec<HkbBlenderGenerator<'a>>: Deserialize<'de>"))]
    HkbBlenderGenerator(Vec<HkbBlenderGenerator<'a>>),

    #[serde(rename = "0xb18c70c2")]
    HkbBlendingTransitionEffectInternalState(Vec<HkbBlendingTransitionEffectInternalState>),

    #[serde(rename = "0xfd8584fe")]
    #[serde(bound(deserialize = "Vec<HkbBlendingTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbBlendingTransitionEffect(Vec<HkbBlendingTransitionEffect<'a>>),

    #[serde(rename = "0xaa8619")]
    HkbBoneIndexArray(Vec<HkbBoneIndexArray>),

    #[serde(rename = "0xcd902b77")]
    HkbBoneWeightArray(Vec<HkbBoneWeightArray>),

    #[serde(rename = "0x514763dc")]
    HkbBoolVariableSequencedDataSample(Vec<HkbBoolVariableSequencedDataSample>),

    #[serde(rename = "0x37416fce")]
    HkbBoolVariableSequencedData(Vec<HkbBoolVariableSequencedData>),

    #[serde(rename = "0x64136982")]
    HkbCameraShakeEventPayload(Vec<HkbCameraShakeEventPayload>),

    #[serde(rename = "0x3544e182")]
    #[serde(bound(deserialize = "Vec<HkbCharacterAddedInfo<'a>>: Deserialize<'de>"))]
    HkbCharacterAddedInfo(Vec<HkbCharacterAddedInfo<'a>>),

    #[serde(rename = "0x7a195d1d")]
    HkbCharacterControlCommand(Vec<HkbCharacterControlCommand>),

    #[serde(rename = "0x5b6c03d9")]
    HkbCharacterControllerControlData(Vec<HkbCharacterControllerControlData>),

    #[serde(rename = "0xf8dfec0d")]
    HkbCharacterControllerModifierInternalState(Vec<HkbCharacterControllerModifierInternalState>),

    #[serde(rename = "0xf675d6fb")]
    HkbCharacterControllerModifier(Vec<HkbCharacterControllerModifier>),

    #[serde(rename = "0xa0f415bf")]
    #[serde(bound(deserialize = "Vec<HkbCharacterDataCharacterControllerInfo<'a>>: Deserialize<'de>"))]
    HkbCharacterDataCharacterControllerInfo(Vec<HkbCharacterDataCharacterControllerInfo<'a>>),

    #[serde(rename = "0x300d6808")]
    #[serde(bound(deserialize = "Vec<HkbCharacterData<'a>>: Deserialize<'de>"))]
    HkbCharacterData(Vec<HkbCharacterData<'a>>),

    #[serde(rename = "0xd9709ff2")]
    HkbCharacterInfo(Vec<HkbCharacterInfo>),

    #[serde(rename = "0xe5a2a413")]
    #[serde(bound(deserialize = "Vec<HkbCharacterSetup<'a>>: Deserialize<'de>"))]
    HkbCharacterSetup(Vec<HkbCharacterSetup<'a>>),

    #[serde(rename = "0x180d900d")]
    HkbCharacterSkinInfo(Vec<HkbCharacterSkinInfo>),

    #[serde(rename = "0x2eda84f8")]
    HkbCharacterSteppedInfo(Vec<HkbCharacterSteppedInfo>),

    #[serde(rename = "0x655b42bc")]
    #[serde(bound(deserialize = "Vec<HkbCharacterStringData<'a>>: Deserialize<'de>"))]
    HkbCharacterStringData(Vec<HkbCharacterStringData<'a>>),

    #[serde(rename = "0x3088a5c5")]
    #[serde(bound(deserialize = "Vec<HkbCharacter<'a>>: Deserialize<'de>"))]
    HkbCharacter(Vec<HkbCharacter<'a>>),

    #[serde(rename = "0xa2624c97")]
    #[serde(bound(deserialize = "Vec<HkbClientCharacterState<'a>>: Deserialize<'de>"))]
    HkbClientCharacterState(Vec<HkbClientCharacterState<'a>>),

    #[serde(rename = "0x750edf40")]
    HkbClipGeneratorEcho(Vec<HkbClipGeneratorEcho>),

    #[serde(rename = "0x26ce5bf3")]
    HkbClipGeneratorInternalState(Vec<HkbClipGeneratorInternalState>),

    #[serde(rename = "0x333b85b9")]
    #[serde(bound(deserialize = "Vec<HkbClipGenerator<'a>>: Deserialize<'de>"))]
    HkbClipGenerator(Vec<HkbClipGenerator<'a>>),

    #[serde(rename = "0x59c23a0f")]
    HkbClipTriggerArray(Vec<HkbClipTriggerArray>),

    #[serde(rename = "0x7eb45cea")]
    HkbClipTrigger(Vec<HkbClipTrigger>),

    #[serde(rename = "0xa92ed39f")]
    HkbGetWorldFromModelModifierInternalState(Vec<HkbGetWorldFromModelModifierInternalState>),

    #[serde(rename = "0xfd1f0b79")]
    HkbCombineTransformsModifier(Vec<HkbCombineTransformsModifier>),

    #[serde(rename = "0xc6aaccc8")]
    HkbCompiledExpressionSetToken(Vec<HkbCompiledExpressionSetToken>),

    #[serde(rename = "0x3a7d76cc")]
    HkbCompiledExpressionSet(Vec<HkbCompiledExpressionSet>),

    #[serde(rename = "0x6ac054d7")]
    HkbComputeDirectionModifierInternalState(Vec<HkbComputeDirectionModifierInternalState>),

    #[serde(rename = "0xdf358bd3")]
    HkbComputeDirectionModifier(Vec<HkbComputeDirectionModifier>),

    #[serde(rename = "0x71cd1eb0")]
    HkbComputeRotationToTargetModifierInternalState(Vec<HkbComputeRotationToTargetModifierInternalState>),

    #[serde(rename = "0x9b3f6936")]
    HkbComputeRotationFromAxisAngleModifier(Vec<HkbComputeRotationFromAxisAngleModifier>),

    #[serde(rename = "0x47665f1c")]
    HkbComputeRotationToTargetModifier(Vec<HkbComputeRotationToTargetModifier>),

    #[serde(rename = "0xe0c4d4a7")]
    #[serde(bound(deserialize = "Vec<HkbContext<'a>>: Deserialize<'de>"))]
    HkbContext(Vec<HkbContext<'a>>),

    #[serde(rename = "0x508d3b36")]
    HkbDampingModifierInternalState(Vec<HkbDampingModifierInternalState>),

    #[serde(rename = "0x9a040f03")]
    HkbDampingModifier(Vec<HkbDampingModifier>),

    #[serde(rename = "0x85fb0b80")]
    HkbDelayedModifierInternalState(Vec<HkbDelayedModifierInternalState>),

    #[serde(rename = "0x8e101a7a")]
    HkbDelayedModifier(Vec<HkbDelayedModifier>),

    #[serde(rename = "0x7b32d942")]
    HkbDetectCloseToGroundModifierInternalState(Vec<HkbDetectCloseToGroundModifierInternalState>),

    #[serde(rename = "0x981687b2")]
    HkbDetectCloseToGroundModifier(Vec<HkbDetectCloseToGroundModifier>),

    #[serde(rename = "0xb8686f6b")]
    HkbEvaluateExpressionModifierInternalExpressionData(Vec<HkbEvaluateExpressionModifierInternalExpressionData>),

    #[serde(rename = "0xb414d58e")]
    HkbEvaluateExpressionModifierInternalState(Vec<HkbEvaluateExpressionModifierInternalState>),

    #[serde(rename = "0xf900f6be")]
    #[serde(bound(deserialize = "Vec<HkbEvaluateExpressionModifier<'a>>: Deserialize<'de>"))]
    HkbEvaluateExpressionModifier(Vec<HkbEvaluateExpressionModifier<'a>>),

    #[serde(rename = "0x79757102")]
    #[serde(bound(deserialize = "Vec<HkbEvaluateHandleModifier<'a>>: Deserialize<'de>"))]
    HkbEvaluateHandleModifier(Vec<HkbEvaluateHandleModifier<'a>>),

    #[serde(rename = "0x76bddb31")]
    #[serde(bound(deserialize = "Vec<HkbEventBase<'a>>: Deserialize<'de>"))]
    HkbEventBase(Vec<HkbEventBase<'a>>),

    #[serde(rename = "0xd14bf000")]
    HkbEventDrivenModifierInternalState(Vec<HkbEventDrivenModifierInternalState>),

    #[serde(rename = "0x7ed3f44e")]
    HkbEventDrivenModifier(Vec<HkbEventDrivenModifier>),

    #[serde(rename = "0x5874eed4")]
    HkbEventInfo(Vec<HkbEventInfo>),

    #[serde(rename = "0x3d2dbd34")]
    #[serde(bound(deserialize = "Vec<HkbEventPayloadList<'a>>: Deserialize<'de>"))]
    HkbEventPayloadList(Vec<HkbEventPayloadList<'a>>),

    #[serde(rename = "0xdb38a15")]
    HkbEventProperty(Vec<HkbEventProperty>),

    #[serde(rename = "0xc02da3")]
    #[serde(bound(deserialize = "Vec<HkbEventRaisedInfo<'a>>: Deserialize<'de>"))]
    HkbEventRaisedInfo(Vec<HkbEventRaisedInfo<'a>>),

    #[serde(rename = "0x330a56ee")]
    HkbEventRangeDataArray(Vec<HkbEventRangeDataArray>),

    #[serde(rename = "0x6cb92c76")]
    HkbEventRangeData(Vec<HkbEventRangeData>),

    #[serde(rename = "0x9139b821")]
    HkbEventSequencedDataSequencedEvent(Vec<HkbEventSequencedDataSequencedEvent>),

    #[serde(rename = "0x76798eb8")]
    HkbEventSequencedData(Vec<HkbEventSequencedData>),

    #[serde(rename = "0xcc47b48d")]
    HkbEventsFromRangeModifierInternalState(Vec<HkbEventsFromRangeModifierInternalState>),

    #[serde(rename = "0xbc561b6e")]
    #[serde(bound(deserialize = "Vec<HkbEventsFromRangeModifier<'a>>: Deserialize<'de>"))]
    HkbEventsFromRangeModifier(Vec<HkbEventsFromRangeModifier<'a>>),

    #[serde(rename = "0x3e0fd810")]
    #[serde(bound(deserialize = "Vec<HkbEvent<'a>>: Deserialize<'de>"))]
    HkbEvent(Vec<HkbEvent<'a>>),

    #[serde(rename = "0x1c3c1045")]
    #[serde(bound(deserialize = "Vec<HkbExpressionCondition<'a>>: Deserialize<'de>"))]
    HkbExpressionCondition(Vec<HkbExpressionCondition<'a>>),

    #[serde(rename = "0x4b9ee1a2")]
    HkbExpressionDataArray(Vec<HkbExpressionDataArray>),

    #[serde(rename = "0x6740042a")]
    #[serde(bound(deserialize = "Vec<HkbExpressionData<'a>>: Deserialize<'de>"))]
    HkbExpressionData(Vec<HkbExpressionData<'a>>),

    #[serde(rename = "0x804dcbab")]
    HkbExtractRagdollPoseModifier(Vec<HkbExtractRagdollPoseModifier>),

    #[serde(rename = "0xa111b704")]
    HkbFootIkControlData(Vec<HkbFootIkControlData>),

    #[serde(rename = "0x9e17091a")]
    HkbFootIkControlsModifierLeg(Vec<HkbFootIkControlsModifierLeg>),

    #[serde(rename = "0xe5b6f544")]
    HkbFootIkControlsModifier(Vec<HkbFootIkControlsModifier>),

    #[serde(rename = "0x224b18d1")]
    HkbFootIkDriverInfoLeg(Vec<HkbFootIkDriverInfoLeg>),

    #[serde(rename = "0xc6a09dbf")]
    HkbFootIkDriverInfo(Vec<HkbFootIkDriverInfo>),

    #[serde(rename = "0xa681b7f0")]
    HkbFootIkGains(Vec<HkbFootIkGains>),

    #[serde(rename = "0xe5ca3677")]
    #[serde(bound(deserialize = "Vec<HkbFootIkModifierInternalLegData<'a>>: Deserialize<'de>"))]
    HkbFootIkModifierInternalLegData(Vec<HkbFootIkModifierInternalLegData<'a>>),

    #[serde(rename = "0x9f3e3a04")]
    HkbFootIkModifierLeg(Vec<HkbFootIkModifierLeg>),

    #[serde(rename = "0xed8966c0")]
    HkbFootIkModifier(Vec<HkbFootIkModifier>),

    #[serde(rename = "0xb597cf92")]
    HkbGeneratorSyncInfoSyncPoint(Vec<HkbGeneratorSyncInfoSyncPoint>),

    #[serde(rename = "0xa3c341f8")]
    HkbGeneratorSyncInfo(Vec<HkbGeneratorSyncInfo>),

    #[serde(rename = "0xd6692b5d")]
    HkbGeneratorTransitionEffectInternalState(Vec<HkbGeneratorTransitionEffectInternalState>),

    #[serde(rename = "0x5f771b12")]
    #[serde(bound(deserialize = "Vec<HkbGeneratorTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbGeneratorTransitionEffect(Vec<HkbGeneratorTransitionEffect<'a>>),

    #[serde(rename = "0xd68aefc")]
    HkbGenerator(Vec<HkbGenerator>),

    #[serde(rename = "0x50c34a17")]
    #[serde(bound(deserialize = "Vec<HkbGetHandleOnBoneModifier<'a>>: Deserialize<'de>"))]
    HkbGetHandleOnBoneModifier(Vec<HkbGetHandleOnBoneModifier<'a>>),

    #[serde(rename = "0xd84cad4a")]
    HkbGetUpModifierInternalState(Vec<HkbGetUpModifierInternalState>),

    #[serde(rename = "0x61cb7ac0")]
    HkbGetUpModifier(Vec<HkbGetUpModifier>),

    #[serde(rename = "0x873fc6f7")]
    HkbGetWorldFromModelModifier(Vec<HkbGetWorldFromModelModifier>),

    #[serde(rename = "0xd72b8d17")]
    #[serde(bound(deserialize = "Vec<HkbHandIkControlData<'a>>: Deserialize<'de>"))]
    HkbHandIkControlData(Vec<HkbHandIkControlData<'a>>),

    #[serde(rename = "0x9c72e9e3")]
    HkbHandIkControlsModifierHand(Vec<HkbHandIkControlsModifierHand>),

    #[serde(rename = "0x9f0488bb")]
    HkbHandIkControlsModifier(Vec<HkbHandIkControlsModifier>),

    #[serde(rename = "0x14dfe1dd")]
    #[serde(bound(deserialize = "Vec<HkbHandIkModifierHand<'a>>: Deserialize<'de>"))]
    HkbHandIkModifierHand(Vec<HkbHandIkModifierHand<'a>>),

    #[serde(rename = "0xc299090a")]
    HkbHandIkDriverInfo(Vec<HkbHandIkDriverInfo>),

    #[serde(rename = "0xef8bc2f7")]
    HkbHandIkModifier(Vec<HkbHandIkModifier>),

    #[serde(rename = "0xd8b6401c")]
    #[serde(bound(deserialize = "Vec<HkbHandle<'a>>: Deserialize<'de>"))]
    HkbHandle(Vec<HkbHandle<'a>>),

    #[serde(rename = "0xebbc1bd3")]
    HkbIntEventPayload(Vec<HkbIntEventPayload>),

    #[serde(rename = "0xbe7ac63c")]
    HkbIntVariableSequencedDataSample(Vec<HkbIntVariableSequencedDataSample>),

    #[serde(rename = "0x7bfc518a")]
    HkbIntVariableSequencedData(Vec<HkbIntVariableSequencedData>),

    #[serde(rename = "0xda41bd9b")]
    HkBitField(Vec<HkBitField>),

    #[serde(rename = "0x72deb7a6")]
    HkbKeyframeBonesModifierKeyframeInfo(Vec<HkbKeyframeBonesModifierKeyframeInfo>),

    #[serde(rename = "0x95f66629")]
    #[serde(bound(deserialize = "Vec<HkbKeyframeBonesModifier<'a>>: Deserialize<'de>"))]
    HkbKeyframeBonesModifier(Vec<HkbKeyframeBonesModifier<'a>>),

    #[serde(rename = "0x6a5094e3")]
    #[serde(bound(deserialize = "Vec<HkbSequenceStringData<'a>>: Deserialize<'de>"))]
    HkbSequenceStringData(Vec<HkbSequenceStringData<'a>>),

    #[serde(rename = "0xa14caba6")]
    HkbLookAtModifierInternalState(Vec<HkbLookAtModifierInternalState>),

    #[serde(rename = "0x3d28e066")]
    HkbLookAtModifier(Vec<HkbLookAtModifier>),

    #[serde(rename = "0x492c6137")]
    HkbManualSelectorGeneratorInternalState(Vec<HkbManualSelectorGeneratorInternalState>),

    #[serde(rename = "0xd932fab8")]
    #[serde(bound(deserialize = "Vec<HkbManualSelectorGenerator<'a>>: Deserialize<'de>"))]
    HkbManualSelectorGenerator(Vec<HkbManualSelectorGenerator<'a>>),

    #[serde(rename = "0x26a196c5")]
    #[serde(bound(deserialize = "Vec<HkbMessageLog<'a>>: Deserialize<'de>"))]
    HkbMessageLog(Vec<HkbMessageLog<'a>>),

    #[serde(rename = "0xc6c2da4f")]
    HkbMirroredSkeletonInfo(Vec<HkbMirroredSkeletonInfo>),

    #[serde(rename = "0xa9a271ea")]
    HkbMirrorModifier(Vec<HkbMirrorModifier>),

    #[serde(rename = "0x1f81fae6")]
    #[serde(bound(deserialize = "Vec<HkbModifierGenerator<'a>>: Deserialize<'de>"))]
    HkbModifierGenerator(Vec<HkbModifierGenerator<'a>>),

    #[serde(rename = "0xa4180ca1")]
    #[serde(bound(deserialize = "Vec<HkbModifierList<'a>>: Deserialize<'de>"))]
    HkbModifierList(Vec<HkbModifierList<'a>>),

    #[serde(rename = "0x3697e044")]
    #[serde(bound(deserialize = "Vec<HkbModifierWrapper<'a>>: Deserialize<'de>"))]
    HkbModifierWrapper(Vec<HkbModifierWrapper<'a>>),

    #[serde(rename = "0x96ec5ced")]
    HkbModifier(Vec<HkbModifier>),

    #[serde(rename = "0x28f67ba0")]
    HkbMoveCharacterModifierInternalState(Vec<HkbMoveCharacterModifierInternalState>),

    #[serde(rename = "0x8f7492a0")]
    HkbMoveCharacterModifier(Vec<HkbMoveCharacterModifier>),

    #[serde(rename = "0x65bdd3a0")]
    #[serde(bound(deserialize = "Vec<HkbNamedEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedEventPayload(Vec<HkbNamedEventPayload<'a>>),

    #[serde(rename = "0x3c99bda4")]
    HkbNamedIntEventPayload(Vec<HkbNamedIntEventPayload>),

    #[serde(rename = "0x9c99fd70")]
    HkbNamedRealEventPayload(Vec<HkbNamedRealEventPayload>),

    #[serde(rename = "0x6caa9113")]
    #[serde(bound(deserialize = "Vec<HkbNamedStringEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedStringEventPayload(Vec<HkbNamedStringEventPayload<'a>>),

    #[serde(rename = "0x7db9971d")]
    #[serde(bound(deserialize = "Vec<HkbNodeInternalStateInfo<'a>>: Deserialize<'de>"))]
    HkbNodeInternalStateInfo(Vec<HkbNodeInternalStateInfo<'a>>),

    #[serde(rename = "0x6d26f61d")]
    #[serde(bound(deserialize = "Vec<HkbNode<'a>>: Deserialize<'de>"))]
    HkbNode(Vec<HkbNode<'a>>),

    #[serde(rename = "0x9df46cd6")]
    HkbParticleSystemEventPayload(Vec<HkbParticleSystemEventPayload>),

    #[serde(rename = "0x552d9dd4")]
    HkbPoseMatchingGeneratorInternalState(Vec<HkbPoseMatchingGeneratorInternalState>),

    #[serde(rename = "0x29e271b4")]
    #[serde(bound(deserialize = "Vec<HkbPoseMatchingGenerator<'a>>: Deserialize<'de>"))]
    HkbPoseMatchingGenerator(Vec<HkbPoseMatchingGenerator<'a>>),

    #[serde(rename = "0xf5ba21b")]
    HkbPoweredRagdollControlData(Vec<HkbPoweredRagdollControlData>),

    #[serde(rename = "0x7cb54065")]
    #[serde(bound(deserialize = "Vec<HkbPoweredRagdollControlsModifier<'a>>: Deserialize<'de>"))]
    HkbPoweredRagdollControlsModifier(Vec<HkbPoweredRagdollControlsModifier<'a>>),

    #[serde(rename = "0x13a39ba7")]
    #[serde(bound(deserialize = "Vec<HkbProjectData<'a>>: Deserialize<'de>"))]
    HkbProjectData(Vec<HkbProjectData<'a>>),

    #[serde(rename = "0x76ad60a")]
    #[serde(bound(deserialize = "Vec<HkbProjectStringData<'a>>: Deserialize<'de>"))]
    HkbProjectStringData(Vec<HkbProjectStringData<'a>>),

    #[serde(rename = "0x39de637e")]
    HkbProxyModifierProxyInfo(Vec<HkbProxyModifierProxyInfo>),

    #[serde(rename = "0x8a41554f")]
    #[serde(bound(deserialize = "Vec<HkbProxyModifier<'a>>: Deserialize<'de>"))]
    HkbProxyModifier(Vec<HkbProxyModifier<'a>>),

    #[serde(rename = "0xa0a7bf9c")]
    HkbRaiseEventCommand(Vec<HkbRaiseEventCommand>),

    #[serde(rename = "0x9416affd")]
    HkbRealEventPayload(Vec<HkbRealEventPayload>),

    #[serde(rename = "0xbb708bbd")]
    HkbRealVariableSequencedDataSample(Vec<HkbRealVariableSequencedDataSample>),

    #[serde(rename = "0xe2862d02")]
    HkbRealVariableSequencedData(Vec<HkbRealVariableSequencedData>),

    #[serde(rename = "0x26a5675a")]
    #[serde(bound(deserialize = "Vec<HkbReferencePoseGenerator<'a>>: Deserialize<'de>"))]
    HkbReferencePoseGenerator(Vec<HkbReferencePoseGenerator<'a>>),

    #[serde(rename = "0x58b1d082")]
    #[serde(bound(deserialize = "Vec<HkbRegisteredGenerator<'a>>: Deserialize<'de>"))]
    HkbRegisteredGenerator(Vec<HkbRegisteredGenerator<'a>>),

    #[serde(rename = "0x1e0bc068")]
    HkbRigidBodyRagdollControlData(Vec<HkbRigidBodyRagdollControlData>),

    #[serde(rename = "0xaa87d1eb")]
    #[serde(bound(deserialize = "Vec<HkbRigidBodyRagdollControlsModifier<'a>>: Deserialize<'de>"))]
    HkbRigidBodyRagdollControlsModifier(Vec<HkbRigidBodyRagdollControlsModifier<'a>>),

    #[serde(rename = "0x3eb2e082")]
    HkbRoleAttribute(Vec<HkbRoleAttribute>),

    #[serde(rename = "0xdc40bf4a")]
    HkbRotateCharacterModifierInternalState(Vec<HkbRotateCharacterModifierInternalState>),

    #[serde(rename = "0x877ebc0b")]
    HkbRotateCharacterModifier(Vec<HkbRotateCharacterModifier>),

    #[serde(rename = "0xfb56b692")]
    HkbSenseHandleModifierRange(Vec<HkbSenseHandleModifierRange>),

    #[serde(rename = "0x2a064d99")]
    #[serde(bound(deserialize = "Vec<HkbSenseHandleModifier<'a>>: Deserialize<'de>"))]
    HkbSenseHandleModifier(Vec<HkbSenseHandleModifier<'a>>),

    #[serde(rename = "0x419b9a05")]
    HkbSequenceInternalState(Vec<HkbSequenceInternalState>),

    #[serde(rename = "0x43182ca3")]
    #[serde(bound(deserialize = "Vec<HkbSequence<'a>>: Deserialize<'de>"))]
    HkbSequence(Vec<HkbSequence<'a>>),

    #[serde(rename = "0xe18b74b9")]
    #[serde(bound(deserialize = "Vec<HkbSetBehaviorCommand<'a>>: Deserialize<'de>"))]
    HkbSetBehaviorCommand(Vec<HkbSetBehaviorCommand<'a>>),

    #[serde(rename = "0xfab12b45")]
    HkbSetLocalTimeOfClipGeneratorCommand(Vec<HkbSetLocalTimeOfClipGeneratorCommand>),

    #[serde(rename = "0xc5160b64")]
    #[serde(bound(deserialize = "Vec<HkbSetNodePropertyCommand<'a>>: Deserialize<'de>"))]
    HkbSetNodePropertyCommand(Vec<HkbSetNodePropertyCommand<'a>>),

    #[serde(rename = "0xf3ae5fca")]
    HkbSetWordVariableCommand(Vec<HkbSetWordVariableCommand>),

    #[serde(rename = "0xafcfa211")]
    HkbSetWorldFromModelModifier(Vec<HkbSetWorldFromModelModifier>),

    #[serde(rename = "0x2a241367")]
    HkbSimulationControlCommand(Vec<HkbSimulationControlCommand>),

    #[serde(rename = "0xa40822b4")]
    HkbSimulationStateInfo(Vec<HkbSimulationStateInfo>),

    #[serde(rename = "0xbb90d54f")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineActiveTransitionInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineActiveTransitionInfo(Vec<HkbStateMachineActiveTransitionInfo<'a>>),

    #[serde(rename = "0x26d5499")]
    HkbStateMachineDelayedTransitionInfo(Vec<HkbStateMachineDelayedTransitionInfo>),

    #[serde(rename = "0xb07b4388")]
    HkbStateMachineEventPropertyArray(Vec<HkbStateMachineEventPropertyArray>),

    #[serde(rename = "0xbd1a7502")]
    HkbStateMachineInternalState(Vec<HkbStateMachineInternalState>),

    #[serde(rename = "0x7358f5da")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineNestedStateMachineData<'a>>: Deserialize<'de>"))]
    HkbStateMachineNestedStateMachineData(Vec<HkbStateMachineNestedStateMachineData<'a>>),

    #[serde(rename = "0x3ab09a2e")]
    HkbStateMachineProspectiveTransitionInfo(Vec<HkbStateMachineProspectiveTransitionInfo>),

    #[serde(rename = "0xed7f9d0")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineStateInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineStateInfo(Vec<HkbStateMachineStateInfo<'a>>),

    #[serde(rename = "0x60a881e5")]
    HkbStateMachineTimeInterval(Vec<HkbStateMachineTimeInterval>),

    #[serde(rename = "0xe397b11e")]
    HkbStateMachineTransitionInfoArray(Vec<HkbStateMachineTransitionInfoArray>),

    #[serde(rename = "0x9810c2d0")]
    HkbStateMachineTransitionInfoReference(Vec<HkbStateMachineTransitionInfoReference>),

    #[serde(rename = "0xcdec8025")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineTransitionInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineTransitionInfo(Vec<HkbStateMachineTransitionInfo<'a>>),

    #[serde(rename = "0x816c1dcb")]
    #[serde(bound(deserialize = "Vec<HkbStateMachine<'a>>: Deserialize<'de>"))]
    HkbStateMachine(Vec<HkbStateMachine<'a>>),

    #[serde(rename = "0x5ab50487")]
    #[serde(bound(deserialize = "Vec<HkbStringCondition<'a>>: Deserialize<'de>"))]
    HkbStringCondition(Vec<HkbStringCondition<'a>>),

    #[serde(rename = "0xed04256a")]
    #[serde(bound(deserialize = "Vec<HkbStringEventPayload<'a>>: Deserialize<'de>"))]
    HkbStringEventPayload(Vec<HkbStringEventPayload<'a>>),

    #[serde(rename = "0xc0fcc436")]
    #[serde(bound(deserialize = "Vec<HkbTestStateChooser<'a>>: Deserialize<'de>"))]
    HkbTestStateChooser(Vec<HkbTestStateChooser<'a>>),

    #[serde(rename = "0x83ec2d42")]
    HkbTimerModifierInternalState(Vec<HkbTimerModifierInternalState>),

    #[serde(rename = "0x338b4879")]
    HkbTimerModifier(Vec<HkbTimerModifier>),

    #[serde(rename = "0x5ca91c99")]
    HkbTransformVectorModifierInternalState(Vec<HkbTransformVectorModifierInternalState>),

    #[serde(rename = "0xf93e0e24")]
    HkbTransformVectorModifier(Vec<HkbTransformVectorModifier>),

    #[serde(rename = "0x945da157")]
    HkbTransitionEffect(Vec<HkbTransitionEffect>),

    #[serde(rename = "0xb6b76b32")]
    HkbTwistModifier(Vec<HkbTwistModifier>),

    #[serde(rename = "0x4d592f72")]
    #[serde(bound(deserialize = "Vec<HkbVariableBindingSetBinding<'a>>: Deserialize<'de>"))]
    HkbVariableBindingSetBinding(Vec<HkbVariableBindingSetBinding<'a>>),

    #[serde(rename = "0x338ad4ff")]
    HkbVariableBindingSet(Vec<HkbVariableBindingSet>),

    #[serde(rename = "0x9e746ba2")]
    HkbVariableInfo(Vec<HkbVariableInfo>),

    #[serde(rename = "0x27812d8d")]
    #[serde(bound(deserialize = "Vec<HkbVariableValueSet<'a>>: Deserialize<'de>"))]
    HkbVariableValueSet(Vec<HkbVariableValueSet<'a>>),

    #[serde(rename = "0xb99bd6a")]
    HkbVariableValue(Vec<HkbVariableValue>),

    #[serde(rename = "0x25640b46")]
    HkbWorldEnums(Vec<HkbWorldEnums>),

    #[serde(rename = "0xa3af8783")]
    HkbWorldFromModelModeData(Vec<HkbWorldFromModelModeData>),

    #[serde(rename = "0xce6f8a6c")]
    #[serde(bound(deserialize = "Vec<HkClassEnumItem<'a>>: Deserialize<'de>"))]
    HkClassEnumItem(Vec<HkClassEnumItem<'a>>),

    #[serde(rename = "0x8a3609cf")]
    #[serde(bound(deserialize = "Vec<HkClassEnum<'a>>: Deserialize<'de>"))]
    HkClassEnum(Vec<HkClassEnum<'a>>),

    #[serde(rename = "0x5c7ea4c2")]
    #[serde(bound(deserialize = "Vec<HkClassMember<'a>>: Deserialize<'de>"))]
    HkClassMember(Vec<HkClassMember<'a>>),

    #[serde(rename = "0x75585ef6")]
    #[serde(bound(deserialize = "Vec<HkClass<'a>>: Deserialize<'de>"))]
    HkClass(Vec<HkClass<'a>>),

    #[serde(rename = "0x106b96ce")]
    HkColor(Vec<HkColor>),

    #[serde(rename = "0x4e32287c")]
    HkContactPointMaterial(Vec<HkContactPointMaterial>),

    #[serde(rename = "0x91d7dd8e")]
    HkContactPoint(Vec<HkContactPoint>),

    #[serde(rename = "0x1388d601")]
    #[serde(bound(deserialize = "Vec<HkCustomAttributesAttribute<'a>>: Deserialize<'de>"))]
    HkCustomAttributesAttribute(Vec<HkCustomAttributesAttribute<'a>>),

    #[serde(rename = "0xbff19005")]
    HkCustomAttributes(Vec<HkCustomAttributes>),

    #[serde(rename = "0x1e3857bb")]
    #[serde(bound(deserialize = "Vec<HkDataObjectTypeAttribute<'a>>: Deserialize<'de>"))]
    HkDataObjectTypeAttribute(Vec<HkDataObjectTypeAttribute<'a>>),

    #[serde(rename = "0xe9f9578a")]
    #[serde(bound(deserialize = "Vec<HkDescriptionAttribute<'a>>: Deserialize<'de>"))]
    HkDescriptionAttribute(Vec<HkDescriptionAttribute<'a>>),

    #[serde(rename = "0x630edd9e")]
    #[serde(bound(deserialize = "Vec<HkDocumentationAttribute<'a>>: Deserialize<'de>"))]
    HkDocumentationAttribute(Vec<HkDocumentationAttribute<'a>>),

    #[serde(rename = "0x9687513b")]
    HkGeometryTriangle(Vec<HkGeometryTriangle>),

    #[serde(rename = "0x98dd8bdc")]
    HkGeometry(Vec<HkGeometry>),

    #[serde(rename = "0x23aadfb6")]
    #[serde(bound(deserialize = "Vec<HkGizmoAttribute<'a>>: Deserialize<'de>"))]
    HkGizmoAttribute(Vec<HkGizmoAttribute<'a>>),

    #[serde(rename = "0x7684dc80")]
    HkHalf8(Vec<HkHalf8>),

    #[serde(rename = "0x87fe6b5c")]
    #[serde(bound(deserialize = "Vec<HkIndexedTransformSet<'a>>: Deserialize<'de>"))]
    HkIndexedTransformSet(Vec<HkIndexedTransformSet<'a>>),

    #[serde(rename = "0x255d8164")]
    HkLinkAttribute(Vec<HkLinkAttribute>),

    #[serde(rename = "0xb1a96c2f")]
    #[serde(bound(deserialize = "Vec<HkLocalFrameGroup<'a>>: Deserialize<'de>"))]
    HkLocalFrameGroup(Vec<HkLocalFrameGroup<'a>>),

    #[serde(rename = "0x94a620a8")]
    #[serde(bound(deserialize = "Vec<HkMemoryMeshBody<'a>>: Deserialize<'de>"))]
    HkMemoryMeshBody(Vec<HkMemoryMeshBody<'a>>),

    #[serde(rename = "0x12156ee3")]
    #[serde(bound(deserialize = "Vec<HkMemoryMeshMaterial<'a>>: Deserialize<'de>"))]
    HkMemoryMeshMaterial(Vec<HkMemoryMeshMaterial<'a>>),

    #[serde(rename = "0xb743a578")]
    #[serde(bound(deserialize = "Vec<HkMemoryMeshShape<'a>>: Deserialize<'de>"))]
    HkMemoryMeshShape(Vec<HkMemoryMeshShape<'a>>),

    #[serde(rename = "0x2db6577c")]
    #[serde(bound(deserialize = "Vec<HkMemoryMeshTexture<'a>>: Deserialize<'de>"))]
    HkMemoryMeshTexture(Vec<HkMemoryMeshTexture<'a>>),

    #[serde(rename = "0xa2e50753")]
    HkMemoryMeshVertexBuffer(Vec<HkMemoryMeshVertexBuffer>),

    #[serde(rename = "0x4762f92a")]
    #[serde(bound(deserialize = "Vec<HkMemoryResourceContainer<'a>>: Deserialize<'de>"))]
    HkMemoryResourceContainer(Vec<HkMemoryResourceContainer<'a>>),

    #[serde(rename = "0x3144d17c")]
    #[serde(bound(deserialize = "Vec<HkMemoryResourceHandleExternalLink<'a>>: Deserialize<'de>"))]
    HkMemoryResourceHandleExternalLink(Vec<HkMemoryResourceHandleExternalLink<'a>>),

    #[serde(rename = "0xbffac086")]
    #[serde(bound(deserialize = "Vec<HkMemoryResourceHandle<'a>>: Deserialize<'de>"))]
    HkMemoryResourceHandle(Vec<HkMemoryResourceHandle<'a>>),

    #[serde(rename = "0xd0be5d7d")]
    HkMeshBody(Vec<HkMeshBody>),

    #[serde(rename = "0x6075f3ff")]
    #[serde(bound(deserialize = "Vec<HkMeshSectionCinfo<'a>>: Deserialize<'de>"))]
    HkMeshSectionCinfo(Vec<HkMeshSectionCinfo<'a>>),

    #[serde(rename = "0x1893c365")]
    #[serde(bound(deserialize = "Vec<HkMeshSection<'a>>: Deserialize<'de>"))]
    HkMeshSection(Vec<HkMeshSection<'a>>),

    #[serde(rename = "0x9117d62e")]
    HkMeshShape(Vec<HkMeshShape>),

    #[serde(rename = "0xc9887918")]
    HkMeshTexture(Vec<HkMeshTexture>),

    #[serde(rename = "0x534b08c8")]
    HkMeshVertexBuffer(Vec<HkMeshVertexBuffer>),

    #[serde(rename = "0x338c092f")]
    HkModelerNodeTypeAttribute(Vec<HkModelerNodeTypeAttribute>),

    #[serde(rename = "0x738fca05")]
    #[serde(bound(deserialize = "Vec<HkMonitorStreamColorTableColorPair<'a>>: Deserialize<'de>"))]
    HkMonitorStreamColorTableColorPair(Vec<HkMonitorStreamColorTableColorPair<'a>>),

    #[serde(rename = "0x79e53e85")]
    HkMonitorStreamColorTable(Vec<HkMonitorStreamColorTable>),

    #[serde(rename = "0x7798b7db")]
    #[serde(bound(deserialize = "Vec<HkMonitorStreamFrameInfo<'a>>: Deserialize<'de>"))]
    HkMonitorStreamFrameInfo(Vec<HkMonitorStreamFrameInfo<'a>>),

    #[serde(rename = "0x2c76ce16")]
    #[serde(bound(deserialize = "Vec<HkMonitorStreamStringMapStringMap<'a>>: Deserialize<'de>"))]
    HkMonitorStreamStringMapStringMap(Vec<HkMonitorStreamStringMapStringMap<'a>>),

    #[serde(rename = "0xc4d3a8b4")]
    HkMonitorStreamStringMap(Vec<HkMonitorStreamStringMap>),

    #[serde(rename = "0x7c338c66")]
    #[serde(bound(deserialize = "Vec<HkMoppBvTreeShapeBase<'a>>: Deserialize<'de>"))]
    HkMoppBvTreeShapeBase(Vec<HkMoppBvTreeShapeBase<'a>>),

    #[serde(rename = "0x5797386e")]
    HkMotionState(Vec<HkMotionState>),

    #[serde(rename = "0x4731fb1b")]
    HkMultipleVertexBufferElementInfo(Vec<HkMultipleVertexBufferElementInfo>),

    #[serde(rename = "0xa0e22afc")]
    HkMultipleVertexBufferLockedElement(Vec<HkMultipleVertexBufferLockedElement>),

    #[serde(rename = "0xdafbe0e6")]
    #[serde(bound(deserialize = "Vec<HkMultipleVertexBufferVertexBufferInfo<'a>>: Deserialize<'de>"))]
    HkMultipleVertexBufferVertexBufferInfo(Vec<HkMultipleVertexBufferVertexBufferInfo<'a>>),

    #[serde(rename = "0xde3ab602")]
    #[serde(bound(deserialize = "Vec<HkMultipleVertexBuffer<'a>>: Deserialize<'de>"))]
    HkMultipleVertexBuffer(Vec<HkMultipleVertexBuffer<'a>>),

    #[serde(rename = "0x11e4408b")]
    HkMultiThreadCheck(Vec<HkMultiThreadCheck>),

    #[serde(rename = "0xdcdb8b8b")]
    Hkp2DAngConstraintAtom(Vec<Hkp2DAngConstraintAtom>),

    #[serde(rename = "0x2c5189dd")]
    #[serde(bound(deserialize = "Vec<HkpAabbPhantom<'a>>: Deserialize<'de>"))]
    HkpAabbPhantom(Vec<HkpAabbPhantom<'a>>),

    #[serde(rename = "0x9c16df5b")]
    HkPackedVector3(Vec<HkPackedVector3>),

    #[serde(rename = "0x79f9ffda")]
    HkPackfileHeader(Vec<HkPackfileHeader>),

    #[serde(rename = "0xf2a92154")]
    HkPackfileSectionHeader(Vec<HkPackfileSectionHeader>),

    #[serde(rename = "0xbdf70a51")]
    #[serde(bound(deserialize = "Vec<HkpAction<'a>>: Deserialize<'de>"))]
    HkpAction(Vec<HkpAction<'a>>),

    #[serde(rename = "0x626e55a")]
    HkpAgent1NSector(Vec<HkpAgent1NSector>),

    #[serde(rename = "0x35bb3cd0")]
    HkpAngConstraintAtom(Vec<HkpAngConstraintAtom>),

    #[serde(rename = "0xf313aa80")]
    HkpAngFrictionConstraintAtom(Vec<HkpAngFrictionConstraintAtom>),

    #[serde(rename = "0x9be0d9d")]
    HkpAngLimitConstraintAtom(Vec<HkpAngLimitConstraintAtom>),

    #[serde(rename = "0x81f087ff")]
    #[serde(bound(deserialize = "Vec<HkpAngMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpAngMotorConstraintAtom(Vec<HkpAngMotorConstraintAtom<'a>>),

    #[serde(rename = "0x35f4c487")]
    HkpAngularDashpotAction(Vec<HkpAngularDashpotAction>),

    #[serde(rename = "0x674bcd2d")]
    #[serde(bound(deserialize = "Vec<HkpArrayAction<'a>>: Deserialize<'de>"))]
    HkpArrayAction(Vec<HkpArrayAction<'a>>),

    #[serde(rename = "0xc73dcaf9")]
    HkpBallAndSocketConstraintDataAtoms(Vec<HkpBallAndSocketConstraintDataAtoms>),

    #[serde(rename = "0x5a6954d9")]
    HkpBallAndSocketConstraintData(Vec<HkpBallAndSocketConstraintData>),

    #[serde(rename = "0x57b06d35")]
    #[serde(bound(deserialize = "Vec<HkpBallGun<'a>>: Deserialize<'de>"))]
    HkpBallGun(Vec<HkpBallGun<'a>>),

    #[serde(rename = "0xc9cbedf2")]
    HkpBallSocketChainDataConstraintInfo(Vec<HkpBallSocketChainDataConstraintInfo>),

    #[serde(rename = "0x102aae9c")]
    HkpBallSocketChainData(Vec<HkpBallSocketChainData>),

    #[serde(rename = "0xe70e4dfa")]
    HkpBallSocketConstraintAtom(Vec<HkpBallSocketConstraintAtom>),

    #[serde(rename = "0xc00f3403")]
    #[serde(bound(deserialize = "Vec<HkpBinaryAction<'a>>: Deserialize<'de>"))]
    HkpBinaryAction(Vec<HkpBinaryAction<'a>>),

    #[serde(rename = "0xbafa2bb7")]
    HkpSphereMotion(Vec<HkpSphereMotion>),

    #[serde(rename = "0x3444d2d5")]
    HkpBoxShape(Vec<HkpBoxShape>),

    #[serde(rename = "0x7d6310c8")]
    #[serde(bound(deserialize = "Vec<HkpBreakableConstraintData<'a>>: Deserialize<'de>"))]
    HkpBreakableConstraintData(Vec<HkpBreakableConstraintData<'a>>),

    #[serde(rename = "0xde152a4d")]
    HkpBridgeAtoms(Vec<HkpBridgeAtoms>),

    #[serde(rename = "0x87a4f31b")]
    #[serde(bound(deserialize = "Vec<HkpBridgeConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpBridgeConstraintAtom(Vec<HkpBridgeConstraintAtom<'a>>),

    #[serde(rename = "0x940569dc")]
    HkpBroadPhaseHandle(Vec<HkpBroadPhaseHandle>),

    #[serde(rename = "0x286eb64c")]
    #[serde(bound(deserialize = "Vec<HkpBvShape<'a>>: Deserialize<'de>"))]
    HkpBvShape(Vec<HkpBvShape<'a>>),

    #[serde(rename = "0xa823d623")]
    HkpBvTreeShape(Vec<HkpBvTreeShape>),

    #[serde(rename = "0xcf227f58")]
    HkpCachingShapePhantom(Vec<HkpCachingShapePhantom>),

    #[serde(rename = "0xafcd79ad")]
    #[serde(bound(deserialize = "Vec<HkpCallbackConstraintMotor<'a>>: Deserialize<'de>"))]
    HkpCallbackConstraintMotor(Vec<HkpCallbackConstraintMotor<'a>>),

    #[serde(rename = "0xdd0b1fd3")]
    HkpCapsuleShape(Vec<HkpCapsuleShape>),

    #[serde(rename = "0x54a4b841")]
    #[serde(bound(deserialize = "Vec<HkpCdBody<'a>>: Deserialize<'de>"))]
    HkpCdBody(Vec<HkpCdBody<'a>>),

    #[serde(rename = "0x1d7dbdd2")]
    HkpCenterOfMassChangerModifierConstraintAtom(Vec<HkpCenterOfMassChangerModifierConstraintAtom>),

    #[serde(rename = "0x586d97b2")]
    #[serde(bound(deserialize = "Vec<HkpCharacterProxyCinfo<'a>>: Deserialize<'de>"))]
    HkpCharacterProxyCinfo(Vec<HkpCharacterProxyCinfo<'a>>),

    #[serde(rename = "0x892f441")]
    #[serde(bound(deserialize = "Vec<HkpCharacterRigidBodyCinfo<'a>>: Deserialize<'de>"))]
    HkpCharacterRigidBodyCinfo(Vec<HkpCharacterRigidBodyCinfo<'a>>),

    #[serde(rename = "0xf2b1f399")]
    HkpCogWheelConstraintAtom(Vec<HkpCogWheelConstraintAtom>),

    #[serde(rename = "0xf855ba44")]
    HkpCogWheelConstraintDataAtoms(Vec<HkpCogWheelConstraintDataAtoms>),

    #[serde(rename = "0x7f0e53fc")]
    HkpCogWheelConstraintData(Vec<HkpCogWheelConstraintData>),

    #[serde(rename = "0xb5f0e6b1")]
    #[serde(bound(deserialize = "Vec<HkpCollidableBoundingVolumeData<'a>>: Deserialize<'de>"))]
    HkpCollidableBoundingVolumeData(Vec<HkpCollidableBoundingVolumeData<'a>>),

    #[serde(rename = "0x9a0e42a5")]
    HkpCollidable(Vec<HkpCollidable>),

    #[serde(rename = "0x2603bf04")]
    #[serde(bound(deserialize = "Vec<HkpCollisionFilterList<'a>>: Deserialize<'de>"))]
    HkpCollisionFilterList(Vec<HkpCollisionFilterList<'a>>),

    #[serde(rename = "0x60960336")]
    HkpCollisionFilter(Vec<HkpCollisionFilter>),

    #[serde(rename = "0xcbfc95a4")]
    HkpCompressedMeshShapeBigTriangle(Vec<HkpCompressedMeshShapeBigTriangle>),

    #[serde(rename = "0x5d0d67bd")]
    HkpCompressedMeshShapeChunk(Vec<HkpCompressedMeshShapeChunk>),

    #[serde(rename = "0x385bb842")]
    HkpCompressedMeshShapeConvexPiece(Vec<HkpCompressedMeshShapeConvexPiece>),

    #[serde(rename = "0xa62d5e6e")]
    #[serde(bound(deserialize = "Vec<HkpCompressedMeshShape<'a>>: Deserialize<'de>"))]
    HkpCompressedMeshShape(Vec<HkpCompressedMeshShape<'a>>),

    #[serde(rename = "0x97b6e143")]
    HkpCompressedSampledHeightFieldShape(Vec<HkpCompressedSampledHeightFieldShape>),

    #[serde(rename = "0xf19443c8")]
    HkpConeLimitConstraintAtom(Vec<HkpConeLimitConstraintAtom>),

    #[serde(rename = "0x20a447fe")]
    #[serde(bound(deserialize = "Vec<HkpConstrainedSystemFilter<'a>>: Deserialize<'de>"))]
    HkpConstrainedSystemFilter(Vec<HkpConstrainedSystemFilter<'a>>),

    #[serde(rename = "0x59d67ef6")]
    HkpConstraintAtom(Vec<HkpConstraintAtom>),

    #[serde(rename = "0x5facc7ff")]
    HkpConstraintChainData(Vec<HkpConstraintChainData>),

    #[serde(rename = "0xc3971189")]
    #[serde(bound(deserialize = "Vec<HkpConstraintChainInstanceAction<'a>>: Deserialize<'de>"))]
    HkpConstraintChainInstanceAction(Vec<HkpConstraintChainInstanceAction<'a>>),

    #[serde(rename = "0x7a490753")]
    #[serde(bound(deserialize = "Vec<HkpConstraintChainInstance<'a>>: Deserialize<'de>"))]
    HkpConstraintChainInstance(Vec<HkpConstraintChainInstance<'a>>),

    #[serde(rename = "0xc3b577b1")]
    HkpConstraintCollisionFilter(Vec<HkpConstraintCollisionFilter>),

    #[serde(rename = "0x80559a4e")]
    HkpConstraintData(Vec<HkpConstraintData>),

    #[serde(rename = "0xee3c2aec")]
    #[serde(bound(deserialize = "Vec<HkpEntitySmallArraySerializeOverrideType<'a>>: Deserialize<'de>"))]
    HkpEntitySmallArraySerializeOverrideType(Vec<HkpEntitySmallArraySerializeOverrideType<'a>>),

    #[serde(rename = "0x34eba5f")]
    #[serde(bound(deserialize = "Vec<HkpConstraintInstance<'a>>: Deserialize<'de>"))]
    HkpConstraintInstance(Vec<HkpConstraintInstance<'a>>),

    #[serde(rename = "0x6a44c317")]
    HkpConstraintMotor(Vec<HkpConstraintMotor>),

    #[serde(rename = "0x81d074a4")]
    HkpConvexListFilter(Vec<HkpConvexListFilter>),

    #[serde(rename = "0x450b26e8")]
    #[serde(bound(deserialize = "Vec<HkpConvexListShape<'a>>: Deserialize<'de>"))]
    HkpConvexListShape(Vec<HkpConvexListShape<'a>>),

    #[serde(rename = "0x38fd3d97")]
    #[serde(bound(deserialize = "Vec<HkpConvexPieceMeshShape<'a>>: Deserialize<'de>"))]
    HkpConvexPieceMeshShape(Vec<HkpConvexPieceMeshShape<'a>>),

    #[serde(rename = "0xa5bd1d6e")]
    HkpConvexPieceStreamData(Vec<HkpConvexPieceStreamData>),

    #[serde(rename = "0xf8f74f85")]
    HkpConvexShape(Vec<HkpConvexShape>),

    #[serde(rename = "0xfbd72f9")]
    HkpConvexTransformShapeBase(Vec<HkpConvexTransformShapeBase>),

    #[serde(rename = "0xae3e5017")]
    HkpConvexTransformShape(Vec<HkpConvexTransformShape>),

    #[serde(rename = "0x5ba0a5f7")]
    HkpConvexTranslateShape(Vec<HkpConvexTranslateShape>),

    #[serde(rename = "0x63d38e9c")]
    HkpConvexVerticesConnectivity(Vec<HkpConvexVerticesConnectivity>),

    #[serde(rename = "0x3d80c5bf")]
    HkpConvexVerticesShapeFourVectors(Vec<HkpConvexVerticesShapeFourVectors>),

    #[serde(rename = "0x28726ad8")]
    #[serde(bound(deserialize = "Vec<HkpConvexVerticesShape<'a>>: Deserialize<'de>"))]
    HkpConvexVerticesShape(Vec<HkpConvexVerticesShape<'a>>),

    #[serde(rename = "0x3e463c3a")]
    HkpCylinderShape(Vec<HkpCylinderShape>),

    #[serde(rename = "0x50746c6e")]
    HkpDashpotAction(Vec<HkpDashpotAction>),

    #[serde(rename = "0xb69c1c02")]
    HkpDefaultConvexListFilter(Vec<HkpDefaultConvexListFilter>),

    #[serde(rename = "0x77d6b19f")]
    HkpDefaultWorldMemoryWatchDog(Vec<HkpDefaultWorldMemoryWatchDog>),

    #[serde(rename = "0xfac3351c")]
    #[serde(bound(deserialize = "Vec<HkpDisableEntityCollisionFilter<'a>>: Deserialize<'de>"))]
    HkpDisableEntityCollisionFilter(Vec<HkpDisableEntityCollisionFilter<'a>>),

    #[serde(rename = "0xc8ae86a7")]
    #[serde(bound(deserialize = "Vec<HkpDisplayBindingDataPhysicsSystem<'a>>: Deserialize<'de>"))]
    HkpDisplayBindingDataPhysicsSystem(Vec<HkpDisplayBindingDataPhysicsSystem<'a>>),

    #[serde(rename = "0xfe16e2a3")]
    #[serde(bound(deserialize = "Vec<HkpDisplayBindingDataRigidBody<'a>>: Deserialize<'de>"))]
    HkpDisplayBindingDataRigidBody(Vec<HkpDisplayBindingDataRigidBody<'a>>),

    #[serde(rename = "0xdc46c906")]
    #[serde(bound(deserialize = "Vec<HkpDisplayBindingData<'a>>: Deserialize<'de>"))]
    HkpDisplayBindingData(Vec<HkpDisplayBindingData<'a>>),

    #[serde(rename = "0xf557023c")]
    HkpEntityExtendedListeners(Vec<HkpEntityExtendedListeners>),

    #[serde(rename = "0x81147f05")]
    #[serde(bound(deserialize = "Vec<HkpEntitySpuCollisionCallback<'a>>: Deserialize<'de>"))]
    HkpEntitySpuCollisionCallback(Vec<HkpEntitySpuCollisionCallback<'a>>),

    #[serde(rename = "0xa03c774b")]
    #[serde(bound(deserialize = "Vec<HkpEntity<'a>>: Deserialize<'de>"))]
    HkpEntity(Vec<HkpEntity<'a>>),

    #[serde(rename = "0xf204b155")]
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShapeShapesSubpart<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShapeShapesSubpart(Vec<HkpExtendedMeshShapeShapesSubpart<'a>>),

    #[serde(rename = "0xf4608207")]
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShapeSubpart<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShapeSubpart(Vec<HkpExtendedMeshShapeSubpart<'a>>),

    #[serde(rename = "0x44c32df6")]
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShapeTrianglesSubpart<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShapeTrianglesSubpart(Vec<HkpExtendedMeshShapeTrianglesSubpart<'a>>),

    #[serde(rename = "0x177114a2")]
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShape<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShape(Vec<HkpExtendedMeshShape<'a>>),

    #[serde(rename = "0x3d3da311")]
    HkpFastMeshShape(Vec<HkpFastMeshShape>),

    #[serde(rename = "0x852ab70b")]
    #[serde(bound(deserialize = "Vec<HkpFirstPersonGun<'a>>: Deserialize<'de>"))]
    HkpFirstPersonGun(Vec<HkpFirstPersonGun<'a>>),

    #[serde(rename = "0x64abf85c")]
    HkpThinBoxMotion(Vec<HkpThinBoxMotion>),

    #[serde(rename = "0xd6421f19")]
    HkpGenericConstraintDataSchemeConstraintInfo(Vec<HkpGenericConstraintDataSchemeConstraintInfo>),

    #[serde(rename = "0x11fd6f6c")]
    #[serde(bound(deserialize = "Vec<HkpGenericConstraintDataScheme<'a>>: Deserialize<'de>"))]
    HkpGenericConstraintDataScheme(Vec<HkpGenericConstraintDataScheme<'a>>),

    #[serde(rename = "0xfa824640")]
    HkpGenericConstraintData(Vec<HkpGenericConstraintData>),

    #[serde(rename = "0x5e2754cd")]
    #[serde(bound(deserialize = "Vec<HkpGravityGun<'a>>: Deserialize<'de>"))]
    HkpGravityGun(Vec<HkpGravityGun<'a>>),

    #[serde(rename = "0x5cc01561")]
    HkpGroupCollisionFilter(Vec<HkpGroupCollisionFilter>),

    #[serde(rename = "0x65ee88e4")]
    HkpGroupFilter(Vec<HkpGroupFilter>),

    #[serde(rename = "0xe7eca7eb")]
    HkpSphereRepShape(Vec<HkpSphereRepShape>),

    #[serde(rename = "0x6958371c")]
    HkpHingeConstraintDataAtoms(Vec<HkpHingeConstraintDataAtoms>),

    #[serde(rename = "0x9590f046")]
    HkpHingeConstraintData(Vec<HkpHingeConstraintData>),

    #[serde(rename = "0x555876ff")]
    HkpHingeLimitsDataAtoms(Vec<HkpHingeLimitsDataAtoms>),

    #[serde(rename = "0xbd46760a")]
    HkpHingeLimitsData(Vec<HkpHingeLimitsData>),

    #[serde(rename = "0x5c6aa14d")]
    HkpViscousSurfaceModifierConstraintAtom(Vec<HkpViscousSurfaceModifierConstraintAtom>),

    #[serde(rename = "0x3377b0b0")]
    HkpLimitedForceConstraintMotor(Vec<HkpLimitedForceConstraintMotor>),

    #[serde(rename = "0x54c7715b")]
    HkpLimitedHingeConstraintDataAtoms(Vec<HkpLimitedHingeConstraintDataAtoms>),

    #[serde(rename = "0x7c15bb6b")]
    HkpLimitedHingeConstraintData(Vec<HkpLimitedHingeConstraintData>),

    #[serde(rename = "0x7b6b0210")]
    HkpLinConstraintAtom(Vec<HkpLinConstraintAtom>),

    #[serde(rename = "0xd7b3be03")]
    HkpLinearParametricCurve(Vec<HkpLinearParametricCurve>),

    #[serde(rename = "0x3e94ef7c")]
    HkpLinFrictionConstraintAtom(Vec<HkpLinFrictionConstraintAtom>),

    #[serde(rename = "0xe1a81497")]
    HkpLinkedCollidable(Vec<HkpLinkedCollidable>),

    #[serde(rename = "0xa44d1b07")]
    HkpLinLimitConstraintAtom(Vec<HkpLinLimitConstraintAtom>),

    #[serde(rename = "0x10312464")]
    #[serde(bound(deserialize = "Vec<HkpLinMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpLinMotorConstraintAtom(Vec<HkpLinMotorConstraintAtom<'a>>),

    #[serde(rename = "0x52b27d69")]
    HkpLinSoftConstraintAtom(Vec<HkpLinSoftConstraintAtom>),

    #[serde(rename = "0x80df0f90")]
    #[serde(bound(deserialize = "Vec<HkpListShapeChildInfo<'a>>: Deserialize<'de>"))]
    HkpListShapeChildInfo(Vec<HkpListShapeChildInfo<'a>>),

    #[serde(rename = "0xa1937cbd")]
    HkpListShape(Vec<HkpListShape>),

    #[serde(rename = "0x6748b2cf")]
    #[serde(bound(deserialize = "Vec<HkpMalleableConstraintData<'a>>: Deserialize<'de>"))]
    HkpMalleableConstraintData(Vec<HkpMalleableConstraintData<'a>>),

    #[serde(rename = "0xb6b28240")]
    HkpMassChangerModifierConstraintAtom(Vec<HkpMassChangerModifierConstraintAtom>),

    #[serde(rename = "0x68a56834")]
    HkpMassProperties(Vec<HkpMassProperties>),

    #[serde(rename = "0x33be6570")]
    HkpMaterial(Vec<HkpMaterial>),

    #[serde(rename = "0x886cde0c")]
    HkpMeshMaterial(Vec<HkpMeshMaterial>),

    #[serde(rename = "0x27336e5d")]
    #[serde(bound(deserialize = "Vec<HkpMeshShapeSubpart<'a>>: Deserialize<'de>"))]
    HkpMeshShapeSubpart(Vec<HkpMeshShapeSubpart<'a>>),

    #[serde(rename = "0x3bf12c0f")]
    HkpMeshShape(Vec<HkpMeshShape>),

    #[serde(rename = "0xb13fef1f")]
    #[serde(bound(deserialize = "Vec<HkpModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpModifierConstraintAtom(Vec<HkpModifierConstraintAtom<'a>>),

    #[serde(rename = "0x90b29d39")]
    HkpMoppBvTreeShape(Vec<HkpMoppBvTreeShape>),

    #[serde(rename = "0xd8fdbb08")]
    HkpMoppCodeCodeInfo(Vec<HkpMoppCodeCodeInfo>),

    #[serde(rename = "0x6ed8ac06")]
    HkpMoppCodeReindexedTerminal(Vec<HkpMoppCodeReindexedTerminal>),

    #[serde(rename = "0x924c2661")]
    HkpMoppCode(Vec<HkpMoppCode>),

    #[serde(rename = "0x98aadb4f")]
    #[serde(bound(deserialize = "Vec<HkpMotion<'a>>: Deserialize<'de>"))]
    HkpMotion(Vec<HkpMotion<'a>>),

    #[serde(rename = "0x8ff131d9")]
    HkpMotorAction(Vec<HkpMotorAction>),

    #[serde(rename = "0x6791ffce")]
    HkpMountedBallGun(Vec<HkpMountedBallGun>),

    #[serde(rename = "0x6e087fd6")]
    #[serde(bound(deserialize = "Vec<HkpMouseSpringAction<'a>>: Deserialize<'de>"))]
    HkpMouseSpringAction(Vec<HkpMouseSpringAction<'a>>),

    #[serde(rename = "0x79ab517d")]
    HkpMovingSurfaceModifierConstraintAtom(Vec<HkpMovingSurfaceModifierConstraintAtom>),

    #[serde(rename = "0xffdc0b65")]
    HkpMultiRayShapeRay(Vec<HkpMultiRayShapeRay>),

    #[serde(rename = "0xea2e7ec9")]
    HkpMultiRayShape(Vec<HkpMultiRayShape>),

    #[serde(rename = "0x61a590fc")]
    HkpMultiSphereShape(Vec<HkpMultiSphereShape>),

    #[serde(rename = "0xc03af40d")]
    HkpMultithreadedVehicleManager(Vec<HkpMultithreadedVehicleManager>),

    #[serde(rename = "0x66b42df1")]
    #[serde(bound(deserialize = "Vec<HkpNamedMeshMaterial<'a>>: Deserialize<'de>"))]
    HkpNamedMeshMaterial(Vec<HkpNamedMeshMaterial<'a>>),

    #[serde(rename = "0xb120a34f")]
    HkpNullCollisionFilter(Vec<HkpNullCollisionFilter>),

    #[serde(rename = "0x903abb2c")]
    #[serde(bound(deserialize = "Vec<HkPostFinishAttribute<'a>>: Deserialize<'de>"))]
    HkPostFinishAttribute(Vec<HkPostFinishAttribute<'a>>),

    #[serde(rename = "0x1f11b467")]
    HkpOverwritePivotConstraintAtom(Vec<HkpOverwritePivotConstraintAtom>),

    #[serde(rename = "0x36195969")]
    #[serde(bound(deserialize = "Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>: Deserialize<'de>"))]
    HkpPairCollisionFilterMapPairFilterKeyOverrideType(Vec<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>),

    #[serde(rename = "0x4abc140e")]
    #[serde(bound(deserialize = "Vec<HkpPairCollisionFilter<'a>>: Deserialize<'de>"))]
    HkpPairCollisionFilter(Vec<HkpPairCollisionFilter<'a>>),

    #[serde(rename = "0x9b7e6f86")]
    #[serde(bound(deserialize = "Vec<HkpPhantom<'a>>: Deserialize<'de>"))]
    HkpPhantom(Vec<HkpPhantom<'a>>),

    #[serde(rename = "0xc2a461e4")]
    #[serde(bound(deserialize = "Vec<HkpPhysicsData<'a>>: Deserialize<'de>"))]
    HkpPhysicsData(Vec<HkpPhysicsData<'a>>),

    #[serde(rename = "0xd0fd4bbe")]
    #[serde(bound(deserialize = "Vec<HkpPhysicsSystemWithContacts<'a>>: Deserialize<'de>"))]
    HkpPhysicsSystemWithContacts(Vec<HkpPhysicsSystemWithContacts<'a>>),

    #[serde(rename = "0xff724c17")]
    #[serde(bound(deserialize = "Vec<HkpPhysicsSystem<'a>>: Deserialize<'de>"))]
    HkpPhysicsSystem(Vec<HkpPhysicsSystem<'a>>),

    #[serde(rename = "0xc36bbd30")]
    HkpPlaneShape(Vec<HkpPlaneShape>),

    #[serde(rename = "0x8e7cb5da")]
    #[serde(bound(deserialize = "Vec<HkpPointToPathConstraintData<'a>>: Deserialize<'de>"))]
    HkpPointToPathConstraintData(Vec<HkpPointToPathConstraintData<'a>>),

    #[serde(rename = "0x749bc260")]
    HkpPointToPlaneConstraintDataAtoms(Vec<HkpPointToPlaneConstraintDataAtoms>),

    #[serde(rename = "0x65c56e17")]
    HkpPointToPlaneConstraintData(Vec<HkpPointToPlaneConstraintData>),

    #[serde(rename = "0x748fb303")]
    HkpPositionConstraintMotor(Vec<HkpPositionConstraintMotor>),

    #[serde(rename = "0xf88aee25")]
    #[serde(bound(deserialize = "Vec<HkpPoweredChainDataConstraintInfo<'a>>: Deserialize<'de>"))]
    HkpPoweredChainDataConstraintInfo(Vec<HkpPoweredChainDataConstraintInfo<'a>>),

    #[serde(rename = "0x38aeafc3")]
    HkpPoweredChainData(Vec<HkpPoweredChainData>),

    #[serde(rename = "0xcf071a1b")]
    #[serde(bound(deserialize = "Vec<HkpPoweredChainMapperLinkInfo<'a>>: Deserialize<'de>"))]
    HkpPoweredChainMapperLinkInfo(Vec<HkpPoweredChainMapperLinkInfo<'a>>),

    #[serde(rename = "0xf651c74d")]
    #[serde(bound(deserialize = "Vec<HkpPoweredChainMapperTarget<'a>>: Deserialize<'de>"))]
    HkpPoweredChainMapperTarget(Vec<HkpPoweredChainMapperTarget<'a>>),

    #[serde(rename = "0x7a77ef5")]
    #[serde(bound(deserialize = "Vec<HkpPoweredChainMapper<'a>>: Deserialize<'de>"))]
    HkpPoweredChainMapper(Vec<HkpPoweredChainMapper<'a>>),

    #[serde(rename = "0x7f516137")]
    HkpPrismaticConstraintDataAtoms(Vec<HkpPrismaticConstraintDataAtoms>),

    #[serde(rename = "0x3996c387")]
    HkpPrismaticConstraintData(Vec<HkpPrismaticConstraintData>),

    #[serde(rename = "0xb4f30148")]
    #[serde(bound(deserialize = "Vec<HkpProjectileGun<'a>>: Deserialize<'de>"))]
    HkpProjectileGun(Vec<HkpProjectileGun<'a>>),

    #[serde(rename = "0xc75925aa")]
    HkpPropertyValue(Vec<HkpPropertyValue>),

    #[serde(rename = "0x9ce308e9")]
    HkpProperty(Vec<HkpProperty>),

    #[serde(rename = "0x94a08848")]
    HkpPulleyConstraintAtom(Vec<HkpPulleyConstraintAtom>),

    #[serde(rename = "0xb149e5a")]
    HkpPulleyConstraintDataAtoms(Vec<HkpPulleyConstraintDataAtoms>),

    #[serde(rename = "0x972058ed")]
    HkpPulleyConstraintData(Vec<HkpPulleyConstraintData>),

    #[serde(rename = "0x30cae006")]
    HkpRackAndPinionConstraintAtom(Vec<HkpRackAndPinionConstraintAtom>),

    #[serde(rename = "0xa58a9659")]
    HkpRackAndPinionConstraintDataAtoms(Vec<HkpRackAndPinionConstraintDataAtoms>),

    #[serde(rename = "0xd180ebe0")]
    HkpRackAndPinionConstraintData(Vec<HkpRackAndPinionConstraintData>),

    #[serde(rename = "0xeed76b00")]
    HkpRagdollConstraintDataAtoms(Vec<HkpRagdollConstraintDataAtoms>),

    #[serde(rename = "0x8fb5dd29")]
    HkpRagdollConstraintData(Vec<HkpRagdollConstraintData>),

    #[serde(rename = "0x82b894c3")]
    HkpRagdollLimitsDataAtoms(Vec<HkpRagdollLimitsDataAtoms>),

    #[serde(rename = "0xcbdb44aa")]
    HkpRagdollLimitsData(Vec<HkpRagdollLimitsData>),

    #[serde(rename = "0x71013826")]
    #[serde(bound(deserialize = "Vec<HkpRagdollMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpRagdollMotorConstraintAtom(Vec<HkpRagdollMotorConstraintAtom<'a>>),

    #[serde(rename = "0xc4fa16c9")]
    #[serde(bound(deserialize = "Vec<HkpRejectChassisListener<'a>>: Deserialize<'de>"))]
    HkpRejectChassisListener(Vec<HkpRejectChassisListener<'a>>),

    #[serde(rename = "0x91367f03")]
    #[serde(bound(deserialize = "Vec<HkpRemoveTerminalsMoppModifier<'a>>: Deserialize<'de>"))]
    HkpRemoveTerminalsMoppModifier(Vec<HkpRemoveTerminalsMoppModifier<'a>>),

    #[serde(rename = "0x2dc0ec6a")]
    HkpReorientAction(Vec<HkpReorientAction>),

    #[serde(rename = "0x75f8d805")]
    HkpRigidBody(Vec<HkpRigidBody>),

    #[serde(rename = "0xa0c64586")]
    HkpRotationalConstraintDataAtoms(Vec<HkpRotationalConstraintDataAtoms>),

    #[serde(rename = "0x74867d9e")]
    HkpRotationalConstraintData(Vec<HkpRotationalConstraintData>),

    #[serde(rename = "0x11213421")]
    HkpSampledHeightFieldShape(Vec<HkpSampledHeightFieldShape>),

    #[serde(rename = "0x49ec7de3")]
    #[serde(bound(deserialize = "Vec<HkpSerializedAgentNnEntry<'a>>: Deserialize<'de>"))]
    HkpSerializedAgentNnEntry(Vec<HkpSerializedAgentNnEntry<'a>>),

    #[serde(rename = "0x54785c77")]
    #[serde(bound(deserialize = "Vec<HkpSerializedDisplayMarkerList<'a>>: Deserialize<'de>"))]
    HkpSerializedDisplayMarkerList(Vec<HkpSerializedDisplayMarkerList<'a>>),

    #[serde(rename = "0xd7c8c54f")]
    HkpSerializedDisplayMarker(Vec<HkpSerializedDisplayMarker>),

    #[serde(rename = "0x94ac5bec")]
    #[serde(bound(deserialize = "Vec<HkpSerializedDisplayRbTransformsDisplayTransformPair<'a>>: Deserialize<'de>"))]
    HkpSerializedDisplayRbTransformsDisplayTransformPair(Vec<HkpSerializedDisplayRbTransformsDisplayTransformPair<'a>>),

    #[serde(rename = "0xc18650ac")]
    HkpSerializedDisplayRbTransforms(Vec<HkpSerializedDisplayRbTransforms>),

    #[serde(rename = "0x10155a")]
    HkpSerializedSubTrack1NInfo(Vec<HkpSerializedSubTrack1NInfo>),

    #[serde(rename = "0xf12d48d9")]
    #[serde(bound(deserialize = "Vec<HkpSerializedTrack1NInfo<'a>>: Deserialize<'de>"))]
    HkpSerializedTrack1NInfo(Vec<HkpSerializedTrack1NInfo<'a>>),

    #[serde(rename = "0xf81db8e")]
    HkpSetLocalRotationsConstraintAtom(Vec<HkpSetLocalRotationsConstraintAtom>),

    #[serde(rename = "0x6e2a5198")]
    HkpSetLocalTransformsConstraintAtom(Vec<HkpSetLocalTransformsConstraintAtom>),

    #[serde(rename = "0x5cbfcf4a")]
    HkpSetLocalTranslationsConstraintAtom(Vec<HkpSetLocalTranslationsConstraintAtom>),

    #[serde(rename = "0xf05d137e")]
    HkpSetupStabilizationAtom(Vec<HkpSetupStabilizationAtom>),

    #[serde(rename = "0xe8c3991d")]
    HkpShapeCollection(Vec<HkpShapeCollection>),

    #[serde(rename = "0xea7f1d08")]
    #[serde(bound(deserialize = "Vec<HkpShapeInfo<'a>>: Deserialize<'de>"))]
    HkpShapeInfo(Vec<HkpShapeInfo<'a>>),

    #[serde(rename = "0xcb22fbcd")]
    HkpShapePhantom(Vec<HkpShapePhantom>),

    #[serde(rename = "0x666490a1")]
    HkpShape(Vec<HkpShape>),

    #[serde(rename = "0x920df11a")]
    HkpSimpleContactConstraintAtom(Vec<HkpSimpleContactConstraintAtom>),

    #[serde(rename = "0xb59d1734")]
    HkpSimpleContactConstraintDataInfo(Vec<HkpSimpleContactConstraintDataInfo>),

    #[serde(rename = "0xd38738c1")]
    HkpSimpleMeshShapeTriangle(Vec<HkpSimpleMeshShapeTriangle>),

    #[serde(rename = "0x16b3c811")]
    HkpSimpleMeshShape(Vec<HkpSimpleMeshShape>),

    #[serde(rename = "0x98bfa6ce")]
    #[serde(bound(deserialize = "Vec<HkpSimpleShapePhantomCollisionDetail<'a>>: Deserialize<'de>"))]
    HkpSimpleShapePhantomCollisionDetail(Vec<HkpSimpleShapePhantomCollisionDetail<'a>>),

    #[serde(rename = "0x32a2a8a8")]
    HkpSimpleShapePhantom(Vec<HkpSimpleShapePhantom>),

    #[serde(rename = "0x97aba922")]
    #[serde(bound(deserialize = "Vec<HkpSimulation<'a>>: Deserialize<'de>"))]
    HkpSimulation(Vec<HkpSimulation<'a>>),

    #[serde(rename = "0x73aa1d38")]
    #[serde(bound(deserialize = "Vec<HkpSingleShapeContainer<'a>>: Deserialize<'de>"))]
    HkpSingleShapeContainer(Vec<HkpSingleShapeContainer<'a>>),

    #[serde(rename = "0xecb34e27")]
    HkpSoftContactModifierConstraintAtom(Vec<HkpSoftContactModifierConstraintAtom>),

    #[serde(rename = "0x795d9fa")]
    HkpSphereShape(Vec<HkpSphereShape>),

    #[serde(rename = "0x88fc09fa")]
    HkpSpringAction(Vec<HkpSpringAction>),

    #[serde(rename = "0x7ead26f6")]
    HkpSpringDamperConstraintMotor(Vec<HkpSpringDamperConstraintMotor>),

    #[serde(rename = "0xc624a180")]
    HkpStiffSpringChainDataConstraintInfo(Vec<HkpStiffSpringChainDataConstraintInfo>),

    #[serde(rename = "0xf170356b")]
    HkpStiffSpringChainData(Vec<HkpStiffSpringChainData>),

    #[serde(rename = "0x6c128096")]
    HkpStiffSpringConstraintAtom(Vec<HkpStiffSpringConstraintAtom>),

    #[serde(rename = "0x207eb376")]
    HkpStiffSpringConstraintDataAtoms(Vec<HkpStiffSpringConstraintDataAtoms>),

    #[serde(rename = "0xb98f66f4")]
    HkpStiffSpringConstraintData(Vec<HkpStiffSpringConstraintData>),

    #[serde(rename = "0x2ca3e906")]
    HkpStorageExtendedMeshShapeMaterial(Vec<HkpStorageExtendedMeshShapeMaterial>),

    #[serde(rename = "0x5aad4de6")]
    HkpStorageExtendedMeshShapeMeshSubpartStorage(Vec<HkpStorageExtendedMeshShapeMeshSubpartStorage>),

    #[serde(rename = "0x3f7d804c")]
    HkpStorageExtendedMeshShapeShapeSubpartStorage(Vec<HkpStorageExtendedMeshShapeShapeSubpartStorage>),

    #[serde(rename = "0xb469efbc")]
    #[serde(bound(deserialize = "Vec<HkpStorageExtendedMeshShape<'a>>: Deserialize<'de>"))]
    HkpStorageExtendedMeshShape(Vec<HkpStorageExtendedMeshShape<'a>>),

    #[serde(rename = "0xbf27438")]
    HkpStorageMeshShapeSubpartStorage(Vec<HkpStorageMeshShapeSubpartStorage>),

    #[serde(rename = "0xbefd8b39")]
    #[serde(bound(deserialize = "Vec<HkpStorageMeshShape<'a>>: Deserialize<'de>"))]
    HkpStorageMeshShape(Vec<HkpStorageMeshShape<'a>>),

    #[serde(rename = "0x15ff414b")]
    HkpStorageSampledHeightFieldShape(Vec<HkpStorageSampledHeightFieldShape>),

    #[serde(rename = "0x787ef513")]
    HkpTransformShape(Vec<HkpTransformShape>),

    #[serde(rename = "0x95ad1a25")]
    HkpTriangleShape(Vec<HkpTriangleShape>),

    #[serde(rename = "0xeb60f431")]
    #[serde(bound(deserialize = "Vec<HkpTriggerVolumeEventInfo<'a>>: Deserialize<'de>"))]
    HkpTriggerVolumeEventInfo(Vec<HkpTriggerVolumeEventInfo<'a>>),

    #[serde(rename = "0xa29a8d1a")]
    #[serde(bound(deserialize = "Vec<HkpTriggerVolume<'a>>: Deserialize<'de>"))]
    HkpTriggerVolume(Vec<HkpTriggerVolume<'a>>),

    #[serde(rename = "0x58e1e585")]
    HkpTriSampledHeightFieldBvTreeShape(Vec<HkpTriSampledHeightFieldBvTreeShape>),

    #[serde(rename = "0xc291ddde")]
    #[serde(bound(deserialize = "Vec<HkpTriSampledHeightFieldCollection<'a>>: Deserialize<'de>"))]
    HkpTriSampledHeightFieldCollection(Vec<HkpTriSampledHeightFieldCollection<'a>>),

    #[serde(rename = "0x7c9b1052")]
    HkpTwistLimitConstraintAtom(Vec<HkpTwistLimitConstraintAtom>),

    #[serde(rename = "0xf4b0f799")]
    HkpTypedBroadPhaseHandle(Vec<HkpTypedBroadPhaseHandle>),

    #[serde(rename = "0x6bb7c5e8")]
    HkpTyremarkPoint(Vec<HkpTyremarkPoint>),

    #[serde(rename = "0x3d0433d6")]
    #[serde(bound(deserialize = "Vec<HkpTyremarksInfo<'a>>: Deserialize<'de>"))]
    HkpTyremarksInfo(Vec<HkpTyremarksInfo<'a>>),

    #[serde(rename = "0x1eaef041")]
    HkpTyremarksWheel(Vec<HkpTyremarksWheel>),

    #[serde(rename = "0x895532c0")]
    #[serde(bound(deserialize = "Vec<HkpUnaryAction<'a>>: Deserialize<'de>"))]
    HkpUnaryAction(Vec<HkpUnaryAction<'a>>),

    #[serde(rename = "0x53340a9")]
    HkpVehicleCastBatchingManager(Vec<HkpVehicleCastBatchingManager>),

    #[serde(rename = "0x82fe40e0")]
    HkpVehicleDataWheelComponentParams(Vec<HkpVehicleDataWheelComponentParams>),

    #[serde(rename = "0x173feb43")]
    HkpVehicleData(Vec<HkpVehicleData>),

    #[serde(rename = "0x42fc5bbd")]
    HkpVehicleDefaultAerodynamics(Vec<HkpVehicleDefaultAerodynamics>),

    #[serde(rename = "0x123a5d50")]
    HkpVehicleDefaultAnalogDriverInput(Vec<HkpVehicleDefaultAnalogDriverInput>),

    #[serde(rename = "0x1ffad971")]
    HkpVehicleDefaultBrakeWheelBrakingProperties(Vec<HkpVehicleDefaultBrakeWheelBrakingProperties>),

    #[serde(rename = "0x4b4f8816")]
    HkpVehicleDefaultBrake(Vec<HkpVehicleDefaultBrake>),

    #[serde(rename = "0x56f8ca24")]
    HkpVehicleDefaultEngine(Vec<HkpVehicleDefaultEngine>),

    #[serde(rename = "0x8f0411c8")]
    HkpVehicleDefaultSteering(Vec<HkpVehicleDefaultSteering>),

    #[serde(rename = "0x7be5bed1")]
    HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(Vec<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>),

    #[serde(rename = "0x21735a24")]
    HkpVehicleDefaultSuspension(Vec<HkpVehicleDefaultSuspension>),

    #[serde(rename = "0x235d5d6b")]
    HkpVehicleDefaultTransmission(Vec<HkpVehicleDefaultTransmission>),

    #[serde(rename = "0x741b8d9e")]
    HkpVehicleDefaultVelocityDamper(Vec<HkpVehicleDefaultVelocityDamper>),

    #[serde(rename = "0x2b4a5803")]
    HkpVehicleDriverInputAnalogStatus(Vec<HkpVehicleDriverInputAnalogStatus>),

    #[serde(rename = "0x59ce153f")]
    HkpVehicleFrictionDescriptionAxisDescription(Vec<HkpVehicleFrictionDescriptionAxisDescription>),

    #[serde(rename = "0x1034549a")]
    HkpVehicleFrictionDescription(Vec<HkpVehicleFrictionDescription>),

    #[serde(rename = "0xe70e2bb4")]
    HkpVehicleFrictionStatusAxisStatus(Vec<HkpVehicleFrictionStatusAxisStatus>),

    #[serde(rename = "0x1c076a84")]
    HkpVehicleFrictionStatus(Vec<HkpVehicleFrictionStatus>),

    #[serde(rename = "0x99f693f0")]
    #[serde(bound(deserialize = "Vec<HkpVehicleInstanceWheelInfo<'a>>: Deserialize<'de>"))]
    HkpVehicleInstanceWheelInfo(Vec<HkpVehicleInstanceWheelInfo<'a>>),

    #[serde(rename = "0x877bb579")]
    #[serde(bound(deserialize = "Vec<HkpVehicleInstance<'a>>: Deserialize<'de>"))]
    HkpVehicleInstance(Vec<HkpVehicleInstance<'a>>),

    #[serde(rename = "0xed529f13")]
    HkpVehicleRayCastBatchingManager(Vec<HkpVehicleRayCastBatchingManager>),

    #[serde(rename = "0x2a9acf98")]
    #[serde(bound(deserialize = "Vec<HkpVehicleLinearCastWheelCollideWheelState<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastWheelCollideWheelState(Vec<HkpVehicleLinearCastWheelCollideWheelState<'a>>),

    #[serde(rename = "0xc59399d0")]
    HkpVehicleLinearCastWheelCollide(Vec<HkpVehicleLinearCastWheelCollide>),

    #[serde(rename = "0xe2f7d6a7")]
    #[serde(bound(deserialize = "Vec<HkpVehicleManager<'a>>: Deserialize<'de>"))]
    HkpVehicleManager(Vec<HkpVehicleManager<'a>>),

    #[serde(rename = "0x41efd9e3")]
    #[serde(bound(deserialize = "Vec<HkpVehicleRayCastWheelCollide<'a>>: Deserialize<'de>"))]
    HkpVehicleRayCastWheelCollide(Vec<HkpVehicleRayCastWheelCollide<'a>>),

    #[serde(rename = "0x358bfe9c")]
    HkpVehicleSuspensionSuspensionWheelParameters(Vec<HkpVehicleSuspensionSuspensionWheelParameters>),

    #[serde(rename = "0xaf5056fa")]
    HkpVehicleSuspension(Vec<HkpVehicleSuspension>),

    #[serde(rename = "0x4a50fcb")]
    HkpVehicleWheelCollide(Vec<HkpVehicleWheelCollide>),

    #[serde(rename = "0xfca2fcc3")]
    HkpVelocityConstraintMotor(Vec<HkpVelocityConstraintMotor>),

    #[serde(rename = "0xb2b41feb")]
    HkpWeldingUtility(Vec<HkpWeldingUtility>),

    #[serde(rename = "0x1188cbe1")]
    HkpWheelConstraintDataAtoms(Vec<HkpWheelConstraintDataAtoms>),

    #[serde(rename = "0xb4c46671")]
    HkpWheelConstraintData(Vec<HkpWheelConstraintData>),

    #[serde(rename = "0xa5255445")]
    #[serde(bound(deserialize = "Vec<HkpWorldCinfo<'a>>: Deserialize<'de>"))]
    HkpWorldCinfo(Vec<HkpWorldCinfo<'a>>),

    #[serde(rename = "0x49fb6f2e")]
    #[serde(bound(deserialize = "Vec<HkpWorldObject<'a>>: Deserialize<'de>"))]
    HkpWorldObject(Vec<HkpWorldObject<'a>>),

    #[serde(rename = "0xaadcec37")]
    #[serde(bound(deserialize = "Vec<HkpWorld<'a>>: Deserialize<'de>"))]
    HkpWorld(Vec<HkpWorld<'a>>),

    #[serde(rename = "0x471a21ee")]
    HkQTransform(Vec<HkQTransform>),

    #[serde(rename = "0x4846be29")]
    HkRangeInt32Attribute(Vec<HkRangeInt32Attribute>),

    #[serde(rename = "0x949db24f")]
    HkRangeRealAttribute(Vec<HkRangeRealAttribute>),

    #[serde(rename = "0x3b1c1113")]
    HkReferencedObject(Vec<HkReferencedObject>),

    #[serde(rename = "0xedb6b8f7")]
    #[serde(bound(deserialize = "Vec<HkReflectedFileAttribute<'a>>: Deserialize<'de>"))]
    HkReflectedFileAttribute(Vec<HkReflectedFileAttribute<'a>>),

    #[serde(rename = "0x660d7cac")]
    HkResourceBase(Vec<HkResourceBase>),

    #[serde(rename = "0x4e94146")]
    HkResourceHandle(Vec<HkResourceHandle>),

    #[serde(rename = "0xb103a2cd")]
    #[serde(bound(deserialize = "Vec<HkRootLevelContainerNamedVariant<'a>>: Deserialize<'de>"))]
    HkRootLevelContainerNamedVariant(Vec<HkRootLevelContainerNamedVariant<'a>>),

    #[serde(rename = "0x2772c11e")]
    HkRootLevelContainer(Vec<HkRootLevelContainer>),

    #[serde(rename = "0x837099c3")]
    HkSemanticsAttribute(Vec<HkSemanticsAttribute>),

    #[serde(rename = "0xe758f63c")]
    #[serde(bound(deserialize = "Vec<HkSimpleLocalFrame<'a>>: Deserialize<'de>"))]
    HkSimpleLocalFrame(Vec<HkSimpleLocalFrame<'a>>),

    #[serde(rename = "0x143dff99")]
    HkSphere(Vec<HkSphere>),

    #[serde(rename = "0xb4e5770")]
    HkSweptTransform(Vec<HkSweptTransform>),

    #[serde(rename = "0x6a4ca82c")]
    HkTraceStreamTitle(Vec<HkTraceStreamTitle>),

    #[serde(rename = "0x9ab3a6ac")]
    HkTrackerSerializableScanSnapshotAllocation(Vec<HkTrackerSerializableScanSnapshotAllocation>),

    #[serde(rename = "0xe7f23e6d")]
    HkTrackerSerializableScanSnapshotBlock(Vec<HkTrackerSerializableScanSnapshotBlock>),

    #[serde(rename = "0x875af1d9")]
    HkTrackerSerializableScanSnapshot(Vec<HkTrackerSerializableScanSnapshot>),

    #[serde(rename = "0xeb6e96e3")]
    #[serde(bound(deserialize = "Vec<HkUiAttribute<'a>>: Deserialize<'de>"))]
    HkUiAttribute(Vec<HkUiAttribute<'a>>),

    #[serde(rename = "0x54867cbf")]
    HkVertexFormatElement(Vec<HkVertexFormatElement>),

    #[serde(rename = "0xf11e3ff7")]
    HkVertexFormat(Vec<HkVertexFormat>),

    #[serde(rename = "0xce8b2fbd")]
    HkxAnimatedFloat(Vec<HkxAnimatedFloat>),

    #[serde(rename = "0x5838e337")]
    HkxAnimatedMatrix(Vec<HkxAnimatedMatrix>),

    #[serde(rename = "0xb4f01baa")]
    HkxAnimatedQuaternion(Vec<HkxAnimatedQuaternion>),

    #[serde(rename = "0x34b1a197")]
    HkxAnimatedVector(Vec<HkxAnimatedVector>),

    #[serde(rename = "0x345ca95d")]
    #[serde(bound(deserialize = "Vec<HkxAttributeGroup<'a>>: Deserialize<'de>"))]
    HkxAttributeGroup(Vec<HkxAttributeGroup<'a>>),

    #[serde(rename = "0x7468cc44")]
    HkxAttributeHolder(Vec<HkxAttributeHolder>),

    #[serde(rename = "0x7375cae3")]
    #[serde(bound(deserialize = "Vec<HkxAttribute<'a>>: Deserialize<'de>"))]
    HkxAttribute(Vec<HkxAttribute<'a>>),

    #[serde(rename = "0xe3597b02")]
    HkxCamera(Vec<HkxCamera>),

    #[serde(rename = "0x9ad32a5e")]
    HkxEdgeSelectionChannel(Vec<HkxEdgeSelectionChannel>),

    #[serde(rename = "0xdf4cf1e9")]
    #[serde(bound(deserialize = "Vec<HkxEnumItem<'a>>: Deserialize<'de>"))]
    HkxEnumItem(Vec<HkxEnumItem<'a>>),

    #[serde(rename = "0xc4e1211")]
    HkxEnum(Vec<HkxEnum>),

    #[serde(rename = "0xa6815115")]
    #[serde(bound(deserialize = "Vec<HkxEnvironmentVariable<'a>>: Deserialize<'de>"))]
    HkxEnvironmentVariable(Vec<HkxEnvironmentVariable<'a>>),

    #[serde(rename = "0x41e1aa5")]
    HkxEnvironment(Vec<HkxEnvironment>),

    #[serde(rename = "0xc12c8197")]
    HkxIndexBuffer(Vec<HkxIndexBuffer>),

    #[serde(rename = "0x81c86d42")]
    HkxLight(Vec<HkxLight>),

    #[serde(rename = "0x1d39f925")]
    #[serde(bound(deserialize = "Vec<HkxMaterialEffect<'a>>: Deserialize<'de>"))]
    HkxMaterialEffect(Vec<HkxMaterialEffect<'a>>),

    #[serde(rename = "0xd295234d")]
    HkxMaterialProperty(Vec<HkxMaterialProperty>),

    #[serde(rename = "0x154650f3")]
    #[serde(bound(deserialize = "Vec<HkxMaterialShaderSet<'a>>: Deserialize<'de>"))]
    HkxMaterialShaderSet(Vec<HkxMaterialShaderSet<'a>>),

    #[serde(rename = "0x28515eff")]
    #[serde(bound(deserialize = "Vec<HkxMaterialShader<'a>>: Deserialize<'de>"))]
    HkxMaterialShader(Vec<HkxMaterialShader<'a>>),

    #[serde(rename = "0xfa6facb2")]
    #[serde(bound(deserialize = "Vec<HkxMaterialTextureStage<'a>>: Deserialize<'de>"))]
    HkxMaterialTextureStage(Vec<HkxMaterialTextureStage<'a>>),

    #[serde(rename = "0x2954537a")]
    #[serde(bound(deserialize = "Vec<HkxMaterial<'a>>: Deserialize<'de>"))]
    HkxMaterial(Vec<HkxMaterial<'a>>),

    #[serde(rename = "0xe2286cf8")]
    #[serde(bound(deserialize = "Vec<HkxMeshSection<'a>>: Deserialize<'de>"))]
    HkxMeshSection(Vec<HkxMeshSection<'a>>),

    #[serde(rename = "0x270724a5")]
    #[serde(bound(deserialize = "Vec<HkxMeshUserChannelInfo<'a>>: Deserialize<'de>"))]
    HkxMeshUserChannelInfo(Vec<HkxMeshUserChannelInfo<'a>>),

    #[serde(rename = "0xf2edcc5f")]
    #[serde(bound(deserialize = "Vec<HkxMesh<'a>>: Deserialize<'de>"))]
    HkxMesh(Vec<HkxMesh<'a>>),

    #[serde(rename = "0x433dee92")]
    #[serde(bound(deserialize = "Vec<HkxNodeAnnotationData<'a>>: Deserialize<'de>"))]
    HkxNodeAnnotationData(Vec<HkxNodeAnnotationData<'a>>),

    #[serde(rename = "0xd753fc4d")]
    #[serde(bound(deserialize = "Vec<HkxNodeSelectionSet<'a>>: Deserialize<'de>"))]
    HkxNodeSelectionSet(Vec<HkxNodeSelectionSet<'a>>),

    #[serde(rename = "0x5a218502")]
    #[serde(bound(deserialize = "Vec<HkxNode<'a>>: Deserialize<'de>"))]
    HkxNode(Vec<HkxNode<'a>>),

    #[serde(rename = "0x5f673ddd")]
    #[serde(bound(deserialize = "Vec<HkxScene<'a>>: Deserialize<'de>"))]
    HkxScene(Vec<HkxScene<'a>>),

    #[serde(rename = "0x5a93f338")]
    #[serde(bound(deserialize = "Vec<HkxSkinBinding<'a>>: Deserialize<'de>"))]
    HkxSkinBinding(Vec<HkxSkinBinding<'a>>),

    #[serde(rename = "0x7a894596")]
    HkxSparselyAnimatedBool(Vec<HkxSparselyAnimatedBool>),

    #[serde(rename = "0x68a47b64")]
    #[serde(bound(deserialize = "Vec<HkxSparselyAnimatedEnum<'a>>: Deserialize<'de>"))]
    HkxSparselyAnimatedEnum(Vec<HkxSparselyAnimatedEnum<'a>>),

    #[serde(rename = "0xca961951")]
    HkxSparselyAnimatedInt(Vec<HkxSparselyAnimatedInt>),

    #[serde(rename = "0x185da6fd")]
    #[serde(bound(deserialize = "Vec<HkxSparselyAnimatedString<'a>>: Deserialize<'de>"))]
    HkxSparselyAnimatedString(Vec<HkxSparselyAnimatedString<'a>>),

    #[serde(rename = "0x1e289259")]
    #[serde(bound(deserialize = "Vec<HkxTextureFile<'a>>: Deserialize<'de>"))]
    HkxTextureFile(Vec<HkxTextureFile<'a>>),

    #[serde(rename = "0xd45841d6")]
    #[serde(bound(deserialize = "Vec<HkxTextureInplace<'a>>: Deserialize<'de>"))]
    HkxTextureInplace(Vec<HkxTextureInplace<'a>>),

    #[serde(rename = "0xa02cfca9")]
    HkxTriangleSelectionChannel(Vec<HkxTriangleSelectionChannel>),

    #[serde(rename = "0xd72b6fd0")]
    HkxVertexBufferVertexData(Vec<HkxVertexBufferVertexData>),

    #[serde(rename = "0x4ab10615")]
    HkxVertexBuffer(Vec<HkxVertexBuffer>),

    #[serde(rename = "0x483a429b")]
    HkxVertexDescriptionElementDecl(Vec<HkxVertexDescriptionElementDecl>),

    #[serde(rename = "0x2df6313d")]
    HkxVertexDescription(Vec<HkxVertexDescription>),

    #[serde(rename = "0xbeeb397c")]
    HkxVertexFloatDataChannel(Vec<HkxVertexFloatDataChannel>),

    #[serde(rename = "0x5a50e673")]
    HkxVertexIntDataChannel(Vec<HkxVertexIntDataChannel>),

    #[serde(rename = "0x866ec6d0")]
    HkxVertexSelectionChannel(Vec<HkxVertexSelectionChannel>),

    #[serde(rename = "0x2ea63179")]
    HkxVertexVectorDataChannel(Vec<HkxVertexVectorDataChannel>),

}

impl<'a> Serialize for Class<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Class", 633)?;
        state.serialize_field("@name", &self.name)?;
        state.serialize_field("@class", &self.class)?;
        state.serialize_field("@signature", &self.signature)?;

        // Serialize hkparam based on signature
        match self.signature.as_ref() {

            "0xc8df2d77" => {
                if let ClassParams::BgsGamebryoSequenceGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc1215be6" => {
                if let ClassParams::BsBoneSwitchGeneratorBoneData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf33d3eea" => {
                if let ClassParams::BsBoneSwitchGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa67f8c46" => {
                if let ClassParams::BsComputeAddBoneAnimModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5119eb06" => {
                if let ClassParams::BsCyclicBlendTransitionGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x31f6b8b6" => {
                if let ClassParams::BsDecomposeVectorModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x19a005c0" => {
                if let ClassParams::BsDirectAtModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb34d2bbd" => {
                if let ClassParams::BsDistTriggerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6030970c" => {
                if let ClassParams::BsEventEveryNEventsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1062d993" => {
                if let ClassParams::BsEventOnDeactivateModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x81d0777a" => {
                if let ClassParams::BsEventOnFalseToTrueModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbda33bfe" => {
                if let ClassParams::BsGetTimeStepModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x29adc802" => {
                if let ClassParams::BsInterpValueModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb0fde45a" => {
                if let ClassParams::BsIsActiveModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6b8a15fc" => {
                if let ClassParams::BsiStateManagerModifierBSiStateData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x99463586" => {
                if let ClassParams::BsiStateManagerModifierBsiStateManagerStateListener(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6cb24f2e" => {
                if let ClassParams::BsiStateManagerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf0826fc1" => {
                if let ClassParams::BSiStateTaggingGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8ea971e5" => {
                if let ClassParams::BsLimbIkModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x29efee59" => {
                if let ClassParams::BsLookAtModifierBoneData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd756fc25" => {
                if let ClassParams::BsLookAtModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1e20a97a" => {
                if let ClassParams::BsModifyOnceModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb8571122" => {
                if let ClassParams::BsOffsetAnimationGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x703d7b66" => {
                if let ClassParams::BsPassByTargetTriggerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8003d8ce" => {
                if let ClassParams::BsRagdollContactListenerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd297fda9" => {
                if let ClassParams::BsSpeedSamplerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd83bea64" => {
                if let ClassParams::BsSynchronizedClipGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x531f3292" => {
                if let ClassParams::BsTimerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd2d9a04" => {
                if let ClassParams::BsTweenerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1d716a17" => {
                if let ClassParams::HkAabbHalf(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x11e7c11" => {
                if let ClassParams::HkAabbUint32(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4a948b16" => {
                if let ClassParams::HkAabb(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xda8c7d7d" => {
                if let ClassParams::HkWorldMemoryAvailableWatchDog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x66eac971" => {
                if let ClassParams::HkaAnimationBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8dc20333" => {
                if let ClassParams::HkaAnimationContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4bc4c3e0" => {
                if let ClassParams::HkaAnimationPreviewColorContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa6fa7e88" => {
                if let ClassParams::HkaAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x623bf34f" => {
                if let ClassParams::HkaAnnotationTrackAnnotation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd4114fdd" => {
                if let ClassParams::HkaAnnotationTrack(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa8ccd5cf" => {
                if let ClassParams::HkaBoneAttachment(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x35912f8a" => {
                if let ClassParams::HkaBone(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6d85e445" => {
                if let ClassParams::HkaDefaultAnimatedReferenceFrame(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x724a7561" => {
                if let ClassParams::HkaWaveletCompressedAnimationQuantizationFormat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x90a68d40" => {
                if let ClassParams::HkaDeltaCompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1d81207c" => {
                if let ClassParams::HkaFootstepAnalysisInfoContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x824faf75" => {
                if let ClassParams::HkaFootstepAnalysisInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x930af031" => {
                if let ClassParams::HkaInterleavedUncompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa3d0ac71" => {
                if let ClassParams::HkaKeyFrameHierarchyUtilityControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7bd5c66f" => {
                if let ClassParams::HkMemoryTrackerAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x207cb01" => {
                if let ClassParams::HkAlignSceneToNodeOptions(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x48aceb75" => {
                if let ClassParams::HkMeshBoneIndexMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x81d9950b" => {
                if let ClassParams::HkaMeshBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf7d64649" => {
                if let ClassParams::HkaQuantizedAnimationTrackCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3920f053" => {
                if let ClassParams::HkaQuantizedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x154948e8" => {
                if let ClassParams::HkaRagdollInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd404a39a" => {
                if let ClassParams::HkArrayTypeAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x52e8043" => {
                if let ClassParams::HkaSkeletonLocalFrameOnBone(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa528f7cf" => {
                if let ClassParams::HkaSkeletonMapperDataChainMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3405deca" => {
                if let ClassParams::HkaSkeletonMapperDataSimpleMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x95687ea0" => {
                if let ClassParams::HkaSkeletonMapperData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x12df42a5" => {
                if let ClassParams::HkaSkeletonMapper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x366e8220" => {
                if let ClassParams::HkaSkeleton(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xde830789" => {
                if let ClassParams::HkaSplineCompressedAnimationAnimationCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x42e878d3" => {
                if let ClassParams::HkaSplineCompressedAnimationTrackCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x792ee0bb" => {
                if let ClassParams::HkaSplineCompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x27c6cafa" => {
                if let ClassParams::HkaWaveletCompressedAnimationCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x77cf0962" => {
                if let ClassParams::HkaWaveletCompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe0708a00" => {
                if let ClassParams::HkpShapeContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcc0aab32" => {
                if let ClassParams::HkbAttachmentModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x774632b" => {
                if let ClassParams::HkbAttachmentSetup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x48b8ad52" => {
                if let ClassParams::HkbAttributeModifierAssignment(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1245d97d" => {
                if let ClassParams::HkbAttributeModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xca0888ca" => {
                if let ClassParams::HkbAuxiliaryNodeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x66840004" => {
                if let ClassParams::HkbBehaviorEventsInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x95aca5d" => {
                if let ClassParams::HkbBehaviorGraphData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x645f898b" => {
                if let ClassParams::HkbBehaviorGraphInternalStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8699b6eb" => {
                if let ClassParams::HkbBehaviorGraphInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc713064e" => {
                if let ClassParams::HkbBehaviorGraphStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb1218f86" => {
                if let ClassParams::HkbBehaviorGraph(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x35a0439a" => {
                if let ClassParams::HkbBehaviorInfoIdToNamePair(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf7645395" => {
                if let ClassParams::HkbBehaviorInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfcb5423" => {
                if let ClassParams::HkbBehaviorReferenceGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2c1432d7" => {
                if let ClassParams::HkbBindable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x23041af0" => {
                if let ClassParams::HkbBlendCurveUtils(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xff7327c0" => {
                if let ClassParams::HkbBlenderGeneratorChildInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe2b384b0" => {
                if let ClassParams::HkbBlenderGeneratorChild(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x84717488" => {
                if let ClassParams::HkbBlenderGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x22df7147" => {
                if let ClassParams::HkbBlenderGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb18c70c2" => {
                if let ClassParams::HkbBlendingTransitionEffectInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfd8584fe" => {
                if let ClassParams::HkbBlendingTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xaa8619" => {
                if let ClassParams::HkbBoneIndexArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcd902b77" => {
                if let ClassParams::HkbBoneWeightArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x514763dc" => {
                if let ClassParams::HkbBoolVariableSequencedDataSample(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x37416fce" => {
                if let ClassParams::HkbBoolVariableSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x64136982" => {
                if let ClassParams::HkbCameraShakeEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3544e182" => {
                if let ClassParams::HkbCharacterAddedInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7a195d1d" => {
                if let ClassParams::HkbCharacterControlCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5b6c03d9" => {
                if let ClassParams::HkbCharacterControllerControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf8dfec0d" => {
                if let ClassParams::HkbCharacterControllerModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf675d6fb" => {
                if let ClassParams::HkbCharacterControllerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa0f415bf" => {
                if let ClassParams::HkbCharacterDataCharacterControllerInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x300d6808" => {
                if let ClassParams::HkbCharacterData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd9709ff2" => {
                if let ClassParams::HkbCharacterInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe5a2a413" => {
                if let ClassParams::HkbCharacterSetup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x180d900d" => {
                if let ClassParams::HkbCharacterSkinInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2eda84f8" => {
                if let ClassParams::HkbCharacterSteppedInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x655b42bc" => {
                if let ClassParams::HkbCharacterStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3088a5c5" => {
                if let ClassParams::HkbCharacter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa2624c97" => {
                if let ClassParams::HkbClientCharacterState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x750edf40" => {
                if let ClassParams::HkbClipGeneratorEcho(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x26ce5bf3" => {
                if let ClassParams::HkbClipGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x333b85b9" => {
                if let ClassParams::HkbClipGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x59c23a0f" => {
                if let ClassParams::HkbClipTriggerArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7eb45cea" => {
                if let ClassParams::HkbClipTrigger(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa92ed39f" => {
                if let ClassParams::HkbGetWorldFromModelModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfd1f0b79" => {
                if let ClassParams::HkbCombineTransformsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc6aaccc8" => {
                if let ClassParams::HkbCompiledExpressionSetToken(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3a7d76cc" => {
                if let ClassParams::HkbCompiledExpressionSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6ac054d7" => {
                if let ClassParams::HkbComputeDirectionModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdf358bd3" => {
                if let ClassParams::HkbComputeDirectionModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x71cd1eb0" => {
                if let ClassParams::HkbComputeRotationToTargetModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9b3f6936" => {
                if let ClassParams::HkbComputeRotationFromAxisAngleModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x47665f1c" => {
                if let ClassParams::HkbComputeRotationToTargetModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe0c4d4a7" => {
                if let ClassParams::HkbContext(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x508d3b36" => {
                if let ClassParams::HkbDampingModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9a040f03" => {
                if let ClassParams::HkbDampingModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x85fb0b80" => {
                if let ClassParams::HkbDelayedModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8e101a7a" => {
                if let ClassParams::HkbDelayedModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7b32d942" => {
                if let ClassParams::HkbDetectCloseToGroundModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x981687b2" => {
                if let ClassParams::HkbDetectCloseToGroundModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb8686f6b" => {
                if let ClassParams::HkbEvaluateExpressionModifierInternalExpressionData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb414d58e" => {
                if let ClassParams::HkbEvaluateExpressionModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf900f6be" => {
                if let ClassParams::HkbEvaluateExpressionModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x79757102" => {
                if let ClassParams::HkbEvaluateHandleModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x76bddb31" => {
                if let ClassParams::HkbEventBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd14bf000" => {
                if let ClassParams::HkbEventDrivenModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7ed3f44e" => {
                if let ClassParams::HkbEventDrivenModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5874eed4" => {
                if let ClassParams::HkbEventInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3d2dbd34" => {
                if let ClassParams::HkbEventPayloadList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdb38a15" => {
                if let ClassParams::HkbEventProperty(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc02da3" => {
                if let ClassParams::HkbEventRaisedInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x330a56ee" => {
                if let ClassParams::HkbEventRangeDataArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6cb92c76" => {
                if let ClassParams::HkbEventRangeData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9139b821" => {
                if let ClassParams::HkbEventSequencedDataSequencedEvent(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x76798eb8" => {
                if let ClassParams::HkbEventSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcc47b48d" => {
                if let ClassParams::HkbEventsFromRangeModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbc561b6e" => {
                if let ClassParams::HkbEventsFromRangeModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3e0fd810" => {
                if let ClassParams::HkbEvent(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1c3c1045" => {
                if let ClassParams::HkbExpressionCondition(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4b9ee1a2" => {
                if let ClassParams::HkbExpressionDataArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6740042a" => {
                if let ClassParams::HkbExpressionData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x804dcbab" => {
                if let ClassParams::HkbExtractRagdollPoseModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa111b704" => {
                if let ClassParams::HkbFootIkControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9e17091a" => {
                if let ClassParams::HkbFootIkControlsModifierLeg(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe5b6f544" => {
                if let ClassParams::HkbFootIkControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x224b18d1" => {
                if let ClassParams::HkbFootIkDriverInfoLeg(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc6a09dbf" => {
                if let ClassParams::HkbFootIkDriverInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa681b7f0" => {
                if let ClassParams::HkbFootIkGains(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe5ca3677" => {
                if let ClassParams::HkbFootIkModifierInternalLegData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9f3e3a04" => {
                if let ClassParams::HkbFootIkModifierLeg(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xed8966c0" => {
                if let ClassParams::HkbFootIkModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb597cf92" => {
                if let ClassParams::HkbGeneratorSyncInfoSyncPoint(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa3c341f8" => {
                if let ClassParams::HkbGeneratorSyncInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd6692b5d" => {
                if let ClassParams::HkbGeneratorTransitionEffectInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5f771b12" => {
                if let ClassParams::HkbGeneratorTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd68aefc" => {
                if let ClassParams::HkbGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x50c34a17" => {
                if let ClassParams::HkbGetHandleOnBoneModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd84cad4a" => {
                if let ClassParams::HkbGetUpModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x61cb7ac0" => {
                if let ClassParams::HkbGetUpModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x873fc6f7" => {
                if let ClassParams::HkbGetWorldFromModelModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd72b8d17" => {
                if let ClassParams::HkbHandIkControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9c72e9e3" => {
                if let ClassParams::HkbHandIkControlsModifierHand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9f0488bb" => {
                if let ClassParams::HkbHandIkControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x14dfe1dd" => {
                if let ClassParams::HkbHandIkModifierHand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc299090a" => {
                if let ClassParams::HkbHandIkDriverInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xef8bc2f7" => {
                if let ClassParams::HkbHandIkModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd8b6401c" => {
                if let ClassParams::HkbHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xebbc1bd3" => {
                if let ClassParams::HkbIntEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbe7ac63c" => {
                if let ClassParams::HkbIntVariableSequencedDataSample(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7bfc518a" => {
                if let ClassParams::HkbIntVariableSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xda41bd9b" => {
                if let ClassParams::HkBitField(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x72deb7a6" => {
                if let ClassParams::HkbKeyframeBonesModifierKeyframeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x95f66629" => {
                if let ClassParams::HkbKeyframeBonesModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6a5094e3" => {
                if let ClassParams::HkbSequenceStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa14caba6" => {
                if let ClassParams::HkbLookAtModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3d28e066" => {
                if let ClassParams::HkbLookAtModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x492c6137" => {
                if let ClassParams::HkbManualSelectorGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd932fab8" => {
                if let ClassParams::HkbManualSelectorGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x26a196c5" => {
                if let ClassParams::HkbMessageLog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc6c2da4f" => {
                if let ClassParams::HkbMirroredSkeletonInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa9a271ea" => {
                if let ClassParams::HkbMirrorModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1f81fae6" => {
                if let ClassParams::HkbModifierGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa4180ca1" => {
                if let ClassParams::HkbModifierList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3697e044" => {
                if let ClassParams::HkbModifierWrapper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x96ec5ced" => {
                if let ClassParams::HkbModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x28f67ba0" => {
                if let ClassParams::HkbMoveCharacterModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8f7492a0" => {
                if let ClassParams::HkbMoveCharacterModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x65bdd3a0" => {
                if let ClassParams::HkbNamedEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3c99bda4" => {
                if let ClassParams::HkbNamedIntEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9c99fd70" => {
                if let ClassParams::HkbNamedRealEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6caa9113" => {
                if let ClassParams::HkbNamedStringEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7db9971d" => {
                if let ClassParams::HkbNodeInternalStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6d26f61d" => {
                if let ClassParams::HkbNode(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9df46cd6" => {
                if let ClassParams::HkbParticleSystemEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x552d9dd4" => {
                if let ClassParams::HkbPoseMatchingGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x29e271b4" => {
                if let ClassParams::HkbPoseMatchingGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf5ba21b" => {
                if let ClassParams::HkbPoweredRagdollControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7cb54065" => {
                if let ClassParams::HkbPoweredRagdollControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x13a39ba7" => {
                if let ClassParams::HkbProjectData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x76ad60a" => {
                if let ClassParams::HkbProjectStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x39de637e" => {
                if let ClassParams::HkbProxyModifierProxyInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8a41554f" => {
                if let ClassParams::HkbProxyModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa0a7bf9c" => {
                if let ClassParams::HkbRaiseEventCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9416affd" => {
                if let ClassParams::HkbRealEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbb708bbd" => {
                if let ClassParams::HkbRealVariableSequencedDataSample(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe2862d02" => {
                if let ClassParams::HkbRealVariableSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x26a5675a" => {
                if let ClassParams::HkbReferencePoseGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x58b1d082" => {
                if let ClassParams::HkbRegisteredGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1e0bc068" => {
                if let ClassParams::HkbRigidBodyRagdollControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xaa87d1eb" => {
                if let ClassParams::HkbRigidBodyRagdollControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3eb2e082" => {
                if let ClassParams::HkbRoleAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdc40bf4a" => {
                if let ClassParams::HkbRotateCharacterModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x877ebc0b" => {
                if let ClassParams::HkbRotateCharacterModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfb56b692" => {
                if let ClassParams::HkbSenseHandleModifierRange(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2a064d99" => {
                if let ClassParams::HkbSenseHandleModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x419b9a05" => {
                if let ClassParams::HkbSequenceInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x43182ca3" => {
                if let ClassParams::HkbSequence(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe18b74b9" => {
                if let ClassParams::HkbSetBehaviorCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfab12b45" => {
                if let ClassParams::HkbSetLocalTimeOfClipGeneratorCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc5160b64" => {
                if let ClassParams::HkbSetNodePropertyCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf3ae5fca" => {
                if let ClassParams::HkbSetWordVariableCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xafcfa211" => {
                if let ClassParams::HkbSetWorldFromModelModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2a241367" => {
                if let ClassParams::HkbSimulationControlCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa40822b4" => {
                if let ClassParams::HkbSimulationStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbb90d54f" => {
                if let ClassParams::HkbStateMachineActiveTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x26d5499" => {
                if let ClassParams::HkbStateMachineDelayedTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb07b4388" => {
                if let ClassParams::HkbStateMachineEventPropertyArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbd1a7502" => {
                if let ClassParams::HkbStateMachineInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7358f5da" => {
                if let ClassParams::HkbStateMachineNestedStateMachineData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3ab09a2e" => {
                if let ClassParams::HkbStateMachineProspectiveTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xed7f9d0" => {
                if let ClassParams::HkbStateMachineStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x60a881e5" => {
                if let ClassParams::HkbStateMachineTimeInterval(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe397b11e" => {
                if let ClassParams::HkbStateMachineTransitionInfoArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9810c2d0" => {
                if let ClassParams::HkbStateMachineTransitionInfoReference(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcdec8025" => {
                if let ClassParams::HkbStateMachineTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x816c1dcb" => {
                if let ClassParams::HkbStateMachine(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5ab50487" => {
                if let ClassParams::HkbStringCondition(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xed04256a" => {
                if let ClassParams::HkbStringEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc0fcc436" => {
                if let ClassParams::HkbTestStateChooser(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x83ec2d42" => {
                if let ClassParams::HkbTimerModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x338b4879" => {
                if let ClassParams::HkbTimerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5ca91c99" => {
                if let ClassParams::HkbTransformVectorModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf93e0e24" => {
                if let ClassParams::HkbTransformVectorModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x945da157" => {
                if let ClassParams::HkbTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb6b76b32" => {
                if let ClassParams::HkbTwistModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4d592f72" => {
                if let ClassParams::HkbVariableBindingSetBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x338ad4ff" => {
                if let ClassParams::HkbVariableBindingSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9e746ba2" => {
                if let ClassParams::HkbVariableInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x27812d8d" => {
                if let ClassParams::HkbVariableValueSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb99bd6a" => {
                if let ClassParams::HkbVariableValue(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x25640b46" => {
                if let ClassParams::HkbWorldEnums(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa3af8783" => {
                if let ClassParams::HkbWorldFromModelModeData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xce6f8a6c" => {
                if let ClassParams::HkClassEnumItem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8a3609cf" => {
                if let ClassParams::HkClassEnum(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5c7ea4c2" => {
                if let ClassParams::HkClassMember(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x75585ef6" => {
                if let ClassParams::HkClass(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x106b96ce" => {
                if let ClassParams::HkColor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4e32287c" => {
                if let ClassParams::HkContactPointMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x91d7dd8e" => {
                if let ClassParams::HkContactPoint(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1388d601" => {
                if let ClassParams::HkCustomAttributesAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbff19005" => {
                if let ClassParams::HkCustomAttributes(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1e3857bb" => {
                if let ClassParams::HkDataObjectTypeAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe9f9578a" => {
                if let ClassParams::HkDescriptionAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x630edd9e" => {
                if let ClassParams::HkDocumentationAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9687513b" => {
                if let ClassParams::HkGeometryTriangle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x98dd8bdc" => {
                if let ClassParams::HkGeometry(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x23aadfb6" => {
                if let ClassParams::HkGizmoAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7684dc80" => {
                if let ClassParams::HkHalf8(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x87fe6b5c" => {
                if let ClassParams::HkIndexedTransformSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x255d8164" => {
                if let ClassParams::HkLinkAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb1a96c2f" => {
                if let ClassParams::HkLocalFrameGroup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x94a620a8" => {
                if let ClassParams::HkMemoryMeshBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x12156ee3" => {
                if let ClassParams::HkMemoryMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb743a578" => {
                if let ClassParams::HkMemoryMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2db6577c" => {
                if let ClassParams::HkMemoryMeshTexture(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa2e50753" => {
                if let ClassParams::HkMemoryMeshVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4762f92a" => {
                if let ClassParams::HkMemoryResourceContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3144d17c" => {
                if let ClassParams::HkMemoryResourceHandleExternalLink(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbffac086" => {
                if let ClassParams::HkMemoryResourceHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd0be5d7d" => {
                if let ClassParams::HkMeshBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6075f3ff" => {
                if let ClassParams::HkMeshSectionCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1893c365" => {
                if let ClassParams::HkMeshSection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9117d62e" => {
                if let ClassParams::HkMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc9887918" => {
                if let ClassParams::HkMeshTexture(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x534b08c8" => {
                if let ClassParams::HkMeshVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x338c092f" => {
                if let ClassParams::HkModelerNodeTypeAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x738fca05" => {
                if let ClassParams::HkMonitorStreamColorTableColorPair(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x79e53e85" => {
                if let ClassParams::HkMonitorStreamColorTable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7798b7db" => {
                if let ClassParams::HkMonitorStreamFrameInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2c76ce16" => {
                if let ClassParams::HkMonitorStreamStringMapStringMap(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc4d3a8b4" => {
                if let ClassParams::HkMonitorStreamStringMap(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7c338c66" => {
                if let ClassParams::HkMoppBvTreeShapeBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5797386e" => {
                if let ClassParams::HkMotionState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4731fb1b" => {
                if let ClassParams::HkMultipleVertexBufferElementInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa0e22afc" => {
                if let ClassParams::HkMultipleVertexBufferLockedElement(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdafbe0e6" => {
                if let ClassParams::HkMultipleVertexBufferVertexBufferInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xde3ab602" => {
                if let ClassParams::HkMultipleVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x11e4408b" => {
                if let ClassParams::HkMultiThreadCheck(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdcdb8b8b" => {
                if let ClassParams::Hkp2DAngConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2c5189dd" => {
                if let ClassParams::HkpAabbPhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9c16df5b" => {
                if let ClassParams::HkPackedVector3(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x79f9ffda" => {
                if let ClassParams::HkPackfileHeader(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf2a92154" => {
                if let ClassParams::HkPackfileSectionHeader(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbdf70a51" => {
                if let ClassParams::HkpAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x626e55a" => {
                if let ClassParams::HkpAgent1NSector(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x35bb3cd0" => {
                if let ClassParams::HkpAngConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf313aa80" => {
                if let ClassParams::HkpAngFrictionConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9be0d9d" => {
                if let ClassParams::HkpAngLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x81f087ff" => {
                if let ClassParams::HkpAngMotorConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x35f4c487" => {
                if let ClassParams::HkpAngularDashpotAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x674bcd2d" => {
                if let ClassParams::HkpArrayAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc73dcaf9" => {
                if let ClassParams::HkpBallAndSocketConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5a6954d9" => {
                if let ClassParams::HkpBallAndSocketConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x57b06d35" => {
                if let ClassParams::HkpBallGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc9cbedf2" => {
                if let ClassParams::HkpBallSocketChainDataConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x102aae9c" => {
                if let ClassParams::HkpBallSocketChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe70e4dfa" => {
                if let ClassParams::HkpBallSocketConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc00f3403" => {
                if let ClassParams::HkpBinaryAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbafa2bb7" => {
                if let ClassParams::HkpSphereMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3444d2d5" => {
                if let ClassParams::HkpBoxShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7d6310c8" => {
                if let ClassParams::HkpBreakableConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xde152a4d" => {
                if let ClassParams::HkpBridgeAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x87a4f31b" => {
                if let ClassParams::HkpBridgeConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x940569dc" => {
                if let ClassParams::HkpBroadPhaseHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x286eb64c" => {
                if let ClassParams::HkpBvShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa823d623" => {
                if let ClassParams::HkpBvTreeShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcf227f58" => {
                if let ClassParams::HkpCachingShapePhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xafcd79ad" => {
                if let ClassParams::HkpCallbackConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdd0b1fd3" => {
                if let ClassParams::HkpCapsuleShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x54a4b841" => {
                if let ClassParams::HkpCdBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1d7dbdd2" => {
                if let ClassParams::HkpCenterOfMassChangerModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x586d97b2" => {
                if let ClassParams::HkpCharacterProxyCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x892f441" => {
                if let ClassParams::HkpCharacterRigidBodyCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf2b1f399" => {
                if let ClassParams::HkpCogWheelConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf855ba44" => {
                if let ClassParams::HkpCogWheelConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7f0e53fc" => {
                if let ClassParams::HkpCogWheelConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb5f0e6b1" => {
                if let ClassParams::HkpCollidableBoundingVolumeData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9a0e42a5" => {
                if let ClassParams::HkpCollidable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2603bf04" => {
                if let ClassParams::HkpCollisionFilterList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x60960336" => {
                if let ClassParams::HkpCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcbfc95a4" => {
                if let ClassParams::HkpCompressedMeshShapeBigTriangle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5d0d67bd" => {
                if let ClassParams::HkpCompressedMeshShapeChunk(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x385bb842" => {
                if let ClassParams::HkpCompressedMeshShapeConvexPiece(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa62d5e6e" => {
                if let ClassParams::HkpCompressedMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x97b6e143" => {
                if let ClassParams::HkpCompressedSampledHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf19443c8" => {
                if let ClassParams::HkpConeLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x20a447fe" => {
                if let ClassParams::HkpConstrainedSystemFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x59d67ef6" => {
                if let ClassParams::HkpConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5facc7ff" => {
                if let ClassParams::HkpConstraintChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc3971189" => {
                if let ClassParams::HkpConstraintChainInstanceAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7a490753" => {
                if let ClassParams::HkpConstraintChainInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc3b577b1" => {
                if let ClassParams::HkpConstraintCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x80559a4e" => {
                if let ClassParams::HkpConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xee3c2aec" => {
                if let ClassParams::HkpEntitySmallArraySerializeOverrideType(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x34eba5f" => {
                if let ClassParams::HkpConstraintInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6a44c317" => {
                if let ClassParams::HkpConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x81d074a4" => {
                if let ClassParams::HkpConvexListFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x450b26e8" => {
                if let ClassParams::HkpConvexListShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x38fd3d97" => {
                if let ClassParams::HkpConvexPieceMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa5bd1d6e" => {
                if let ClassParams::HkpConvexPieceStreamData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf8f74f85" => {
                if let ClassParams::HkpConvexShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfbd72f9" => {
                if let ClassParams::HkpConvexTransformShapeBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xae3e5017" => {
                if let ClassParams::HkpConvexTransformShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5ba0a5f7" => {
                if let ClassParams::HkpConvexTranslateShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x63d38e9c" => {
                if let ClassParams::HkpConvexVerticesConnectivity(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3d80c5bf" => {
                if let ClassParams::HkpConvexVerticesShapeFourVectors(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x28726ad8" => {
                if let ClassParams::HkpConvexVerticesShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3e463c3a" => {
                if let ClassParams::HkpCylinderShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x50746c6e" => {
                if let ClassParams::HkpDashpotAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb69c1c02" => {
                if let ClassParams::HkpDefaultConvexListFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x77d6b19f" => {
                if let ClassParams::HkpDefaultWorldMemoryWatchDog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfac3351c" => {
                if let ClassParams::HkpDisableEntityCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc8ae86a7" => {
                if let ClassParams::HkpDisplayBindingDataPhysicsSystem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfe16e2a3" => {
                if let ClassParams::HkpDisplayBindingDataRigidBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdc46c906" => {
                if let ClassParams::HkpDisplayBindingData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf557023c" => {
                if let ClassParams::HkpEntityExtendedListeners(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x81147f05" => {
                if let ClassParams::HkpEntitySpuCollisionCallback(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa03c774b" => {
                if let ClassParams::HkpEntity(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf204b155" => {
                if let ClassParams::HkpExtendedMeshShapeShapesSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf4608207" => {
                if let ClassParams::HkpExtendedMeshShapeSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x44c32df6" => {
                if let ClassParams::HkpExtendedMeshShapeTrianglesSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x177114a2" => {
                if let ClassParams::HkpExtendedMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3d3da311" => {
                if let ClassParams::HkpFastMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x852ab70b" => {
                if let ClassParams::HkpFirstPersonGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x64abf85c" => {
                if let ClassParams::HkpThinBoxMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd6421f19" => {
                if let ClassParams::HkpGenericConstraintDataSchemeConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x11fd6f6c" => {
                if let ClassParams::HkpGenericConstraintDataScheme(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfa824640" => {
                if let ClassParams::HkpGenericConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5e2754cd" => {
                if let ClassParams::HkpGravityGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5cc01561" => {
                if let ClassParams::HkpGroupCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x65ee88e4" => {
                if let ClassParams::HkpGroupFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe7eca7eb" => {
                if let ClassParams::HkpSphereRepShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6958371c" => {
                if let ClassParams::HkpHingeConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9590f046" => {
                if let ClassParams::HkpHingeConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x555876ff" => {
                if let ClassParams::HkpHingeLimitsDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbd46760a" => {
                if let ClassParams::HkpHingeLimitsData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5c6aa14d" => {
                if let ClassParams::HkpViscousSurfaceModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3377b0b0" => {
                if let ClassParams::HkpLimitedForceConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x54c7715b" => {
                if let ClassParams::HkpLimitedHingeConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7c15bb6b" => {
                if let ClassParams::HkpLimitedHingeConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7b6b0210" => {
                if let ClassParams::HkpLinConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd7b3be03" => {
                if let ClassParams::HkpLinearParametricCurve(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3e94ef7c" => {
                if let ClassParams::HkpLinFrictionConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe1a81497" => {
                if let ClassParams::HkpLinkedCollidable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa44d1b07" => {
                if let ClassParams::HkpLinLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x10312464" => {
                if let ClassParams::HkpLinMotorConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x52b27d69" => {
                if let ClassParams::HkpLinSoftConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x80df0f90" => {
                if let ClassParams::HkpListShapeChildInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa1937cbd" => {
                if let ClassParams::HkpListShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6748b2cf" => {
                if let ClassParams::HkpMalleableConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb6b28240" => {
                if let ClassParams::HkpMassChangerModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x68a56834" => {
                if let ClassParams::HkpMassProperties(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x33be6570" => {
                if let ClassParams::HkpMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x886cde0c" => {
                if let ClassParams::HkpMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x27336e5d" => {
                if let ClassParams::HkpMeshShapeSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3bf12c0f" => {
                if let ClassParams::HkpMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb13fef1f" => {
                if let ClassParams::HkpModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x90b29d39" => {
                if let ClassParams::HkpMoppBvTreeShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd8fdbb08" => {
                if let ClassParams::HkpMoppCodeCodeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6ed8ac06" => {
                if let ClassParams::HkpMoppCodeReindexedTerminal(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x924c2661" => {
                if let ClassParams::HkpMoppCode(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x98aadb4f" => {
                if let ClassParams::HkpMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8ff131d9" => {
                if let ClassParams::HkpMotorAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6791ffce" => {
                if let ClassParams::HkpMountedBallGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6e087fd6" => {
                if let ClassParams::HkpMouseSpringAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x79ab517d" => {
                if let ClassParams::HkpMovingSurfaceModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xffdc0b65" => {
                if let ClassParams::HkpMultiRayShapeRay(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xea2e7ec9" => {
                if let ClassParams::HkpMultiRayShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x61a590fc" => {
                if let ClassParams::HkpMultiSphereShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc03af40d" => {
                if let ClassParams::HkpMultithreadedVehicleManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x66b42df1" => {
                if let ClassParams::HkpNamedMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb120a34f" => {
                if let ClassParams::HkpNullCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x903abb2c" => {
                if let ClassParams::HkPostFinishAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1f11b467" => {
                if let ClassParams::HkpOverwritePivotConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x36195969" => {
                if let ClassParams::HkpPairCollisionFilterMapPairFilterKeyOverrideType(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4abc140e" => {
                if let ClassParams::HkpPairCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9b7e6f86" => {
                if let ClassParams::HkpPhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc2a461e4" => {
                if let ClassParams::HkpPhysicsData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd0fd4bbe" => {
                if let ClassParams::HkpPhysicsSystemWithContacts(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xff724c17" => {
                if let ClassParams::HkpPhysicsSystem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc36bbd30" => {
                if let ClassParams::HkpPlaneShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8e7cb5da" => {
                if let ClassParams::HkpPointToPathConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x749bc260" => {
                if let ClassParams::HkpPointToPlaneConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x65c56e17" => {
                if let ClassParams::HkpPointToPlaneConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x748fb303" => {
                if let ClassParams::HkpPositionConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf88aee25" => {
                if let ClassParams::HkpPoweredChainDataConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x38aeafc3" => {
                if let ClassParams::HkpPoweredChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcf071a1b" => {
                if let ClassParams::HkpPoweredChainMapperLinkInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf651c74d" => {
                if let ClassParams::HkpPoweredChainMapperTarget(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7a77ef5" => {
                if let ClassParams::HkpPoweredChainMapper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7f516137" => {
                if let ClassParams::HkpPrismaticConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3996c387" => {
                if let ClassParams::HkpPrismaticConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb4f30148" => {
                if let ClassParams::HkpProjectileGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc75925aa" => {
                if let ClassParams::HkpPropertyValue(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9ce308e9" => {
                if let ClassParams::HkpProperty(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x94a08848" => {
                if let ClassParams::HkpPulleyConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb149e5a" => {
                if let ClassParams::HkpPulleyConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x972058ed" => {
                if let ClassParams::HkpPulleyConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x30cae006" => {
                if let ClassParams::HkpRackAndPinionConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa58a9659" => {
                if let ClassParams::HkpRackAndPinionConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd180ebe0" => {
                if let ClassParams::HkpRackAndPinionConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xeed76b00" => {
                if let ClassParams::HkpRagdollConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8fb5dd29" => {
                if let ClassParams::HkpRagdollConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x82b894c3" => {
                if let ClassParams::HkpRagdollLimitsDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcbdb44aa" => {
                if let ClassParams::HkpRagdollLimitsData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x71013826" => {
                if let ClassParams::HkpRagdollMotorConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc4fa16c9" => {
                if let ClassParams::HkpRejectChassisListener(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x91367f03" => {
                if let ClassParams::HkpRemoveTerminalsMoppModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2dc0ec6a" => {
                if let ClassParams::HkpReorientAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x75f8d805" => {
                if let ClassParams::HkpRigidBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa0c64586" => {
                if let ClassParams::HkpRotationalConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x74867d9e" => {
                if let ClassParams::HkpRotationalConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x11213421" => {
                if let ClassParams::HkpSampledHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x49ec7de3" => {
                if let ClassParams::HkpSerializedAgentNnEntry(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x54785c77" => {
                if let ClassParams::HkpSerializedDisplayMarkerList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd7c8c54f" => {
                if let ClassParams::HkpSerializedDisplayMarker(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x94ac5bec" => {
                if let ClassParams::HkpSerializedDisplayRbTransformsDisplayTransformPair(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc18650ac" => {
                if let ClassParams::HkpSerializedDisplayRbTransforms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x10155a" => {
                if let ClassParams::HkpSerializedSubTrack1NInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf12d48d9" => {
                if let ClassParams::HkpSerializedTrack1NInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf81db8e" => {
                if let ClassParams::HkpSetLocalRotationsConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6e2a5198" => {
                if let ClassParams::HkpSetLocalTransformsConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5cbfcf4a" => {
                if let ClassParams::HkpSetLocalTranslationsConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf05d137e" => {
                if let ClassParams::HkpSetupStabilizationAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe8c3991d" => {
                if let ClassParams::HkpShapeCollection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xea7f1d08" => {
                if let ClassParams::HkpShapeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xcb22fbcd" => {
                if let ClassParams::HkpShapePhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x666490a1" => {
                if let ClassParams::HkpShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x920df11a" => {
                if let ClassParams::HkpSimpleContactConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb59d1734" => {
                if let ClassParams::HkpSimpleContactConstraintDataInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd38738c1" => {
                if let ClassParams::HkpSimpleMeshShapeTriangle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x16b3c811" => {
                if let ClassParams::HkpSimpleMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x98bfa6ce" => {
                if let ClassParams::HkpSimpleShapePhantomCollisionDetail(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x32a2a8a8" => {
                if let ClassParams::HkpSimpleShapePhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x97aba922" => {
                if let ClassParams::HkpSimulation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x73aa1d38" => {
                if let ClassParams::HkpSingleShapeContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xecb34e27" => {
                if let ClassParams::HkpSoftContactModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x795d9fa" => {
                if let ClassParams::HkpSphereShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x88fc09fa" => {
                if let ClassParams::HkpSpringAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7ead26f6" => {
                if let ClassParams::HkpSpringDamperConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc624a180" => {
                if let ClassParams::HkpStiffSpringChainDataConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf170356b" => {
                if let ClassParams::HkpStiffSpringChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6c128096" => {
                if let ClassParams::HkpStiffSpringConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x207eb376" => {
                if let ClassParams::HkpStiffSpringConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb98f66f4" => {
                if let ClassParams::HkpStiffSpringConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2ca3e906" => {
                if let ClassParams::HkpStorageExtendedMeshShapeMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5aad4de6" => {
                if let ClassParams::HkpStorageExtendedMeshShapeMeshSubpartStorage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3f7d804c" => {
                if let ClassParams::HkpStorageExtendedMeshShapeShapeSubpartStorage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb469efbc" => {
                if let ClassParams::HkpStorageExtendedMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbf27438" => {
                if let ClassParams::HkpStorageMeshShapeSubpartStorage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbefd8b39" => {
                if let ClassParams::HkpStorageMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x15ff414b" => {
                if let ClassParams::HkpStorageSampledHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x787ef513" => {
                if let ClassParams::HkpTransformShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x95ad1a25" => {
                if let ClassParams::HkpTriangleShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xeb60f431" => {
                if let ClassParams::HkpTriggerVolumeEventInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa29a8d1a" => {
                if let ClassParams::HkpTriggerVolume(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x58e1e585" => {
                if let ClassParams::HkpTriSampledHeightFieldBvTreeShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc291ddde" => {
                if let ClassParams::HkpTriSampledHeightFieldCollection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7c9b1052" => {
                if let ClassParams::HkpTwistLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf4b0f799" => {
                if let ClassParams::HkpTypedBroadPhaseHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6bb7c5e8" => {
                if let ClassParams::HkpTyremarkPoint(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3d0433d6" => {
                if let ClassParams::HkpTyremarksInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1eaef041" => {
                if let ClassParams::HkpTyremarksWheel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x895532c0" => {
                if let ClassParams::HkpUnaryAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x53340a9" => {
                if let ClassParams::HkpVehicleCastBatchingManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x82fe40e0" => {
                if let ClassParams::HkpVehicleDataWheelComponentParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x173feb43" => {
                if let ClassParams::HkpVehicleData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x42fc5bbd" => {
                if let ClassParams::HkpVehicleDefaultAerodynamics(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x123a5d50" => {
                if let ClassParams::HkpVehicleDefaultAnalogDriverInput(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1ffad971" => {
                if let ClassParams::HkpVehicleDefaultBrakeWheelBrakingProperties(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4b4f8816" => {
                if let ClassParams::HkpVehicleDefaultBrake(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x56f8ca24" => {
                if let ClassParams::HkpVehicleDefaultEngine(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x8f0411c8" => {
                if let ClassParams::HkpVehicleDefaultSteering(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7be5bed1" => {
                if let ClassParams::HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x21735a24" => {
                if let ClassParams::HkpVehicleDefaultSuspension(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x235d5d6b" => {
                if let ClassParams::HkpVehicleDefaultTransmission(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x741b8d9e" => {
                if let ClassParams::HkpVehicleDefaultVelocityDamper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2b4a5803" => {
                if let ClassParams::HkpVehicleDriverInputAnalogStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x59ce153f" => {
                if let ClassParams::HkpVehicleFrictionDescriptionAxisDescription(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1034549a" => {
                if let ClassParams::HkpVehicleFrictionDescription(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe70e2bb4" => {
                if let ClassParams::HkpVehicleFrictionStatusAxisStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1c076a84" => {
                if let ClassParams::HkpVehicleFrictionStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x99f693f0" => {
                if let ClassParams::HkpVehicleInstanceWheelInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x877bb579" => {
                if let ClassParams::HkpVehicleInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xed529f13" => {
                if let ClassParams::HkpVehicleRayCastBatchingManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2a9acf98" => {
                if let ClassParams::HkpVehicleLinearCastWheelCollideWheelState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc59399d0" => {
                if let ClassParams::HkpVehicleLinearCastWheelCollide(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe2f7d6a7" => {
                if let ClassParams::HkpVehicleManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x41efd9e3" => {
                if let ClassParams::HkpVehicleRayCastWheelCollide(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x358bfe9c" => {
                if let ClassParams::HkpVehicleSuspensionSuspensionWheelParameters(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xaf5056fa" => {
                if let ClassParams::HkpVehicleSuspension(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4a50fcb" => {
                if let ClassParams::HkpVehicleWheelCollide(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfca2fcc3" => {
                if let ClassParams::HkpVelocityConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb2b41feb" => {
                if let ClassParams::HkpWeldingUtility(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1188cbe1" => {
                if let ClassParams::HkpWheelConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb4c46671" => {
                if let ClassParams::HkpWheelConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa5255445" => {
                if let ClassParams::HkpWorldCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x49fb6f2e" => {
                if let ClassParams::HkpWorldObject(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xaadcec37" => {
                if let ClassParams::HkpWorld(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x471a21ee" => {
                if let ClassParams::HkQTransform(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4846be29" => {
                if let ClassParams::HkRangeInt32Attribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x949db24f" => {
                if let ClassParams::HkRangeRealAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x3b1c1113" => {
                if let ClassParams::HkReferencedObject(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xedb6b8f7" => {
                if let ClassParams::HkReflectedFileAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x660d7cac" => {
                if let ClassParams::HkResourceBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4e94146" => {
                if let ClassParams::HkResourceHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb103a2cd" => {
                if let ClassParams::HkRootLevelContainerNamedVariant(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2772c11e" => {
                if let ClassParams::HkRootLevelContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x837099c3" => {
                if let ClassParams::HkSemanticsAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe758f63c" => {
                if let ClassParams::HkSimpleLocalFrame(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x143dff99" => {
                if let ClassParams::HkSphere(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb4e5770" => {
                if let ClassParams::HkSweptTransform(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x6a4ca82c" => {
                if let ClassParams::HkTraceStreamTitle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9ab3a6ac" => {
                if let ClassParams::HkTrackerSerializableScanSnapshotAllocation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe7f23e6d" => {
                if let ClassParams::HkTrackerSerializableScanSnapshotBlock(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x875af1d9" => {
                if let ClassParams::HkTrackerSerializableScanSnapshot(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xeb6e96e3" => {
                if let ClassParams::HkUiAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x54867cbf" => {
                if let ClassParams::HkVertexFormatElement(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf11e3ff7" => {
                if let ClassParams::HkVertexFormat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xce8b2fbd" => {
                if let ClassParams::HkxAnimatedFloat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5838e337" => {
                if let ClassParams::HkxAnimatedMatrix(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xb4f01baa" => {
                if let ClassParams::HkxAnimatedQuaternion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x34b1a197" => {
                if let ClassParams::HkxAnimatedVector(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x345ca95d" => {
                if let ClassParams::HkxAttributeGroup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7468cc44" => {
                if let ClassParams::HkxAttributeHolder(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7375cae3" => {
                if let ClassParams::HkxAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe3597b02" => {
                if let ClassParams::HkxCamera(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x9ad32a5e" => {
                if let ClassParams::HkxEdgeSelectionChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xdf4cf1e9" => {
                if let ClassParams::HkxEnumItem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc4e1211" => {
                if let ClassParams::HkxEnum(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa6815115" => {
                if let ClassParams::HkxEnvironmentVariable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x41e1aa5" => {
                if let ClassParams::HkxEnvironment(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xc12c8197" => {
                if let ClassParams::HkxIndexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x81c86d42" => {
                if let ClassParams::HkxLight(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1d39f925" => {
                if let ClassParams::HkxMaterialEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd295234d" => {
                if let ClassParams::HkxMaterialProperty(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x154650f3" => {
                if let ClassParams::HkxMaterialShaderSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x28515eff" => {
                if let ClassParams::HkxMaterialShader(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xfa6facb2" => {
                if let ClassParams::HkxMaterialTextureStage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2954537a" => {
                if let ClassParams::HkxMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xe2286cf8" => {
                if let ClassParams::HkxMeshSection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x270724a5" => {
                if let ClassParams::HkxMeshUserChannelInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xf2edcc5f" => {
                if let ClassParams::HkxMesh(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x433dee92" => {
                if let ClassParams::HkxNodeAnnotationData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd753fc4d" => {
                if let ClassParams::HkxNodeSelectionSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5a218502" => {
                if let ClassParams::HkxNode(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5f673ddd" => {
                if let ClassParams::HkxScene(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5a93f338" => {
                if let ClassParams::HkxSkinBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x7a894596" => {
                if let ClassParams::HkxSparselyAnimatedBool(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x68a47b64" => {
                if let ClassParams::HkxSparselyAnimatedEnum(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xca961951" => {
                if let ClassParams::HkxSparselyAnimatedInt(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x185da6fd" => {
                if let ClassParams::HkxSparselyAnimatedString(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x1e289259" => {
                if let ClassParams::HkxTextureFile(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd45841d6" => {
                if let ClassParams::HkxTextureInplace(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xa02cfca9" => {
                if let ClassParams::HkxTriangleSelectionChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xd72b6fd0" => {
                if let ClassParams::HkxVertexBufferVertexData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x4ab10615" => {
                if let ClassParams::HkxVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x483a429b" => {
                if let ClassParams::HkxVertexDescriptionElementDecl(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2df6313d" => {
                if let ClassParams::HkxVertexDescription(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0xbeeb397c" => {
                if let ClassParams::HkxVertexFloatDataChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x5a50e673" => {
                if let ClassParams::HkxVertexIntDataChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x866ec6d0" => {
                if let ClassParams::HkxVertexSelectionChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "0x2ea63179" => {
                if let ClassParams::HkxVertexVectorDataChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            _ => {}
        }
        state.end()
    }
}

// # Note
// In [`quick_xml::impl_deserialize_for_internally_tagged_enum`], only the first attribute can be deserialized by tag.
// What we need this time is the third attribute, "signature". Therefore, we need to deserialize it on our own initiative.
impl<'de> Deserialize<'de> for Class<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClassVisitor;

        impl<'de> Visitor<'de> for ClassVisitor {
            type Value = Class<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Class")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name: Option<Cow<'de, str>> = None;
                let mut class: Option<Cow<'de, str>> = None;
                let mut signature: Option<Cow<'de, str>> = None;
                let mut hkparam: Option<ClassParams<'de>> = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_ref() {
                        "@name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("@name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "@class" => {
                            if class.is_some() {
                                return Err(de::Error::duplicate_field("@class"));
                            }
                            class = Some(map.next_value()?);
                        }
                        "@signature" => {
                            if signature.is_some() {
                                return Err(de::Error::duplicate_field("@signature"));
                            }
                            signature = Some(map.next_value()?);
                        }
                        "hkparam" => {
                            if let Some(ref signature) = signature {
                                hkparam = Some(Ok(match signature.as_ref() {
                                    "0xc8df2d77" => {
                                                        ClassParams::BgsGamebryoSequenceGenerator(map.next_value()?)
                                                    },
                                    "0xc1215be6" => {
                                                        ClassParams::BsBoneSwitchGeneratorBoneData(map.next_value()?)
                                                    },
                                    "0xf33d3eea" => {
                                                        ClassParams::BsBoneSwitchGenerator(map.next_value()?)
                                                    },
                                    "0xa67f8c46" => {
                                                        ClassParams::BsComputeAddBoneAnimModifier(map.next_value()?)
                                                    },
                                    "0x5119eb06" => {
                                                        ClassParams::BsCyclicBlendTransitionGenerator(map.next_value()?)
                                                    },
                                    "0x31f6b8b6" => {
                                                        ClassParams::BsDecomposeVectorModifier(map.next_value()?)
                                                    },
                                    "0x19a005c0" => {
                                                        ClassParams::BsDirectAtModifier(map.next_value()?)
                                                    },
                                    "0xb34d2bbd" => {
                                                        ClassParams::BsDistTriggerModifier(map.next_value()?)
                                                    },
                                    "0x6030970c" => {
                                                        ClassParams::BsEventEveryNEventsModifier(map.next_value()?)
                                                    },
                                    "0x1062d993" => {
                                                        ClassParams::BsEventOnDeactivateModifier(map.next_value()?)
                                                    },
                                    "0x81d0777a" => {
                                                        ClassParams::BsEventOnFalseToTrueModifier(map.next_value()?)
                                                    },
                                    "0xbda33bfe" => {
                                                        ClassParams::BsGetTimeStepModifier(map.next_value()?)
                                                    },
                                    "0x29adc802" => {
                                                        ClassParams::BsInterpValueModifier(map.next_value()?)
                                                    },
                                    "0xb0fde45a" => {
                                                        ClassParams::BsIsActiveModifier(map.next_value()?)
                                                    },
                                    "0x6b8a15fc" => {
                                                        ClassParams::BsiStateManagerModifierBSiStateData(map.next_value()?)
                                                    },
                                    "0x99463586" => {
                                                        ClassParams::BsiStateManagerModifierBsiStateManagerStateListener(map.next_value()?)
                                                    },
                                    "0x6cb24f2e" => {
                                                        ClassParams::BsiStateManagerModifier(map.next_value()?)
                                                    },
                                    "0xf0826fc1" => {
                                                        ClassParams::BSiStateTaggingGenerator(map.next_value()?)
                                                    },
                                    "0x8ea971e5" => {
                                                        ClassParams::BsLimbIkModifier(map.next_value()?)
                                                    },
                                    "0x29efee59" => {
                                                        ClassParams::BsLookAtModifierBoneData(map.next_value()?)
                                                    },
                                    "0xd756fc25" => {
                                                        ClassParams::BsLookAtModifier(map.next_value()?)
                                                    },
                                    "0x1e20a97a" => {
                                                        ClassParams::BsModifyOnceModifier(map.next_value()?)
                                                    },
                                    "0xb8571122" => {
                                                        ClassParams::BsOffsetAnimationGenerator(map.next_value()?)
                                                    },
                                    "0x703d7b66" => {
                                                        ClassParams::BsPassByTargetTriggerModifier(map.next_value()?)
                                                    },
                                    "0x8003d8ce" => {
                                                        ClassParams::BsRagdollContactListenerModifier(map.next_value()?)
                                                    },
                                    "0xd297fda9" => {
                                                        ClassParams::BsSpeedSamplerModifier(map.next_value()?)
                                                    },
                                    "0xd83bea64" => {
                                                        ClassParams::BsSynchronizedClipGenerator(map.next_value()?)
                                                    },
                                    "0x531f3292" => {
                                                        ClassParams::BsTimerModifier(map.next_value()?)
                                                    },
                                    "0xd2d9a04" => {
                                                        ClassParams::BsTweenerModifier(map.next_value()?)
                                                    },
                                    "0x1d716a17" => {
                                                        ClassParams::HkAabbHalf(map.next_value()?)
                                                    },
                                    "0x11e7c11" => {
                                                        ClassParams::HkAabbUint32(map.next_value()?)
                                                    },
                                    "0x4a948b16" => {
                                                        ClassParams::HkAabb(map.next_value()?)
                                                    },
                                    "0xda8c7d7d" => {
                                                        ClassParams::HkWorldMemoryAvailableWatchDog(map.next_value()?)
                                                    },
                                    "0x66eac971" => {
                                                        ClassParams::HkaAnimationBinding(map.next_value()?)
                                                    },
                                    "0x8dc20333" => {
                                                        ClassParams::HkaAnimationContainer(map.next_value()?)
                                                    },
                                    "0x4bc4c3e0" => {
                                                        ClassParams::HkaAnimationPreviewColorContainer(map.next_value()?)
                                                    },
                                    "0xa6fa7e88" => {
                                                        ClassParams::HkaAnimation(map.next_value()?)
                                                    },
                                    "0x623bf34f" => {
                                                        ClassParams::HkaAnnotationTrackAnnotation(map.next_value()?)
                                                    },
                                    "0xd4114fdd" => {
                                                        ClassParams::HkaAnnotationTrack(map.next_value()?)
                                                    },
                                    "0xa8ccd5cf" => {
                                                        ClassParams::HkaBoneAttachment(map.next_value()?)
                                                    },
                                    "0x35912f8a" => {
                                                        ClassParams::HkaBone(map.next_value()?)
                                                    },
                                    "0x6d85e445" => {
                                                        ClassParams::HkaDefaultAnimatedReferenceFrame(map.next_value()?)
                                                    },
                                    "0x724a7561" => {
                                                        ClassParams::HkaWaveletCompressedAnimationQuantizationFormat(map.next_value()?)
                                                    },
                                    "0x90a68d40" => {
                                                        ClassParams::HkaDeltaCompressedAnimation(map.next_value()?)
                                                    },
                                    "0x1d81207c" => {
                                                        ClassParams::HkaFootstepAnalysisInfoContainer(map.next_value()?)
                                                    },
                                    "0x824faf75" => {
                                                        ClassParams::HkaFootstepAnalysisInfo(map.next_value()?)
                                                    },
                                    "0x930af031" => {
                                                        ClassParams::HkaInterleavedUncompressedAnimation(map.next_value()?)
                                                    },
                                    "0xa3d0ac71" => {
                                                        ClassParams::HkaKeyFrameHierarchyUtilityControlData(map.next_value()?)
                                                    },
                                    "0x7bd5c66f" => {
                                                        ClassParams::HkMemoryTrackerAttribute(map.next_value()?)
                                                    },
                                    "0x207cb01" => {
                                                        ClassParams::HkAlignSceneToNodeOptions(map.next_value()?)
                                                    },
                                    "0x48aceb75" => {
                                                        ClassParams::HkMeshBoneIndexMapping(map.next_value()?)
                                                    },
                                    "0x81d9950b" => {
                                                        ClassParams::HkaMeshBinding(map.next_value()?)
                                                    },
                                    "0xf7d64649" => {
                                                        ClassParams::HkaQuantizedAnimationTrackCompressionParams(map.next_value()?)
                                                    },
                                    "0x3920f053" => {
                                                        ClassParams::HkaQuantizedAnimation(map.next_value()?)
                                                    },
                                    "0x154948e8" => {
                                                        ClassParams::HkaRagdollInstance(map.next_value()?)
                                                    },
                                    "0xd404a39a" => {
                                                        ClassParams::HkArrayTypeAttribute(map.next_value()?)
                                                    },
                                    "0x52e8043" => {
                                                        ClassParams::HkaSkeletonLocalFrameOnBone(map.next_value()?)
                                                    },
                                    "0xa528f7cf" => {
                                                        ClassParams::HkaSkeletonMapperDataChainMapping(map.next_value()?)
                                                    },
                                    "0x3405deca" => {
                                                        ClassParams::HkaSkeletonMapperDataSimpleMapping(map.next_value()?)
                                                    },
                                    "0x95687ea0" => {
                                                        ClassParams::HkaSkeletonMapperData(map.next_value()?)
                                                    },
                                    "0x12df42a5" => {
                                                        ClassParams::HkaSkeletonMapper(map.next_value()?)
                                                    },
                                    "0x366e8220" => {
                                                        ClassParams::HkaSkeleton(map.next_value()?)
                                                    },
                                    "0xde830789" => {
                                                        ClassParams::HkaSplineCompressedAnimationAnimationCompressionParams(map.next_value()?)
                                                    },
                                    "0x42e878d3" => {
                                                        ClassParams::HkaSplineCompressedAnimationTrackCompressionParams(map.next_value()?)
                                                    },
                                    "0x792ee0bb" => {
                                                        ClassParams::HkaSplineCompressedAnimation(map.next_value()?)
                                                    },
                                    "0x27c6cafa" => {
                                                        ClassParams::HkaWaveletCompressedAnimationCompressionParams(map.next_value()?)
                                                    },
                                    "0x77cf0962" => {
                                                        ClassParams::HkaWaveletCompressedAnimation(map.next_value()?)
                                                    },
                                    "0xe0708a00" => {
                                                        ClassParams::HkpShapeContainer(map.next_value()?)
                                                    },
                                    "0xcc0aab32" => {
                                                        ClassParams::HkbAttachmentModifier(map.next_value()?)
                                                    },
                                    "0x774632b" => {
                                                        ClassParams::HkbAttachmentSetup(map.next_value()?)
                                                    },
                                    "0x48b8ad52" => {
                                                        ClassParams::HkbAttributeModifierAssignment(map.next_value()?)
                                                    },
                                    "0x1245d97d" => {
                                                        ClassParams::HkbAttributeModifier(map.next_value()?)
                                                    },
                                    "0xca0888ca" => {
                                                        ClassParams::HkbAuxiliaryNodeInfo(map.next_value()?)
                                                    },
                                    "0x66840004" => {
                                                        ClassParams::HkbBehaviorEventsInfo(map.next_value()?)
                                                    },
                                    "0x95aca5d" => {
                                                        ClassParams::HkbBehaviorGraphData(map.next_value()?)
                                                    },
                                    "0x645f898b" => {
                                                        ClassParams::HkbBehaviorGraphInternalStateInfo(map.next_value()?)
                                                    },
                                    "0x8699b6eb" => {
                                                        ClassParams::HkbBehaviorGraphInternalState(map.next_value()?)
                                                    },
                                    "0xc713064e" => {
                                                        ClassParams::HkbBehaviorGraphStringData(map.next_value()?)
                                                    },
                                    "0xb1218f86" => {
                                                        ClassParams::HkbBehaviorGraph(map.next_value()?)
                                                    },
                                    "0x35a0439a" => {
                                                        ClassParams::HkbBehaviorInfoIdToNamePair(map.next_value()?)
                                                    },
                                    "0xf7645395" => {
                                                        ClassParams::HkbBehaviorInfo(map.next_value()?)
                                                    },
                                    "0xfcb5423" => {
                                                        ClassParams::HkbBehaviorReferenceGenerator(map.next_value()?)
                                                    },
                                    "0x2c1432d7" => {
                                                        ClassParams::HkbBindable(map.next_value()?)
                                                    },
                                    "0x23041af0" => {
                                                        ClassParams::HkbBlendCurveUtils(map.next_value()?)
                                                    },
                                    "0xff7327c0" => {
                                                        ClassParams::HkbBlenderGeneratorChildInternalState(map.next_value()?)
                                                    },
                                    "0xe2b384b0" => {
                                                        ClassParams::HkbBlenderGeneratorChild(map.next_value()?)
                                                    },
                                    "0x84717488" => {
                                                        ClassParams::HkbBlenderGeneratorInternalState(map.next_value()?)
                                                    },
                                    "0x22df7147" => {
                                                        ClassParams::HkbBlenderGenerator(map.next_value()?)
                                                    },
                                    "0xb18c70c2" => {
                                                        ClassParams::HkbBlendingTransitionEffectInternalState(map.next_value()?)
                                                    },
                                    "0xfd8584fe" => {
                                                        ClassParams::HkbBlendingTransitionEffect(map.next_value()?)
                                                    },
                                    "0xaa8619" => {
                                                        ClassParams::HkbBoneIndexArray(map.next_value()?)
                                                    },
                                    "0xcd902b77" => {
                                                        ClassParams::HkbBoneWeightArray(map.next_value()?)
                                                    },
                                    "0x514763dc" => {
                                                        ClassParams::HkbBoolVariableSequencedDataSample(map.next_value()?)
                                                    },
                                    "0x37416fce" => {
                                                        ClassParams::HkbBoolVariableSequencedData(map.next_value()?)
                                                    },
                                    "0x64136982" => {
                                                        ClassParams::HkbCameraShakeEventPayload(map.next_value()?)
                                                    },
                                    "0x3544e182" => {
                                                        ClassParams::HkbCharacterAddedInfo(map.next_value()?)
                                                    },
                                    "0x7a195d1d" => {
                                                        ClassParams::HkbCharacterControlCommand(map.next_value()?)
                                                    },
                                    "0x5b6c03d9" => {
                                                        ClassParams::HkbCharacterControllerControlData(map.next_value()?)
                                                    },
                                    "0xf8dfec0d" => {
                                                        ClassParams::HkbCharacterControllerModifierInternalState(map.next_value()?)
                                                    },
                                    "0xf675d6fb" => {
                                                        ClassParams::HkbCharacterControllerModifier(map.next_value()?)
                                                    },
                                    "0xa0f415bf" => {
                                                        ClassParams::HkbCharacterDataCharacterControllerInfo(map.next_value()?)
                                                    },
                                    "0x300d6808" => {
                                                        ClassParams::HkbCharacterData(map.next_value()?)
                                                    },
                                    "0xd9709ff2" => {
                                                        ClassParams::HkbCharacterInfo(map.next_value()?)
                                                    },
                                    "0xe5a2a413" => {
                                                        ClassParams::HkbCharacterSetup(map.next_value()?)
                                                    },
                                    "0x180d900d" => {
                                                        ClassParams::HkbCharacterSkinInfo(map.next_value()?)
                                                    },
                                    "0x2eda84f8" => {
                                                        ClassParams::HkbCharacterSteppedInfo(map.next_value()?)
                                                    },
                                    "0x655b42bc" => {
                                                        ClassParams::HkbCharacterStringData(map.next_value()?)
                                                    },
                                    "0x3088a5c5" => {
                                                        ClassParams::HkbCharacter(map.next_value()?)
                                                    },
                                    "0xa2624c97" => {
                                                        ClassParams::HkbClientCharacterState(map.next_value()?)
                                                    },
                                    "0x750edf40" => {
                                                        ClassParams::HkbClipGeneratorEcho(map.next_value()?)
                                                    },
                                    "0x26ce5bf3" => {
                                                        ClassParams::HkbClipGeneratorInternalState(map.next_value()?)
                                                    },
                                    "0x333b85b9" => {
                                                        ClassParams::HkbClipGenerator(map.next_value()?)
                                                    },
                                    "0x59c23a0f" => {
                                                        ClassParams::HkbClipTriggerArray(map.next_value()?)
                                                    },
                                    "0x7eb45cea" => {
                                                        ClassParams::HkbClipTrigger(map.next_value()?)
                                                    },
                                    "0xa92ed39f" => {
                                                        ClassParams::HkbGetWorldFromModelModifierInternalState(map.next_value()?)
                                                    },
                                    "0xfd1f0b79" => {
                                                        ClassParams::HkbCombineTransformsModifier(map.next_value()?)
                                                    },
                                    "0xc6aaccc8" => {
                                                        ClassParams::HkbCompiledExpressionSetToken(map.next_value()?)
                                                    },
                                    "0x3a7d76cc" => {
                                                        ClassParams::HkbCompiledExpressionSet(map.next_value()?)
                                                    },
                                    "0x6ac054d7" => {
                                                        ClassParams::HkbComputeDirectionModifierInternalState(map.next_value()?)
                                                    },
                                    "0xdf358bd3" => {
                                                        ClassParams::HkbComputeDirectionModifier(map.next_value()?)
                                                    },
                                    "0x71cd1eb0" => {
                                                        ClassParams::HkbComputeRotationToTargetModifierInternalState(map.next_value()?)
                                                    },
                                    "0x9b3f6936" => {
                                                        ClassParams::HkbComputeRotationFromAxisAngleModifier(map.next_value()?)
                                                    },
                                    "0x47665f1c" => {
                                                        ClassParams::HkbComputeRotationToTargetModifier(map.next_value()?)
                                                    },
                                    "0xe0c4d4a7" => {
                                                        ClassParams::HkbContext(map.next_value()?)
                                                    },
                                    "0x508d3b36" => {
                                                        ClassParams::HkbDampingModifierInternalState(map.next_value()?)
                                                    },
                                    "0x9a040f03" => {
                                                        ClassParams::HkbDampingModifier(map.next_value()?)
                                                    },
                                    "0x85fb0b80" => {
                                                        ClassParams::HkbDelayedModifierInternalState(map.next_value()?)
                                                    },
                                    "0x8e101a7a" => {
                                                        ClassParams::HkbDelayedModifier(map.next_value()?)
                                                    },
                                    "0x7b32d942" => {
                                                        ClassParams::HkbDetectCloseToGroundModifierInternalState(map.next_value()?)
                                                    },
                                    "0x981687b2" => {
                                                        ClassParams::HkbDetectCloseToGroundModifier(map.next_value()?)
                                                    },
                                    "0xb8686f6b" => {
                                                        ClassParams::HkbEvaluateExpressionModifierInternalExpressionData(map.next_value()?)
                                                    },
                                    "0xb414d58e" => {
                                                        ClassParams::HkbEvaluateExpressionModifierInternalState(map.next_value()?)
                                                    },
                                    "0xf900f6be" => {
                                                        ClassParams::HkbEvaluateExpressionModifier(map.next_value()?)
                                                    },
                                    "0x79757102" => {
                                                        ClassParams::HkbEvaluateHandleModifier(map.next_value()?)
                                                    },
                                    "0x76bddb31" => {
                                                        ClassParams::HkbEventBase(map.next_value()?)
                                                    },
                                    "0xd14bf000" => {
                                                        ClassParams::HkbEventDrivenModifierInternalState(map.next_value()?)
                                                    },
                                    "0x7ed3f44e" => {
                                                        ClassParams::HkbEventDrivenModifier(map.next_value()?)
                                                    },
                                    "0x5874eed4" => {
                                                        ClassParams::HkbEventInfo(map.next_value()?)
                                                    },
                                    "0x3d2dbd34" => {
                                                        ClassParams::HkbEventPayloadList(map.next_value()?)
                                                    },
                                    "0xdb38a15" => {
                                                        ClassParams::HkbEventProperty(map.next_value()?)
                                                    },
                                    "0xc02da3" => {
                                                        ClassParams::HkbEventRaisedInfo(map.next_value()?)
                                                    },
                                    "0x330a56ee" => {
                                                        ClassParams::HkbEventRangeDataArray(map.next_value()?)
                                                    },
                                    "0x6cb92c76" => {
                                                        ClassParams::HkbEventRangeData(map.next_value()?)
                                                    },
                                    "0x9139b821" => {
                                                        ClassParams::HkbEventSequencedDataSequencedEvent(map.next_value()?)
                                                    },
                                    "0x76798eb8" => {
                                                        ClassParams::HkbEventSequencedData(map.next_value()?)
                                                    },
                                    "0xcc47b48d" => {
                                                        ClassParams::HkbEventsFromRangeModifierInternalState(map.next_value()?)
                                                    },
                                    "0xbc561b6e" => {
                                                        ClassParams::HkbEventsFromRangeModifier(map.next_value()?)
                                                    },
                                    "0x3e0fd810" => {
                                                        ClassParams::HkbEvent(map.next_value()?)
                                                    },
                                    "0x1c3c1045" => {
                                                        ClassParams::HkbExpressionCondition(map.next_value()?)
                                                    },
                                    "0x4b9ee1a2" => {
                                                        ClassParams::HkbExpressionDataArray(map.next_value()?)
                                                    },
                                    "0x6740042a" => {
                                                        ClassParams::HkbExpressionData(map.next_value()?)
                                                    },
                                    "0x804dcbab" => {
                                                        ClassParams::HkbExtractRagdollPoseModifier(map.next_value()?)
                                                    },
                                    "0xa111b704" => {
                                                        ClassParams::HkbFootIkControlData(map.next_value()?)
                                                    },
                                    "0x9e17091a" => {
                                                        ClassParams::HkbFootIkControlsModifierLeg(map.next_value()?)
                                                    },
                                    "0xe5b6f544" => {
                                                        ClassParams::HkbFootIkControlsModifier(map.next_value()?)
                                                    },
                                    "0x224b18d1" => {
                                                        ClassParams::HkbFootIkDriverInfoLeg(map.next_value()?)
                                                    },
                                    "0xc6a09dbf" => {
                                                        ClassParams::HkbFootIkDriverInfo(map.next_value()?)
                                                    },
                                    "0xa681b7f0" => {
                                                        ClassParams::HkbFootIkGains(map.next_value()?)
                                                    },
                                    "0xe5ca3677" => {
                                                        ClassParams::HkbFootIkModifierInternalLegData(map.next_value()?)
                                                    },
                                    "0x9f3e3a04" => {
                                                        ClassParams::HkbFootIkModifierLeg(map.next_value()?)
                                                    },
                                    "0xed8966c0" => {
                                                        ClassParams::HkbFootIkModifier(map.next_value()?)
                                                    },
                                    "0xb597cf92" => {
                                                        ClassParams::HkbGeneratorSyncInfoSyncPoint(map.next_value()?)
                                                    },
                                    "0xa3c341f8" => {
                                                        ClassParams::HkbGeneratorSyncInfo(map.next_value()?)
                                                    },
                                    "0xd6692b5d" => {
                                                        ClassParams::HkbGeneratorTransitionEffectInternalState(map.next_value()?)
                                                    },
                                    "0x5f771b12" => {
                                                        ClassParams::HkbGeneratorTransitionEffect(map.next_value()?)
                                                    },
                                    "0xd68aefc" => {
                                                        ClassParams::HkbGenerator(map.next_value()?)
                                                    },
                                    "0x50c34a17" => {
                                                        ClassParams::HkbGetHandleOnBoneModifier(map.next_value()?)
                                                    },
                                    "0xd84cad4a" => {
                                                        ClassParams::HkbGetUpModifierInternalState(map.next_value()?)
                                                    },
                                    "0x61cb7ac0" => {
                                                        ClassParams::HkbGetUpModifier(map.next_value()?)
                                                    },
                                    "0x873fc6f7" => {
                                                        ClassParams::HkbGetWorldFromModelModifier(map.next_value()?)
                                                    },
                                    "0xd72b8d17" => {
                                                        ClassParams::HkbHandIkControlData(map.next_value()?)
                                                    },
                                    "0x9c72e9e3" => {
                                                        ClassParams::HkbHandIkControlsModifierHand(map.next_value()?)
                                                    },
                                    "0x9f0488bb" => {
                                                        ClassParams::HkbHandIkControlsModifier(map.next_value()?)
                                                    },
                                    "0x14dfe1dd" => {
                                                        ClassParams::HkbHandIkModifierHand(map.next_value()?)
                                                    },
                                    "0xc299090a" => {
                                                        ClassParams::HkbHandIkDriverInfo(map.next_value()?)
                                                    },
                                    "0xef8bc2f7" => {
                                                        ClassParams::HkbHandIkModifier(map.next_value()?)
                                                    },
                                    "0xd8b6401c" => {
                                                        ClassParams::HkbHandle(map.next_value()?)
                                                    },
                                    "0xebbc1bd3" => {
                                                        ClassParams::HkbIntEventPayload(map.next_value()?)
                                                    },
                                    "0xbe7ac63c" => {
                                                        ClassParams::HkbIntVariableSequencedDataSample(map.next_value()?)
                                                    },
                                    "0x7bfc518a" => {
                                                        ClassParams::HkbIntVariableSequencedData(map.next_value()?)
                                                    },
                                    "0xda41bd9b" => {
                                                        ClassParams::HkBitField(map.next_value()?)
                                                    },
                                    "0x72deb7a6" => {
                                                        ClassParams::HkbKeyframeBonesModifierKeyframeInfo(map.next_value()?)
                                                    },
                                    "0x95f66629" => {
                                                        ClassParams::HkbKeyframeBonesModifier(map.next_value()?)
                                                    },
                                    "0x6a5094e3" => {
                                                        ClassParams::HkbSequenceStringData(map.next_value()?)
                                                    },
                                    "0xa14caba6" => {
                                                        ClassParams::HkbLookAtModifierInternalState(map.next_value()?)
                                                    },
                                    "0x3d28e066" => {
                                                        ClassParams::HkbLookAtModifier(map.next_value()?)
                                                    },
                                    "0x492c6137" => {
                                                        ClassParams::HkbManualSelectorGeneratorInternalState(map.next_value()?)
                                                    },
                                    "0xd932fab8" => {
                                                        ClassParams::HkbManualSelectorGenerator(map.next_value()?)
                                                    },
                                    "0x26a196c5" => {
                                                        ClassParams::HkbMessageLog(map.next_value()?)
                                                    },
                                    "0xc6c2da4f" => {
                                                        ClassParams::HkbMirroredSkeletonInfo(map.next_value()?)
                                                    },
                                    "0xa9a271ea" => {
                                                        ClassParams::HkbMirrorModifier(map.next_value()?)
                                                    },
                                    "0x1f81fae6" => {
                                                        ClassParams::HkbModifierGenerator(map.next_value()?)
                                                    },
                                    "0xa4180ca1" => {
                                                        ClassParams::HkbModifierList(map.next_value()?)
                                                    },
                                    "0x3697e044" => {
                                                        ClassParams::HkbModifierWrapper(map.next_value()?)
                                                    },
                                    "0x96ec5ced" => {
                                                        ClassParams::HkbModifier(map.next_value()?)
                                                    },
                                    "0x28f67ba0" => {
                                                        ClassParams::HkbMoveCharacterModifierInternalState(map.next_value()?)
                                                    },
                                    "0x8f7492a0" => {
                                                        ClassParams::HkbMoveCharacterModifier(map.next_value()?)
                                                    },
                                    "0x65bdd3a0" => {
                                                        ClassParams::HkbNamedEventPayload(map.next_value()?)
                                                    },
                                    "0x3c99bda4" => {
                                                        ClassParams::HkbNamedIntEventPayload(map.next_value()?)
                                                    },
                                    "0x9c99fd70" => {
                                                        ClassParams::HkbNamedRealEventPayload(map.next_value()?)
                                                    },
                                    "0x6caa9113" => {
                                                        ClassParams::HkbNamedStringEventPayload(map.next_value()?)
                                                    },
                                    "0x7db9971d" => {
                                                        ClassParams::HkbNodeInternalStateInfo(map.next_value()?)
                                                    },
                                    "0x6d26f61d" => {
                                                        ClassParams::HkbNode(map.next_value()?)
                                                    },
                                    "0x9df46cd6" => {
                                                        ClassParams::HkbParticleSystemEventPayload(map.next_value()?)
                                                    },
                                    "0x552d9dd4" => {
                                                        ClassParams::HkbPoseMatchingGeneratorInternalState(map.next_value()?)
                                                    },
                                    "0x29e271b4" => {
                                                        ClassParams::HkbPoseMatchingGenerator(map.next_value()?)
                                                    },
                                    "0xf5ba21b" => {
                                                        ClassParams::HkbPoweredRagdollControlData(map.next_value()?)
                                                    },
                                    "0x7cb54065" => {
                                                        ClassParams::HkbPoweredRagdollControlsModifier(map.next_value()?)
                                                    },
                                    "0x13a39ba7" => {
                                                        ClassParams::HkbProjectData(map.next_value()?)
                                                    },
                                    "0x76ad60a" => {
                                                        ClassParams::HkbProjectStringData(map.next_value()?)
                                                    },
                                    "0x39de637e" => {
                                                        ClassParams::HkbProxyModifierProxyInfo(map.next_value()?)
                                                    },
                                    "0x8a41554f" => {
                                                        ClassParams::HkbProxyModifier(map.next_value()?)
                                                    },
                                    "0xa0a7bf9c" => {
                                                        ClassParams::HkbRaiseEventCommand(map.next_value()?)
                                                    },
                                    "0x9416affd" => {
                                                        ClassParams::HkbRealEventPayload(map.next_value()?)
                                                    },
                                    "0xbb708bbd" => {
                                                        ClassParams::HkbRealVariableSequencedDataSample(map.next_value()?)
                                                    },
                                    "0xe2862d02" => {
                                                        ClassParams::HkbRealVariableSequencedData(map.next_value()?)
                                                    },
                                    "0x26a5675a" => {
                                                        ClassParams::HkbReferencePoseGenerator(map.next_value()?)
                                                    },
                                    "0x58b1d082" => {
                                                        ClassParams::HkbRegisteredGenerator(map.next_value()?)
                                                    },
                                    "0x1e0bc068" => {
                                                        ClassParams::HkbRigidBodyRagdollControlData(map.next_value()?)
                                                    },
                                    "0xaa87d1eb" => {
                                                        ClassParams::HkbRigidBodyRagdollControlsModifier(map.next_value()?)
                                                    },
                                    "0x3eb2e082" => {
                                                        ClassParams::HkbRoleAttribute(map.next_value()?)
                                                    },
                                    "0xdc40bf4a" => {
                                                        ClassParams::HkbRotateCharacterModifierInternalState(map.next_value()?)
                                                    },
                                    "0x877ebc0b" => {
                                                        ClassParams::HkbRotateCharacterModifier(map.next_value()?)
                                                    },
                                    "0xfb56b692" => {
                                                        ClassParams::HkbSenseHandleModifierRange(map.next_value()?)
                                                    },
                                    "0x2a064d99" => {
                                                        ClassParams::HkbSenseHandleModifier(map.next_value()?)
                                                    },
                                    "0x419b9a05" => {
                                                        ClassParams::HkbSequenceInternalState(map.next_value()?)
                                                    },
                                    "0x43182ca3" => {
                                                        ClassParams::HkbSequence(map.next_value()?)
                                                    },
                                    "0xe18b74b9" => {
                                                        ClassParams::HkbSetBehaviorCommand(map.next_value()?)
                                                    },
                                    "0xfab12b45" => {
                                                        ClassParams::HkbSetLocalTimeOfClipGeneratorCommand(map.next_value()?)
                                                    },
                                    "0xc5160b64" => {
                                                        ClassParams::HkbSetNodePropertyCommand(map.next_value()?)
                                                    },
                                    "0xf3ae5fca" => {
                                                        ClassParams::HkbSetWordVariableCommand(map.next_value()?)
                                                    },
                                    "0xafcfa211" => {
                                                        ClassParams::HkbSetWorldFromModelModifier(map.next_value()?)
                                                    },
                                    "0x2a241367" => {
                                                        ClassParams::HkbSimulationControlCommand(map.next_value()?)
                                                    },
                                    "0xa40822b4" => {
                                                        ClassParams::HkbSimulationStateInfo(map.next_value()?)
                                                    },
                                    "0xbb90d54f" => {
                                                        ClassParams::HkbStateMachineActiveTransitionInfo(map.next_value()?)
                                                    },
                                    "0x26d5499" => {
                                                        ClassParams::HkbStateMachineDelayedTransitionInfo(map.next_value()?)
                                                    },
                                    "0xb07b4388" => {
                                                        ClassParams::HkbStateMachineEventPropertyArray(map.next_value()?)
                                                    },
                                    "0xbd1a7502" => {
                                                        ClassParams::HkbStateMachineInternalState(map.next_value()?)
                                                    },
                                    "0x7358f5da" => {
                                                        ClassParams::HkbStateMachineNestedStateMachineData(map.next_value()?)
                                                    },
                                    "0x3ab09a2e" => {
                                                        ClassParams::HkbStateMachineProspectiveTransitionInfo(map.next_value()?)
                                                    },
                                    "0xed7f9d0" => {
                                                        ClassParams::HkbStateMachineStateInfo(map.next_value()?)
                                                    },
                                    "0x60a881e5" => {
                                                        ClassParams::HkbStateMachineTimeInterval(map.next_value()?)
                                                    },
                                    "0xe397b11e" => {
                                                        ClassParams::HkbStateMachineTransitionInfoArray(map.next_value()?)
                                                    },
                                    "0x9810c2d0" => {
                                                        ClassParams::HkbStateMachineTransitionInfoReference(map.next_value()?)
                                                    },
                                    "0xcdec8025" => {
                                                        ClassParams::HkbStateMachineTransitionInfo(map.next_value()?)
                                                    },
                                    "0x816c1dcb" => {
                                                        ClassParams::HkbStateMachine(map.next_value()?)
                                                    },
                                    "0x5ab50487" => {
                                                        ClassParams::HkbStringCondition(map.next_value()?)
                                                    },
                                    "0xed04256a" => {
                                                        ClassParams::HkbStringEventPayload(map.next_value()?)
                                                    },
                                    "0xc0fcc436" => {
                                                        ClassParams::HkbTestStateChooser(map.next_value()?)
                                                    },
                                    "0x83ec2d42" => {
                                                        ClassParams::HkbTimerModifierInternalState(map.next_value()?)
                                                    },
                                    "0x338b4879" => {
                                                        ClassParams::HkbTimerModifier(map.next_value()?)
                                                    },
                                    "0x5ca91c99" => {
                                                        ClassParams::HkbTransformVectorModifierInternalState(map.next_value()?)
                                                    },
                                    "0xf93e0e24" => {
                                                        ClassParams::HkbTransformVectorModifier(map.next_value()?)
                                                    },
                                    "0x945da157" => {
                                                        ClassParams::HkbTransitionEffect(map.next_value()?)
                                                    },
                                    "0xb6b76b32" => {
                                                        ClassParams::HkbTwistModifier(map.next_value()?)
                                                    },
                                    "0x4d592f72" => {
                                                        ClassParams::HkbVariableBindingSetBinding(map.next_value()?)
                                                    },
                                    "0x338ad4ff" => {
                                                        ClassParams::HkbVariableBindingSet(map.next_value()?)
                                                    },
                                    "0x9e746ba2" => {
                                                        ClassParams::HkbVariableInfo(map.next_value()?)
                                                    },
                                    "0x27812d8d" => {
                                                        ClassParams::HkbVariableValueSet(map.next_value()?)
                                                    },
                                    "0xb99bd6a" => {
                                                        ClassParams::HkbVariableValue(map.next_value()?)
                                                    },
                                    "0x25640b46" => {
                                                        ClassParams::HkbWorldEnums(map.next_value()?)
                                                    },
                                    "0xa3af8783" => {
                                                        ClassParams::HkbWorldFromModelModeData(map.next_value()?)
                                                    },
                                    "0xce6f8a6c" => {
                                                        ClassParams::HkClassEnumItem(map.next_value()?)
                                                    },
                                    "0x8a3609cf" => {
                                                        ClassParams::HkClassEnum(map.next_value()?)
                                                    },
                                    "0x5c7ea4c2" => {
                                                        ClassParams::HkClassMember(map.next_value()?)
                                                    },
                                    "0x75585ef6" => {
                                                        ClassParams::HkClass(map.next_value()?)
                                                    },
                                    "0x106b96ce" => {
                                                        ClassParams::HkColor(map.next_value()?)
                                                    },
                                    "0x4e32287c" => {
                                                        ClassParams::HkContactPointMaterial(map.next_value()?)
                                                    },
                                    "0x91d7dd8e" => {
                                                        ClassParams::HkContactPoint(map.next_value()?)
                                                    },
                                    "0x1388d601" => {
                                                        ClassParams::HkCustomAttributesAttribute(map.next_value()?)
                                                    },
                                    "0xbff19005" => {
                                                        ClassParams::HkCustomAttributes(map.next_value()?)
                                                    },
                                    "0x1e3857bb" => {
                                                        ClassParams::HkDataObjectTypeAttribute(map.next_value()?)
                                                    },
                                    "0xe9f9578a" => {
                                                        ClassParams::HkDescriptionAttribute(map.next_value()?)
                                                    },
                                    "0x630edd9e" => {
                                                        ClassParams::HkDocumentationAttribute(map.next_value()?)
                                                    },
                                    "0x9687513b" => {
                                                        ClassParams::HkGeometryTriangle(map.next_value()?)
                                                    },
                                    "0x98dd8bdc" => {
                                                        ClassParams::HkGeometry(map.next_value()?)
                                                    },
                                    "0x23aadfb6" => {
                                                        ClassParams::HkGizmoAttribute(map.next_value()?)
                                                    },
                                    "0x7684dc80" => {
                                                        ClassParams::HkHalf8(map.next_value()?)
                                                    },
                                    "0x87fe6b5c" => {
                                                        ClassParams::HkIndexedTransformSet(map.next_value()?)
                                                    },
                                    "0x255d8164" => {
                                                        ClassParams::HkLinkAttribute(map.next_value()?)
                                                    },
                                    "0xb1a96c2f" => {
                                                        ClassParams::HkLocalFrameGroup(map.next_value()?)
                                                    },
                                    "0x94a620a8" => {
                                                        ClassParams::HkMemoryMeshBody(map.next_value()?)
                                                    },
                                    "0x12156ee3" => {
                                                        ClassParams::HkMemoryMeshMaterial(map.next_value()?)
                                                    },
                                    "0xb743a578" => {
                                                        ClassParams::HkMemoryMeshShape(map.next_value()?)
                                                    },
                                    "0x2db6577c" => {
                                                        ClassParams::HkMemoryMeshTexture(map.next_value()?)
                                                    },
                                    "0xa2e50753" => {
                                                        ClassParams::HkMemoryMeshVertexBuffer(map.next_value()?)
                                                    },
                                    "0x4762f92a" => {
                                                        ClassParams::HkMemoryResourceContainer(map.next_value()?)
                                                    },
                                    "0x3144d17c" => {
                                                        ClassParams::HkMemoryResourceHandleExternalLink(map.next_value()?)
                                                    },
                                    "0xbffac086" => {
                                                        ClassParams::HkMemoryResourceHandle(map.next_value()?)
                                                    },
                                    "0xd0be5d7d" => {
                                                        ClassParams::HkMeshBody(map.next_value()?)
                                                    },
                                    "0x6075f3ff" => {
                                                        ClassParams::HkMeshSectionCinfo(map.next_value()?)
                                                    },
                                    "0x1893c365" => {
                                                        ClassParams::HkMeshSection(map.next_value()?)
                                                    },
                                    "0x9117d62e" => {
                                                        ClassParams::HkMeshShape(map.next_value()?)
                                                    },
                                    "0xc9887918" => {
                                                        ClassParams::HkMeshTexture(map.next_value()?)
                                                    },
                                    "0x534b08c8" => {
                                                        ClassParams::HkMeshVertexBuffer(map.next_value()?)
                                                    },
                                    "0x338c092f" => {
                                                        ClassParams::HkModelerNodeTypeAttribute(map.next_value()?)
                                                    },
                                    "0x738fca05" => {
                                                        ClassParams::HkMonitorStreamColorTableColorPair(map.next_value()?)
                                                    },
                                    "0x79e53e85" => {
                                                        ClassParams::HkMonitorStreamColorTable(map.next_value()?)
                                                    },
                                    "0x7798b7db" => {
                                                        ClassParams::HkMonitorStreamFrameInfo(map.next_value()?)
                                                    },
                                    "0x2c76ce16" => {
                                                        ClassParams::HkMonitorStreamStringMapStringMap(map.next_value()?)
                                                    },
                                    "0xc4d3a8b4" => {
                                                        ClassParams::HkMonitorStreamStringMap(map.next_value()?)
                                                    },
                                    "0x7c338c66" => {
                                                        ClassParams::HkMoppBvTreeShapeBase(map.next_value()?)
                                                    },
                                    "0x5797386e" => {
                                                        ClassParams::HkMotionState(map.next_value()?)
                                                    },
                                    "0x4731fb1b" => {
                                                        ClassParams::HkMultipleVertexBufferElementInfo(map.next_value()?)
                                                    },
                                    "0xa0e22afc" => {
                                                        ClassParams::HkMultipleVertexBufferLockedElement(map.next_value()?)
                                                    },
                                    "0xdafbe0e6" => {
                                                        ClassParams::HkMultipleVertexBufferVertexBufferInfo(map.next_value()?)
                                                    },
                                    "0xde3ab602" => {
                                                        ClassParams::HkMultipleVertexBuffer(map.next_value()?)
                                                    },
                                    "0x11e4408b" => {
                                                        ClassParams::HkMultiThreadCheck(map.next_value()?)
                                                    },
                                    "0xdcdb8b8b" => {
                                                        ClassParams::Hkp2DAngConstraintAtom(map.next_value()?)
                                                    },
                                    "0x2c5189dd" => {
                                                        ClassParams::HkpAabbPhantom(map.next_value()?)
                                                    },
                                    "0x9c16df5b" => {
                                                        ClassParams::HkPackedVector3(map.next_value()?)
                                                    },
                                    "0x79f9ffda" => {
                                                        ClassParams::HkPackfileHeader(map.next_value()?)
                                                    },
                                    "0xf2a92154" => {
                                                        ClassParams::HkPackfileSectionHeader(map.next_value()?)
                                                    },
                                    "0xbdf70a51" => {
                                                        ClassParams::HkpAction(map.next_value()?)
                                                    },
                                    "0x626e55a" => {
                                                        ClassParams::HkpAgent1NSector(map.next_value()?)
                                                    },
                                    "0x35bb3cd0" => {
                                                        ClassParams::HkpAngConstraintAtom(map.next_value()?)
                                                    },
                                    "0xf313aa80" => {
                                                        ClassParams::HkpAngFrictionConstraintAtom(map.next_value()?)
                                                    },
                                    "0x9be0d9d" => {
                                                        ClassParams::HkpAngLimitConstraintAtom(map.next_value()?)
                                                    },
                                    "0x81f087ff" => {
                                                        ClassParams::HkpAngMotorConstraintAtom(map.next_value()?)
                                                    },
                                    "0x35f4c487" => {
                                                        ClassParams::HkpAngularDashpotAction(map.next_value()?)
                                                    },
                                    "0x674bcd2d" => {
                                                        ClassParams::HkpArrayAction(map.next_value()?)
                                                    },
                                    "0xc73dcaf9" => {
                                                        ClassParams::HkpBallAndSocketConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x5a6954d9" => {
                                                        ClassParams::HkpBallAndSocketConstraintData(map.next_value()?)
                                                    },
                                    "0x57b06d35" => {
                                                        ClassParams::HkpBallGun(map.next_value()?)
                                                    },
                                    "0xc9cbedf2" => {
                                                        ClassParams::HkpBallSocketChainDataConstraintInfo(map.next_value()?)
                                                    },
                                    "0x102aae9c" => {
                                                        ClassParams::HkpBallSocketChainData(map.next_value()?)
                                                    },
                                    "0xe70e4dfa" => {
                                                        ClassParams::HkpBallSocketConstraintAtom(map.next_value()?)
                                                    },
                                    "0xc00f3403" => {
                                                        ClassParams::HkpBinaryAction(map.next_value()?)
                                                    },
                                    "0xbafa2bb7" => {
                                                        ClassParams::HkpSphereMotion(map.next_value()?)
                                                    },
                                    "0x3444d2d5" => {
                                                        ClassParams::HkpBoxShape(map.next_value()?)
                                                    },
                                    "0x7d6310c8" => {
                                                        ClassParams::HkpBreakableConstraintData(map.next_value()?)
                                                    },
                                    "0xde152a4d" => {
                                                        ClassParams::HkpBridgeAtoms(map.next_value()?)
                                                    },
                                    "0x87a4f31b" => {
                                                        ClassParams::HkpBridgeConstraintAtom(map.next_value()?)
                                                    },
                                    "0x940569dc" => {
                                                        ClassParams::HkpBroadPhaseHandle(map.next_value()?)
                                                    },
                                    "0x286eb64c" => {
                                                        ClassParams::HkpBvShape(map.next_value()?)
                                                    },
                                    "0xa823d623" => {
                                                        ClassParams::HkpBvTreeShape(map.next_value()?)
                                                    },
                                    "0xcf227f58" => {
                                                        ClassParams::HkpCachingShapePhantom(map.next_value()?)
                                                    },
                                    "0xafcd79ad" => {
                                                        ClassParams::HkpCallbackConstraintMotor(map.next_value()?)
                                                    },
                                    "0xdd0b1fd3" => {
                                                        ClassParams::HkpCapsuleShape(map.next_value()?)
                                                    },
                                    "0x54a4b841" => {
                                                        ClassParams::HkpCdBody(map.next_value()?)
                                                    },
                                    "0x1d7dbdd2" => {
                                                        ClassParams::HkpCenterOfMassChangerModifierConstraintAtom(map.next_value()?)
                                                    },
                                    "0x586d97b2" => {
                                                        ClassParams::HkpCharacterProxyCinfo(map.next_value()?)
                                                    },
                                    "0x892f441" => {
                                                        ClassParams::HkpCharacterRigidBodyCinfo(map.next_value()?)
                                                    },
                                    "0xf2b1f399" => {
                                                        ClassParams::HkpCogWheelConstraintAtom(map.next_value()?)
                                                    },
                                    "0xf855ba44" => {
                                                        ClassParams::HkpCogWheelConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x7f0e53fc" => {
                                                        ClassParams::HkpCogWheelConstraintData(map.next_value()?)
                                                    },
                                    "0xb5f0e6b1" => {
                                                        ClassParams::HkpCollidableBoundingVolumeData(map.next_value()?)
                                                    },
                                    "0x9a0e42a5" => {
                                                        ClassParams::HkpCollidable(map.next_value()?)
                                                    },
                                    "0x2603bf04" => {
                                                        ClassParams::HkpCollisionFilterList(map.next_value()?)
                                                    },
                                    "0x60960336" => {
                                                        ClassParams::HkpCollisionFilter(map.next_value()?)
                                                    },
                                    "0xcbfc95a4" => {
                                                        ClassParams::HkpCompressedMeshShapeBigTriangle(map.next_value()?)
                                                    },
                                    "0x5d0d67bd" => {
                                                        ClassParams::HkpCompressedMeshShapeChunk(map.next_value()?)
                                                    },
                                    "0x385bb842" => {
                                                        ClassParams::HkpCompressedMeshShapeConvexPiece(map.next_value()?)
                                                    },
                                    "0xa62d5e6e" => {
                                                        ClassParams::HkpCompressedMeshShape(map.next_value()?)
                                                    },
                                    "0x97b6e143" => {
                                                        ClassParams::HkpCompressedSampledHeightFieldShape(map.next_value()?)
                                                    },
                                    "0xf19443c8" => {
                                                        ClassParams::HkpConeLimitConstraintAtom(map.next_value()?)
                                                    },
                                    "0x20a447fe" => {
                                                        ClassParams::HkpConstrainedSystemFilter(map.next_value()?)
                                                    },
                                    "0x59d67ef6" => {
                                                        ClassParams::HkpConstraintAtom(map.next_value()?)
                                                    },
                                    "0x5facc7ff" => {
                                                        ClassParams::HkpConstraintChainData(map.next_value()?)
                                                    },
                                    "0xc3971189" => {
                                                        ClassParams::HkpConstraintChainInstanceAction(map.next_value()?)
                                                    },
                                    "0x7a490753" => {
                                                        ClassParams::HkpConstraintChainInstance(map.next_value()?)
                                                    },
                                    "0xc3b577b1" => {
                                                        ClassParams::HkpConstraintCollisionFilter(map.next_value()?)
                                                    },
                                    "0x80559a4e" => {
                                                        ClassParams::HkpConstraintData(map.next_value()?)
                                                    },
                                    "0xee3c2aec" => {
                                                        ClassParams::HkpEntitySmallArraySerializeOverrideType(map.next_value()?)
                                                    },
                                    "0x34eba5f" => {
                                                        ClassParams::HkpConstraintInstance(map.next_value()?)
                                                    },
                                    "0x6a44c317" => {
                                                        ClassParams::HkpConstraintMotor(map.next_value()?)
                                                    },
                                    "0x81d074a4" => {
                                                        ClassParams::HkpConvexListFilter(map.next_value()?)
                                                    },
                                    "0x450b26e8" => {
                                                        ClassParams::HkpConvexListShape(map.next_value()?)
                                                    },
                                    "0x38fd3d97" => {
                                                        ClassParams::HkpConvexPieceMeshShape(map.next_value()?)
                                                    },
                                    "0xa5bd1d6e" => {
                                                        ClassParams::HkpConvexPieceStreamData(map.next_value()?)
                                                    },
                                    "0xf8f74f85" => {
                                                        ClassParams::HkpConvexShape(map.next_value()?)
                                                    },
                                    "0xfbd72f9" => {
                                                        ClassParams::HkpConvexTransformShapeBase(map.next_value()?)
                                                    },
                                    "0xae3e5017" => {
                                                        ClassParams::HkpConvexTransformShape(map.next_value()?)
                                                    },
                                    "0x5ba0a5f7" => {
                                                        ClassParams::HkpConvexTranslateShape(map.next_value()?)
                                                    },
                                    "0x63d38e9c" => {
                                                        ClassParams::HkpConvexVerticesConnectivity(map.next_value()?)
                                                    },
                                    "0x3d80c5bf" => {
                                                        ClassParams::HkpConvexVerticesShapeFourVectors(map.next_value()?)
                                                    },
                                    "0x28726ad8" => {
                                                        ClassParams::HkpConvexVerticesShape(map.next_value()?)
                                                    },
                                    "0x3e463c3a" => {
                                                        ClassParams::HkpCylinderShape(map.next_value()?)
                                                    },
                                    "0x50746c6e" => {
                                                        ClassParams::HkpDashpotAction(map.next_value()?)
                                                    },
                                    "0xb69c1c02" => {
                                                        ClassParams::HkpDefaultConvexListFilter(map.next_value()?)
                                                    },
                                    "0x77d6b19f" => {
                                                        ClassParams::HkpDefaultWorldMemoryWatchDog(map.next_value()?)
                                                    },
                                    "0xfac3351c" => {
                                                        ClassParams::HkpDisableEntityCollisionFilter(map.next_value()?)
                                                    },
                                    "0xc8ae86a7" => {
                                                        ClassParams::HkpDisplayBindingDataPhysicsSystem(map.next_value()?)
                                                    },
                                    "0xfe16e2a3" => {
                                                        ClassParams::HkpDisplayBindingDataRigidBody(map.next_value()?)
                                                    },
                                    "0xdc46c906" => {
                                                        ClassParams::HkpDisplayBindingData(map.next_value()?)
                                                    },
                                    "0xf557023c" => {
                                                        ClassParams::HkpEntityExtendedListeners(map.next_value()?)
                                                    },
                                    "0x81147f05" => {
                                                        ClassParams::HkpEntitySpuCollisionCallback(map.next_value()?)
                                                    },
                                    "0xa03c774b" => {
                                                        ClassParams::HkpEntity(map.next_value()?)
                                                    },
                                    "0xf204b155" => {
                                                        ClassParams::HkpExtendedMeshShapeShapesSubpart(map.next_value()?)
                                                    },
                                    "0xf4608207" => {
                                                        ClassParams::HkpExtendedMeshShapeSubpart(map.next_value()?)
                                                    },
                                    "0x44c32df6" => {
                                                        ClassParams::HkpExtendedMeshShapeTrianglesSubpart(map.next_value()?)
                                                    },
                                    "0x177114a2" => {
                                                        ClassParams::HkpExtendedMeshShape(map.next_value()?)
                                                    },
                                    "0x3d3da311" => {
                                                        ClassParams::HkpFastMeshShape(map.next_value()?)
                                                    },
                                    "0x852ab70b" => {
                                                        ClassParams::HkpFirstPersonGun(map.next_value()?)
                                                    },
                                    "0x64abf85c" => {
                                                        ClassParams::HkpThinBoxMotion(map.next_value()?)
                                                    },
                                    "0xd6421f19" => {
                                                        ClassParams::HkpGenericConstraintDataSchemeConstraintInfo(map.next_value()?)
                                                    },
                                    "0x11fd6f6c" => {
                                                        ClassParams::HkpGenericConstraintDataScheme(map.next_value()?)
                                                    },
                                    "0xfa824640" => {
                                                        ClassParams::HkpGenericConstraintData(map.next_value()?)
                                                    },
                                    "0x5e2754cd" => {
                                                        ClassParams::HkpGravityGun(map.next_value()?)
                                                    },
                                    "0x5cc01561" => {
                                                        ClassParams::HkpGroupCollisionFilter(map.next_value()?)
                                                    },
                                    "0x65ee88e4" => {
                                                        ClassParams::HkpGroupFilter(map.next_value()?)
                                                    },
                                    "0xe7eca7eb" => {
                                                        ClassParams::HkpSphereRepShape(map.next_value()?)
                                                    },
                                    "0x6958371c" => {
                                                        ClassParams::HkpHingeConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x9590f046" => {
                                                        ClassParams::HkpHingeConstraintData(map.next_value()?)
                                                    },
                                    "0x555876ff" => {
                                                        ClassParams::HkpHingeLimitsDataAtoms(map.next_value()?)
                                                    },
                                    "0xbd46760a" => {
                                                        ClassParams::HkpHingeLimitsData(map.next_value()?)
                                                    },
                                    "0x5c6aa14d" => {
                                                        ClassParams::HkpViscousSurfaceModifierConstraintAtom(map.next_value()?)
                                                    },
                                    "0x3377b0b0" => {
                                                        ClassParams::HkpLimitedForceConstraintMotor(map.next_value()?)
                                                    },
                                    "0x54c7715b" => {
                                                        ClassParams::HkpLimitedHingeConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x7c15bb6b" => {
                                                        ClassParams::HkpLimitedHingeConstraintData(map.next_value()?)
                                                    },
                                    "0x7b6b0210" => {
                                                        ClassParams::HkpLinConstraintAtom(map.next_value()?)
                                                    },
                                    "0xd7b3be03" => {
                                                        ClassParams::HkpLinearParametricCurve(map.next_value()?)
                                                    },
                                    "0x3e94ef7c" => {
                                                        ClassParams::HkpLinFrictionConstraintAtom(map.next_value()?)
                                                    },
                                    "0xe1a81497" => {
                                                        ClassParams::HkpLinkedCollidable(map.next_value()?)
                                                    },
                                    "0xa44d1b07" => {
                                                        ClassParams::HkpLinLimitConstraintAtom(map.next_value()?)
                                                    },
                                    "0x10312464" => {
                                                        ClassParams::HkpLinMotorConstraintAtom(map.next_value()?)
                                                    },
                                    "0x52b27d69" => {
                                                        ClassParams::HkpLinSoftConstraintAtom(map.next_value()?)
                                                    },
                                    "0x80df0f90" => {
                                                        ClassParams::HkpListShapeChildInfo(map.next_value()?)
                                                    },
                                    "0xa1937cbd" => {
                                                        ClassParams::HkpListShape(map.next_value()?)
                                                    },
                                    "0x6748b2cf" => {
                                                        ClassParams::HkpMalleableConstraintData(map.next_value()?)
                                                    },
                                    "0xb6b28240" => {
                                                        ClassParams::HkpMassChangerModifierConstraintAtom(map.next_value()?)
                                                    },
                                    "0x68a56834" => {
                                                        ClassParams::HkpMassProperties(map.next_value()?)
                                                    },
                                    "0x33be6570" => {
                                                        ClassParams::HkpMaterial(map.next_value()?)
                                                    },
                                    "0x886cde0c" => {
                                                        ClassParams::HkpMeshMaterial(map.next_value()?)
                                                    },
                                    "0x27336e5d" => {
                                                        ClassParams::HkpMeshShapeSubpart(map.next_value()?)
                                                    },
                                    "0x3bf12c0f" => {
                                                        ClassParams::HkpMeshShape(map.next_value()?)
                                                    },
                                    "0xb13fef1f" => {
                                                        ClassParams::HkpModifierConstraintAtom(map.next_value()?)
                                                    },
                                    "0x90b29d39" => {
                                                        ClassParams::HkpMoppBvTreeShape(map.next_value()?)
                                                    },
                                    "0xd8fdbb08" => {
                                                        ClassParams::HkpMoppCodeCodeInfo(map.next_value()?)
                                                    },
                                    "0x6ed8ac06" => {
                                                        ClassParams::HkpMoppCodeReindexedTerminal(map.next_value()?)
                                                    },
                                    "0x924c2661" => {
                                                        ClassParams::HkpMoppCode(map.next_value()?)
                                                    },
                                    "0x98aadb4f" => {
                                                        ClassParams::HkpMotion(map.next_value()?)
                                                    },
                                    "0x8ff131d9" => {
                                                        ClassParams::HkpMotorAction(map.next_value()?)
                                                    },
                                    "0x6791ffce" => {
                                                        ClassParams::HkpMountedBallGun(map.next_value()?)
                                                    },
                                    "0x6e087fd6" => {
                                                        ClassParams::HkpMouseSpringAction(map.next_value()?)
                                                    },
                                    "0x79ab517d" => {
                                                        ClassParams::HkpMovingSurfaceModifierConstraintAtom(map.next_value()?)
                                                    },
                                    "0xffdc0b65" => {
                                                        ClassParams::HkpMultiRayShapeRay(map.next_value()?)
                                                    },
                                    "0xea2e7ec9" => {
                                                        ClassParams::HkpMultiRayShape(map.next_value()?)
                                                    },
                                    "0x61a590fc" => {
                                                        ClassParams::HkpMultiSphereShape(map.next_value()?)
                                                    },
                                    "0xc03af40d" => {
                                                        ClassParams::HkpMultithreadedVehicleManager(map.next_value()?)
                                                    },
                                    "0x66b42df1" => {
                                                        ClassParams::HkpNamedMeshMaterial(map.next_value()?)
                                                    },
                                    "0xb120a34f" => {
                                                        ClassParams::HkpNullCollisionFilter(map.next_value()?)
                                                    },
                                    "0x903abb2c" => {
                                                        ClassParams::HkPostFinishAttribute(map.next_value()?)
                                                    },
                                    "0x1f11b467" => {
                                                        ClassParams::HkpOverwritePivotConstraintAtom(map.next_value()?)
                                                    },
                                    "0x36195969" => {
                                                        ClassParams::HkpPairCollisionFilterMapPairFilterKeyOverrideType(map.next_value()?)
                                                    },
                                    "0x4abc140e" => {
                                                        ClassParams::HkpPairCollisionFilter(map.next_value()?)
                                                    },
                                    "0x9b7e6f86" => {
                                                        ClassParams::HkpPhantom(map.next_value()?)
                                                    },
                                    "0xc2a461e4" => {
                                                        ClassParams::HkpPhysicsData(map.next_value()?)
                                                    },
                                    "0xd0fd4bbe" => {
                                                        ClassParams::HkpPhysicsSystemWithContacts(map.next_value()?)
                                                    },
                                    "0xff724c17" => {
                                                        ClassParams::HkpPhysicsSystem(map.next_value()?)
                                                    },
                                    "0xc36bbd30" => {
                                                        ClassParams::HkpPlaneShape(map.next_value()?)
                                                    },
                                    "0x8e7cb5da" => {
                                                        ClassParams::HkpPointToPathConstraintData(map.next_value()?)
                                                    },
                                    "0x749bc260" => {
                                                        ClassParams::HkpPointToPlaneConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x65c56e17" => {
                                                        ClassParams::HkpPointToPlaneConstraintData(map.next_value()?)
                                                    },
                                    "0x748fb303" => {
                                                        ClassParams::HkpPositionConstraintMotor(map.next_value()?)
                                                    },
                                    "0xf88aee25" => {
                                                        ClassParams::HkpPoweredChainDataConstraintInfo(map.next_value()?)
                                                    },
                                    "0x38aeafc3" => {
                                                        ClassParams::HkpPoweredChainData(map.next_value()?)
                                                    },
                                    "0xcf071a1b" => {
                                                        ClassParams::HkpPoweredChainMapperLinkInfo(map.next_value()?)
                                                    },
                                    "0xf651c74d" => {
                                                        ClassParams::HkpPoweredChainMapperTarget(map.next_value()?)
                                                    },
                                    "0x7a77ef5" => {
                                                        ClassParams::HkpPoweredChainMapper(map.next_value()?)
                                                    },
                                    "0x7f516137" => {
                                                        ClassParams::HkpPrismaticConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x3996c387" => {
                                                        ClassParams::HkpPrismaticConstraintData(map.next_value()?)
                                                    },
                                    "0xb4f30148" => {
                                                        ClassParams::HkpProjectileGun(map.next_value()?)
                                                    },
                                    "0xc75925aa" => {
                                                        ClassParams::HkpPropertyValue(map.next_value()?)
                                                    },
                                    "0x9ce308e9" => {
                                                        ClassParams::HkpProperty(map.next_value()?)
                                                    },
                                    "0x94a08848" => {
                                                        ClassParams::HkpPulleyConstraintAtom(map.next_value()?)
                                                    },
                                    "0xb149e5a" => {
                                                        ClassParams::HkpPulleyConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x972058ed" => {
                                                        ClassParams::HkpPulleyConstraintData(map.next_value()?)
                                                    },
                                    "0x30cae006" => {
                                                        ClassParams::HkpRackAndPinionConstraintAtom(map.next_value()?)
                                                    },
                                    "0xa58a9659" => {
                                                        ClassParams::HkpRackAndPinionConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0xd180ebe0" => {
                                                        ClassParams::HkpRackAndPinionConstraintData(map.next_value()?)
                                                    },
                                    "0xeed76b00" => {
                                                        ClassParams::HkpRagdollConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x8fb5dd29" => {
                                                        ClassParams::HkpRagdollConstraintData(map.next_value()?)
                                                    },
                                    "0x82b894c3" => {
                                                        ClassParams::HkpRagdollLimitsDataAtoms(map.next_value()?)
                                                    },
                                    "0xcbdb44aa" => {
                                                        ClassParams::HkpRagdollLimitsData(map.next_value()?)
                                                    },
                                    "0x71013826" => {
                                                        ClassParams::HkpRagdollMotorConstraintAtom(map.next_value()?)
                                                    },
                                    "0xc4fa16c9" => {
                                                        ClassParams::HkpRejectChassisListener(map.next_value()?)
                                                    },
                                    "0x91367f03" => {
                                                        ClassParams::HkpRemoveTerminalsMoppModifier(map.next_value()?)
                                                    },
                                    "0x2dc0ec6a" => {
                                                        ClassParams::HkpReorientAction(map.next_value()?)
                                                    },
                                    "0x75f8d805" => {
                                                        ClassParams::HkpRigidBody(map.next_value()?)
                                                    },
                                    "0xa0c64586" => {
                                                        ClassParams::HkpRotationalConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0x74867d9e" => {
                                                        ClassParams::HkpRotationalConstraintData(map.next_value()?)
                                                    },
                                    "0x11213421" => {
                                                        ClassParams::HkpSampledHeightFieldShape(map.next_value()?)
                                                    },
                                    "0x49ec7de3" => {
                                                        ClassParams::HkpSerializedAgentNnEntry(map.next_value()?)
                                                    },
                                    "0x54785c77" => {
                                                        ClassParams::HkpSerializedDisplayMarkerList(map.next_value()?)
                                                    },
                                    "0xd7c8c54f" => {
                                                        ClassParams::HkpSerializedDisplayMarker(map.next_value()?)
                                                    },
                                    "0x94ac5bec" => {
                                                        ClassParams::HkpSerializedDisplayRbTransformsDisplayTransformPair(map.next_value()?)
                                                    },
                                    "0xc18650ac" => {
                                                        ClassParams::HkpSerializedDisplayRbTransforms(map.next_value()?)
                                                    },
                                    "0x10155a" => {
                                                        ClassParams::HkpSerializedSubTrack1NInfo(map.next_value()?)
                                                    },
                                    "0xf12d48d9" => {
                                                        ClassParams::HkpSerializedTrack1NInfo(map.next_value()?)
                                                    },
                                    "0xf81db8e" => {
                                                        ClassParams::HkpSetLocalRotationsConstraintAtom(map.next_value()?)
                                                    },
                                    "0x6e2a5198" => {
                                                        ClassParams::HkpSetLocalTransformsConstraintAtom(map.next_value()?)
                                                    },
                                    "0x5cbfcf4a" => {
                                                        ClassParams::HkpSetLocalTranslationsConstraintAtom(map.next_value()?)
                                                    },
                                    "0xf05d137e" => {
                                                        ClassParams::HkpSetupStabilizationAtom(map.next_value()?)
                                                    },
                                    "0xe8c3991d" => {
                                                        ClassParams::HkpShapeCollection(map.next_value()?)
                                                    },
                                    "0xea7f1d08" => {
                                                        ClassParams::HkpShapeInfo(map.next_value()?)
                                                    },
                                    "0xcb22fbcd" => {
                                                        ClassParams::HkpShapePhantom(map.next_value()?)
                                                    },
                                    "0x666490a1" => {
                                                        ClassParams::HkpShape(map.next_value()?)
                                                    },
                                    "0x920df11a" => {
                                                        ClassParams::HkpSimpleContactConstraintAtom(map.next_value()?)
                                                    },
                                    "0xb59d1734" => {
                                                        ClassParams::HkpSimpleContactConstraintDataInfo(map.next_value()?)
                                                    },
                                    "0xd38738c1" => {
                                                        ClassParams::HkpSimpleMeshShapeTriangle(map.next_value()?)
                                                    },
                                    "0x16b3c811" => {
                                                        ClassParams::HkpSimpleMeshShape(map.next_value()?)
                                                    },
                                    "0x98bfa6ce" => {
                                                        ClassParams::HkpSimpleShapePhantomCollisionDetail(map.next_value()?)
                                                    },
                                    "0x32a2a8a8" => {
                                                        ClassParams::HkpSimpleShapePhantom(map.next_value()?)
                                                    },
                                    "0x97aba922" => {
                                                        ClassParams::HkpSimulation(map.next_value()?)
                                                    },
                                    "0x73aa1d38" => {
                                                        ClassParams::HkpSingleShapeContainer(map.next_value()?)
                                                    },
                                    "0xecb34e27" => {
                                                        ClassParams::HkpSoftContactModifierConstraintAtom(map.next_value()?)
                                                    },
                                    "0x795d9fa" => {
                                                        ClassParams::HkpSphereShape(map.next_value()?)
                                                    },
                                    "0x88fc09fa" => {
                                                        ClassParams::HkpSpringAction(map.next_value()?)
                                                    },
                                    "0x7ead26f6" => {
                                                        ClassParams::HkpSpringDamperConstraintMotor(map.next_value()?)
                                                    },
                                    "0xc624a180" => {
                                                        ClassParams::HkpStiffSpringChainDataConstraintInfo(map.next_value()?)
                                                    },
                                    "0xf170356b" => {
                                                        ClassParams::HkpStiffSpringChainData(map.next_value()?)
                                                    },
                                    "0x6c128096" => {
                                                        ClassParams::HkpStiffSpringConstraintAtom(map.next_value()?)
                                                    },
                                    "0x207eb376" => {
                                                        ClassParams::HkpStiffSpringConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0xb98f66f4" => {
                                                        ClassParams::HkpStiffSpringConstraintData(map.next_value()?)
                                                    },
                                    "0x2ca3e906" => {
                                                        ClassParams::HkpStorageExtendedMeshShapeMaterial(map.next_value()?)
                                                    },
                                    "0x5aad4de6" => {
                                                        ClassParams::HkpStorageExtendedMeshShapeMeshSubpartStorage(map.next_value()?)
                                                    },
                                    "0x3f7d804c" => {
                                                        ClassParams::HkpStorageExtendedMeshShapeShapeSubpartStorage(map.next_value()?)
                                                    },
                                    "0xb469efbc" => {
                                                        ClassParams::HkpStorageExtendedMeshShape(map.next_value()?)
                                                    },
                                    "0xbf27438" => {
                                                        ClassParams::HkpStorageMeshShapeSubpartStorage(map.next_value()?)
                                                    },
                                    "0xbefd8b39" => {
                                                        ClassParams::HkpStorageMeshShape(map.next_value()?)
                                                    },
                                    "0x15ff414b" => {
                                                        ClassParams::HkpStorageSampledHeightFieldShape(map.next_value()?)
                                                    },
                                    "0x787ef513" => {
                                                        ClassParams::HkpTransformShape(map.next_value()?)
                                                    },
                                    "0x95ad1a25" => {
                                                        ClassParams::HkpTriangleShape(map.next_value()?)
                                                    },
                                    "0xeb60f431" => {
                                                        ClassParams::HkpTriggerVolumeEventInfo(map.next_value()?)
                                                    },
                                    "0xa29a8d1a" => {
                                                        ClassParams::HkpTriggerVolume(map.next_value()?)
                                                    },
                                    "0x58e1e585" => {
                                                        ClassParams::HkpTriSampledHeightFieldBvTreeShape(map.next_value()?)
                                                    },
                                    "0xc291ddde" => {
                                                        ClassParams::HkpTriSampledHeightFieldCollection(map.next_value()?)
                                                    },
                                    "0x7c9b1052" => {
                                                        ClassParams::HkpTwistLimitConstraintAtom(map.next_value()?)
                                                    },
                                    "0xf4b0f799" => {
                                                        ClassParams::HkpTypedBroadPhaseHandle(map.next_value()?)
                                                    },
                                    "0x6bb7c5e8" => {
                                                        ClassParams::HkpTyremarkPoint(map.next_value()?)
                                                    },
                                    "0x3d0433d6" => {
                                                        ClassParams::HkpTyremarksInfo(map.next_value()?)
                                                    },
                                    "0x1eaef041" => {
                                                        ClassParams::HkpTyremarksWheel(map.next_value()?)
                                                    },
                                    "0x895532c0" => {
                                                        ClassParams::HkpUnaryAction(map.next_value()?)
                                                    },
                                    "0x53340a9" => {
                                                        ClassParams::HkpVehicleCastBatchingManager(map.next_value()?)
                                                    },
                                    "0x82fe40e0" => {
                                                        ClassParams::HkpVehicleDataWheelComponentParams(map.next_value()?)
                                                    },
                                    "0x173feb43" => {
                                                        ClassParams::HkpVehicleData(map.next_value()?)
                                                    },
                                    "0x42fc5bbd" => {
                                                        ClassParams::HkpVehicleDefaultAerodynamics(map.next_value()?)
                                                    },
                                    "0x123a5d50" => {
                                                        ClassParams::HkpVehicleDefaultAnalogDriverInput(map.next_value()?)
                                                    },
                                    "0x1ffad971" => {
                                                        ClassParams::HkpVehicleDefaultBrakeWheelBrakingProperties(map.next_value()?)
                                                    },
                                    "0x4b4f8816" => {
                                                        ClassParams::HkpVehicleDefaultBrake(map.next_value()?)
                                                    },
                                    "0x56f8ca24" => {
                                                        ClassParams::HkpVehicleDefaultEngine(map.next_value()?)
                                                    },
                                    "0x8f0411c8" => {
                                                        ClassParams::HkpVehicleDefaultSteering(map.next_value()?)
                                                    },
                                    "0x7be5bed1" => {
                                                        ClassParams::HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(map.next_value()?)
                                                    },
                                    "0x21735a24" => {
                                                        ClassParams::HkpVehicleDefaultSuspension(map.next_value()?)
                                                    },
                                    "0x235d5d6b" => {
                                                        ClassParams::HkpVehicleDefaultTransmission(map.next_value()?)
                                                    },
                                    "0x741b8d9e" => {
                                                        ClassParams::HkpVehicleDefaultVelocityDamper(map.next_value()?)
                                                    },
                                    "0x2b4a5803" => {
                                                        ClassParams::HkpVehicleDriverInputAnalogStatus(map.next_value()?)
                                                    },
                                    "0x59ce153f" => {
                                                        ClassParams::HkpVehicleFrictionDescriptionAxisDescription(map.next_value()?)
                                                    },
                                    "0x1034549a" => {
                                                        ClassParams::HkpVehicleFrictionDescription(map.next_value()?)
                                                    },
                                    "0xe70e2bb4" => {
                                                        ClassParams::HkpVehicleFrictionStatusAxisStatus(map.next_value()?)
                                                    },
                                    "0x1c076a84" => {
                                                        ClassParams::HkpVehicleFrictionStatus(map.next_value()?)
                                                    },
                                    "0x99f693f0" => {
                                                        ClassParams::HkpVehicleInstanceWheelInfo(map.next_value()?)
                                                    },
                                    "0x877bb579" => {
                                                        ClassParams::HkpVehicleInstance(map.next_value()?)
                                                    },
                                    "0xed529f13" => {
                                                        ClassParams::HkpVehicleRayCastBatchingManager(map.next_value()?)
                                                    },
                                    "0x2a9acf98" => {
                                                        ClassParams::HkpVehicleLinearCastWheelCollideWheelState(map.next_value()?)
                                                    },
                                    "0xc59399d0" => {
                                                        ClassParams::HkpVehicleLinearCastWheelCollide(map.next_value()?)
                                                    },
                                    "0xe2f7d6a7" => {
                                                        ClassParams::HkpVehicleManager(map.next_value()?)
                                                    },
                                    "0x41efd9e3" => {
                                                        ClassParams::HkpVehicleRayCastWheelCollide(map.next_value()?)
                                                    },
                                    "0x358bfe9c" => {
                                                        ClassParams::HkpVehicleSuspensionSuspensionWheelParameters(map.next_value()?)
                                                    },
                                    "0xaf5056fa" => {
                                                        ClassParams::HkpVehicleSuspension(map.next_value()?)
                                                    },
                                    "0x4a50fcb" => {
                                                        ClassParams::HkpVehicleWheelCollide(map.next_value()?)
                                                    },
                                    "0xfca2fcc3" => {
                                                        ClassParams::HkpVelocityConstraintMotor(map.next_value()?)
                                                    },
                                    "0xb2b41feb" => {
                                                        ClassParams::HkpWeldingUtility(map.next_value()?)
                                                    },
                                    "0x1188cbe1" => {
                                                        ClassParams::HkpWheelConstraintDataAtoms(map.next_value()?)
                                                    },
                                    "0xb4c46671" => {
                                                        ClassParams::HkpWheelConstraintData(map.next_value()?)
                                                    },
                                    "0xa5255445" => {
                                                        ClassParams::HkpWorldCinfo(map.next_value()?)
                                                    },
                                    "0x49fb6f2e" => {
                                                        ClassParams::HkpWorldObject(map.next_value()?)
                                                    },
                                    "0xaadcec37" => {
                                                        ClassParams::HkpWorld(map.next_value()?)
                                                    },
                                    "0x471a21ee" => {
                                                        ClassParams::HkQTransform(map.next_value()?)
                                                    },
                                    "0x4846be29" => {
                                                        ClassParams::HkRangeInt32Attribute(map.next_value()?)
                                                    },
                                    "0x949db24f" => {
                                                        ClassParams::HkRangeRealAttribute(map.next_value()?)
                                                    },
                                    "0x3b1c1113" => {
                                                        ClassParams::HkReferencedObject(map.next_value()?)
                                                    },
                                    "0xedb6b8f7" => {
                                                        ClassParams::HkReflectedFileAttribute(map.next_value()?)
                                                    },
                                    "0x660d7cac" => {
                                                        ClassParams::HkResourceBase(map.next_value()?)
                                                    },
                                    "0x4e94146" => {
                                                        ClassParams::HkResourceHandle(map.next_value()?)
                                                    },
                                    "0xb103a2cd" => {
                                                        ClassParams::HkRootLevelContainerNamedVariant(map.next_value()?)
                                                    },
                                    "0x2772c11e" => {
                                                        ClassParams::HkRootLevelContainer(map.next_value()?)
                                                    },
                                    "0x837099c3" => {
                                                        ClassParams::HkSemanticsAttribute(map.next_value()?)
                                                    },
                                    "0xe758f63c" => {
                                                        ClassParams::HkSimpleLocalFrame(map.next_value()?)
                                                    },
                                    "0x143dff99" => {
                                                        ClassParams::HkSphere(map.next_value()?)
                                                    },
                                    "0xb4e5770" => {
                                                        ClassParams::HkSweptTransform(map.next_value()?)
                                                    },
                                    "0x6a4ca82c" => {
                                                        ClassParams::HkTraceStreamTitle(map.next_value()?)
                                                    },
                                    "0x9ab3a6ac" => {
                                                        ClassParams::HkTrackerSerializableScanSnapshotAllocation(map.next_value()?)
                                                    },
                                    "0xe7f23e6d" => {
                                                        ClassParams::HkTrackerSerializableScanSnapshotBlock(map.next_value()?)
                                                    },
                                    "0x875af1d9" => {
                                                        ClassParams::HkTrackerSerializableScanSnapshot(map.next_value()?)
                                                    },
                                    "0xeb6e96e3" => {
                                                        ClassParams::HkUiAttribute(map.next_value()?)
                                                    },
                                    "0x54867cbf" => {
                                                        ClassParams::HkVertexFormatElement(map.next_value()?)
                                                    },
                                    "0xf11e3ff7" => {
                                                        ClassParams::HkVertexFormat(map.next_value()?)
                                                    },
                                    "0xce8b2fbd" => {
                                                        ClassParams::HkxAnimatedFloat(map.next_value()?)
                                                    },
                                    "0x5838e337" => {
                                                        ClassParams::HkxAnimatedMatrix(map.next_value()?)
                                                    },
                                    "0xb4f01baa" => {
                                                        ClassParams::HkxAnimatedQuaternion(map.next_value()?)
                                                    },
                                    "0x34b1a197" => {
                                                        ClassParams::HkxAnimatedVector(map.next_value()?)
                                                    },
                                    "0x345ca95d" => {
                                                        ClassParams::HkxAttributeGroup(map.next_value()?)
                                                    },
                                    "0x7468cc44" => {
                                                        ClassParams::HkxAttributeHolder(map.next_value()?)
                                                    },
                                    "0x7375cae3" => {
                                                        ClassParams::HkxAttribute(map.next_value()?)
                                                    },
                                    "0xe3597b02" => {
                                                        ClassParams::HkxCamera(map.next_value()?)
                                                    },
                                    "0x9ad32a5e" => {
                                                        ClassParams::HkxEdgeSelectionChannel(map.next_value()?)
                                                    },
                                    "0xdf4cf1e9" => {
                                                        ClassParams::HkxEnumItem(map.next_value()?)
                                                    },
                                    "0xc4e1211" => {
                                                        ClassParams::HkxEnum(map.next_value()?)
                                                    },
                                    "0xa6815115" => {
                                                        ClassParams::HkxEnvironmentVariable(map.next_value()?)
                                                    },
                                    "0x41e1aa5" => {
                                                        ClassParams::HkxEnvironment(map.next_value()?)
                                                    },
                                    "0xc12c8197" => {
                                                        ClassParams::HkxIndexBuffer(map.next_value()?)
                                                    },
                                    "0x81c86d42" => {
                                                        ClassParams::HkxLight(map.next_value()?)
                                                    },
                                    "0x1d39f925" => {
                                                        ClassParams::HkxMaterialEffect(map.next_value()?)
                                                    },
                                    "0xd295234d" => {
                                                        ClassParams::HkxMaterialProperty(map.next_value()?)
                                                    },
                                    "0x154650f3" => {
                                                        ClassParams::HkxMaterialShaderSet(map.next_value()?)
                                                    },
                                    "0x28515eff" => {
                                                        ClassParams::HkxMaterialShader(map.next_value()?)
                                                    },
                                    "0xfa6facb2" => {
                                                        ClassParams::HkxMaterialTextureStage(map.next_value()?)
                                                    },
                                    "0x2954537a" => {
                                                        ClassParams::HkxMaterial(map.next_value()?)
                                                    },
                                    "0xe2286cf8" => {
                                                        ClassParams::HkxMeshSection(map.next_value()?)
                                                    },
                                    "0x270724a5" => {
                                                        ClassParams::HkxMeshUserChannelInfo(map.next_value()?)
                                                    },
                                    "0xf2edcc5f" => {
                                                        ClassParams::HkxMesh(map.next_value()?)
                                                    },
                                    "0x433dee92" => {
                                                        ClassParams::HkxNodeAnnotationData(map.next_value()?)
                                                    },
                                    "0xd753fc4d" => {
                                                        ClassParams::HkxNodeSelectionSet(map.next_value()?)
                                                    },
                                    "0x5a218502" => {
                                                        ClassParams::HkxNode(map.next_value()?)
                                                    },
                                    "0x5f673ddd" => {
                                                        ClassParams::HkxScene(map.next_value()?)
                                                    },
                                    "0x5a93f338" => {
                                                        ClassParams::HkxSkinBinding(map.next_value()?)
                                                    },
                                    "0x7a894596" => {
                                                        ClassParams::HkxSparselyAnimatedBool(map.next_value()?)
                                                    },
                                    "0x68a47b64" => {
                                                        ClassParams::HkxSparselyAnimatedEnum(map.next_value()?)
                                                    },
                                    "0xca961951" => {
                                                        ClassParams::HkxSparselyAnimatedInt(map.next_value()?)
                                                    },
                                    "0x185da6fd" => {
                                                        ClassParams::HkxSparselyAnimatedString(map.next_value()?)
                                                    },
                                    "0x1e289259" => {
                                                        ClassParams::HkxTextureFile(map.next_value()?)
                                                    },
                                    "0xd45841d6" => {
                                                        ClassParams::HkxTextureInplace(map.next_value()?)
                                                    },
                                    "0xa02cfca9" => {
                                                        ClassParams::HkxTriangleSelectionChannel(map.next_value()?)
                                                    },
                                    "0xd72b6fd0" => {
                                                        ClassParams::HkxVertexBufferVertexData(map.next_value()?)
                                                    },
                                    "0x4ab10615" => {
                                                        ClassParams::HkxVertexBuffer(map.next_value()?)
                                                    },
                                    "0x483a429b" => {
                                                        ClassParams::HkxVertexDescriptionElementDecl(map.next_value()?)
                                                    },
                                    "0x2df6313d" => {
                                                        ClassParams::HkxVertexDescription(map.next_value()?)
                                                    },
                                    "0xbeeb397c" => {
                                                        ClassParams::HkxVertexFloatDataChannel(map.next_value()?)
                                                    },
                                    "0x5a50e673" => {
                                                        ClassParams::HkxVertexIntDataChannel(map.next_value()?)
                                                    },
                                    "0x866ec6d0" => {
                                                        ClassParams::HkxVertexSelectionChannel(map.next_value()?)
                                                    },
                                    "0x2ea63179" => {
                                                        ClassParams::HkxVertexVectorDataChannel(map.next_value()?)
                                                    },

                                    unknown => {
                                        return Err(de::Error::custom(format!(
                                            "Unexpected value {unknown}"
                                        )))
                                    }
                                })?);
                            } else {
                                return Err(de::Error::custom("Processing an array of `hkparam` requires identification by `signature` first, but the `signature` attribute did not exist"));
                            }
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("@name"))?;
                let class = class.ok_or_else(|| de::Error::missing_field("@class"))?;
                let signature = signature.ok_or_else(|| de::Error::missing_field("@signature"))?;
                let hkparam = hkparam.ok_or_else(|| de::Error::missing_field("hkparam"))?;

                Ok(Class {
                    name,
                    class,
                    signature,
                    hkparams: hkparam,
                })
            }
        }

        deserializer.deserialize_map(ClassVisitor)
    }
}
