
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
    #[serde(bound(deserialize = "Vec<BsDecomposeVectorModifier<'a>>: Deserialize<'de>"))]
    BsDecomposeVectorModifier(Vec<BsDecomposeVectorModifier<'a>>),

    #[serde(rename = "0x19a005c0")]
    #[serde(bound(deserialize = "Vec<BsDirectAtModifier<'a>>: Deserialize<'de>"))]
    BsDirectAtModifier(Vec<BsDirectAtModifier<'a>>),

    #[serde(rename = "0xb34d2bbd")]
    #[serde(bound(deserialize = "Vec<BsDistTriggerModifier<'a>>: Deserialize<'de>"))]
    BsDistTriggerModifier(Vec<BsDistTriggerModifier<'a>>),

    #[serde(rename = "0x6030970c")]
    #[serde(bound(deserialize = "Vec<BsEventEveryNEventsModifier<'a>>: Deserialize<'de>"))]
    BsEventEveryNEventsModifier(Vec<BsEventEveryNEventsModifier<'a>>),

    #[serde(rename = "0x1062d993")]
    #[serde(bound(deserialize = "Vec<BsEventOnDeactivateModifier<'a>>: Deserialize<'de>"))]
    BsEventOnDeactivateModifier(Vec<BsEventOnDeactivateModifier<'a>>),

    #[serde(rename = "0x81d0777a")]
    #[serde(bound(deserialize = "Vec<BsEventOnFalseToTrueModifier<'a>>: Deserialize<'de>"))]
    BsEventOnFalseToTrueModifier(Vec<BsEventOnFalseToTrueModifier<'a>>),

    #[serde(rename = "0xbda33bfe")]
    #[serde(bound(deserialize = "Vec<BsGetTimeStepModifier<'a>>: Deserialize<'de>"))]
    BsGetTimeStepModifier(Vec<BsGetTimeStepModifier<'a>>),

    #[serde(rename = "0x29adc802")]
    #[serde(bound(deserialize = "Vec<BsInterpValueModifier<'a>>: Deserialize<'de>"))]
    BsInterpValueModifier(Vec<BsInterpValueModifier<'a>>),

    #[serde(rename = "0xb0fde45a")]
    #[serde(bound(deserialize = "Vec<BsIsActiveModifier<'a>>: Deserialize<'de>"))]
    BsIsActiveModifier(Vec<BsIsActiveModifier<'a>>),

    #[serde(rename = "0x6b8a15fc")]
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierBSiStateData<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifierBSiStateData(Vec<BsiStateManagerModifierBSiStateData<'a>>),

    #[serde(rename = "0x99463586")]
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierBsiStateManagerStateListener<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifierBsiStateManagerStateListener(Vec<BsiStateManagerModifierBsiStateManagerStateListener<'a>>),

    #[serde(rename = "0x6cb24f2e")]
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifier<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifier(Vec<BsiStateManagerModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<BsPassByTargetTriggerModifier<'a>>: Deserialize<'de>"))]
    BsPassByTargetTriggerModifier(Vec<BsPassByTargetTriggerModifier<'a>>),

    #[serde(rename = "0x8003d8ce")]
    #[serde(bound(deserialize = "Vec<BsRagdollContactListenerModifier<'a>>: Deserialize<'de>"))]
    BsRagdollContactListenerModifier(Vec<BsRagdollContactListenerModifier<'a>>),

    #[serde(rename = "0xd297fda9")]
    #[serde(bound(deserialize = "Vec<BsSpeedSamplerModifier<'a>>: Deserialize<'de>"))]
    BsSpeedSamplerModifier(Vec<BsSpeedSamplerModifier<'a>>),

    #[serde(rename = "0xd83bea64")]
    #[serde(bound(deserialize = "Vec<BsSynchronizedClipGenerator<'a>>: Deserialize<'de>"))]
    BsSynchronizedClipGenerator(Vec<BsSynchronizedClipGenerator<'a>>),

    #[serde(rename = "0x531f3292")]
    #[serde(bound(deserialize = "Vec<BsTimerModifier<'a>>: Deserialize<'de>"))]
    BsTimerModifier(Vec<BsTimerModifier<'a>>),

    #[serde(rename = "0xd2d9a04")]
    #[serde(bound(deserialize = "Vec<BsTweenerModifier<'a>>: Deserialize<'de>"))]
    BsTweenerModifier(Vec<BsTweenerModifier<'a>>),

    #[serde(rename = "0x1d716a17")]
    HkAabbHalf(Vec<HkAabbHalf>),

    #[serde(rename = "0x11e7c11")]
    HkAabbUint32(Vec<HkAabbUint32>),

    #[serde(rename = "0x4a948b16")]
    HkAabb(Vec<HkAabb>),

    #[serde(rename = "0xda8c7d7d")]
    HkaAnimatedReferenceFrame(Vec<HkaAnimatedReferenceFrame>),

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
    HkaDeltaCompressedAnimationQuantizationFormat(Vec<HkaDeltaCompressedAnimationQuantizationFormat>),

    #[serde(rename = "0x90a68d40")]
    #[serde(bound(deserialize = "Vec<HkaDeltaCompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaDeltaCompressedAnimation(Vec<HkaDeltaCompressedAnimation<'a>>),

    #[serde(rename = "0x1d81207c")]
    #[serde(bound(deserialize = "Vec<HkaFootstepAnalysisInfoContainer<'a>>: Deserialize<'de>"))]
    HkaFootstepAnalysisInfoContainer(Vec<HkaFootstepAnalysisInfoContainer<'a>>),

    #[serde(rename = "0x824faf75")]
    HkaFootstepAnalysisInfo(Vec<HkaFootstepAnalysisInfo>),

    #[serde(rename = "0x930af031")]
    #[serde(bound(deserialize = "Vec<HkaInterleavedUncompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaInterleavedUncompressedAnimation(Vec<HkaInterleavedUncompressedAnimation<'a>>),

    #[serde(rename = "0xa3d0ac71")]
    HkaKeyFrameHierarchyUtilityControlData(Vec<HkaKeyFrameHierarchyUtilityControlData>),

    #[serde(rename = "0x7bd5c66f")]
    HkaKeyFrameHierarchyUtility(Vec<HkaKeyFrameHierarchyUtility>),

    #[serde(rename = "0x207cb01")]
    #[serde(bound(deserialize = "Vec<HkAlignSceneToNodeOptions<'a>>: Deserialize<'de>"))]
    HkAlignSceneToNodeOptions(Vec<HkAlignSceneToNodeOptions<'a>>),

    #[serde(rename = "0x48aceb75")]
    HkaMeshBindingMapping(Vec<HkaMeshBindingMapping>),

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
    #[serde(bound(deserialize = "Vec<HkaSkeletonMapper<'a>>: Deserialize<'de>"))]
    HkaSkeletonMapper(Vec<HkaSkeletonMapper<'a>>),

    #[serde(rename = "0x366e8220")]
    #[serde(bound(deserialize = "Vec<HkaSkeleton<'a>>: Deserialize<'de>"))]
    HkaSkeleton(Vec<HkaSkeleton<'a>>),

    #[serde(rename = "0xde830789")]
    HkaSplineCompressedAnimationAnimationCompressionParams(Vec<HkaSplineCompressedAnimationAnimationCompressionParams>),

    #[serde(rename = "0x42e878d3")]
    HkaSplineCompressedAnimationTrackCompressionParams(Vec<HkaSplineCompressedAnimationTrackCompressionParams>),

    #[serde(rename = "0x792ee0bb")]
    #[serde(bound(deserialize = "Vec<HkaSplineCompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaSplineCompressedAnimation(Vec<HkaSplineCompressedAnimation<'a>>),

    #[serde(rename = "0x27c6cafa")]
    HkaWaveletCompressedAnimationCompressionParams(Vec<HkaWaveletCompressedAnimationCompressionParams>),

    #[serde(rename = "0x724a7561")]
    HkaWaveletCompressedAnimationQuantizationFormat(Vec<HkaWaveletCompressedAnimationQuantizationFormat>),

    #[serde(rename = "0x77cf0962")]
    #[serde(bound(deserialize = "Vec<HkaWaveletCompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaWaveletCompressedAnimation(Vec<HkaWaveletCompressedAnimation<'a>>),

    #[serde(rename = "0xe0708a00")]
    HkBaseObject(Vec<HkBaseObject>),

    #[serde(rename = "0xcc0aab32")]
    #[serde(bound(deserialize = "Vec<HkbAttachmentModifier<'a>>: Deserialize<'de>"))]
    HkbAttachmentModifier(Vec<HkbAttachmentModifier<'a>>),

    #[serde(rename = "0x774632b")]
    HkbAttachmentSetup(Vec<HkbAttachmentSetup>),

    #[serde(rename = "0x48b8ad52")]
    HkbAttributeModifierAssignment(Vec<HkbAttributeModifierAssignment>),

    #[serde(rename = "0x1245d97d")]
    #[serde(bound(deserialize = "Vec<HkbAttributeModifier<'a>>: Deserialize<'de>"))]
    HkbAttributeModifier(Vec<HkbAttributeModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbBoneIndexArray<'a>>: Deserialize<'de>"))]
    HkbBoneIndexArray(Vec<HkbBoneIndexArray<'a>>),

    #[serde(rename = "0xcd902b77")]
    #[serde(bound(deserialize = "Vec<HkbBoneWeightArray<'a>>: Deserialize<'de>"))]
    HkbBoneWeightArray(Vec<HkbBoneWeightArray<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbCharacterControllerModifier<'a>>: Deserialize<'de>"))]
    HkbCharacterControllerModifier(Vec<HkbCharacterControllerModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbClipTrigger<'a>>: Deserialize<'de>"))]
    HkbClipTrigger(Vec<HkbClipTrigger<'a>>),

    #[serde(rename = "0xa92ed39f")]
    HkbCombineTransformsModifierInternalState(Vec<HkbCombineTransformsModifierInternalState>),

    #[serde(rename = "0xfd1f0b79")]
    #[serde(bound(deserialize = "Vec<HkbCombineTransformsModifier<'a>>: Deserialize<'de>"))]
    HkbCombineTransformsModifier(Vec<HkbCombineTransformsModifier<'a>>),

    #[serde(rename = "0xc6aaccc8")]
    HkbCompiledExpressionSetToken(Vec<HkbCompiledExpressionSetToken>),

    #[serde(rename = "0x3a7d76cc")]
    HkbCompiledExpressionSet(Vec<HkbCompiledExpressionSet>),

    #[serde(rename = "0x6ac054d7")]
    HkbComputeDirectionModifierInternalState(Vec<HkbComputeDirectionModifierInternalState>),

    #[serde(rename = "0xdf358bd3")]
    #[serde(bound(deserialize = "Vec<HkbComputeDirectionModifier<'a>>: Deserialize<'de>"))]
    HkbComputeDirectionModifier(Vec<HkbComputeDirectionModifier<'a>>),

    #[serde(rename = "0x71cd1eb0")]
    HkbComputeRotationFromAxisAngleModifierInternalState(Vec<HkbComputeRotationFromAxisAngleModifierInternalState>),

    #[serde(rename = "0x9b3f6936")]
    #[serde(bound(deserialize = "Vec<HkbComputeRotationFromAxisAngleModifier<'a>>: Deserialize<'de>"))]
    HkbComputeRotationFromAxisAngleModifier(Vec<HkbComputeRotationFromAxisAngleModifier<'a>>),

    #[serde(rename = "0x71cd1eb0")]
    HkbComputeRotationToTargetModifierInternalState(Vec<HkbComputeRotationToTargetModifierInternalState>),

    #[serde(rename = "0x47665f1c")]
    #[serde(bound(deserialize = "Vec<HkbComputeRotationToTargetModifier<'a>>: Deserialize<'de>"))]
    HkbComputeRotationToTargetModifier(Vec<HkbComputeRotationToTargetModifier<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbCondition(Vec<HkbCondition>),

    #[serde(rename = "0xe0c4d4a7")]
    #[serde(bound(deserialize = "Vec<HkbContext<'a>>: Deserialize<'de>"))]
    HkbContext(Vec<HkbContext<'a>>),

    #[serde(rename = "0x508d3b36")]
    HkbDampingModifierInternalState(Vec<HkbDampingModifierInternalState>),

    #[serde(rename = "0x9a040f03")]
    #[serde(bound(deserialize = "Vec<HkbDampingModifier<'a>>: Deserialize<'de>"))]
    HkbDampingModifier(Vec<HkbDampingModifier<'a>>),

    #[serde(rename = "0x7bd5c66f")]
    HkbDefaultMessageLog(Vec<HkbDefaultMessageLog>),

    #[serde(rename = "0x85fb0b80")]
    HkbDelayedModifierInternalState(Vec<HkbDelayedModifierInternalState>),

    #[serde(rename = "0x8e101a7a")]
    #[serde(bound(deserialize = "Vec<HkbDelayedModifier<'a>>: Deserialize<'de>"))]
    HkbDelayedModifier(Vec<HkbDelayedModifier<'a>>),

    #[serde(rename = "0x7b32d942")]
    HkbDetectCloseToGroundModifierInternalState(Vec<HkbDetectCloseToGroundModifierInternalState>),

    #[serde(rename = "0x981687b2")]
    #[serde(bound(deserialize = "Vec<HkbDetectCloseToGroundModifier<'a>>: Deserialize<'de>"))]
    HkbDetectCloseToGroundModifier(Vec<HkbDetectCloseToGroundModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbEventDrivenModifier<'a>>: Deserialize<'de>"))]
    HkbEventDrivenModifier(Vec<HkbEventDrivenModifier<'a>>),

    #[serde(rename = "0x5874eed4")]
    HkbEventInfo(Vec<HkbEventInfo>),

    #[serde(rename = "0x3d2dbd34")]
    #[serde(bound(deserialize = "Vec<HkbEventPayloadList<'a>>: Deserialize<'de>"))]
    HkbEventPayloadList(Vec<HkbEventPayloadList<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbEventPayload(Vec<HkbEventPayload>),

    #[serde(rename = "0xdb38a15")]
    #[serde(bound(deserialize = "Vec<HkbEventProperty<'a>>: Deserialize<'de>"))]
    HkbEventProperty(Vec<HkbEventProperty<'a>>),

    #[serde(rename = "0xc02da3")]
    #[serde(bound(deserialize = "Vec<HkbEventRaisedInfo<'a>>: Deserialize<'de>"))]
    HkbEventRaisedInfo(Vec<HkbEventRaisedInfo<'a>>),

    #[serde(rename = "0x330a56ee")]
    HkbEventRangeDataArray(Vec<HkbEventRangeDataArray>),

    #[serde(rename = "0x6cb92c76")]
    #[serde(bound(deserialize = "Vec<HkbEventRangeData<'a>>: Deserialize<'de>"))]
    HkbEventRangeData(Vec<HkbEventRangeData<'a>>),

    #[serde(rename = "0x9139b821")]
    #[serde(bound(deserialize = "Vec<HkbEventSequencedDataSequencedEvent<'a>>: Deserialize<'de>"))]
    HkbEventSequencedDataSequencedEvent(Vec<HkbEventSequencedDataSequencedEvent<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbExpressionDataArray<'a>>: Deserialize<'de>"))]
    HkbExpressionDataArray(Vec<HkbExpressionDataArray<'a>>),

    #[serde(rename = "0x6740042a")]
    #[serde(bound(deserialize = "Vec<HkbExpressionData<'a>>: Deserialize<'de>"))]
    HkbExpressionData(Vec<HkbExpressionData<'a>>),

    #[serde(rename = "0x804dcbab")]
    #[serde(bound(deserialize = "Vec<HkbExtractRagdollPoseModifier<'a>>: Deserialize<'de>"))]
    HkbExtractRagdollPoseModifier(Vec<HkbExtractRagdollPoseModifier<'a>>),

    #[serde(rename = "0xa111b704")]
    HkbFootIkControlData(Vec<HkbFootIkControlData>),

    #[serde(rename = "0x9e17091a")]
    #[serde(bound(deserialize = "Vec<HkbFootIkControlsModifierLeg<'a>>: Deserialize<'de>"))]
    HkbFootIkControlsModifierLeg(Vec<HkbFootIkControlsModifierLeg<'a>>),

    #[serde(rename = "0xe5b6f544")]
    #[serde(bound(deserialize = "Vec<HkbFootIkControlsModifier<'a>>: Deserialize<'de>"))]
    HkbFootIkControlsModifier(Vec<HkbFootIkControlsModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbFootIkModifierLeg<'a>>: Deserialize<'de>"))]
    HkbFootIkModifierLeg(Vec<HkbFootIkModifierLeg<'a>>),

    #[serde(rename = "0xed8966c0")]
    #[serde(bound(deserialize = "Vec<HkbFootIkModifier<'a>>: Deserialize<'de>"))]
    HkbFootIkModifier(Vec<HkbFootIkModifier<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbGeneratorOutputListener(Vec<HkbGeneratorOutputListener>),

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
    #[serde(bound(deserialize = "Vec<HkbGenerator<'a>>: Deserialize<'de>"))]
    HkbGenerator(Vec<HkbGenerator<'a>>),

    #[serde(rename = "0x50c34a17")]
    #[serde(bound(deserialize = "Vec<HkbGetHandleOnBoneModifier<'a>>: Deserialize<'de>"))]
    HkbGetHandleOnBoneModifier(Vec<HkbGetHandleOnBoneModifier<'a>>),

    #[serde(rename = "0xd84cad4a")]
    HkbGetUpModifierInternalState(Vec<HkbGetUpModifierInternalState>),

    #[serde(rename = "0x61cb7ac0")]
    #[serde(bound(deserialize = "Vec<HkbGetUpModifier<'a>>: Deserialize<'de>"))]
    HkbGetUpModifier(Vec<HkbGetUpModifier<'a>>),

    #[serde(rename = "0xa92ed39f")]
    HkbGetWorldFromModelModifierInternalState(Vec<HkbGetWorldFromModelModifierInternalState>),

    #[serde(rename = "0x873fc6f7")]
    #[serde(bound(deserialize = "Vec<HkbGetWorldFromModelModifier<'a>>: Deserialize<'de>"))]
    HkbGetWorldFromModelModifier(Vec<HkbGetWorldFromModelModifier<'a>>),

    #[serde(rename = "0xd72b8d17")]
    #[serde(bound(deserialize = "Vec<HkbHandIkControlData<'a>>: Deserialize<'de>"))]
    HkbHandIkControlData(Vec<HkbHandIkControlData<'a>>),

    #[serde(rename = "0x9c72e9e3")]
    #[serde(bound(deserialize = "Vec<HkbHandIkControlsModifierHand<'a>>: Deserialize<'de>"))]
    HkbHandIkControlsModifierHand(Vec<HkbHandIkControlsModifierHand<'a>>),

    #[serde(rename = "0x9f0488bb")]
    #[serde(bound(deserialize = "Vec<HkbHandIkControlsModifier<'a>>: Deserialize<'de>"))]
    HkbHandIkControlsModifier(Vec<HkbHandIkControlsModifier<'a>>),

    #[serde(rename = "0x14dfe1dd")]
    #[serde(bound(deserialize = "Vec<HkbHandIkDriverInfoHand<'a>>: Deserialize<'de>"))]
    HkbHandIkDriverInfoHand(Vec<HkbHandIkDriverInfoHand<'a>>),

    #[serde(rename = "0xc299090a")]
    #[serde(bound(deserialize = "Vec<HkbHandIkDriverInfo<'a>>: Deserialize<'de>"))]
    HkbHandIkDriverInfo(Vec<HkbHandIkDriverInfo<'a>>),

    #[serde(rename = "0x14dfe1dd")]
    #[serde(bound(deserialize = "Vec<HkbHandIkModifierHand<'a>>: Deserialize<'de>"))]
    HkbHandIkModifierHand(Vec<HkbHandIkModifierHand<'a>>),

    #[serde(rename = "0xef8bc2f7")]
    #[serde(bound(deserialize = "Vec<HkbHandIkModifier<'a>>: Deserialize<'de>"))]
    HkbHandIkModifier(Vec<HkbHandIkModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbLinkedSymbolInfo<'a>>: Deserialize<'de>"))]
    HkbLinkedSymbolInfo(Vec<HkbLinkedSymbolInfo<'a>>),

    #[serde(rename = "0xa14caba6")]
    HkbLookAtModifierInternalState(Vec<HkbLookAtModifierInternalState>),

    #[serde(rename = "0x3d28e066")]
    #[serde(bound(deserialize = "Vec<HkbLookAtModifier<'a>>: Deserialize<'de>"))]
    HkbLookAtModifier(Vec<HkbLookAtModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbMirrorModifier<'a>>: Deserialize<'de>"))]
    HkbMirrorModifier(Vec<HkbMirrorModifier<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbModifier<'a>>: Deserialize<'de>"))]
    HkbModifier(Vec<HkbModifier<'a>>),

    #[serde(rename = "0x28f67ba0")]
    HkbMoveCharacterModifierInternalState(Vec<HkbMoveCharacterModifierInternalState>),

    #[serde(rename = "0x8f7492a0")]
    #[serde(bound(deserialize = "Vec<HkbMoveCharacterModifier<'a>>: Deserialize<'de>"))]
    HkbMoveCharacterModifier(Vec<HkbMoveCharacterModifier<'a>>),

    #[serde(rename = "0x65bdd3a0")]
    #[serde(bound(deserialize = "Vec<HkbNamedEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedEventPayload(Vec<HkbNamedEventPayload<'a>>),

    #[serde(rename = "0x3c99bda4")]
    #[serde(bound(deserialize = "Vec<HkbNamedIntEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedIntEventPayload(Vec<HkbNamedIntEventPayload<'a>>),

    #[serde(rename = "0x9c99fd70")]
    #[serde(bound(deserialize = "Vec<HkbNamedRealEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedRealEventPayload(Vec<HkbNamedRealEventPayload<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbRotateCharacterModifier<'a>>: Deserialize<'de>"))]
    HkbRotateCharacterModifier(Vec<HkbRotateCharacterModifier<'a>>),

    #[serde(rename = "0xfb56b692")]
    #[serde(bound(deserialize = "Vec<HkbSenseHandleModifierRange<'a>>: Deserialize<'de>"))]
    HkbSenseHandleModifierRange(Vec<HkbSenseHandleModifierRange<'a>>),

    #[serde(rename = "0x2a064d99")]
    #[serde(bound(deserialize = "Vec<HkbSenseHandleModifier<'a>>: Deserialize<'de>"))]
    HkbSenseHandleModifier(Vec<HkbSenseHandleModifier<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbSequencedData(Vec<HkbSequencedData>),

    #[serde(rename = "0x419b9a05")]
    HkbSequenceInternalState(Vec<HkbSequenceInternalState>),

    #[serde(rename = "0x6a5094e3")]
    #[serde(bound(deserialize = "Vec<HkbSequenceStringData<'a>>: Deserialize<'de>"))]
    HkbSequenceStringData(Vec<HkbSequenceStringData<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbSetWorldFromModelModifier<'a>>: Deserialize<'de>"))]
    HkbSetWorldFromModelModifier(Vec<HkbSetWorldFromModelModifier<'a>>),

    #[serde(rename = "0x2a241367")]
    HkbSimulationControlCommand(Vec<HkbSimulationControlCommand>),

    #[serde(rename = "0xa40822b4")]
    HkbSimulationStateInfo(Vec<HkbSimulationStateInfo>),

    #[serde(rename = "0xda8c7d7d")]
    HkbStateChooser(Vec<HkbStateChooser>),

    #[serde(rename = "0xda8c7d7d")]
    HkbStateListener(Vec<HkbStateListener>),

    #[serde(rename = "0xbb90d54f")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineActiveTransitionInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineActiveTransitionInfo(Vec<HkbStateMachineActiveTransitionInfo<'a>>),

    #[serde(rename = "0x26d5499")]
    HkbStateMachineDelayedTransitionInfo(Vec<HkbStateMachineDelayedTransitionInfo>),

    #[serde(rename = "0xb07b4388")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineEventPropertyArray<'a>>: Deserialize<'de>"))]
    HkbStateMachineEventPropertyArray(Vec<HkbStateMachineEventPropertyArray<'a>>),

    #[serde(rename = "0xbd1a7502")]
    #[serde(bound(deserialize = "Vec<HkbStateMachineInternalState<'a>>: Deserialize<'de>"))]
    HkbStateMachineInternalState(Vec<HkbStateMachineInternalState<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbStateMachineTransitionInfoArray<'a>>: Deserialize<'de>"))]
    HkbStateMachineTransitionInfoArray(Vec<HkbStateMachineTransitionInfoArray<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkbTimerModifier<'a>>: Deserialize<'de>"))]
    HkbTimerModifier(Vec<HkbTimerModifier<'a>>),

    #[serde(rename = "0x5ca91c99")]
    HkbTransformVectorModifierInternalState(Vec<HkbTransformVectorModifierInternalState>),

    #[serde(rename = "0xf93e0e24")]
    #[serde(bound(deserialize = "Vec<HkbTransformVectorModifier<'a>>: Deserialize<'de>"))]
    HkbTransformVectorModifier(Vec<HkbTransformVectorModifier<'a>>),

    #[serde(rename = "0x945da157")]
    #[serde(bound(deserialize = "Vec<HkbTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbTransitionEffect(Vec<HkbTransitionEffect<'a>>),

    #[serde(rename = "0xb6b76b32")]
    #[serde(bound(deserialize = "Vec<HkbTwistModifier<'a>>: Deserialize<'de>"))]
    HkbTwistModifier(Vec<HkbTwistModifier<'a>>),

    #[serde(rename = "0x4d592f72")]
    #[serde(bound(deserialize = "Vec<HkbVariableBindingSetBinding<'a>>: Deserialize<'de>"))]
    HkbVariableBindingSetBinding(Vec<HkbVariableBindingSetBinding<'a>>),

    #[serde(rename = "0x338ad4ff")]
    #[serde(bound(deserialize = "Vec<HkbVariableBindingSet<'a>>: Deserialize<'de>"))]
    HkbVariableBindingSet(Vec<HkbVariableBindingSet<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkCustomAttributes<'a>>: Deserialize<'de>"))]
    HkCustomAttributes(Vec<HkCustomAttributes<'a>>),

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

    #[serde(rename = "0xda8c7d7d")]
    HkLocalFrame(Vec<HkLocalFrame>),

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

    #[serde(rename = "0x7bd5c66f")]
    HkMemoryTrackerAttribute(Vec<HkMemoryTrackerAttribute>),

    #[serde(rename = "0xd0be5d7d")]
    HkMeshBody(Vec<HkMeshBody>),

    #[serde(rename = "0x48aceb75")]
    HkMeshBoneIndexMapping(Vec<HkMeshBoneIndexMapping>),

    #[serde(rename = "0xda8c7d7d")]
    HkMeshMaterial(Vec<HkMeshMaterial>),

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
    #[serde(bound(deserialize = "Vec<HkMonitorStreamColorTable<'a>>: Deserialize<'de>"))]
    HkMonitorStreamColorTable(Vec<HkMonitorStreamColorTable<'a>>),

    #[serde(rename = "0x7798b7db")]
    #[serde(bound(deserialize = "Vec<HkMonitorStreamFrameInfo<'a>>: Deserialize<'de>"))]
    HkMonitorStreamFrameInfo(Vec<HkMonitorStreamFrameInfo<'a>>),

    #[serde(rename = "0x2c76ce16")]
    #[serde(bound(deserialize = "Vec<HkMonitorStreamStringMapStringMap<'a>>: Deserialize<'de>"))]
    HkMonitorStreamStringMapStringMap(Vec<HkMonitorStreamStringMapStringMap<'a>>),

    #[serde(rename = "0xc4d3a8b4")]
    #[serde(bound(deserialize = "Vec<HkMonitorStreamStringMap<'a>>: Deserialize<'de>"))]
    HkMonitorStreamStringMap(Vec<HkMonitorStreamStringMap<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpAngularDashpotAction<'a>>: Deserialize<'de>"))]
    HkpAngularDashpotAction(Vec<HkpAngularDashpotAction<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpBoxMotion<'a>>: Deserialize<'de>"))]
    HkpBoxMotion(Vec<HkpBoxMotion<'a>>),

    #[serde(rename = "0x3444d2d5")]
    HkpBoxShape(Vec<HkpBoxShape>),

    #[serde(rename = "0xda8c7d7d")]
    HkpBreakableBody(Vec<HkpBreakableBody>),

    #[serde(rename = "0x7d6310c8")]
    #[serde(bound(deserialize = "Vec<HkpBreakableConstraintData<'a>>: Deserialize<'de>"))]
    HkpBreakableConstraintData(Vec<HkpBreakableConstraintData<'a>>),

    #[serde(rename = "0xde152a4d")]
    #[serde(bound(deserialize = "Vec<HkpBridgeAtoms<'a>>: Deserialize<'de>"))]
    HkpBridgeAtoms(Vec<HkpBridgeAtoms<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpCachingShapePhantom<'a>>: Deserialize<'de>"))]
    HkpCachingShapePhantom(Vec<HkpCachingShapePhantom<'a>>),

    #[serde(rename = "0xafcd79ad")]
    #[serde(bound(deserialize = "Vec<HkpCallbackConstraintMotor<'a>>: Deserialize<'de>"))]
    HkpCallbackConstraintMotor(Vec<HkpCallbackConstraintMotor<'a>>),

    #[serde(rename = "0xdd0b1fd3")]
    HkpCapsuleShape(Vec<HkpCapsuleShape>),

    #[serde(rename = "0x54a4b841")]
    #[serde(bound(deserialize = "Vec<HkpCdBody<'a>>: Deserialize<'de>"))]
    HkpCdBody(Vec<HkpCdBody<'a>>),

    #[serde(rename = "0x1d7dbdd2")]
    #[serde(bound(deserialize = "Vec<HkpCenterOfMassChangerModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpCenterOfMassChangerModifierConstraintAtom(Vec<HkpCenterOfMassChangerModifierConstraintAtom<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpCharacterControllerCinfo(Vec<HkpCharacterControllerCinfo>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Vec<HkpCharacterMotion<'a>>: Deserialize<'de>"))]
    HkpCharacterMotion(Vec<HkpCharacterMotion<'a>>),

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

    #[serde(rename = "0xe0708a00")]
    HkpCollidableCollidableFilter(Vec<HkpCollidableCollidableFilter>),

    #[serde(rename = "0x9a0e42a5")]
    #[serde(bound(deserialize = "Vec<HkpCollidable<'a>>: Deserialize<'de>"))]
    HkpCollidable(Vec<HkpCollidable<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpConstraintCollisionFilter<'a>>: Deserialize<'de>"))]
    HkpConstraintCollisionFilter(Vec<HkpConstraintCollisionFilter<'a>>),

    #[serde(rename = "0x80559a4e")]
    HkpConstraintData(Vec<HkpConstraintData>),

    #[serde(rename = "0xee3c2aec")]
    #[serde(bound(deserialize = "Vec<HkpConstraintInstanceSmallArraySerializeOverrideType<'a>>: Deserialize<'de>"))]
    HkpConstraintInstanceSmallArraySerializeOverrideType(Vec<HkpConstraintInstanceSmallArraySerializeOverrideType<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpConvexTransformShapeBase<'a>>: Deserialize<'de>"))]
    HkpConvexTransformShapeBase(Vec<HkpConvexTransformShapeBase<'a>>),

    #[serde(rename = "0xae3e5017")]
    #[serde(bound(deserialize = "Vec<HkpConvexTransformShape<'a>>: Deserialize<'de>"))]
    HkpConvexTransformShape(Vec<HkpConvexTransformShape<'a>>),

    #[serde(rename = "0x5ba0a5f7")]
    #[serde(bound(deserialize = "Vec<HkpConvexTranslateShape<'a>>: Deserialize<'de>"))]
    HkpConvexTranslateShape(Vec<HkpConvexTranslateShape<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpDashpotAction<'a>>: Deserialize<'de>"))]
    HkpDashpotAction(Vec<HkpDashpotAction<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpEntityExtendedListeners<'a>>: Deserialize<'de>"))]
    HkpEntityExtendedListeners(Vec<HkpEntityExtendedListeners<'a>>),

    #[serde(rename = "0xee3c2aec")]
    #[serde(bound(deserialize = "Vec<HkpEntitySmallArraySerializeOverrideType<'a>>: Deserialize<'de>"))]
    HkpEntitySmallArraySerializeOverrideType(Vec<HkpEntitySmallArraySerializeOverrideType<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpFastMeshShape<'a>>: Deserialize<'de>"))]
    HkpFastMeshShape(Vec<HkpFastMeshShape<'a>>),

    #[serde(rename = "0x852ab70b")]
    #[serde(bound(deserialize = "Vec<HkpFirstPersonGun<'a>>: Deserialize<'de>"))]
    HkpFirstPersonGun(Vec<HkpFirstPersonGun<'a>>),

    #[serde(rename = "0x64abf85c")]
    #[serde(bound(deserialize = "Vec<HkpFixedRigidMotion<'a>>: Deserialize<'de>"))]
    HkpFixedRigidMotion(Vec<HkpFixedRigidMotion<'a>>),

    #[serde(rename = "0xd6421f19")]
    HkpGenericConstraintDataSchemeConstraintInfo(Vec<HkpGenericConstraintDataSchemeConstraintInfo>),

    #[serde(rename = "0x11fd6f6c")]
    #[serde(bound(deserialize = "Vec<HkpGenericConstraintDataScheme<'a>>: Deserialize<'de>"))]
    HkpGenericConstraintDataScheme(Vec<HkpGenericConstraintDataScheme<'a>>),

    #[serde(rename = "0xfa824640")]
    #[serde(bound(deserialize = "Vec<HkpGenericConstraintData<'a>>: Deserialize<'de>"))]
    HkpGenericConstraintData(Vec<HkpGenericConstraintData<'a>>),

    #[serde(rename = "0x5e2754cd")]
    #[serde(bound(deserialize = "Vec<HkpGravityGun<'a>>: Deserialize<'de>"))]
    HkpGravityGun(Vec<HkpGravityGun<'a>>),

    #[serde(rename = "0x5cc01561")]
    HkpGroupCollisionFilter(Vec<HkpGroupCollisionFilter>),

    #[serde(rename = "0x65ee88e4")]
    HkpGroupFilter(Vec<HkpGroupFilter>),

    #[serde(rename = "0xe7eca7eb")]
    HkpHeightFieldShape(Vec<HkpHeightFieldShape>),

    #[serde(rename = "0x6958371c")]
    HkpHingeConstraintDataAtoms(Vec<HkpHingeConstraintDataAtoms>),

    #[serde(rename = "0x9590f046")]
    HkpHingeConstraintData(Vec<HkpHingeConstraintData>),

    #[serde(rename = "0x555876ff")]
    HkpHingeLimitsDataAtoms(Vec<HkpHingeLimitsDataAtoms>),

    #[serde(rename = "0xbd46760a")]
    HkpHingeLimitsData(Vec<HkpHingeLimitsData>),

    #[serde(rename = "0x5c6aa14d")]
    #[serde(bound(deserialize = "Vec<HkpIgnoreModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpIgnoreModifierConstraintAtom(Vec<HkpIgnoreModifierConstraintAtom<'a>>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Vec<HkpKeyframedRigidMotion<'a>>: Deserialize<'de>"))]
    HkpKeyframedRigidMotion(Vec<HkpKeyframedRigidMotion<'a>>),

    #[serde(rename = "0x3377b0b0")]
    HkpLimitedForceConstraintMotor(Vec<HkpLimitedForceConstraintMotor>),

    #[serde(rename = "0x54c7715b")]
    #[serde(bound(deserialize = "Vec<HkpLimitedHingeConstraintDataAtoms<'a>>: Deserialize<'de>"))]
    HkpLimitedHingeConstraintDataAtoms(Vec<HkpLimitedHingeConstraintDataAtoms<'a>>),

    #[serde(rename = "0x7c15bb6b")]
    HkpLimitedHingeConstraintData(Vec<HkpLimitedHingeConstraintData>),

    #[serde(rename = "0x7b6b0210")]
    HkpLinConstraintAtom(Vec<HkpLinConstraintAtom>),

    #[serde(rename = "0xd7b3be03")]
    HkpLinearParametricCurve(Vec<HkpLinearParametricCurve>),

    #[serde(rename = "0x3e94ef7c")]
    HkpLinFrictionConstraintAtom(Vec<HkpLinFrictionConstraintAtom>),

    #[serde(rename = "0xe1a81497")]
    #[serde(bound(deserialize = "Vec<HkpLinkedCollidable<'a>>: Deserialize<'de>"))]
    HkpLinkedCollidable(Vec<HkpLinkedCollidable<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpListShape<'a>>: Deserialize<'de>"))]
    HkpListShape(Vec<HkpListShape<'a>>),

    #[serde(rename = "0x6748b2cf")]
    #[serde(bound(deserialize = "Vec<HkpMalleableConstraintData<'a>>: Deserialize<'de>"))]
    HkpMalleableConstraintData(Vec<HkpMalleableConstraintData<'a>>),

    #[serde(rename = "0xb6b28240")]
    #[serde(bound(deserialize = "Vec<HkpMassChangerModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpMassChangerModifierConstraintAtom(Vec<HkpMassChangerModifierConstraintAtom<'a>>),

    #[serde(rename = "0x68a56834")]
    HkpMassProperties(Vec<HkpMassProperties>),

    #[serde(rename = "0x33be6570")]
    HkpMaterial(Vec<HkpMaterial>),

    #[serde(rename = "0x64abf85c")]
    #[serde(bound(deserialize = "Vec<HkpMaxSizeMotion<'a>>: Deserialize<'de>"))]
    HkpMaxSizeMotion(Vec<HkpMaxSizeMotion<'a>>),

    #[serde(rename = "0x886cde0c")]
    HkpMeshMaterial(Vec<HkpMeshMaterial>),

    #[serde(rename = "0x27336e5d")]
    #[serde(bound(deserialize = "Vec<HkpMeshShapeSubpart<'a>>: Deserialize<'de>"))]
    HkpMeshShapeSubpart(Vec<HkpMeshShapeSubpart<'a>>),

    #[serde(rename = "0x3bf12c0f")]
    #[serde(bound(deserialize = "Vec<HkpMeshShape<'a>>: Deserialize<'de>"))]
    HkpMeshShape(Vec<HkpMeshShape<'a>>),

    #[serde(rename = "0xb13fef1f")]
    #[serde(bound(deserialize = "Vec<HkpModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpModifierConstraintAtom(Vec<HkpModifierConstraintAtom<'a>>),

    #[serde(rename = "0x90b29d39")]
    #[serde(bound(deserialize = "Vec<HkpMoppBvTreeShape<'a>>: Deserialize<'de>"))]
    HkpMoppBvTreeShape(Vec<HkpMoppBvTreeShape<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpMotorAction<'a>>: Deserialize<'de>"))]
    HkpMotorAction(Vec<HkpMotorAction<'a>>),

    #[serde(rename = "0x6791ffce")]
    #[serde(bound(deserialize = "Vec<HkpMountedBallGun<'a>>: Deserialize<'de>"))]
    HkpMountedBallGun(Vec<HkpMountedBallGun<'a>>),

    #[serde(rename = "0x6e087fd6")]
    #[serde(bound(deserialize = "Vec<HkpMouseSpringAction<'a>>: Deserialize<'de>"))]
    HkpMouseSpringAction(Vec<HkpMouseSpringAction<'a>>),

    #[serde(rename = "0x79ab517d")]
    #[serde(bound(deserialize = "Vec<HkpMovingSurfaceModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpMovingSurfaceModifierConstraintAtom(Vec<HkpMovingSurfaceModifierConstraintAtom<'a>>),

    #[serde(rename = "0xffdc0b65")]
    HkpMultiRayShapeRay(Vec<HkpMultiRayShapeRay>),

    #[serde(rename = "0xea2e7ec9")]
    HkpMultiRayShape(Vec<HkpMultiRayShape>),

    #[serde(rename = "0x61a590fc")]
    HkpMultiSphereShape(Vec<HkpMultiSphereShape>),

    #[serde(rename = "0xc03af40d")]
    #[serde(bound(deserialize = "Vec<HkpMultithreadedVehicleManager<'a>>: Deserialize<'de>"))]
    HkpMultithreadedVehicleManager(Vec<HkpMultithreadedVehicleManager<'a>>),

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

    #[serde(rename = "0xda8c7d7d")]
    HkpParametricCurve(Vec<HkpParametricCurve>),

    #[serde(rename = "0xe7eca7eb")]
    HkpPhantomCallbackShape(Vec<HkpPhantomCallbackShape>),

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
    #[serde(bound(deserialize = "Vec<HkpPoweredChainData<'a>>: Deserialize<'de>"))]
    HkpPoweredChainData(Vec<HkpPoweredChainData<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpPrismaticConstraintDataAtoms<'a>>: Deserialize<'de>"))]
    HkpPrismaticConstraintDataAtoms(Vec<HkpPrismaticConstraintDataAtoms<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpRagdollConstraintDataAtoms<'a>>: Deserialize<'de>"))]
    HkpRagdollConstraintDataAtoms(Vec<HkpRagdollConstraintDataAtoms<'a>>),

    #[serde(rename = "0x8fb5dd29")]
    HkpRagdollConstraintData(Vec<HkpRagdollConstraintData>),

    #[serde(rename = "0x82b894c3")]
    HkpRagdollLimitsDataAtoms(Vec<HkpRagdollLimitsDataAtoms>),

    #[serde(rename = "0xcbdb44aa")]
    HkpRagdollLimitsData(Vec<HkpRagdollLimitsData>),

    #[serde(rename = "0x71013826")]
    #[serde(bound(deserialize = "Vec<HkpRagdollMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpRagdollMotorConstraintAtom(Vec<HkpRagdollMotorConstraintAtom<'a>>),

    #[serde(rename = "0xe0708a00")]
    HkpRayCollidableFilter(Vec<HkpRayCollidableFilter>),

    #[serde(rename = "0xe0708a00")]
    HkpRayShapeCollectionFilter(Vec<HkpRayShapeCollectionFilter>),

    #[serde(rename = "0xc4fa16c9")]
    #[serde(bound(deserialize = "Vec<HkpRejectChassisListener<'a>>: Deserialize<'de>"))]
    HkpRejectChassisListener(Vec<HkpRejectChassisListener<'a>>),

    #[serde(rename = "0x91367f03")]
    #[serde(bound(deserialize = "Vec<HkpRemoveTerminalsMoppModifier<'a>>: Deserialize<'de>"))]
    HkpRemoveTerminalsMoppModifier(Vec<HkpRemoveTerminalsMoppModifier<'a>>),

    #[serde(rename = "0x2dc0ec6a")]
    #[serde(bound(deserialize = "Vec<HkpReorientAction<'a>>: Deserialize<'de>"))]
    HkpReorientAction(Vec<HkpReorientAction<'a>>),

    #[serde(rename = "0x75f8d805")]
    #[serde(bound(deserialize = "Vec<HkpRigidBody<'a>>: Deserialize<'de>"))]
    HkpRigidBody(Vec<HkpRigidBody<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpSerializedDisplayRbTransforms<'a>>: Deserialize<'de>"))]
    HkpSerializedDisplayRbTransforms(Vec<HkpSerializedDisplayRbTransforms<'a>>),

    #[serde(rename = "0x10155a")]
    #[serde(bound(deserialize = "Vec<HkpSerializedSubTrack1NInfo<'a>>: Deserialize<'de>"))]
    HkpSerializedSubTrack1NInfo(Vec<HkpSerializedSubTrack1NInfo<'a>>),

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

    #[serde(rename = "0xe0708a00")]
    HkpShapeCollectionFilter(Vec<HkpShapeCollectionFilter>),

    #[serde(rename = "0xe8c3991d")]
    HkpShapeCollection(Vec<HkpShapeCollection>),

    #[serde(rename = "0xe0708a00")]
    HkpShapeContainer(Vec<HkpShapeContainer>),

    #[serde(rename = "0xea7f1d08")]
    #[serde(bound(deserialize = "Vec<HkpShapeInfo<'a>>: Deserialize<'de>"))]
    HkpShapeInfo(Vec<HkpShapeInfo<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpShapeModifier(Vec<HkpShapeModifier>),

    #[serde(rename = "0xcb22fbcd")]
    #[serde(bound(deserialize = "Vec<HkpShapePhantom<'a>>: Deserialize<'de>"))]
    HkpShapePhantom(Vec<HkpShapePhantom<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpSimpleShapePhantom<'a>>: Deserialize<'de>"))]
    HkpSimpleShapePhantom(Vec<HkpSimpleShapePhantom<'a>>),

    #[serde(rename = "0x97aba922")]
    #[serde(bound(deserialize = "Vec<HkpSimulation<'a>>: Deserialize<'de>"))]
    HkpSimulation(Vec<HkpSimulation<'a>>),

    #[serde(rename = "0x73aa1d38")]
    #[serde(bound(deserialize = "Vec<HkpSingleShapeContainer<'a>>: Deserialize<'de>"))]
    HkpSingleShapeContainer(Vec<HkpSingleShapeContainer<'a>>),

    #[serde(rename = "0xecb34e27")]
    #[serde(bound(deserialize = "Vec<HkpSoftContactModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpSoftContactModifierConstraintAtom(Vec<HkpSoftContactModifierConstraintAtom<'a>>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Vec<HkpSphereMotion<'a>>: Deserialize<'de>"))]
    HkpSphereMotion(Vec<HkpSphereMotion<'a>>),

    #[serde(rename = "0xe7eca7eb")]
    HkpSphereRepShape(Vec<HkpSphereRepShape>),

    #[serde(rename = "0x795d9fa")]
    HkpSphereShape(Vec<HkpSphereShape>),

    #[serde(rename = "0x88fc09fa")]
    #[serde(bound(deserialize = "Vec<HkpSpringAction<'a>>: Deserialize<'de>"))]
    HkpSpringAction(Vec<HkpSpringAction<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkpStorageExtendedMeshShapeMeshSubpartStorage<'a>>: Deserialize<'de>"))]
    HkpStorageExtendedMeshShapeMeshSubpartStorage(Vec<HkpStorageExtendedMeshShapeMeshSubpartStorage<'a>>),

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

    #[serde(rename = "0x64abf85c")]
    #[serde(bound(deserialize = "Vec<HkpThinBoxMotion<'a>>: Deserialize<'de>"))]
    HkpThinBoxMotion(Vec<HkpThinBoxMotion<'a>>),

    #[serde(rename = "0x787ef513")]
    #[serde(bound(deserialize = "Vec<HkpTransformShape<'a>>: Deserialize<'de>"))]
    HkpTransformShape(Vec<HkpTransformShape<'a>>),

    #[serde(rename = "0x95ad1a25")]
    HkpTriangleShape(Vec<HkpTriangleShape>),

    #[serde(rename = "0xeb60f431")]
    #[serde(bound(deserialize = "Vec<HkpTriggerVolumeEventInfo<'a>>: Deserialize<'de>"))]
    HkpTriggerVolumeEventInfo(Vec<HkpTriggerVolumeEventInfo<'a>>),

    #[serde(rename = "0xa29a8d1a")]
    #[serde(bound(deserialize = "Vec<HkpTriggerVolume<'a>>: Deserialize<'de>"))]
    HkpTriggerVolume(Vec<HkpTriggerVolume<'a>>),

    #[serde(rename = "0x58e1e585")]
    #[serde(bound(deserialize = "Vec<HkpTriSampledHeightFieldBvTreeShape<'a>>: Deserialize<'de>"))]
    HkpTriSampledHeightFieldBvTreeShape(Vec<HkpTriSampledHeightFieldBvTreeShape<'a>>),

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

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleAerodynamics(Vec<HkpVehicleAerodynamics>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleBrake(Vec<HkpVehicleBrake>),

    #[serde(rename = "0x53340a9")]
    #[serde(bound(deserialize = "Vec<HkpVehicleCastBatchingManager<'a>>: Deserialize<'de>"))]
    HkpVehicleCastBatchingManager(Vec<HkpVehicleCastBatchingManager<'a>>),

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

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleDriverInputStatus(Vec<HkpVehicleDriverInputStatus>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleDriverInput(Vec<HkpVehicleDriverInput>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleEngine(Vec<HkpVehicleEngine>),

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
    #[serde(bound(deserialize = "Vec<HkpVehicleLinearCastBatchingManager<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastBatchingManager(Vec<HkpVehicleLinearCastBatchingManager<'a>>),

    #[serde(rename = "0x2a9acf98")]
    #[serde(bound(deserialize = "Vec<HkpVehicleLinearCastWheelCollideWheelState<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastWheelCollideWheelState(Vec<HkpVehicleLinearCastWheelCollideWheelState<'a>>),

    #[serde(rename = "0xc59399d0")]
    #[serde(bound(deserialize = "Vec<HkpVehicleLinearCastWheelCollide<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastWheelCollide(Vec<HkpVehicleLinearCastWheelCollide<'a>>),

    #[serde(rename = "0xe2f7d6a7")]
    #[serde(bound(deserialize = "Vec<HkpVehicleManager<'a>>: Deserialize<'de>"))]
    HkpVehicleManager(Vec<HkpVehicleManager<'a>>),

    #[serde(rename = "0xed529f13")]
    #[serde(bound(deserialize = "Vec<HkpVehicleRayCastBatchingManager<'a>>: Deserialize<'de>"))]
    HkpVehicleRayCastBatchingManager(Vec<HkpVehicleRayCastBatchingManager<'a>>),

    #[serde(rename = "0x41efd9e3")]
    #[serde(bound(deserialize = "Vec<HkpVehicleRayCastWheelCollide<'a>>: Deserialize<'de>"))]
    HkpVehicleRayCastWheelCollide(Vec<HkpVehicleRayCastWheelCollide<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleSteering(Vec<HkpVehicleSteering>),

    #[serde(rename = "0x358bfe9c")]
    HkpVehicleSuspensionSuspensionWheelParameters(Vec<HkpVehicleSuspensionSuspensionWheelParameters>),

    #[serde(rename = "0xaf5056fa")]
    HkpVehicleSuspension(Vec<HkpVehicleSuspension>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleTransmission(Vec<HkpVehicleTransmission>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleVelocityDamper(Vec<HkpVehicleVelocityDamper>),

    #[serde(rename = "0x4a50fcb")]
    HkpVehicleWheelCollide(Vec<HkpVehicleWheelCollide>),

    #[serde(rename = "0xfca2fcc3")]
    HkpVelocityConstraintMotor(Vec<HkpVelocityConstraintMotor>),

    #[serde(rename = "0x5c6aa14d")]
    #[serde(bound(deserialize = "Vec<HkpViscousSurfaceModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpViscousSurfaceModifierConstraintAtom(Vec<HkpViscousSurfaceModifierConstraintAtom<'a>>),

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
    HkResourceContainer(Vec<HkResourceContainer>),

    #[serde(rename = "0x4e94146")]
    HkResourceHandle(Vec<HkResourceHandle>),

    #[serde(rename = "0xb103a2cd")]
    #[serde(bound(deserialize = "Vec<HkRootLevelContainerNamedVariant<'a>>: Deserialize<'de>"))]
    HkRootLevelContainerNamedVariant(Vec<HkRootLevelContainerNamedVariant<'a>>),

    #[serde(rename = "0x2772c11e")]
    #[serde(bound(deserialize = "Vec<HkRootLevelContainer<'a>>: Deserialize<'de>"))]
    HkRootLevelContainer(Vec<HkRootLevelContainer<'a>>),

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

    #[serde(rename = "0xda8c7d7d")]
    HkWorldMemoryAvailableWatchDog(Vec<HkWorldMemoryAvailableWatchDog>),

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
    #[serde(bound(deserialize = "Vec<HkxAttributeHolder<'a>>: Deserialize<'de>"))]
    HkxAttributeHolder(Vec<HkxAttributeHolder<'a>>),

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
    #[serde(bound(deserialize = "Vec<HkxEnum<'a>>: Deserialize<'de>"))]
    HkxEnum(Vec<HkxEnum<'a>>),

    #[serde(rename = "0xa6815115")]
    #[serde(bound(deserialize = "Vec<HkxEnvironmentVariable<'a>>: Deserialize<'de>"))]
    HkxEnvironmentVariable(Vec<HkxEnvironmentVariable<'a>>),

    #[serde(rename = "0x41e1aa5")]
    #[serde(bound(deserialize = "Vec<HkxEnvironment<'a>>: Deserialize<'de>"))]
    HkxEnvironment(Vec<HkxEnvironment<'a>>),

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

        let mut state = serializer.serialize_struct("Class", 678)?;
        state.serialize_field("@name", &self.name)?;
        state.serialize_field("@class", &self.class)?;
        state.serialize_field("@signature", &self.signature)?;

        // Serialize hkparam based on class(C++ class name)
        match self.class.as_ref() {

            "BGSGamebryoSequenceGenerator" => {
                if let ClassParams::BgsGamebryoSequenceGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSBoneSwitchGeneratorBoneData" => {
                if let ClassParams::BsBoneSwitchGeneratorBoneData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSBoneSwitchGenerator" => {
                if let ClassParams::BsBoneSwitchGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSComputeAddBoneAnimModifier" => {
                if let ClassParams::BsComputeAddBoneAnimModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSCyclicBlendTransitionGenerator" => {
                if let ClassParams::BsCyclicBlendTransitionGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSDecomposeVectorModifier" => {
                if let ClassParams::BsDecomposeVectorModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSDirectAtModifier" => {
                if let ClassParams::BsDirectAtModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSDistTriggerModifier" => {
                if let ClassParams::BsDistTriggerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSEventEveryNEventsModifier" => {
                if let ClassParams::BsEventEveryNEventsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSEventOnDeactivateModifier" => {
                if let ClassParams::BsEventOnDeactivateModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSEventOnFalseToTrueModifier" => {
                if let ClassParams::BsEventOnFalseToTrueModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSGetTimeStepModifier" => {
                if let ClassParams::BsGetTimeStepModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSInterpValueModifier" => {
                if let ClassParams::BsInterpValueModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSIsActiveModifier" => {
                if let ClassParams::BsIsActiveModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSIStateManagerModifierBSiStateData" => {
                if let ClassParams::BsiStateManagerModifierBSiStateData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSIStateManagerModifierBSIStateManagerStateListener" => {
                if let ClassParams::BsiStateManagerModifierBsiStateManagerStateListener(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSIStateManagerModifier" => {
                if let ClassParams::BsiStateManagerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSiStateTaggingGenerator" => {
                if let ClassParams::BSiStateTaggingGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSLimbIKModifier" => {
                if let ClassParams::BsLimbIkModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSLookAtModifierBoneData" => {
                if let ClassParams::BsLookAtModifierBoneData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSLookAtModifier" => {
                if let ClassParams::BsLookAtModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSModifyOnceModifier" => {
                if let ClassParams::BsModifyOnceModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSOffsetAnimationGenerator" => {
                if let ClassParams::BsOffsetAnimationGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSPassByTargetTriggerModifier" => {
                if let ClassParams::BsPassByTargetTriggerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSRagdollContactListenerModifier" => {
                if let ClassParams::BsRagdollContactListenerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSSpeedSamplerModifier" => {
                if let ClassParams::BsSpeedSamplerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSSynchronizedClipGenerator" => {
                if let ClassParams::BsSynchronizedClipGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSTimerModifier" => {
                if let ClassParams::BsTimerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "BSTweenerModifier" => {
                if let ClassParams::BsTweenerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkAabbHalf" => {
                if let ClassParams::HkAabbHalf(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkAabbUint32" => {
                if let ClassParams::HkAabbUint32(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkAabb" => {
                if let ClassParams::HkAabb(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnimatedReferenceFrame" => {
                if let ClassParams::HkaAnimatedReferenceFrame(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnimationBinding" => {
                if let ClassParams::HkaAnimationBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnimationContainer" => {
                if let ClassParams::HkaAnimationContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnimationPreviewColorContainer" => {
                if let ClassParams::HkaAnimationPreviewColorContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnimation" => {
                if let ClassParams::HkaAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnnotationTrackAnnotation" => {
                if let ClassParams::HkaAnnotationTrackAnnotation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaAnnotationTrack" => {
                if let ClassParams::HkaAnnotationTrack(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaBoneAttachment" => {
                if let ClassParams::HkaBoneAttachment(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaBone" => {
                if let ClassParams::HkaBone(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaDefaultAnimatedReferenceFrame" => {
                if let ClassParams::HkaDefaultAnimatedReferenceFrame(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaDeltaCompressedAnimationQuantizationFormat" => {
                if let ClassParams::HkaDeltaCompressedAnimationQuantizationFormat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaDeltaCompressedAnimation" => {
                if let ClassParams::HkaDeltaCompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaFootstepAnalysisInfoContainer" => {
                if let ClassParams::HkaFootstepAnalysisInfoContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaFootstepAnalysisInfo" => {
                if let ClassParams::HkaFootstepAnalysisInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaInterleavedUncompressedAnimation" => {
                if let ClassParams::HkaInterleavedUncompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaKeyFrameHierarchyUtilityControlData" => {
                if let ClassParams::HkaKeyFrameHierarchyUtilityControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaKeyFrameHierarchyUtility" => {
                if let ClassParams::HkaKeyFrameHierarchyUtility(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkAlignSceneToNodeOptions" => {
                if let ClassParams::HkAlignSceneToNodeOptions(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaMeshBindingMapping" => {
                if let ClassParams::HkaMeshBindingMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaMeshBinding" => {
                if let ClassParams::HkaMeshBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaQuantizedAnimationTrackCompressionParams" => {
                if let ClassParams::HkaQuantizedAnimationTrackCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaQuantizedAnimation" => {
                if let ClassParams::HkaQuantizedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaRagdollInstance" => {
                if let ClassParams::HkaRagdollInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkArrayTypeAttribute" => {
                if let ClassParams::HkArrayTypeAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSkeletonLocalFrameOnBone" => {
                if let ClassParams::HkaSkeletonLocalFrameOnBone(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSkeletonMapperDataChainMapping" => {
                if let ClassParams::HkaSkeletonMapperDataChainMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSkeletonMapperDataSimpleMapping" => {
                if let ClassParams::HkaSkeletonMapperDataSimpleMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSkeletonMapperData" => {
                if let ClassParams::HkaSkeletonMapperData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSkeletonMapper" => {
                if let ClassParams::HkaSkeletonMapper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSkeleton" => {
                if let ClassParams::HkaSkeleton(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSplineCompressedAnimationAnimationCompressionParams" => {
                if let ClassParams::HkaSplineCompressedAnimationAnimationCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSplineCompressedAnimationTrackCompressionParams" => {
                if let ClassParams::HkaSplineCompressedAnimationTrackCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaSplineCompressedAnimation" => {
                if let ClassParams::HkaSplineCompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaWaveletCompressedAnimationCompressionParams" => {
                if let ClassParams::HkaWaveletCompressedAnimationCompressionParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaWaveletCompressedAnimationQuantizationFormat" => {
                if let ClassParams::HkaWaveletCompressedAnimationQuantizationFormat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkaWaveletCompressedAnimation" => {
                if let ClassParams::HkaWaveletCompressedAnimation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkBaseObject" => {
                if let ClassParams::HkBaseObject(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbAttachmentModifier" => {
                if let ClassParams::HkbAttachmentModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbAttachmentSetup" => {
                if let ClassParams::HkbAttachmentSetup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbAttributeModifierAssignment" => {
                if let ClassParams::HkbAttributeModifierAssignment(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbAttributeModifier" => {
                if let ClassParams::HkbAttributeModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbAuxiliaryNodeInfo" => {
                if let ClassParams::HkbAuxiliaryNodeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorEventsInfo" => {
                if let ClassParams::HkbBehaviorEventsInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorGraphData" => {
                if let ClassParams::HkbBehaviorGraphData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorGraphInternalStateInfo" => {
                if let ClassParams::HkbBehaviorGraphInternalStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorGraphInternalState" => {
                if let ClassParams::HkbBehaviorGraphInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorGraphStringData" => {
                if let ClassParams::HkbBehaviorGraphStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorGraph" => {
                if let ClassParams::HkbBehaviorGraph(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorInfoIdToNamePair" => {
                if let ClassParams::HkbBehaviorInfoIdToNamePair(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorInfo" => {
                if let ClassParams::HkbBehaviorInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBehaviorReferenceGenerator" => {
                if let ClassParams::HkbBehaviorReferenceGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBindable" => {
                if let ClassParams::HkbBindable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlendCurveUtils" => {
                if let ClassParams::HkbBlendCurveUtils(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlenderGeneratorChildInternalState" => {
                if let ClassParams::HkbBlenderGeneratorChildInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlenderGeneratorChild" => {
                if let ClassParams::HkbBlenderGeneratorChild(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlenderGeneratorInternalState" => {
                if let ClassParams::HkbBlenderGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlenderGenerator" => {
                if let ClassParams::HkbBlenderGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlendingTransitionEffectInternalState" => {
                if let ClassParams::HkbBlendingTransitionEffectInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBlendingTransitionEffect" => {
                if let ClassParams::HkbBlendingTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBoneIndexArray" => {
                if let ClassParams::HkbBoneIndexArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBoneWeightArray" => {
                if let ClassParams::HkbBoneWeightArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBoolVariableSequencedDataSample" => {
                if let ClassParams::HkbBoolVariableSequencedDataSample(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbBoolVariableSequencedData" => {
                if let ClassParams::HkbBoolVariableSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCameraShakeEventPayload" => {
                if let ClassParams::HkbCameraShakeEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterAddedInfo" => {
                if let ClassParams::HkbCharacterAddedInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterControlCommand" => {
                if let ClassParams::HkbCharacterControlCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterControllerControlData" => {
                if let ClassParams::HkbCharacterControllerControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterControllerModifierInternalState" => {
                if let ClassParams::HkbCharacterControllerModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterControllerModifier" => {
                if let ClassParams::HkbCharacterControllerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterDataCharacterControllerInfo" => {
                if let ClassParams::HkbCharacterDataCharacterControllerInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterData" => {
                if let ClassParams::HkbCharacterData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterInfo" => {
                if let ClassParams::HkbCharacterInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterSetup" => {
                if let ClassParams::HkbCharacterSetup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterSkinInfo" => {
                if let ClassParams::HkbCharacterSkinInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterSteppedInfo" => {
                if let ClassParams::HkbCharacterSteppedInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacterStringData" => {
                if let ClassParams::HkbCharacterStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCharacter" => {
                if let ClassParams::HkbCharacter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbClientCharacterState" => {
                if let ClassParams::HkbClientCharacterState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbClipGeneratorEcho" => {
                if let ClassParams::HkbClipGeneratorEcho(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbClipGeneratorInternalState" => {
                if let ClassParams::HkbClipGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbClipGenerator" => {
                if let ClassParams::HkbClipGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbClipTriggerArray" => {
                if let ClassParams::HkbClipTriggerArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbClipTrigger" => {
                if let ClassParams::HkbClipTrigger(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCombineTransformsModifierInternalState" => {
                if let ClassParams::HkbCombineTransformsModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCombineTransformsModifier" => {
                if let ClassParams::HkbCombineTransformsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCompiledExpressionSetToken" => {
                if let ClassParams::HkbCompiledExpressionSetToken(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCompiledExpressionSet" => {
                if let ClassParams::HkbCompiledExpressionSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbComputeDirectionModifierInternalState" => {
                if let ClassParams::HkbComputeDirectionModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbComputeDirectionModifier" => {
                if let ClassParams::HkbComputeDirectionModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbComputeRotationFromAxisAngleModifierInternalState" => {
                if let ClassParams::HkbComputeRotationFromAxisAngleModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbComputeRotationFromAxisAngleModifier" => {
                if let ClassParams::HkbComputeRotationFromAxisAngleModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbComputeRotationToTargetModifierInternalState" => {
                if let ClassParams::HkbComputeRotationToTargetModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbComputeRotationToTargetModifier" => {
                if let ClassParams::HkbComputeRotationToTargetModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbCondition" => {
                if let ClassParams::HkbCondition(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbContext" => {
                if let ClassParams::HkbContext(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDampingModifierInternalState" => {
                if let ClassParams::HkbDampingModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDampingModifier" => {
                if let ClassParams::HkbDampingModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDefaultMessageLog" => {
                if let ClassParams::HkbDefaultMessageLog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDelayedModifierInternalState" => {
                if let ClassParams::HkbDelayedModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDelayedModifier" => {
                if let ClassParams::HkbDelayedModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDetectCloseToGroundModifierInternalState" => {
                if let ClassParams::HkbDetectCloseToGroundModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbDetectCloseToGroundModifier" => {
                if let ClassParams::HkbDetectCloseToGroundModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEvaluateExpressionModifierInternalExpressionData" => {
                if let ClassParams::HkbEvaluateExpressionModifierInternalExpressionData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEvaluateExpressionModifierInternalState" => {
                if let ClassParams::HkbEvaluateExpressionModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEvaluateExpressionModifier" => {
                if let ClassParams::HkbEvaluateExpressionModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEvaluateHandleModifier" => {
                if let ClassParams::HkbEvaluateHandleModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventBase" => {
                if let ClassParams::HkbEventBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventDrivenModifierInternalState" => {
                if let ClassParams::HkbEventDrivenModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventDrivenModifier" => {
                if let ClassParams::HkbEventDrivenModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventInfo" => {
                if let ClassParams::HkbEventInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventPayloadList" => {
                if let ClassParams::HkbEventPayloadList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventPayload" => {
                if let ClassParams::HkbEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventProperty" => {
                if let ClassParams::HkbEventProperty(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventRaisedInfo" => {
                if let ClassParams::HkbEventRaisedInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventRangeDataArray" => {
                if let ClassParams::HkbEventRangeDataArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventRangeData" => {
                if let ClassParams::HkbEventRangeData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventSequencedDataSequencedEvent" => {
                if let ClassParams::HkbEventSequencedDataSequencedEvent(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventSequencedData" => {
                if let ClassParams::HkbEventSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventsFromRangeModifierInternalState" => {
                if let ClassParams::HkbEventsFromRangeModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEventsFromRangeModifier" => {
                if let ClassParams::HkbEventsFromRangeModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbEvent" => {
                if let ClassParams::HkbEvent(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbExpressionCondition" => {
                if let ClassParams::HkbExpressionCondition(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbExpressionDataArray" => {
                if let ClassParams::HkbExpressionDataArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbExpressionData" => {
                if let ClassParams::HkbExpressionData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbExtractRagdollPoseModifier" => {
                if let ClassParams::HkbExtractRagdollPoseModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkControlData" => {
                if let ClassParams::HkbFootIkControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkControlsModifierLeg" => {
                if let ClassParams::HkbFootIkControlsModifierLeg(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkControlsModifier" => {
                if let ClassParams::HkbFootIkControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkDriverInfoLeg" => {
                if let ClassParams::HkbFootIkDriverInfoLeg(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkDriverInfo" => {
                if let ClassParams::HkbFootIkDriverInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkGains" => {
                if let ClassParams::HkbFootIkGains(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkModifierInternalLegData" => {
                if let ClassParams::HkbFootIkModifierInternalLegData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkModifierLeg" => {
                if let ClassParams::HkbFootIkModifierLeg(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbFootIkModifier" => {
                if let ClassParams::HkbFootIkModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGeneratorOutputListener" => {
                if let ClassParams::HkbGeneratorOutputListener(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGeneratorSyncInfoSyncPoint" => {
                if let ClassParams::HkbGeneratorSyncInfoSyncPoint(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGeneratorSyncInfo" => {
                if let ClassParams::HkbGeneratorSyncInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGeneratorTransitionEffectInternalState" => {
                if let ClassParams::HkbGeneratorTransitionEffectInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGeneratorTransitionEffect" => {
                if let ClassParams::HkbGeneratorTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGenerator" => {
                if let ClassParams::HkbGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGetHandleOnBoneModifier" => {
                if let ClassParams::HkbGetHandleOnBoneModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGetUpModifierInternalState" => {
                if let ClassParams::HkbGetUpModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGetUpModifier" => {
                if let ClassParams::HkbGetUpModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGetWorldFromModelModifierInternalState" => {
                if let ClassParams::HkbGetWorldFromModelModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbGetWorldFromModelModifier" => {
                if let ClassParams::HkbGetWorldFromModelModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkControlData" => {
                if let ClassParams::HkbHandIkControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkControlsModifierHand" => {
                if let ClassParams::HkbHandIkControlsModifierHand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkControlsModifier" => {
                if let ClassParams::HkbHandIkControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkDriverInfoHand" => {
                if let ClassParams::HkbHandIkDriverInfoHand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkDriverInfo" => {
                if let ClassParams::HkbHandIkDriverInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkModifierHand" => {
                if let ClassParams::HkbHandIkModifierHand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandIkModifier" => {
                if let ClassParams::HkbHandIkModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbHandle" => {
                if let ClassParams::HkbHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbIntEventPayload" => {
                if let ClassParams::HkbIntEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbIntVariableSequencedDataSample" => {
                if let ClassParams::HkbIntVariableSequencedDataSample(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbIntVariableSequencedData" => {
                if let ClassParams::HkbIntVariableSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkBitField" => {
                if let ClassParams::HkBitField(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbKeyframeBonesModifierKeyframeInfo" => {
                if let ClassParams::HkbKeyframeBonesModifierKeyframeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbKeyframeBonesModifier" => {
                if let ClassParams::HkbKeyframeBonesModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbLinkedSymbolInfo" => {
                if let ClassParams::HkbLinkedSymbolInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbLookAtModifierInternalState" => {
                if let ClassParams::HkbLookAtModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbLookAtModifier" => {
                if let ClassParams::HkbLookAtModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbManualSelectorGeneratorInternalState" => {
                if let ClassParams::HkbManualSelectorGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbManualSelectorGenerator" => {
                if let ClassParams::HkbManualSelectorGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbMessageLog" => {
                if let ClassParams::HkbMessageLog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbMirroredSkeletonInfo" => {
                if let ClassParams::HkbMirroredSkeletonInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbMirrorModifier" => {
                if let ClassParams::HkbMirrorModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbModifierGenerator" => {
                if let ClassParams::HkbModifierGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbModifierList" => {
                if let ClassParams::HkbModifierList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbModifierWrapper" => {
                if let ClassParams::HkbModifierWrapper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbModifier" => {
                if let ClassParams::HkbModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbMoveCharacterModifierInternalState" => {
                if let ClassParams::HkbMoveCharacterModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbMoveCharacterModifier" => {
                if let ClassParams::HkbMoveCharacterModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbNamedEventPayload" => {
                if let ClassParams::HkbNamedEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbNamedIntEventPayload" => {
                if let ClassParams::HkbNamedIntEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbNamedRealEventPayload" => {
                if let ClassParams::HkbNamedRealEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbNamedStringEventPayload" => {
                if let ClassParams::HkbNamedStringEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbNodeInternalStateInfo" => {
                if let ClassParams::HkbNodeInternalStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbNode" => {
                if let ClassParams::HkbNode(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbParticleSystemEventPayload" => {
                if let ClassParams::HkbParticleSystemEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbPoseMatchingGeneratorInternalState" => {
                if let ClassParams::HkbPoseMatchingGeneratorInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbPoseMatchingGenerator" => {
                if let ClassParams::HkbPoseMatchingGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbPoweredRagdollControlData" => {
                if let ClassParams::HkbPoweredRagdollControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbPoweredRagdollControlsModifier" => {
                if let ClassParams::HkbPoweredRagdollControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbProjectData" => {
                if let ClassParams::HkbProjectData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbProjectStringData" => {
                if let ClassParams::HkbProjectStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbProxyModifierProxyInfo" => {
                if let ClassParams::HkbProxyModifierProxyInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbProxyModifier" => {
                if let ClassParams::HkbProxyModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRaiseEventCommand" => {
                if let ClassParams::HkbRaiseEventCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRealEventPayload" => {
                if let ClassParams::HkbRealEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRealVariableSequencedDataSample" => {
                if let ClassParams::HkbRealVariableSequencedDataSample(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRealVariableSequencedData" => {
                if let ClassParams::HkbRealVariableSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbReferencePoseGenerator" => {
                if let ClassParams::HkbReferencePoseGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRegisteredGenerator" => {
                if let ClassParams::HkbRegisteredGenerator(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRigidBodyRagdollControlData" => {
                if let ClassParams::HkbRigidBodyRagdollControlData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRigidBodyRagdollControlsModifier" => {
                if let ClassParams::HkbRigidBodyRagdollControlsModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRoleAttribute" => {
                if let ClassParams::HkbRoleAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRotateCharacterModifierInternalState" => {
                if let ClassParams::HkbRotateCharacterModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbRotateCharacterModifier" => {
                if let ClassParams::HkbRotateCharacterModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSenseHandleModifierRange" => {
                if let ClassParams::HkbSenseHandleModifierRange(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSenseHandleModifier" => {
                if let ClassParams::HkbSenseHandleModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSequencedData" => {
                if let ClassParams::HkbSequencedData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSequenceInternalState" => {
                if let ClassParams::HkbSequenceInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSequenceStringData" => {
                if let ClassParams::HkbSequenceStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSequence" => {
                if let ClassParams::HkbSequence(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSetBehaviorCommand" => {
                if let ClassParams::HkbSetBehaviorCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSetLocalTimeOfClipGeneratorCommand" => {
                if let ClassParams::HkbSetLocalTimeOfClipGeneratorCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSetNodePropertyCommand" => {
                if let ClassParams::HkbSetNodePropertyCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSetWordVariableCommand" => {
                if let ClassParams::HkbSetWordVariableCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSetWorldFromModelModifier" => {
                if let ClassParams::HkbSetWorldFromModelModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSimulationControlCommand" => {
                if let ClassParams::HkbSimulationControlCommand(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbSimulationStateInfo" => {
                if let ClassParams::HkbSimulationStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateChooser" => {
                if let ClassParams::HkbStateChooser(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateListener" => {
                if let ClassParams::HkbStateListener(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineActiveTransitionInfo" => {
                if let ClassParams::HkbStateMachineActiveTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineDelayedTransitionInfo" => {
                if let ClassParams::HkbStateMachineDelayedTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineEventPropertyArray" => {
                if let ClassParams::HkbStateMachineEventPropertyArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineInternalState" => {
                if let ClassParams::HkbStateMachineInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineNestedStateMachineData" => {
                if let ClassParams::HkbStateMachineNestedStateMachineData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineProspectiveTransitionInfo" => {
                if let ClassParams::HkbStateMachineProspectiveTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineStateInfo" => {
                if let ClassParams::HkbStateMachineStateInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineTimeInterval" => {
                if let ClassParams::HkbStateMachineTimeInterval(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineTransitionInfoArray" => {
                if let ClassParams::HkbStateMachineTransitionInfoArray(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineTransitionInfoReference" => {
                if let ClassParams::HkbStateMachineTransitionInfoReference(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachineTransitionInfo" => {
                if let ClassParams::HkbStateMachineTransitionInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStateMachine" => {
                if let ClassParams::HkbStateMachine(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStringCondition" => {
                if let ClassParams::HkbStringCondition(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbStringEventPayload" => {
                if let ClassParams::HkbStringEventPayload(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTestStateChooser" => {
                if let ClassParams::HkbTestStateChooser(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTimerModifierInternalState" => {
                if let ClassParams::HkbTimerModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTimerModifier" => {
                if let ClassParams::HkbTimerModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTransformVectorModifierInternalState" => {
                if let ClassParams::HkbTransformVectorModifierInternalState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTransformVectorModifier" => {
                if let ClassParams::HkbTransformVectorModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTransitionEffect" => {
                if let ClassParams::HkbTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTwistModifier" => {
                if let ClassParams::HkbTwistModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbVariableBindingSetBinding" => {
                if let ClassParams::HkbVariableBindingSetBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbVariableBindingSet" => {
                if let ClassParams::HkbVariableBindingSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbVariableInfo" => {
                if let ClassParams::HkbVariableInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbVariableValueSet" => {
                if let ClassParams::HkbVariableValueSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbVariableValue" => {
                if let ClassParams::HkbVariableValue(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbWorldEnums" => {
                if let ClassParams::HkbWorldEnums(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbWorldFromModelModeData" => {
                if let ClassParams::HkbWorldFromModelModeData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkClassEnumItem" => {
                if let ClassParams::HkClassEnumItem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkClassEnum" => {
                if let ClassParams::HkClassEnum(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkClassMember" => {
                if let ClassParams::HkClassMember(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkClass" => {
                if let ClassParams::HkClass(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkColor" => {
                if let ClassParams::HkColor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkContactPointMaterial" => {
                if let ClassParams::HkContactPointMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkContactPoint" => {
                if let ClassParams::HkContactPoint(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkCustomAttributesAttribute" => {
                if let ClassParams::HkCustomAttributesAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkCustomAttributes" => {
                if let ClassParams::HkCustomAttributes(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkDataObjectTypeAttribute" => {
                if let ClassParams::HkDataObjectTypeAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkDescriptionAttribute" => {
                if let ClassParams::HkDescriptionAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkDocumentationAttribute" => {
                if let ClassParams::HkDocumentationAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkGeometryTriangle" => {
                if let ClassParams::HkGeometryTriangle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkGeometry" => {
                if let ClassParams::HkGeometry(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkGizmoAttribute" => {
                if let ClassParams::HkGizmoAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkHalf8" => {
                if let ClassParams::HkHalf8(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkIndexedTransformSet" => {
                if let ClassParams::HkIndexedTransformSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkLinkAttribute" => {
                if let ClassParams::HkLinkAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkLocalFrameGroup" => {
                if let ClassParams::HkLocalFrameGroup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkLocalFrame" => {
                if let ClassParams::HkLocalFrame(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryMeshBody" => {
                if let ClassParams::HkMemoryMeshBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryMeshMaterial" => {
                if let ClassParams::HkMemoryMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryMeshShape" => {
                if let ClassParams::HkMemoryMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryMeshTexture" => {
                if let ClassParams::HkMemoryMeshTexture(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryMeshVertexBuffer" => {
                if let ClassParams::HkMemoryMeshVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryResourceContainer" => {
                if let ClassParams::HkMemoryResourceContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryResourceHandleExternalLink" => {
                if let ClassParams::HkMemoryResourceHandleExternalLink(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryResourceHandle" => {
                if let ClassParams::HkMemoryResourceHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMemoryTrackerAttribute" => {
                if let ClassParams::HkMemoryTrackerAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshBody" => {
                if let ClassParams::HkMeshBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshBoneIndexMapping" => {
                if let ClassParams::HkMeshBoneIndexMapping(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshMaterial" => {
                if let ClassParams::HkMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshSectionCinfo" => {
                if let ClassParams::HkMeshSectionCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshSection" => {
                if let ClassParams::HkMeshSection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshShape" => {
                if let ClassParams::HkMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshTexture" => {
                if let ClassParams::HkMeshTexture(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMeshVertexBuffer" => {
                if let ClassParams::HkMeshVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkModelerNodeTypeAttribute" => {
                if let ClassParams::HkModelerNodeTypeAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMonitorStreamColorTableColorPair" => {
                if let ClassParams::HkMonitorStreamColorTableColorPair(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMonitorStreamColorTable" => {
                if let ClassParams::HkMonitorStreamColorTable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMonitorStreamFrameInfo" => {
                if let ClassParams::HkMonitorStreamFrameInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMonitorStreamStringMapStringMap" => {
                if let ClassParams::HkMonitorStreamStringMapStringMap(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMonitorStreamStringMap" => {
                if let ClassParams::HkMonitorStreamStringMap(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMoppBvTreeShapeBase" => {
                if let ClassParams::HkMoppBvTreeShapeBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMotionState" => {
                if let ClassParams::HkMotionState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMultipleVertexBufferElementInfo" => {
                if let ClassParams::HkMultipleVertexBufferElementInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMultipleVertexBufferLockedElement" => {
                if let ClassParams::HkMultipleVertexBufferLockedElement(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMultipleVertexBufferVertexBufferInfo" => {
                if let ClassParams::HkMultipleVertexBufferVertexBufferInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMultipleVertexBuffer" => {
                if let ClassParams::HkMultipleVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkMultiThreadCheck" => {
                if let ClassParams::HkMultiThreadCheck(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkp2dAngConstraintAtom" => {
                if let ClassParams::Hkp2DAngConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAabbPhantom" => {
                if let ClassParams::HkpAabbPhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkPackedVector3" => {
                if let ClassParams::HkPackedVector3(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkPackfileHeader" => {
                if let ClassParams::HkPackfileHeader(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkPackfileSectionHeader" => {
                if let ClassParams::HkPackfileSectionHeader(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAction" => {
                if let ClassParams::HkpAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAgent1nSector" => {
                if let ClassParams::HkpAgent1NSector(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAngConstraintAtom" => {
                if let ClassParams::HkpAngConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAngFrictionConstraintAtom" => {
                if let ClassParams::HkpAngFrictionConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAngLimitConstraintAtom" => {
                if let ClassParams::HkpAngLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAngMotorConstraintAtom" => {
                if let ClassParams::HkpAngMotorConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpAngularDashpotAction" => {
                if let ClassParams::HkpAngularDashpotAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpArrayAction" => {
                if let ClassParams::HkpArrayAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBallAndSocketConstraintDataAtoms" => {
                if let ClassParams::HkpBallAndSocketConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBallAndSocketConstraintData" => {
                if let ClassParams::HkpBallAndSocketConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBallGun" => {
                if let ClassParams::HkpBallGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBallSocketChainDataConstraintInfo" => {
                if let ClassParams::HkpBallSocketChainDataConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBallSocketChainData" => {
                if let ClassParams::HkpBallSocketChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBallSocketConstraintAtom" => {
                if let ClassParams::HkpBallSocketConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBinaryAction" => {
                if let ClassParams::HkpBinaryAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBoxMotion" => {
                if let ClassParams::HkpBoxMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBoxShape" => {
                if let ClassParams::HkpBoxShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBreakableBody" => {
                if let ClassParams::HkpBreakableBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBreakableConstraintData" => {
                if let ClassParams::HkpBreakableConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBridgeAtoms" => {
                if let ClassParams::HkpBridgeAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBridgeConstraintAtom" => {
                if let ClassParams::HkpBridgeConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBroadPhaseHandle" => {
                if let ClassParams::HkpBroadPhaseHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBvShape" => {
                if let ClassParams::HkpBvShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpBvTreeShape" => {
                if let ClassParams::HkpBvTreeShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCachingShapePhantom" => {
                if let ClassParams::HkpCachingShapePhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCallbackConstraintMotor" => {
                if let ClassParams::HkpCallbackConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCapsuleShape" => {
                if let ClassParams::HkpCapsuleShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCdBody" => {
                if let ClassParams::HkpCdBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCenterOfMassChangerModifierConstraintAtom" => {
                if let ClassParams::HkpCenterOfMassChangerModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCharacterControllerCinfo" => {
                if let ClassParams::HkpCharacterControllerCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCharacterMotion" => {
                if let ClassParams::HkpCharacterMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCharacterProxyCinfo" => {
                if let ClassParams::HkpCharacterProxyCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCharacterRigidBodyCinfo" => {
                if let ClassParams::HkpCharacterRigidBodyCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCogWheelConstraintAtom" => {
                if let ClassParams::HkpCogWheelConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCogWheelConstraintDataAtoms" => {
                if let ClassParams::HkpCogWheelConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCogWheelConstraintData" => {
                if let ClassParams::HkpCogWheelConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCollidableBoundingVolumeData" => {
                if let ClassParams::HkpCollidableBoundingVolumeData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCollidableCollidableFilter" => {
                if let ClassParams::HkpCollidableCollidableFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCollidable" => {
                if let ClassParams::HkpCollidable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCollisionFilterList" => {
                if let ClassParams::HkpCollisionFilterList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCollisionFilter" => {
                if let ClassParams::HkpCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCompressedMeshShapeBigTriangle" => {
                if let ClassParams::HkpCompressedMeshShapeBigTriangle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCompressedMeshShapeChunk" => {
                if let ClassParams::HkpCompressedMeshShapeChunk(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCompressedMeshShapeConvexPiece" => {
                if let ClassParams::HkpCompressedMeshShapeConvexPiece(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCompressedMeshShape" => {
                if let ClassParams::HkpCompressedMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCompressedSampledHeightFieldShape" => {
                if let ClassParams::HkpCompressedSampledHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConeLimitConstraintAtom" => {
                if let ClassParams::HkpConeLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstrainedSystemFilter" => {
                if let ClassParams::HkpConstrainedSystemFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintAtom" => {
                if let ClassParams::HkpConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintChainData" => {
                if let ClassParams::HkpConstraintChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintChainInstanceAction" => {
                if let ClassParams::HkpConstraintChainInstanceAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintChainInstance" => {
                if let ClassParams::HkpConstraintChainInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintCollisionFilter" => {
                if let ClassParams::HkpConstraintCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintData" => {
                if let ClassParams::HkpConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintInstanceSmallArraySerializeOverrideType" => {
                if let ClassParams::HkpConstraintInstanceSmallArraySerializeOverrideType(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintInstance" => {
                if let ClassParams::HkpConstraintInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConstraintMotor" => {
                if let ClassParams::HkpConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexListFilter" => {
                if let ClassParams::HkpConvexListFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexListShape" => {
                if let ClassParams::HkpConvexListShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexPieceMeshShape" => {
                if let ClassParams::HkpConvexPieceMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexPieceStreamData" => {
                if let ClassParams::HkpConvexPieceStreamData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexShape" => {
                if let ClassParams::HkpConvexShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexTransformShapeBase" => {
                if let ClassParams::HkpConvexTransformShapeBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexTransformShape" => {
                if let ClassParams::HkpConvexTransformShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexTranslateShape" => {
                if let ClassParams::HkpConvexTranslateShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexVerticesConnectivity" => {
                if let ClassParams::HkpConvexVerticesConnectivity(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexVerticesShapeFourVectors" => {
                if let ClassParams::HkpConvexVerticesShapeFourVectors(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpConvexVerticesShape" => {
                if let ClassParams::HkpConvexVerticesShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpCylinderShape" => {
                if let ClassParams::HkpCylinderShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDashpotAction" => {
                if let ClassParams::HkpDashpotAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDefaultConvexListFilter" => {
                if let ClassParams::HkpDefaultConvexListFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDefaultWorldMemoryWatchDog" => {
                if let ClassParams::HkpDefaultWorldMemoryWatchDog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDisableEntityCollisionFilter" => {
                if let ClassParams::HkpDisableEntityCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDisplayBindingDataPhysicsSystem" => {
                if let ClassParams::HkpDisplayBindingDataPhysicsSystem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDisplayBindingDataRigidBody" => {
                if let ClassParams::HkpDisplayBindingDataRigidBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpDisplayBindingData" => {
                if let ClassParams::HkpDisplayBindingData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpEntityExtendedListeners" => {
                if let ClassParams::HkpEntityExtendedListeners(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpEntitySmallArraySerializeOverrideType" => {
                if let ClassParams::HkpEntitySmallArraySerializeOverrideType(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpEntitySpuCollisionCallback" => {
                if let ClassParams::HkpEntitySpuCollisionCallback(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpEntity" => {
                if let ClassParams::HkpEntity(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpExtendedMeshShapeShapesSubpart" => {
                if let ClassParams::HkpExtendedMeshShapeShapesSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpExtendedMeshShapeSubpart" => {
                if let ClassParams::HkpExtendedMeshShapeSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpExtendedMeshShapeTrianglesSubpart" => {
                if let ClassParams::HkpExtendedMeshShapeTrianglesSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpExtendedMeshShape" => {
                if let ClassParams::HkpExtendedMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpFastMeshShape" => {
                if let ClassParams::HkpFastMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpFirstPersonGun" => {
                if let ClassParams::HkpFirstPersonGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpFixedRigidMotion" => {
                if let ClassParams::HkpFixedRigidMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpGenericConstraintDataSchemeConstraintInfo" => {
                if let ClassParams::HkpGenericConstraintDataSchemeConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpGenericConstraintDataScheme" => {
                if let ClassParams::HkpGenericConstraintDataScheme(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpGenericConstraintData" => {
                if let ClassParams::HkpGenericConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpGravityGun" => {
                if let ClassParams::HkpGravityGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpGroupCollisionFilter" => {
                if let ClassParams::HkpGroupCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpGroupFilter" => {
                if let ClassParams::HkpGroupFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpHeightFieldShape" => {
                if let ClassParams::HkpHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpHingeConstraintDataAtoms" => {
                if let ClassParams::HkpHingeConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpHingeConstraintData" => {
                if let ClassParams::HkpHingeConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpHingeLimitsDataAtoms" => {
                if let ClassParams::HkpHingeLimitsDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpHingeLimitsData" => {
                if let ClassParams::HkpHingeLimitsData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpIgnoreModifierConstraintAtom" => {
                if let ClassParams::HkpIgnoreModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpKeyframedRigidMotion" => {
                if let ClassParams::HkpKeyframedRigidMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLimitedForceConstraintMotor" => {
                if let ClassParams::HkpLimitedForceConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLimitedHingeConstraintDataAtoms" => {
                if let ClassParams::HkpLimitedHingeConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLimitedHingeConstraintData" => {
                if let ClassParams::HkpLimitedHingeConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinConstraintAtom" => {
                if let ClassParams::HkpLinConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinearParametricCurve" => {
                if let ClassParams::HkpLinearParametricCurve(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinFrictionConstraintAtom" => {
                if let ClassParams::HkpLinFrictionConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinkedCollidable" => {
                if let ClassParams::HkpLinkedCollidable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinLimitConstraintAtom" => {
                if let ClassParams::HkpLinLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinMotorConstraintAtom" => {
                if let ClassParams::HkpLinMotorConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpLinSoftConstraintAtom" => {
                if let ClassParams::HkpLinSoftConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpListShapeChildInfo" => {
                if let ClassParams::HkpListShapeChildInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpListShape" => {
                if let ClassParams::HkpListShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMalleableConstraintData" => {
                if let ClassParams::HkpMalleableConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMassChangerModifierConstraintAtom" => {
                if let ClassParams::HkpMassChangerModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMassProperties" => {
                if let ClassParams::HkpMassProperties(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMaterial" => {
                if let ClassParams::HkpMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMaxSizeMotion" => {
                if let ClassParams::HkpMaxSizeMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMeshMaterial" => {
                if let ClassParams::HkpMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMeshShapeSubpart" => {
                if let ClassParams::HkpMeshShapeSubpart(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMeshShape" => {
                if let ClassParams::HkpMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpModifierConstraintAtom" => {
                if let ClassParams::HkpModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMoppBvTreeShape" => {
                if let ClassParams::HkpMoppBvTreeShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMoppCodeCodeInfo" => {
                if let ClassParams::HkpMoppCodeCodeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMoppCodeReindexedTerminal" => {
                if let ClassParams::HkpMoppCodeReindexedTerminal(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMoppCode" => {
                if let ClassParams::HkpMoppCode(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMotion" => {
                if let ClassParams::HkpMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMotorAction" => {
                if let ClassParams::HkpMotorAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMountedBallGun" => {
                if let ClassParams::HkpMountedBallGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMouseSpringAction" => {
                if let ClassParams::HkpMouseSpringAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMovingSurfaceModifierConstraintAtom" => {
                if let ClassParams::HkpMovingSurfaceModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMultiRayShapeRay" => {
                if let ClassParams::HkpMultiRayShapeRay(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMultiRayShape" => {
                if let ClassParams::HkpMultiRayShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMultiSphereShape" => {
                if let ClassParams::HkpMultiSphereShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpMultithreadedVehicleManager" => {
                if let ClassParams::HkpMultithreadedVehicleManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpNamedMeshMaterial" => {
                if let ClassParams::HkpNamedMeshMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpNullCollisionFilter" => {
                if let ClassParams::HkpNullCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkPostFinishAttribute" => {
                if let ClassParams::HkPostFinishAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpOverwritePivotConstraintAtom" => {
                if let ClassParams::HkpOverwritePivotConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPairCollisionFilterMapPairFilterKeyOverrideType" => {
                if let ClassParams::HkpPairCollisionFilterMapPairFilterKeyOverrideType(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPairCollisionFilter" => {
                if let ClassParams::HkpPairCollisionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpParametricCurve" => {
                if let ClassParams::HkpParametricCurve(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPhantomCallbackShape" => {
                if let ClassParams::HkpPhantomCallbackShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPhantom" => {
                if let ClassParams::HkpPhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPhysicsData" => {
                if let ClassParams::HkpPhysicsData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPhysicsSystemWithContacts" => {
                if let ClassParams::HkpPhysicsSystemWithContacts(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPhysicsSystem" => {
                if let ClassParams::HkpPhysicsSystem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPlaneShape" => {
                if let ClassParams::HkpPlaneShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPointToPathConstraintData" => {
                if let ClassParams::HkpPointToPathConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPointToPlaneConstraintDataAtoms" => {
                if let ClassParams::HkpPointToPlaneConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPointToPlaneConstraintData" => {
                if let ClassParams::HkpPointToPlaneConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPositionConstraintMotor" => {
                if let ClassParams::HkpPositionConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPoweredChainDataConstraintInfo" => {
                if let ClassParams::HkpPoweredChainDataConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPoweredChainData" => {
                if let ClassParams::HkpPoweredChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPoweredChainMapperLinkInfo" => {
                if let ClassParams::HkpPoweredChainMapperLinkInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPoweredChainMapperTarget" => {
                if let ClassParams::HkpPoweredChainMapperTarget(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPoweredChainMapper" => {
                if let ClassParams::HkpPoweredChainMapper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPrismaticConstraintDataAtoms" => {
                if let ClassParams::HkpPrismaticConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPrismaticConstraintData" => {
                if let ClassParams::HkpPrismaticConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpProjectileGun" => {
                if let ClassParams::HkpProjectileGun(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPropertyValue" => {
                if let ClassParams::HkpPropertyValue(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpProperty" => {
                if let ClassParams::HkpProperty(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPulleyConstraintAtom" => {
                if let ClassParams::HkpPulleyConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPulleyConstraintDataAtoms" => {
                if let ClassParams::HkpPulleyConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpPulleyConstraintData" => {
                if let ClassParams::HkpPulleyConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRackAndPinionConstraintAtom" => {
                if let ClassParams::HkpRackAndPinionConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRackAndPinionConstraintDataAtoms" => {
                if let ClassParams::HkpRackAndPinionConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRackAndPinionConstraintData" => {
                if let ClassParams::HkpRackAndPinionConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRagdollConstraintDataAtoms" => {
                if let ClassParams::HkpRagdollConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRagdollConstraintData" => {
                if let ClassParams::HkpRagdollConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRagdollLimitsDataAtoms" => {
                if let ClassParams::HkpRagdollLimitsDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRagdollLimitsData" => {
                if let ClassParams::HkpRagdollLimitsData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRagdollMotorConstraintAtom" => {
                if let ClassParams::HkpRagdollMotorConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRayCollidableFilter" => {
                if let ClassParams::HkpRayCollidableFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRayShapeCollectionFilter" => {
                if let ClassParams::HkpRayShapeCollectionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRejectChassisListener" => {
                if let ClassParams::HkpRejectChassisListener(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRemoveTerminalsMoppModifier" => {
                if let ClassParams::HkpRemoveTerminalsMoppModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpReorientAction" => {
                if let ClassParams::HkpReorientAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRigidBody" => {
                if let ClassParams::HkpRigidBody(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRotationalConstraintDataAtoms" => {
                if let ClassParams::HkpRotationalConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpRotationalConstraintData" => {
                if let ClassParams::HkpRotationalConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSampledHeightFieldShape" => {
                if let ClassParams::HkpSampledHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedAgentNnEntry" => {
                if let ClassParams::HkpSerializedAgentNnEntry(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedDisplayMarkerList" => {
                if let ClassParams::HkpSerializedDisplayMarkerList(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedDisplayMarker" => {
                if let ClassParams::HkpSerializedDisplayMarker(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedDisplayRbTransformsDisplayTransformPair" => {
                if let ClassParams::HkpSerializedDisplayRbTransformsDisplayTransformPair(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedDisplayRbTransforms" => {
                if let ClassParams::HkpSerializedDisplayRbTransforms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedSubTrack1nInfo" => {
                if let ClassParams::HkpSerializedSubTrack1NInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSerializedTrack1nInfo" => {
                if let ClassParams::HkpSerializedTrack1NInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSetLocalRotationsConstraintAtom" => {
                if let ClassParams::HkpSetLocalRotationsConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSetLocalTransformsConstraintAtom" => {
                if let ClassParams::HkpSetLocalTransformsConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSetLocalTranslationsConstraintAtom" => {
                if let ClassParams::HkpSetLocalTranslationsConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSetupStabilizationAtom" => {
                if let ClassParams::HkpSetupStabilizationAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShapeCollectionFilter" => {
                if let ClassParams::HkpShapeCollectionFilter(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShapeCollection" => {
                if let ClassParams::HkpShapeCollection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShapeContainer" => {
                if let ClassParams::HkpShapeContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShapeInfo" => {
                if let ClassParams::HkpShapeInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShapeModifier" => {
                if let ClassParams::HkpShapeModifier(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShapePhantom" => {
                if let ClassParams::HkpShapePhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpShape" => {
                if let ClassParams::HkpShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimpleContactConstraintAtom" => {
                if let ClassParams::HkpSimpleContactConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimpleContactConstraintDataInfo" => {
                if let ClassParams::HkpSimpleContactConstraintDataInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimpleMeshShapeTriangle" => {
                if let ClassParams::HkpSimpleMeshShapeTriangle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimpleMeshShape" => {
                if let ClassParams::HkpSimpleMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimpleShapePhantomCollisionDetail" => {
                if let ClassParams::HkpSimpleShapePhantomCollisionDetail(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimpleShapePhantom" => {
                if let ClassParams::HkpSimpleShapePhantom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSimulation" => {
                if let ClassParams::HkpSimulation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSingleShapeContainer" => {
                if let ClassParams::HkpSingleShapeContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSoftContactModifierConstraintAtom" => {
                if let ClassParams::HkpSoftContactModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSphereMotion" => {
                if let ClassParams::HkpSphereMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSphereRepShape" => {
                if let ClassParams::HkpSphereRepShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSphereShape" => {
                if let ClassParams::HkpSphereShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSpringAction" => {
                if let ClassParams::HkpSpringAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpSpringDamperConstraintMotor" => {
                if let ClassParams::HkpSpringDamperConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStiffSpringChainDataConstraintInfo" => {
                if let ClassParams::HkpStiffSpringChainDataConstraintInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStiffSpringChainData" => {
                if let ClassParams::HkpStiffSpringChainData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStiffSpringConstraintAtom" => {
                if let ClassParams::HkpStiffSpringConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStiffSpringConstraintDataAtoms" => {
                if let ClassParams::HkpStiffSpringConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStiffSpringConstraintData" => {
                if let ClassParams::HkpStiffSpringConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageExtendedMeshShapeMaterial" => {
                if let ClassParams::HkpStorageExtendedMeshShapeMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageExtendedMeshShapeMeshSubpartStorage" => {
                if let ClassParams::HkpStorageExtendedMeshShapeMeshSubpartStorage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageExtendedMeshShapeShapeSubpartStorage" => {
                if let ClassParams::HkpStorageExtendedMeshShapeShapeSubpartStorage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageExtendedMeshShape" => {
                if let ClassParams::HkpStorageExtendedMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageMeshShapeSubpartStorage" => {
                if let ClassParams::HkpStorageMeshShapeSubpartStorage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageMeshShape" => {
                if let ClassParams::HkpStorageMeshShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpStorageSampledHeightFieldShape" => {
                if let ClassParams::HkpStorageSampledHeightFieldShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpThinBoxMotion" => {
                if let ClassParams::HkpThinBoxMotion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTransformShape" => {
                if let ClassParams::HkpTransformShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTriangleShape" => {
                if let ClassParams::HkpTriangleShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTriggerVolumeEventInfo" => {
                if let ClassParams::HkpTriggerVolumeEventInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTriggerVolume" => {
                if let ClassParams::HkpTriggerVolume(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTriSampledHeightFieldBvTreeShape" => {
                if let ClassParams::HkpTriSampledHeightFieldBvTreeShape(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTriSampledHeightFieldCollection" => {
                if let ClassParams::HkpTriSampledHeightFieldCollection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTwistLimitConstraintAtom" => {
                if let ClassParams::HkpTwistLimitConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTypedBroadPhaseHandle" => {
                if let ClassParams::HkpTypedBroadPhaseHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTyremarkPoint" => {
                if let ClassParams::HkpTyremarkPoint(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTyremarksInfo" => {
                if let ClassParams::HkpTyremarksInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpTyremarksWheel" => {
                if let ClassParams::HkpTyremarksWheel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpUnaryAction" => {
                if let ClassParams::HkpUnaryAction(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleAerodynamics" => {
                if let ClassParams::HkpVehicleAerodynamics(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleBrake" => {
                if let ClassParams::HkpVehicleBrake(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleCastBatchingManager" => {
                if let ClassParams::HkpVehicleCastBatchingManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDataWheelComponentParams" => {
                if let ClassParams::HkpVehicleDataWheelComponentParams(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleData" => {
                if let ClassParams::HkpVehicleData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultAerodynamics" => {
                if let ClassParams::HkpVehicleDefaultAerodynamics(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultAnalogDriverInput" => {
                if let ClassParams::HkpVehicleDefaultAnalogDriverInput(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultBrakeWheelBrakingProperties" => {
                if let ClassParams::HkpVehicleDefaultBrakeWheelBrakingProperties(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultBrake" => {
                if let ClassParams::HkpVehicleDefaultBrake(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultEngine" => {
                if let ClassParams::HkpVehicleDefaultEngine(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultSteering" => {
                if let ClassParams::HkpVehicleDefaultSteering(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters" => {
                if let ClassParams::HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultSuspension" => {
                if let ClassParams::HkpVehicleDefaultSuspension(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultTransmission" => {
                if let ClassParams::HkpVehicleDefaultTransmission(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDefaultVelocityDamper" => {
                if let ClassParams::HkpVehicleDefaultVelocityDamper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDriverInputAnalogStatus" => {
                if let ClassParams::HkpVehicleDriverInputAnalogStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDriverInputStatus" => {
                if let ClassParams::HkpVehicleDriverInputStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleDriverInput" => {
                if let ClassParams::HkpVehicleDriverInput(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleEngine" => {
                if let ClassParams::HkpVehicleEngine(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleFrictionDescriptionAxisDescription" => {
                if let ClassParams::HkpVehicleFrictionDescriptionAxisDescription(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleFrictionDescription" => {
                if let ClassParams::HkpVehicleFrictionDescription(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleFrictionStatusAxisStatus" => {
                if let ClassParams::HkpVehicleFrictionStatusAxisStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleFrictionStatus" => {
                if let ClassParams::HkpVehicleFrictionStatus(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleInstanceWheelInfo" => {
                if let ClassParams::HkpVehicleInstanceWheelInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleInstance" => {
                if let ClassParams::HkpVehicleInstance(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleLinearCastBatchingManager" => {
                if let ClassParams::HkpVehicleLinearCastBatchingManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleLinearCastWheelCollideWheelState" => {
                if let ClassParams::HkpVehicleLinearCastWheelCollideWheelState(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleLinearCastWheelCollide" => {
                if let ClassParams::HkpVehicleLinearCastWheelCollide(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleManager" => {
                if let ClassParams::HkpVehicleManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleRayCastBatchingManager" => {
                if let ClassParams::HkpVehicleRayCastBatchingManager(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleRayCastWheelCollide" => {
                if let ClassParams::HkpVehicleRayCastWheelCollide(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleSteering" => {
                if let ClassParams::HkpVehicleSteering(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleSuspensionSuspensionWheelParameters" => {
                if let ClassParams::HkpVehicleSuspensionSuspensionWheelParameters(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleSuspension" => {
                if let ClassParams::HkpVehicleSuspension(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleTransmission" => {
                if let ClassParams::HkpVehicleTransmission(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleVelocityDamper" => {
                if let ClassParams::HkpVehicleVelocityDamper(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVehicleWheelCollide" => {
                if let ClassParams::HkpVehicleWheelCollide(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpVelocityConstraintMotor" => {
                if let ClassParams::HkpVelocityConstraintMotor(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpViscousSurfaceModifierConstraintAtom" => {
                if let ClassParams::HkpViscousSurfaceModifierConstraintAtom(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpWeldingUtility" => {
                if let ClassParams::HkpWeldingUtility(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpWheelConstraintDataAtoms" => {
                if let ClassParams::HkpWheelConstraintDataAtoms(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpWheelConstraintData" => {
                if let ClassParams::HkpWheelConstraintData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpWorldCinfo" => {
                if let ClassParams::HkpWorldCinfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpWorldObject" => {
                if let ClassParams::HkpWorldObject(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkpWorld" => {
                if let ClassParams::HkpWorld(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkQTransform" => {
                if let ClassParams::HkQTransform(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkRangeInt32Attribute" => {
                if let ClassParams::HkRangeInt32Attribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkRangeRealAttribute" => {
                if let ClassParams::HkRangeRealAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkReferencedObject" => {
                if let ClassParams::HkReferencedObject(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkReflectedFileAttribute" => {
                if let ClassParams::HkReflectedFileAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkResourceBase" => {
                if let ClassParams::HkResourceBase(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkResourceContainer" => {
                if let ClassParams::HkResourceContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkResourceHandle" => {
                if let ClassParams::HkResourceHandle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkRootLevelContainerNamedVariant" => {
                if let ClassParams::HkRootLevelContainerNamedVariant(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkRootLevelContainer" => {
                if let ClassParams::HkRootLevelContainer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkSemanticsAttribute" => {
                if let ClassParams::HkSemanticsAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkSimpleLocalFrame" => {
                if let ClassParams::HkSimpleLocalFrame(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkSphere" => {
                if let ClassParams::HkSphere(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkSweptTransform" => {
                if let ClassParams::HkSweptTransform(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkTraceStreamTitle" => {
                if let ClassParams::HkTraceStreamTitle(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkTrackerSerializableScanSnapshotAllocation" => {
                if let ClassParams::HkTrackerSerializableScanSnapshotAllocation(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkTrackerSerializableScanSnapshotBlock" => {
                if let ClassParams::HkTrackerSerializableScanSnapshotBlock(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkTrackerSerializableScanSnapshot" => {
                if let ClassParams::HkTrackerSerializableScanSnapshot(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkUiAttribute" => {
                if let ClassParams::HkUiAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkVertexFormatElement" => {
                if let ClassParams::HkVertexFormatElement(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkVertexFormat" => {
                if let ClassParams::HkVertexFormat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkWorldMemoryAvailableWatchDog" => {
                if let ClassParams::HkWorldMemoryAvailableWatchDog(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAnimatedFloat" => {
                if let ClassParams::HkxAnimatedFloat(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAnimatedMatrix" => {
                if let ClassParams::HkxAnimatedMatrix(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAnimatedQuaternion" => {
                if let ClassParams::HkxAnimatedQuaternion(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAnimatedVector" => {
                if let ClassParams::HkxAnimatedVector(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAttributeGroup" => {
                if let ClassParams::HkxAttributeGroup(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAttributeHolder" => {
                if let ClassParams::HkxAttributeHolder(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxAttribute" => {
                if let ClassParams::HkxAttribute(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxCamera" => {
                if let ClassParams::HkxCamera(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxEdgeSelectionChannel" => {
                if let ClassParams::HkxEdgeSelectionChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxEnumItem" => {
                if let ClassParams::HkxEnumItem(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxEnum" => {
                if let ClassParams::HkxEnum(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxEnvironmentVariable" => {
                if let ClassParams::HkxEnvironmentVariable(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxEnvironment" => {
                if let ClassParams::HkxEnvironment(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxIndexBuffer" => {
                if let ClassParams::HkxIndexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxLight" => {
                if let ClassParams::HkxLight(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMaterialEffect" => {
                if let ClassParams::HkxMaterialEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMaterialProperty" => {
                if let ClassParams::HkxMaterialProperty(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMaterialShaderSet" => {
                if let ClassParams::HkxMaterialShaderSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMaterialShader" => {
                if let ClassParams::HkxMaterialShader(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMaterialTextureStage" => {
                if let ClassParams::HkxMaterialTextureStage(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMaterial" => {
                if let ClassParams::HkxMaterial(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMeshSection" => {
                if let ClassParams::HkxMeshSection(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMeshUserChannelInfo" => {
                if let ClassParams::HkxMeshUserChannelInfo(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxMesh" => {
                if let ClassParams::HkxMesh(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxNodeAnnotationData" => {
                if let ClassParams::HkxNodeAnnotationData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxNodeSelectionSet" => {
                if let ClassParams::HkxNodeSelectionSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxNode" => {
                if let ClassParams::HkxNode(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxScene" => {
                if let ClassParams::HkxScene(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxSkinBinding" => {
                if let ClassParams::HkxSkinBinding(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxSparselyAnimatedBool" => {
                if let ClassParams::HkxSparselyAnimatedBool(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxSparselyAnimatedEnum" => {
                if let ClassParams::HkxSparselyAnimatedEnum(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxSparselyAnimatedInt" => {
                if let ClassParams::HkxSparselyAnimatedInt(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxSparselyAnimatedString" => {
                if let ClassParams::HkxSparselyAnimatedString(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxTextureFile" => {
                if let ClassParams::HkxTextureFile(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxTextureInplace" => {
                if let ClassParams::HkxTextureInplace(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxTriangleSelectionChannel" => {
                if let ClassParams::HkxTriangleSelectionChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexBufferVertexData" => {
                if let ClassParams::HkxVertexBufferVertexData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexBuffer" => {
                if let ClassParams::HkxVertexBuffer(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexDescriptionElementDecl" => {
                if let ClassParams::HkxVertexDescriptionElementDecl(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexDescription" => {
                if let ClassParams::HkxVertexDescription(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexFloatDataChannel" => {
                if let ClassParams::HkxVertexFloatDataChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexIntDataChannel" => {
                if let ClassParams::HkxVertexIntDataChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexSelectionChannel" => {
                if let ClassParams::HkxVertexSelectionChannel(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkxVertexVectorDataChannel" => {
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
                            if let Some(ref class_name) = class {
                                hkparam = Some(Ok(match class_name.as_ref() {
                                    "BGSGamebryoSequenceGenerator" => {
                                                        ClassParams::BgsGamebryoSequenceGenerator(map.next_value()?)
                                    },
                                    "BSBoneSwitchGeneratorBoneData" => {
                                                        ClassParams::BsBoneSwitchGeneratorBoneData(map.next_value()?)
                                    },
                                    "BSBoneSwitchGenerator" => {
                                                        ClassParams::BsBoneSwitchGenerator(map.next_value()?)
                                    },
                                    "BSComputeAddBoneAnimModifier" => {
                                                        ClassParams::BsComputeAddBoneAnimModifier(map.next_value()?)
                                    },
                                    "BSCyclicBlendTransitionGenerator" => {
                                                        ClassParams::BsCyclicBlendTransitionGenerator(map.next_value()?)
                                    },
                                    "BSDecomposeVectorModifier" => {
                                                        ClassParams::BsDecomposeVectorModifier(map.next_value()?)
                                    },
                                    "BSDirectAtModifier" => {
                                                        ClassParams::BsDirectAtModifier(map.next_value()?)
                                    },
                                    "BSDistTriggerModifier" => {
                                                        ClassParams::BsDistTriggerModifier(map.next_value()?)
                                    },
                                    "BSEventEveryNEventsModifier" => {
                                                        ClassParams::BsEventEveryNEventsModifier(map.next_value()?)
                                    },
                                    "BSEventOnDeactivateModifier" => {
                                                        ClassParams::BsEventOnDeactivateModifier(map.next_value()?)
                                    },
                                    "BSEventOnFalseToTrueModifier" => {
                                                        ClassParams::BsEventOnFalseToTrueModifier(map.next_value()?)
                                    },
                                    "BSGetTimeStepModifier" => {
                                                        ClassParams::BsGetTimeStepModifier(map.next_value()?)
                                    },
                                    "BSInterpValueModifier" => {
                                                        ClassParams::BsInterpValueModifier(map.next_value()?)
                                    },
                                    "BSIsActiveModifier" => {
                                                        ClassParams::BsIsActiveModifier(map.next_value()?)
                                    },
                                    "BSIStateManagerModifierBSiStateData" => {
                                                        ClassParams::BsiStateManagerModifierBSiStateData(map.next_value()?)
                                    },
                                    "BSIStateManagerModifierBSIStateManagerStateListener" => {
                                                        ClassParams::BsiStateManagerModifierBsiStateManagerStateListener(map.next_value()?)
                                    },
                                    "BSIStateManagerModifier" => {
                                                        ClassParams::BsiStateManagerModifier(map.next_value()?)
                                    },
                                    "BSiStateTaggingGenerator" => {
                                                        ClassParams::BSiStateTaggingGenerator(map.next_value()?)
                                    },
                                    "BSLimbIKModifier" => {
                                                        ClassParams::BsLimbIkModifier(map.next_value()?)
                                    },
                                    "BSLookAtModifierBoneData" => {
                                                        ClassParams::BsLookAtModifierBoneData(map.next_value()?)
                                    },
                                    "BSLookAtModifier" => {
                                                        ClassParams::BsLookAtModifier(map.next_value()?)
                                    },
                                    "BSModifyOnceModifier" => {
                                                        ClassParams::BsModifyOnceModifier(map.next_value()?)
                                    },
                                    "BSOffsetAnimationGenerator" => {
                                                        ClassParams::BsOffsetAnimationGenerator(map.next_value()?)
                                    },
                                    "BSPassByTargetTriggerModifier" => {
                                                        ClassParams::BsPassByTargetTriggerModifier(map.next_value()?)
                                    },
                                    "BSRagdollContactListenerModifier" => {
                                                        ClassParams::BsRagdollContactListenerModifier(map.next_value()?)
                                    },
                                    "BSSpeedSamplerModifier" => {
                                                        ClassParams::BsSpeedSamplerModifier(map.next_value()?)
                                    },
                                    "BSSynchronizedClipGenerator" => {
                                                        ClassParams::BsSynchronizedClipGenerator(map.next_value()?)
                                    },
                                    "BSTimerModifier" => {
                                                        ClassParams::BsTimerModifier(map.next_value()?)
                                    },
                                    "BSTweenerModifier" => {
                                                        ClassParams::BsTweenerModifier(map.next_value()?)
                                    },
                                    "hkAabbHalf" => {
                                                        ClassParams::HkAabbHalf(map.next_value()?)
                                    },
                                    "hkAabbUint32" => {
                                                        ClassParams::HkAabbUint32(map.next_value()?)
                                    },
                                    "hkAabb" => {
                                                        ClassParams::HkAabb(map.next_value()?)
                                    },
                                    "hkaAnimatedReferenceFrame" => {
                                                        ClassParams::HkaAnimatedReferenceFrame(map.next_value()?)
                                    },
                                    "hkaAnimationBinding" => {
                                                        ClassParams::HkaAnimationBinding(map.next_value()?)
                                    },
                                    "hkaAnimationContainer" => {
                                                        ClassParams::HkaAnimationContainer(map.next_value()?)
                                    },
                                    "hkaAnimationPreviewColorContainer" => {
                                                        ClassParams::HkaAnimationPreviewColorContainer(map.next_value()?)
                                    },
                                    "hkaAnimation" => {
                                                        ClassParams::HkaAnimation(map.next_value()?)
                                    },
                                    "hkaAnnotationTrackAnnotation" => {
                                                        ClassParams::HkaAnnotationTrackAnnotation(map.next_value()?)
                                    },
                                    "hkaAnnotationTrack" => {
                                                        ClassParams::HkaAnnotationTrack(map.next_value()?)
                                    },
                                    "hkaBoneAttachment" => {
                                                        ClassParams::HkaBoneAttachment(map.next_value()?)
                                    },
                                    "hkaBone" => {
                                                        ClassParams::HkaBone(map.next_value()?)
                                    },
                                    "hkaDefaultAnimatedReferenceFrame" => {
                                                        ClassParams::HkaDefaultAnimatedReferenceFrame(map.next_value()?)
                                    },
                                    "hkaDeltaCompressedAnimationQuantizationFormat" => {
                                                        ClassParams::HkaDeltaCompressedAnimationQuantizationFormat(map.next_value()?)
                                    },
                                    "hkaDeltaCompressedAnimation" => {
                                                        ClassParams::HkaDeltaCompressedAnimation(map.next_value()?)
                                    },
                                    "hkaFootstepAnalysisInfoContainer" => {
                                                        ClassParams::HkaFootstepAnalysisInfoContainer(map.next_value()?)
                                    },
                                    "hkaFootstepAnalysisInfo" => {
                                                        ClassParams::HkaFootstepAnalysisInfo(map.next_value()?)
                                    },
                                    "hkaInterleavedUncompressedAnimation" => {
                                                        ClassParams::HkaInterleavedUncompressedAnimation(map.next_value()?)
                                    },
                                    "hkaKeyFrameHierarchyUtilityControlData" => {
                                                        ClassParams::HkaKeyFrameHierarchyUtilityControlData(map.next_value()?)
                                    },
                                    "hkaKeyFrameHierarchyUtility" => {
                                                        ClassParams::HkaKeyFrameHierarchyUtility(map.next_value()?)
                                    },
                                    "hkAlignSceneToNodeOptions" => {
                                                        ClassParams::HkAlignSceneToNodeOptions(map.next_value()?)
                                    },
                                    "hkaMeshBindingMapping" => {
                                                        ClassParams::HkaMeshBindingMapping(map.next_value()?)
                                    },
                                    "hkaMeshBinding" => {
                                                        ClassParams::HkaMeshBinding(map.next_value()?)
                                    },
                                    "hkaQuantizedAnimationTrackCompressionParams" => {
                                                        ClassParams::HkaQuantizedAnimationTrackCompressionParams(map.next_value()?)
                                    },
                                    "hkaQuantizedAnimation" => {
                                                        ClassParams::HkaQuantizedAnimation(map.next_value()?)
                                    },
                                    "hkaRagdollInstance" => {
                                                        ClassParams::HkaRagdollInstance(map.next_value()?)
                                    },
                                    "hkArrayTypeAttribute" => {
                                                        ClassParams::HkArrayTypeAttribute(map.next_value()?)
                                    },
                                    "hkaSkeletonLocalFrameOnBone" => {
                                                        ClassParams::HkaSkeletonLocalFrameOnBone(map.next_value()?)
                                    },
                                    "hkaSkeletonMapperDataChainMapping" => {
                                                        ClassParams::HkaSkeletonMapperDataChainMapping(map.next_value()?)
                                    },
                                    "hkaSkeletonMapperDataSimpleMapping" => {
                                                        ClassParams::HkaSkeletonMapperDataSimpleMapping(map.next_value()?)
                                    },
                                    "hkaSkeletonMapperData" => {
                                                        ClassParams::HkaSkeletonMapperData(map.next_value()?)
                                    },
                                    "hkaSkeletonMapper" => {
                                                        ClassParams::HkaSkeletonMapper(map.next_value()?)
                                    },
                                    "hkaSkeleton" => {
                                                        ClassParams::HkaSkeleton(map.next_value()?)
                                    },
                                    "hkaSplineCompressedAnimationAnimationCompressionParams" => {
                                                        ClassParams::HkaSplineCompressedAnimationAnimationCompressionParams(map.next_value()?)
                                    },
                                    "hkaSplineCompressedAnimationTrackCompressionParams" => {
                                                        ClassParams::HkaSplineCompressedAnimationTrackCompressionParams(map.next_value()?)
                                    },
                                    "hkaSplineCompressedAnimation" => {
                                                        ClassParams::HkaSplineCompressedAnimation(map.next_value()?)
                                    },
                                    "hkaWaveletCompressedAnimationCompressionParams" => {
                                                        ClassParams::HkaWaveletCompressedAnimationCompressionParams(map.next_value()?)
                                    },
                                    "hkaWaveletCompressedAnimationQuantizationFormat" => {
                                                        ClassParams::HkaWaveletCompressedAnimationQuantizationFormat(map.next_value()?)
                                    },
                                    "hkaWaveletCompressedAnimation" => {
                                                        ClassParams::HkaWaveletCompressedAnimation(map.next_value()?)
                                    },
                                    "hkBaseObject" => {
                                                        ClassParams::HkBaseObject(map.next_value()?)
                                    },
                                    "hkbAttachmentModifier" => {
                                                        ClassParams::HkbAttachmentModifier(map.next_value()?)
                                    },
                                    "hkbAttachmentSetup" => {
                                                        ClassParams::HkbAttachmentSetup(map.next_value()?)
                                    },
                                    "hkbAttributeModifierAssignment" => {
                                                        ClassParams::HkbAttributeModifierAssignment(map.next_value()?)
                                    },
                                    "hkbAttributeModifier" => {
                                                        ClassParams::HkbAttributeModifier(map.next_value()?)
                                    },
                                    "hkbAuxiliaryNodeInfo" => {
                                                        ClassParams::HkbAuxiliaryNodeInfo(map.next_value()?)
                                    },
                                    "hkbBehaviorEventsInfo" => {
                                                        ClassParams::HkbBehaviorEventsInfo(map.next_value()?)
                                    },
                                    "hkbBehaviorGraphData" => {
                                                        ClassParams::HkbBehaviorGraphData(map.next_value()?)
                                    },
                                    "hkbBehaviorGraphInternalStateInfo" => {
                                                        ClassParams::HkbBehaviorGraphInternalStateInfo(map.next_value()?)
                                    },
                                    "hkbBehaviorGraphInternalState" => {
                                                        ClassParams::HkbBehaviorGraphInternalState(map.next_value()?)
                                    },
                                    "hkbBehaviorGraphStringData" => {
                                                        ClassParams::HkbBehaviorGraphStringData(map.next_value()?)
                                    },
                                    "hkbBehaviorGraph" => {
                                                        ClassParams::HkbBehaviorGraph(map.next_value()?)
                                    },
                                    "hkbBehaviorInfoIdToNamePair" => {
                                                        ClassParams::HkbBehaviorInfoIdToNamePair(map.next_value()?)
                                    },
                                    "hkbBehaviorInfo" => {
                                                        ClassParams::HkbBehaviorInfo(map.next_value()?)
                                    },
                                    "hkbBehaviorReferenceGenerator" => {
                                                        ClassParams::HkbBehaviorReferenceGenerator(map.next_value()?)
                                    },
                                    "hkbBindable" => {
                                                        ClassParams::HkbBindable(map.next_value()?)
                                    },
                                    "hkbBlendCurveUtils" => {
                                                        ClassParams::HkbBlendCurveUtils(map.next_value()?)
                                    },
                                    "hkbBlenderGeneratorChildInternalState" => {
                                                        ClassParams::HkbBlenderGeneratorChildInternalState(map.next_value()?)
                                    },
                                    "hkbBlenderGeneratorChild" => {
                                                        ClassParams::HkbBlenderGeneratorChild(map.next_value()?)
                                    },
                                    "hkbBlenderGeneratorInternalState" => {
                                                        ClassParams::HkbBlenderGeneratorInternalState(map.next_value()?)
                                    },
                                    "hkbBlenderGenerator" => {
                                                        ClassParams::HkbBlenderGenerator(map.next_value()?)
                                    },
                                    "hkbBlendingTransitionEffectInternalState" => {
                                                        ClassParams::HkbBlendingTransitionEffectInternalState(map.next_value()?)
                                    },
                                    "hkbBlendingTransitionEffect" => {
                                                        ClassParams::HkbBlendingTransitionEffect(map.next_value()?)
                                    },
                                    "hkbBoneIndexArray" => {
                                                        ClassParams::HkbBoneIndexArray(map.next_value()?)
                                    },
                                    "hkbBoneWeightArray" => {
                                                        ClassParams::HkbBoneWeightArray(map.next_value()?)
                                    },
                                    "hkbBoolVariableSequencedDataSample" => {
                                                        ClassParams::HkbBoolVariableSequencedDataSample(map.next_value()?)
                                    },
                                    "hkbBoolVariableSequencedData" => {
                                                        ClassParams::HkbBoolVariableSequencedData(map.next_value()?)
                                    },
                                    "hkbCameraShakeEventPayload" => {
                                                        ClassParams::HkbCameraShakeEventPayload(map.next_value()?)
                                    },
                                    "hkbCharacterAddedInfo" => {
                                                        ClassParams::HkbCharacterAddedInfo(map.next_value()?)
                                    },
                                    "hkbCharacterControlCommand" => {
                                                        ClassParams::HkbCharacterControlCommand(map.next_value()?)
                                    },
                                    "hkbCharacterControllerControlData" => {
                                                        ClassParams::HkbCharacterControllerControlData(map.next_value()?)
                                    },
                                    "hkbCharacterControllerModifierInternalState" => {
                                                        ClassParams::HkbCharacterControllerModifierInternalState(map.next_value()?)
                                    },
                                    "hkbCharacterControllerModifier" => {
                                                        ClassParams::HkbCharacterControllerModifier(map.next_value()?)
                                    },
                                    "hkbCharacterDataCharacterControllerInfo" => {
                                                        ClassParams::HkbCharacterDataCharacterControllerInfo(map.next_value()?)
                                    },
                                    "hkbCharacterData" => {
                                                        ClassParams::HkbCharacterData(map.next_value()?)
                                    },
                                    "hkbCharacterInfo" => {
                                                        ClassParams::HkbCharacterInfo(map.next_value()?)
                                    },
                                    "hkbCharacterSetup" => {
                                                        ClassParams::HkbCharacterSetup(map.next_value()?)
                                    },
                                    "hkbCharacterSkinInfo" => {
                                                        ClassParams::HkbCharacterSkinInfo(map.next_value()?)
                                    },
                                    "hkbCharacterSteppedInfo" => {
                                                        ClassParams::HkbCharacterSteppedInfo(map.next_value()?)
                                    },
                                    "hkbCharacterStringData" => {
                                                        ClassParams::HkbCharacterStringData(map.next_value()?)
                                    },
                                    "hkbCharacter" => {
                                                        ClassParams::HkbCharacter(map.next_value()?)
                                    },
                                    "hkbClientCharacterState" => {
                                                        ClassParams::HkbClientCharacterState(map.next_value()?)
                                    },
                                    "hkbClipGeneratorEcho" => {
                                                        ClassParams::HkbClipGeneratorEcho(map.next_value()?)
                                    },
                                    "hkbClipGeneratorInternalState" => {
                                                        ClassParams::HkbClipGeneratorInternalState(map.next_value()?)
                                    },
                                    "hkbClipGenerator" => {
                                                        ClassParams::HkbClipGenerator(map.next_value()?)
                                    },
                                    "hkbClipTriggerArray" => {
                                                        ClassParams::HkbClipTriggerArray(map.next_value()?)
                                    },
                                    "hkbClipTrigger" => {
                                                        ClassParams::HkbClipTrigger(map.next_value()?)
                                    },
                                    "hkbCombineTransformsModifierInternalState" => {
                                                        ClassParams::HkbCombineTransformsModifierInternalState(map.next_value()?)
                                    },
                                    "hkbCombineTransformsModifier" => {
                                                        ClassParams::HkbCombineTransformsModifier(map.next_value()?)
                                    },
                                    "hkbCompiledExpressionSetToken" => {
                                                        ClassParams::HkbCompiledExpressionSetToken(map.next_value()?)
                                    },
                                    "hkbCompiledExpressionSet" => {
                                                        ClassParams::HkbCompiledExpressionSet(map.next_value()?)
                                    },
                                    "hkbComputeDirectionModifierInternalState" => {
                                                        ClassParams::HkbComputeDirectionModifierInternalState(map.next_value()?)
                                    },
                                    "hkbComputeDirectionModifier" => {
                                                        ClassParams::HkbComputeDirectionModifier(map.next_value()?)
                                    },
                                    "hkbComputeRotationFromAxisAngleModifierInternalState" => {
                                                        ClassParams::HkbComputeRotationFromAxisAngleModifierInternalState(map.next_value()?)
                                    },
                                    "hkbComputeRotationFromAxisAngleModifier" => {
                                                        ClassParams::HkbComputeRotationFromAxisAngleModifier(map.next_value()?)
                                    },
                                    "hkbComputeRotationToTargetModifierInternalState" => {
                                                        ClassParams::HkbComputeRotationToTargetModifierInternalState(map.next_value()?)
                                    },
                                    "hkbComputeRotationToTargetModifier" => {
                                                        ClassParams::HkbComputeRotationToTargetModifier(map.next_value()?)
                                    },
                                    "hkbCondition" => {
                                                        ClassParams::HkbCondition(map.next_value()?)
                                    },
                                    "hkbContext" => {
                                                        ClassParams::HkbContext(map.next_value()?)
                                    },
                                    "hkbDampingModifierInternalState" => {
                                                        ClassParams::HkbDampingModifierInternalState(map.next_value()?)
                                    },
                                    "hkbDampingModifier" => {
                                                        ClassParams::HkbDampingModifier(map.next_value()?)
                                    },
                                    "hkbDefaultMessageLog" => {
                                                        ClassParams::HkbDefaultMessageLog(map.next_value()?)
                                    },
                                    "hkbDelayedModifierInternalState" => {
                                                        ClassParams::HkbDelayedModifierInternalState(map.next_value()?)
                                    },
                                    "hkbDelayedModifier" => {
                                                        ClassParams::HkbDelayedModifier(map.next_value()?)
                                    },
                                    "hkbDetectCloseToGroundModifierInternalState" => {
                                                        ClassParams::HkbDetectCloseToGroundModifierInternalState(map.next_value()?)
                                    },
                                    "hkbDetectCloseToGroundModifier" => {
                                                        ClassParams::HkbDetectCloseToGroundModifier(map.next_value()?)
                                    },
                                    "hkbEvaluateExpressionModifierInternalExpressionData" => {
                                                        ClassParams::HkbEvaluateExpressionModifierInternalExpressionData(map.next_value()?)
                                    },
                                    "hkbEvaluateExpressionModifierInternalState" => {
                                                        ClassParams::HkbEvaluateExpressionModifierInternalState(map.next_value()?)
                                    },
                                    "hkbEvaluateExpressionModifier" => {
                                                        ClassParams::HkbEvaluateExpressionModifier(map.next_value()?)
                                    },
                                    "hkbEvaluateHandleModifier" => {
                                                        ClassParams::HkbEvaluateHandleModifier(map.next_value()?)
                                    },
                                    "hkbEventBase" => {
                                                        ClassParams::HkbEventBase(map.next_value()?)
                                    },
                                    "hkbEventDrivenModifierInternalState" => {
                                                        ClassParams::HkbEventDrivenModifierInternalState(map.next_value()?)
                                    },
                                    "hkbEventDrivenModifier" => {
                                                        ClassParams::HkbEventDrivenModifier(map.next_value()?)
                                    },
                                    "hkbEventInfo" => {
                                                        ClassParams::HkbEventInfo(map.next_value()?)
                                    },
                                    "hkbEventPayloadList" => {
                                                        ClassParams::HkbEventPayloadList(map.next_value()?)
                                    },
                                    "hkbEventPayload" => {
                                                        ClassParams::HkbEventPayload(map.next_value()?)
                                    },
                                    "hkbEventProperty" => {
                                                        ClassParams::HkbEventProperty(map.next_value()?)
                                    },
                                    "hkbEventRaisedInfo" => {
                                                        ClassParams::HkbEventRaisedInfo(map.next_value()?)
                                    },
                                    "hkbEventRangeDataArray" => {
                                                        ClassParams::HkbEventRangeDataArray(map.next_value()?)
                                    },
                                    "hkbEventRangeData" => {
                                                        ClassParams::HkbEventRangeData(map.next_value()?)
                                    },
                                    "hkbEventSequencedDataSequencedEvent" => {
                                                        ClassParams::HkbEventSequencedDataSequencedEvent(map.next_value()?)
                                    },
                                    "hkbEventSequencedData" => {
                                                        ClassParams::HkbEventSequencedData(map.next_value()?)
                                    },
                                    "hkbEventsFromRangeModifierInternalState" => {
                                                        ClassParams::HkbEventsFromRangeModifierInternalState(map.next_value()?)
                                    },
                                    "hkbEventsFromRangeModifier" => {
                                                        ClassParams::HkbEventsFromRangeModifier(map.next_value()?)
                                    },
                                    "hkbEvent" => {
                                                        ClassParams::HkbEvent(map.next_value()?)
                                    },
                                    "hkbExpressionCondition" => {
                                                        ClassParams::HkbExpressionCondition(map.next_value()?)
                                    },
                                    "hkbExpressionDataArray" => {
                                                        ClassParams::HkbExpressionDataArray(map.next_value()?)
                                    },
                                    "hkbExpressionData" => {
                                                        ClassParams::HkbExpressionData(map.next_value()?)
                                    },
                                    "hkbExtractRagdollPoseModifier" => {
                                                        ClassParams::HkbExtractRagdollPoseModifier(map.next_value()?)
                                    },
                                    "hkbFootIkControlData" => {
                                                        ClassParams::HkbFootIkControlData(map.next_value()?)
                                    },
                                    "hkbFootIkControlsModifierLeg" => {
                                                        ClassParams::HkbFootIkControlsModifierLeg(map.next_value()?)
                                    },
                                    "hkbFootIkControlsModifier" => {
                                                        ClassParams::HkbFootIkControlsModifier(map.next_value()?)
                                    },
                                    "hkbFootIkDriverInfoLeg" => {
                                                        ClassParams::HkbFootIkDriverInfoLeg(map.next_value()?)
                                    },
                                    "hkbFootIkDriverInfo" => {
                                                        ClassParams::HkbFootIkDriverInfo(map.next_value()?)
                                    },
                                    "hkbFootIkGains" => {
                                                        ClassParams::HkbFootIkGains(map.next_value()?)
                                    },
                                    "hkbFootIkModifierInternalLegData" => {
                                                        ClassParams::HkbFootIkModifierInternalLegData(map.next_value()?)
                                    },
                                    "hkbFootIkModifierLeg" => {
                                                        ClassParams::HkbFootIkModifierLeg(map.next_value()?)
                                    },
                                    "hkbFootIkModifier" => {
                                                        ClassParams::HkbFootIkModifier(map.next_value()?)
                                    },
                                    "hkbGeneratorOutputListener" => {
                                                        ClassParams::HkbGeneratorOutputListener(map.next_value()?)
                                    },
                                    "hkbGeneratorSyncInfoSyncPoint" => {
                                                        ClassParams::HkbGeneratorSyncInfoSyncPoint(map.next_value()?)
                                    },
                                    "hkbGeneratorSyncInfo" => {
                                                        ClassParams::HkbGeneratorSyncInfo(map.next_value()?)
                                    },
                                    "hkbGeneratorTransitionEffectInternalState" => {
                                                        ClassParams::HkbGeneratorTransitionEffectInternalState(map.next_value()?)
                                    },
                                    "hkbGeneratorTransitionEffect" => {
                                                        ClassParams::HkbGeneratorTransitionEffect(map.next_value()?)
                                    },
                                    "hkbGenerator" => {
                                                        ClassParams::HkbGenerator(map.next_value()?)
                                    },
                                    "hkbGetHandleOnBoneModifier" => {
                                                        ClassParams::HkbGetHandleOnBoneModifier(map.next_value()?)
                                    },
                                    "hkbGetUpModifierInternalState" => {
                                                        ClassParams::HkbGetUpModifierInternalState(map.next_value()?)
                                    },
                                    "hkbGetUpModifier" => {
                                                        ClassParams::HkbGetUpModifier(map.next_value()?)
                                    },
                                    "hkbGetWorldFromModelModifierInternalState" => {
                                                        ClassParams::HkbGetWorldFromModelModifierInternalState(map.next_value()?)
                                    },
                                    "hkbGetWorldFromModelModifier" => {
                                                        ClassParams::HkbGetWorldFromModelModifier(map.next_value()?)
                                    },
                                    "hkbHandIkControlData" => {
                                                        ClassParams::HkbHandIkControlData(map.next_value()?)
                                    },
                                    "hkbHandIkControlsModifierHand" => {
                                                        ClassParams::HkbHandIkControlsModifierHand(map.next_value()?)
                                    },
                                    "hkbHandIkControlsModifier" => {
                                                        ClassParams::HkbHandIkControlsModifier(map.next_value()?)
                                    },
                                    "hkbHandIkDriverInfoHand" => {
                                                        ClassParams::HkbHandIkDriverInfoHand(map.next_value()?)
                                    },
                                    "hkbHandIkDriverInfo" => {
                                                        ClassParams::HkbHandIkDriverInfo(map.next_value()?)
                                    },
                                    "hkbHandIkModifierHand" => {
                                                        ClassParams::HkbHandIkModifierHand(map.next_value()?)
                                    },
                                    "hkbHandIkModifier" => {
                                                        ClassParams::HkbHandIkModifier(map.next_value()?)
                                    },
                                    "hkbHandle" => {
                                                        ClassParams::HkbHandle(map.next_value()?)
                                    },
                                    "hkbIntEventPayload" => {
                                                        ClassParams::HkbIntEventPayload(map.next_value()?)
                                    },
                                    "hkbIntVariableSequencedDataSample" => {
                                                        ClassParams::HkbIntVariableSequencedDataSample(map.next_value()?)
                                    },
                                    "hkbIntVariableSequencedData" => {
                                                        ClassParams::HkbIntVariableSequencedData(map.next_value()?)
                                    },
                                    "hkBitField" => {
                                                        ClassParams::HkBitField(map.next_value()?)
                                    },
                                    "hkbKeyframeBonesModifierKeyframeInfo" => {
                                                        ClassParams::HkbKeyframeBonesModifierKeyframeInfo(map.next_value()?)
                                    },
                                    "hkbKeyframeBonesModifier" => {
                                                        ClassParams::HkbKeyframeBonesModifier(map.next_value()?)
                                    },
                                    "hkbLinkedSymbolInfo" => {
                                                        ClassParams::HkbLinkedSymbolInfo(map.next_value()?)
                                    },
                                    "hkbLookAtModifierInternalState" => {
                                                        ClassParams::HkbLookAtModifierInternalState(map.next_value()?)
                                    },
                                    "hkbLookAtModifier" => {
                                                        ClassParams::HkbLookAtModifier(map.next_value()?)
                                    },
                                    "hkbManualSelectorGeneratorInternalState" => {
                                                        ClassParams::HkbManualSelectorGeneratorInternalState(map.next_value()?)
                                    },
                                    "hkbManualSelectorGenerator" => {
                                                        ClassParams::HkbManualSelectorGenerator(map.next_value()?)
                                    },
                                    "hkbMessageLog" => {
                                                        ClassParams::HkbMessageLog(map.next_value()?)
                                    },
                                    "hkbMirroredSkeletonInfo" => {
                                                        ClassParams::HkbMirroredSkeletonInfo(map.next_value()?)
                                    },
                                    "hkbMirrorModifier" => {
                                                        ClassParams::HkbMirrorModifier(map.next_value()?)
                                    },
                                    "hkbModifierGenerator" => {
                                                        ClassParams::HkbModifierGenerator(map.next_value()?)
                                    },
                                    "hkbModifierList" => {
                                                        ClassParams::HkbModifierList(map.next_value()?)
                                    },
                                    "hkbModifierWrapper" => {
                                                        ClassParams::HkbModifierWrapper(map.next_value()?)
                                    },
                                    "hkbModifier" => {
                                                        ClassParams::HkbModifier(map.next_value()?)
                                    },
                                    "hkbMoveCharacterModifierInternalState" => {
                                                        ClassParams::HkbMoveCharacterModifierInternalState(map.next_value()?)
                                    },
                                    "hkbMoveCharacterModifier" => {
                                                        ClassParams::HkbMoveCharacterModifier(map.next_value()?)
                                    },
                                    "hkbNamedEventPayload" => {
                                                        ClassParams::HkbNamedEventPayload(map.next_value()?)
                                    },
                                    "hkbNamedIntEventPayload" => {
                                                        ClassParams::HkbNamedIntEventPayload(map.next_value()?)
                                    },
                                    "hkbNamedRealEventPayload" => {
                                                        ClassParams::HkbNamedRealEventPayload(map.next_value()?)
                                    },
                                    "hkbNamedStringEventPayload" => {
                                                        ClassParams::HkbNamedStringEventPayload(map.next_value()?)
                                    },
                                    "hkbNodeInternalStateInfo" => {
                                                        ClassParams::HkbNodeInternalStateInfo(map.next_value()?)
                                    },
                                    "hkbNode" => {
                                                        ClassParams::HkbNode(map.next_value()?)
                                    },
                                    "hkbParticleSystemEventPayload" => {
                                                        ClassParams::HkbParticleSystemEventPayload(map.next_value()?)
                                    },
                                    "hkbPoseMatchingGeneratorInternalState" => {
                                                        ClassParams::HkbPoseMatchingGeneratorInternalState(map.next_value()?)
                                    },
                                    "hkbPoseMatchingGenerator" => {
                                                        ClassParams::HkbPoseMatchingGenerator(map.next_value()?)
                                    },
                                    "hkbPoweredRagdollControlData" => {
                                                        ClassParams::HkbPoweredRagdollControlData(map.next_value()?)
                                    },
                                    "hkbPoweredRagdollControlsModifier" => {
                                                        ClassParams::HkbPoweredRagdollControlsModifier(map.next_value()?)
                                    },
                                    "hkbProjectData" => {
                                                        ClassParams::HkbProjectData(map.next_value()?)
                                    },
                                    "hkbProjectStringData" => {
                                                        ClassParams::HkbProjectStringData(map.next_value()?)
                                    },
                                    "hkbProxyModifierProxyInfo" => {
                                                        ClassParams::HkbProxyModifierProxyInfo(map.next_value()?)
                                    },
                                    "hkbProxyModifier" => {
                                                        ClassParams::HkbProxyModifier(map.next_value()?)
                                    },
                                    "hkbRaiseEventCommand" => {
                                                        ClassParams::HkbRaiseEventCommand(map.next_value()?)
                                    },
                                    "hkbRealEventPayload" => {
                                                        ClassParams::HkbRealEventPayload(map.next_value()?)
                                    },
                                    "hkbRealVariableSequencedDataSample" => {
                                                        ClassParams::HkbRealVariableSequencedDataSample(map.next_value()?)
                                    },
                                    "hkbRealVariableSequencedData" => {
                                                        ClassParams::HkbRealVariableSequencedData(map.next_value()?)
                                    },
                                    "hkbReferencePoseGenerator" => {
                                                        ClassParams::HkbReferencePoseGenerator(map.next_value()?)
                                    },
                                    "hkbRegisteredGenerator" => {
                                                        ClassParams::HkbRegisteredGenerator(map.next_value()?)
                                    },
                                    "hkbRigidBodyRagdollControlData" => {
                                                        ClassParams::HkbRigidBodyRagdollControlData(map.next_value()?)
                                    },
                                    "hkbRigidBodyRagdollControlsModifier" => {
                                                        ClassParams::HkbRigidBodyRagdollControlsModifier(map.next_value()?)
                                    },
                                    "hkbRoleAttribute" => {
                                                        ClassParams::HkbRoleAttribute(map.next_value()?)
                                    },
                                    "hkbRotateCharacterModifierInternalState" => {
                                                        ClassParams::HkbRotateCharacterModifierInternalState(map.next_value()?)
                                    },
                                    "hkbRotateCharacterModifier" => {
                                                        ClassParams::HkbRotateCharacterModifier(map.next_value()?)
                                    },
                                    "hkbSenseHandleModifierRange" => {
                                                        ClassParams::HkbSenseHandleModifierRange(map.next_value()?)
                                    },
                                    "hkbSenseHandleModifier" => {
                                                        ClassParams::HkbSenseHandleModifier(map.next_value()?)
                                    },
                                    "hkbSequencedData" => {
                                                        ClassParams::HkbSequencedData(map.next_value()?)
                                    },
                                    "hkbSequenceInternalState" => {
                                                        ClassParams::HkbSequenceInternalState(map.next_value()?)
                                    },
                                    "hkbSequenceStringData" => {
                                                        ClassParams::HkbSequenceStringData(map.next_value()?)
                                    },
                                    "hkbSequence" => {
                                                        ClassParams::HkbSequence(map.next_value()?)
                                    },
                                    "hkbSetBehaviorCommand" => {
                                                        ClassParams::HkbSetBehaviorCommand(map.next_value()?)
                                    },
                                    "hkbSetLocalTimeOfClipGeneratorCommand" => {
                                                        ClassParams::HkbSetLocalTimeOfClipGeneratorCommand(map.next_value()?)
                                    },
                                    "hkbSetNodePropertyCommand" => {
                                                        ClassParams::HkbSetNodePropertyCommand(map.next_value()?)
                                    },
                                    "hkbSetWordVariableCommand" => {
                                                        ClassParams::HkbSetWordVariableCommand(map.next_value()?)
                                    },
                                    "hkbSetWorldFromModelModifier" => {
                                                        ClassParams::HkbSetWorldFromModelModifier(map.next_value()?)
                                    },
                                    "hkbSimulationControlCommand" => {
                                                        ClassParams::HkbSimulationControlCommand(map.next_value()?)
                                    },
                                    "hkbSimulationStateInfo" => {
                                                        ClassParams::HkbSimulationStateInfo(map.next_value()?)
                                    },
                                    "hkbStateChooser" => {
                                                        ClassParams::HkbStateChooser(map.next_value()?)
                                    },
                                    "hkbStateListener" => {
                                                        ClassParams::HkbStateListener(map.next_value()?)
                                    },
                                    "hkbStateMachineActiveTransitionInfo" => {
                                                        ClassParams::HkbStateMachineActiveTransitionInfo(map.next_value()?)
                                    },
                                    "hkbStateMachineDelayedTransitionInfo" => {
                                                        ClassParams::HkbStateMachineDelayedTransitionInfo(map.next_value()?)
                                    },
                                    "hkbStateMachineEventPropertyArray" => {
                                                        ClassParams::HkbStateMachineEventPropertyArray(map.next_value()?)
                                    },
                                    "hkbStateMachineInternalState" => {
                                                        ClassParams::HkbStateMachineInternalState(map.next_value()?)
                                    },
                                    "hkbStateMachineNestedStateMachineData" => {
                                                        ClassParams::HkbStateMachineNestedStateMachineData(map.next_value()?)
                                    },
                                    "hkbStateMachineProspectiveTransitionInfo" => {
                                                        ClassParams::HkbStateMachineProspectiveTransitionInfo(map.next_value()?)
                                    },
                                    "hkbStateMachineStateInfo" => {
                                                        ClassParams::HkbStateMachineStateInfo(map.next_value()?)
                                    },
                                    "hkbStateMachineTimeInterval" => {
                                                        ClassParams::HkbStateMachineTimeInterval(map.next_value()?)
                                    },
                                    "hkbStateMachineTransitionInfoArray" => {
                                                        ClassParams::HkbStateMachineTransitionInfoArray(map.next_value()?)
                                    },
                                    "hkbStateMachineTransitionInfoReference" => {
                                                        ClassParams::HkbStateMachineTransitionInfoReference(map.next_value()?)
                                    },
                                    "hkbStateMachineTransitionInfo" => {
                                                        ClassParams::HkbStateMachineTransitionInfo(map.next_value()?)
                                    },
                                    "hkbStateMachine" => {
                                                        ClassParams::HkbStateMachine(map.next_value()?)
                                    },
                                    "hkbStringCondition" => {
                                                        ClassParams::HkbStringCondition(map.next_value()?)
                                    },
                                    "hkbStringEventPayload" => {
                                                        ClassParams::HkbStringEventPayload(map.next_value()?)
                                    },
                                    "hkbTestStateChooser" => {
                                                        ClassParams::HkbTestStateChooser(map.next_value()?)
                                    },
                                    "hkbTimerModifierInternalState" => {
                                                        ClassParams::HkbTimerModifierInternalState(map.next_value()?)
                                    },
                                    "hkbTimerModifier" => {
                                                        ClassParams::HkbTimerModifier(map.next_value()?)
                                    },
                                    "hkbTransformVectorModifierInternalState" => {
                                                        ClassParams::HkbTransformVectorModifierInternalState(map.next_value()?)
                                    },
                                    "hkbTransformVectorModifier" => {
                                                        ClassParams::HkbTransformVectorModifier(map.next_value()?)
                                    },
                                    "hkbTransitionEffect" => {
                                                        ClassParams::HkbTransitionEffect(map.next_value()?)
                                    },
                                    "hkbTwistModifier" => {
                                                        ClassParams::HkbTwistModifier(map.next_value()?)
                                    },
                                    "hkbVariableBindingSetBinding" => {
                                                        ClassParams::HkbVariableBindingSetBinding(map.next_value()?)
                                    },
                                    "hkbVariableBindingSet" => {
                                                        ClassParams::HkbVariableBindingSet(map.next_value()?)
                                    },
                                    "hkbVariableInfo" => {
                                                        ClassParams::HkbVariableInfo(map.next_value()?)
                                    },
                                    "hkbVariableValueSet" => {
                                                        ClassParams::HkbVariableValueSet(map.next_value()?)
                                    },
                                    "hkbVariableValue" => {
                                                        ClassParams::HkbVariableValue(map.next_value()?)
                                    },
                                    "hkbWorldEnums" => {
                                                        ClassParams::HkbWorldEnums(map.next_value()?)
                                    },
                                    "hkbWorldFromModelModeData" => {
                                                        ClassParams::HkbWorldFromModelModeData(map.next_value()?)
                                    },
                                    "hkClassEnumItem" => {
                                                        ClassParams::HkClassEnumItem(map.next_value()?)
                                    },
                                    "hkClassEnum" => {
                                                        ClassParams::HkClassEnum(map.next_value()?)
                                    },
                                    "hkClassMember" => {
                                                        ClassParams::HkClassMember(map.next_value()?)
                                    },
                                    "hkClass" => {
                                                        ClassParams::HkClass(map.next_value()?)
                                    },
                                    "hkColor" => {
                                                        ClassParams::HkColor(map.next_value()?)
                                    },
                                    "hkContactPointMaterial" => {
                                                        ClassParams::HkContactPointMaterial(map.next_value()?)
                                    },
                                    "hkContactPoint" => {
                                                        ClassParams::HkContactPoint(map.next_value()?)
                                    },
                                    "hkCustomAttributesAttribute" => {
                                                        ClassParams::HkCustomAttributesAttribute(map.next_value()?)
                                    },
                                    "hkCustomAttributes" => {
                                                        ClassParams::HkCustomAttributes(map.next_value()?)
                                    },
                                    "hkDataObjectTypeAttribute" => {
                                                        ClassParams::HkDataObjectTypeAttribute(map.next_value()?)
                                    },
                                    "hkDescriptionAttribute" => {
                                                        ClassParams::HkDescriptionAttribute(map.next_value()?)
                                    },
                                    "hkDocumentationAttribute" => {
                                                        ClassParams::HkDocumentationAttribute(map.next_value()?)
                                    },
                                    "hkGeometryTriangle" => {
                                                        ClassParams::HkGeometryTriangle(map.next_value()?)
                                    },
                                    "hkGeometry" => {
                                                        ClassParams::HkGeometry(map.next_value()?)
                                    },
                                    "hkGizmoAttribute" => {
                                                        ClassParams::HkGizmoAttribute(map.next_value()?)
                                    },
                                    "hkHalf8" => {
                                                        ClassParams::HkHalf8(map.next_value()?)
                                    },
                                    "hkIndexedTransformSet" => {
                                                        ClassParams::HkIndexedTransformSet(map.next_value()?)
                                    },
                                    "hkLinkAttribute" => {
                                                        ClassParams::HkLinkAttribute(map.next_value()?)
                                    },
                                    "hkLocalFrameGroup" => {
                                                        ClassParams::HkLocalFrameGroup(map.next_value()?)
                                    },
                                    "hkLocalFrame" => {
                                                        ClassParams::HkLocalFrame(map.next_value()?)
                                    },
                                    "hkMemoryMeshBody" => {
                                                        ClassParams::HkMemoryMeshBody(map.next_value()?)
                                    },
                                    "hkMemoryMeshMaterial" => {
                                                        ClassParams::HkMemoryMeshMaterial(map.next_value()?)
                                    },
                                    "hkMemoryMeshShape" => {
                                                        ClassParams::HkMemoryMeshShape(map.next_value()?)
                                    },
                                    "hkMemoryMeshTexture" => {
                                                        ClassParams::HkMemoryMeshTexture(map.next_value()?)
                                    },
                                    "hkMemoryMeshVertexBuffer" => {
                                                        ClassParams::HkMemoryMeshVertexBuffer(map.next_value()?)
                                    },
                                    "hkMemoryResourceContainer" => {
                                                        ClassParams::HkMemoryResourceContainer(map.next_value()?)
                                    },
                                    "hkMemoryResourceHandleExternalLink" => {
                                                        ClassParams::HkMemoryResourceHandleExternalLink(map.next_value()?)
                                    },
                                    "hkMemoryResourceHandle" => {
                                                        ClassParams::HkMemoryResourceHandle(map.next_value()?)
                                    },
                                    "hkMemoryTrackerAttribute" => {
                                                        ClassParams::HkMemoryTrackerAttribute(map.next_value()?)
                                    },
                                    "hkMeshBody" => {
                                                        ClassParams::HkMeshBody(map.next_value()?)
                                    },
                                    "hkMeshBoneIndexMapping" => {
                                                        ClassParams::HkMeshBoneIndexMapping(map.next_value()?)
                                    },
                                    "hkMeshMaterial" => {
                                                        ClassParams::HkMeshMaterial(map.next_value()?)
                                    },
                                    "hkMeshSectionCinfo" => {
                                                        ClassParams::HkMeshSectionCinfo(map.next_value()?)
                                    },
                                    "hkMeshSection" => {
                                                        ClassParams::HkMeshSection(map.next_value()?)
                                    },
                                    "hkMeshShape" => {
                                                        ClassParams::HkMeshShape(map.next_value()?)
                                    },
                                    "hkMeshTexture" => {
                                                        ClassParams::HkMeshTexture(map.next_value()?)
                                    },
                                    "hkMeshVertexBuffer" => {
                                                        ClassParams::HkMeshVertexBuffer(map.next_value()?)
                                    },
                                    "hkModelerNodeTypeAttribute" => {
                                                        ClassParams::HkModelerNodeTypeAttribute(map.next_value()?)
                                    },
                                    "hkMonitorStreamColorTableColorPair" => {
                                                        ClassParams::HkMonitorStreamColorTableColorPair(map.next_value()?)
                                    },
                                    "hkMonitorStreamColorTable" => {
                                                        ClassParams::HkMonitorStreamColorTable(map.next_value()?)
                                    },
                                    "hkMonitorStreamFrameInfo" => {
                                                        ClassParams::HkMonitorStreamFrameInfo(map.next_value()?)
                                    },
                                    "hkMonitorStreamStringMapStringMap" => {
                                                        ClassParams::HkMonitorStreamStringMapStringMap(map.next_value()?)
                                    },
                                    "hkMonitorStreamStringMap" => {
                                                        ClassParams::HkMonitorStreamStringMap(map.next_value()?)
                                    },
                                    "hkMoppBvTreeShapeBase" => {
                                                        ClassParams::HkMoppBvTreeShapeBase(map.next_value()?)
                                    },
                                    "hkMotionState" => {
                                                        ClassParams::HkMotionState(map.next_value()?)
                                    },
                                    "hkMultipleVertexBufferElementInfo" => {
                                                        ClassParams::HkMultipleVertexBufferElementInfo(map.next_value()?)
                                    },
                                    "hkMultipleVertexBufferLockedElement" => {
                                                        ClassParams::HkMultipleVertexBufferLockedElement(map.next_value()?)
                                    },
                                    "hkMultipleVertexBufferVertexBufferInfo" => {
                                                        ClassParams::HkMultipleVertexBufferVertexBufferInfo(map.next_value()?)
                                    },
                                    "hkMultipleVertexBuffer" => {
                                                        ClassParams::HkMultipleVertexBuffer(map.next_value()?)
                                    },
                                    "hkMultiThreadCheck" => {
                                                        ClassParams::HkMultiThreadCheck(map.next_value()?)
                                    },
                                    "hkp2dAngConstraintAtom" => {
                                                        ClassParams::Hkp2DAngConstraintAtom(map.next_value()?)
                                    },
                                    "hkpAabbPhantom" => {
                                                        ClassParams::HkpAabbPhantom(map.next_value()?)
                                    },
                                    "hkPackedVector3" => {
                                                        ClassParams::HkPackedVector3(map.next_value()?)
                                    },
                                    "hkPackfileHeader" => {
                                                        ClassParams::HkPackfileHeader(map.next_value()?)
                                    },
                                    "hkPackfileSectionHeader" => {
                                                        ClassParams::HkPackfileSectionHeader(map.next_value()?)
                                    },
                                    "hkpAction" => {
                                                        ClassParams::HkpAction(map.next_value()?)
                                    },
                                    "hkpAgent1nSector" => {
                                                        ClassParams::HkpAgent1NSector(map.next_value()?)
                                    },
                                    "hkpAngConstraintAtom" => {
                                                        ClassParams::HkpAngConstraintAtom(map.next_value()?)
                                    },
                                    "hkpAngFrictionConstraintAtom" => {
                                                        ClassParams::HkpAngFrictionConstraintAtom(map.next_value()?)
                                    },
                                    "hkpAngLimitConstraintAtom" => {
                                                        ClassParams::HkpAngLimitConstraintAtom(map.next_value()?)
                                    },
                                    "hkpAngMotorConstraintAtom" => {
                                                        ClassParams::HkpAngMotorConstraintAtom(map.next_value()?)
                                    },
                                    "hkpAngularDashpotAction" => {
                                                        ClassParams::HkpAngularDashpotAction(map.next_value()?)
                                    },
                                    "hkpArrayAction" => {
                                                        ClassParams::HkpArrayAction(map.next_value()?)
                                    },
                                    "hkpBallAndSocketConstraintDataAtoms" => {
                                                        ClassParams::HkpBallAndSocketConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpBallAndSocketConstraintData" => {
                                                        ClassParams::HkpBallAndSocketConstraintData(map.next_value()?)
                                    },
                                    "hkpBallGun" => {
                                                        ClassParams::HkpBallGun(map.next_value()?)
                                    },
                                    "hkpBallSocketChainDataConstraintInfo" => {
                                                        ClassParams::HkpBallSocketChainDataConstraintInfo(map.next_value()?)
                                    },
                                    "hkpBallSocketChainData" => {
                                                        ClassParams::HkpBallSocketChainData(map.next_value()?)
                                    },
                                    "hkpBallSocketConstraintAtom" => {
                                                        ClassParams::HkpBallSocketConstraintAtom(map.next_value()?)
                                    },
                                    "hkpBinaryAction" => {
                                                        ClassParams::HkpBinaryAction(map.next_value()?)
                                    },
                                    "hkpBoxMotion" => {
                                                        ClassParams::HkpBoxMotion(map.next_value()?)
                                    },
                                    "hkpBoxShape" => {
                                                        ClassParams::HkpBoxShape(map.next_value()?)
                                    },
                                    "hkpBreakableBody" => {
                                                        ClassParams::HkpBreakableBody(map.next_value()?)
                                    },
                                    "hkpBreakableConstraintData" => {
                                                        ClassParams::HkpBreakableConstraintData(map.next_value()?)
                                    },
                                    "hkpBridgeAtoms" => {
                                                        ClassParams::HkpBridgeAtoms(map.next_value()?)
                                    },
                                    "hkpBridgeConstraintAtom" => {
                                                        ClassParams::HkpBridgeConstraintAtom(map.next_value()?)
                                    },
                                    "hkpBroadPhaseHandle" => {
                                                        ClassParams::HkpBroadPhaseHandle(map.next_value()?)
                                    },
                                    "hkpBvShape" => {
                                                        ClassParams::HkpBvShape(map.next_value()?)
                                    },
                                    "hkpBvTreeShape" => {
                                                        ClassParams::HkpBvTreeShape(map.next_value()?)
                                    },
                                    "hkpCachingShapePhantom" => {
                                                        ClassParams::HkpCachingShapePhantom(map.next_value()?)
                                    },
                                    "hkpCallbackConstraintMotor" => {
                                                        ClassParams::HkpCallbackConstraintMotor(map.next_value()?)
                                    },
                                    "hkpCapsuleShape" => {
                                                        ClassParams::HkpCapsuleShape(map.next_value()?)
                                    },
                                    "hkpCdBody" => {
                                                        ClassParams::HkpCdBody(map.next_value()?)
                                    },
                                    "hkpCenterOfMassChangerModifierConstraintAtom" => {
                                                        ClassParams::HkpCenterOfMassChangerModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpCharacterControllerCinfo" => {
                                                        ClassParams::HkpCharacterControllerCinfo(map.next_value()?)
                                    },
                                    "hkpCharacterMotion" => {
                                                        ClassParams::HkpCharacterMotion(map.next_value()?)
                                    },
                                    "hkpCharacterProxyCinfo" => {
                                                        ClassParams::HkpCharacterProxyCinfo(map.next_value()?)
                                    },
                                    "hkpCharacterRigidBodyCinfo" => {
                                                        ClassParams::HkpCharacterRigidBodyCinfo(map.next_value()?)
                                    },
                                    "hkpCogWheelConstraintAtom" => {
                                                        ClassParams::HkpCogWheelConstraintAtom(map.next_value()?)
                                    },
                                    "hkpCogWheelConstraintDataAtoms" => {
                                                        ClassParams::HkpCogWheelConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpCogWheelConstraintData" => {
                                                        ClassParams::HkpCogWheelConstraintData(map.next_value()?)
                                    },
                                    "hkpCollidableBoundingVolumeData" => {
                                                        ClassParams::HkpCollidableBoundingVolumeData(map.next_value()?)
                                    },
                                    "hkpCollidableCollidableFilter" => {
                                                        ClassParams::HkpCollidableCollidableFilter(map.next_value()?)
                                    },
                                    "hkpCollidable" => {
                                                        ClassParams::HkpCollidable(map.next_value()?)
                                    },
                                    "hkpCollisionFilterList" => {
                                                        ClassParams::HkpCollisionFilterList(map.next_value()?)
                                    },
                                    "hkpCollisionFilter" => {
                                                        ClassParams::HkpCollisionFilter(map.next_value()?)
                                    },
                                    "hkpCompressedMeshShapeBigTriangle" => {
                                                        ClassParams::HkpCompressedMeshShapeBigTriangle(map.next_value()?)
                                    },
                                    "hkpCompressedMeshShapeChunk" => {
                                                        ClassParams::HkpCompressedMeshShapeChunk(map.next_value()?)
                                    },
                                    "hkpCompressedMeshShapeConvexPiece" => {
                                                        ClassParams::HkpCompressedMeshShapeConvexPiece(map.next_value()?)
                                    },
                                    "hkpCompressedMeshShape" => {
                                                        ClassParams::HkpCompressedMeshShape(map.next_value()?)
                                    },
                                    "hkpCompressedSampledHeightFieldShape" => {
                                                        ClassParams::HkpCompressedSampledHeightFieldShape(map.next_value()?)
                                    },
                                    "hkpConeLimitConstraintAtom" => {
                                                        ClassParams::HkpConeLimitConstraintAtom(map.next_value()?)
                                    },
                                    "hkpConstrainedSystemFilter" => {
                                                        ClassParams::HkpConstrainedSystemFilter(map.next_value()?)
                                    },
                                    "hkpConstraintAtom" => {
                                                        ClassParams::HkpConstraintAtom(map.next_value()?)
                                    },
                                    "hkpConstraintChainData" => {
                                                        ClassParams::HkpConstraintChainData(map.next_value()?)
                                    },
                                    "hkpConstraintChainInstanceAction" => {
                                                        ClassParams::HkpConstraintChainInstanceAction(map.next_value()?)
                                    },
                                    "hkpConstraintChainInstance" => {
                                                        ClassParams::HkpConstraintChainInstance(map.next_value()?)
                                    },
                                    "hkpConstraintCollisionFilter" => {
                                                        ClassParams::HkpConstraintCollisionFilter(map.next_value()?)
                                    },
                                    "hkpConstraintData" => {
                                                        ClassParams::HkpConstraintData(map.next_value()?)
                                    },
                                    "hkpConstraintInstanceSmallArraySerializeOverrideType" => {
                                                        ClassParams::HkpConstraintInstanceSmallArraySerializeOverrideType(map.next_value()?)
                                    },
                                    "hkpConstraintInstance" => {
                                                        ClassParams::HkpConstraintInstance(map.next_value()?)
                                    },
                                    "hkpConstraintMotor" => {
                                                        ClassParams::HkpConstraintMotor(map.next_value()?)
                                    },
                                    "hkpConvexListFilter" => {
                                                        ClassParams::HkpConvexListFilter(map.next_value()?)
                                    },
                                    "hkpConvexListShape" => {
                                                        ClassParams::HkpConvexListShape(map.next_value()?)
                                    },
                                    "hkpConvexPieceMeshShape" => {
                                                        ClassParams::HkpConvexPieceMeshShape(map.next_value()?)
                                    },
                                    "hkpConvexPieceStreamData" => {
                                                        ClassParams::HkpConvexPieceStreamData(map.next_value()?)
                                    },
                                    "hkpConvexShape" => {
                                                        ClassParams::HkpConvexShape(map.next_value()?)
                                    },
                                    "hkpConvexTransformShapeBase" => {
                                                        ClassParams::HkpConvexTransformShapeBase(map.next_value()?)
                                    },
                                    "hkpConvexTransformShape" => {
                                                        ClassParams::HkpConvexTransformShape(map.next_value()?)
                                    },
                                    "hkpConvexTranslateShape" => {
                                                        ClassParams::HkpConvexTranslateShape(map.next_value()?)
                                    },
                                    "hkpConvexVerticesConnectivity" => {
                                                        ClassParams::HkpConvexVerticesConnectivity(map.next_value()?)
                                    },
                                    "hkpConvexVerticesShapeFourVectors" => {
                                                        ClassParams::HkpConvexVerticesShapeFourVectors(map.next_value()?)
                                    },
                                    "hkpConvexVerticesShape" => {
                                                        ClassParams::HkpConvexVerticesShape(map.next_value()?)
                                    },
                                    "hkpCylinderShape" => {
                                                        ClassParams::HkpCylinderShape(map.next_value()?)
                                    },
                                    "hkpDashpotAction" => {
                                                        ClassParams::HkpDashpotAction(map.next_value()?)
                                    },
                                    "hkpDefaultConvexListFilter" => {
                                                        ClassParams::HkpDefaultConvexListFilter(map.next_value()?)
                                    },
                                    "hkpDefaultWorldMemoryWatchDog" => {
                                                        ClassParams::HkpDefaultWorldMemoryWatchDog(map.next_value()?)
                                    },
                                    "hkpDisableEntityCollisionFilter" => {
                                                        ClassParams::HkpDisableEntityCollisionFilter(map.next_value()?)
                                    },
                                    "hkpDisplayBindingDataPhysicsSystem" => {
                                                        ClassParams::HkpDisplayBindingDataPhysicsSystem(map.next_value()?)
                                    },
                                    "hkpDisplayBindingDataRigidBody" => {
                                                        ClassParams::HkpDisplayBindingDataRigidBody(map.next_value()?)
                                    },
                                    "hkpDisplayBindingData" => {
                                                        ClassParams::HkpDisplayBindingData(map.next_value()?)
                                    },
                                    "hkpEntityExtendedListeners" => {
                                                        ClassParams::HkpEntityExtendedListeners(map.next_value()?)
                                    },
                                    "hkpEntitySmallArraySerializeOverrideType" => {
                                                        ClassParams::HkpEntitySmallArraySerializeOverrideType(map.next_value()?)
                                    },
                                    "hkpEntitySpuCollisionCallback" => {
                                                        ClassParams::HkpEntitySpuCollisionCallback(map.next_value()?)
                                    },
                                    "hkpEntity" => {
                                                        ClassParams::HkpEntity(map.next_value()?)
                                    },
                                    "hkpExtendedMeshShapeShapesSubpart" => {
                                                        ClassParams::HkpExtendedMeshShapeShapesSubpart(map.next_value()?)
                                    },
                                    "hkpExtendedMeshShapeSubpart" => {
                                                        ClassParams::HkpExtendedMeshShapeSubpart(map.next_value()?)
                                    },
                                    "hkpExtendedMeshShapeTrianglesSubpart" => {
                                                        ClassParams::HkpExtendedMeshShapeTrianglesSubpart(map.next_value()?)
                                    },
                                    "hkpExtendedMeshShape" => {
                                                        ClassParams::HkpExtendedMeshShape(map.next_value()?)
                                    },
                                    "hkpFastMeshShape" => {
                                                        ClassParams::HkpFastMeshShape(map.next_value()?)
                                    },
                                    "hkpFirstPersonGun" => {
                                                        ClassParams::HkpFirstPersonGun(map.next_value()?)
                                    },
                                    "hkpFixedRigidMotion" => {
                                                        ClassParams::HkpFixedRigidMotion(map.next_value()?)
                                    },
                                    "hkpGenericConstraintDataSchemeConstraintInfo" => {
                                                        ClassParams::HkpGenericConstraintDataSchemeConstraintInfo(map.next_value()?)
                                    },
                                    "hkpGenericConstraintDataScheme" => {
                                                        ClassParams::HkpGenericConstraintDataScheme(map.next_value()?)
                                    },
                                    "hkpGenericConstraintData" => {
                                                        ClassParams::HkpGenericConstraintData(map.next_value()?)
                                    },
                                    "hkpGravityGun" => {
                                                        ClassParams::HkpGravityGun(map.next_value()?)
                                    },
                                    "hkpGroupCollisionFilter" => {
                                                        ClassParams::HkpGroupCollisionFilter(map.next_value()?)
                                    },
                                    "hkpGroupFilter" => {
                                                        ClassParams::HkpGroupFilter(map.next_value()?)
                                    },
                                    "hkpHeightFieldShape" => {
                                                        ClassParams::HkpHeightFieldShape(map.next_value()?)
                                    },
                                    "hkpHingeConstraintDataAtoms" => {
                                                        ClassParams::HkpHingeConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpHingeConstraintData" => {
                                                        ClassParams::HkpHingeConstraintData(map.next_value()?)
                                    },
                                    "hkpHingeLimitsDataAtoms" => {
                                                        ClassParams::HkpHingeLimitsDataAtoms(map.next_value()?)
                                    },
                                    "hkpHingeLimitsData" => {
                                                        ClassParams::HkpHingeLimitsData(map.next_value()?)
                                    },
                                    "hkpIgnoreModifierConstraintAtom" => {
                                                        ClassParams::HkpIgnoreModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpKeyframedRigidMotion" => {
                                                        ClassParams::HkpKeyframedRigidMotion(map.next_value()?)
                                    },
                                    "hkpLimitedForceConstraintMotor" => {
                                                        ClassParams::HkpLimitedForceConstraintMotor(map.next_value()?)
                                    },
                                    "hkpLimitedHingeConstraintDataAtoms" => {
                                                        ClassParams::HkpLimitedHingeConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpLimitedHingeConstraintData" => {
                                                        ClassParams::HkpLimitedHingeConstraintData(map.next_value()?)
                                    },
                                    "hkpLinConstraintAtom" => {
                                                        ClassParams::HkpLinConstraintAtom(map.next_value()?)
                                    },
                                    "hkpLinearParametricCurve" => {
                                                        ClassParams::HkpLinearParametricCurve(map.next_value()?)
                                    },
                                    "hkpLinFrictionConstraintAtom" => {
                                                        ClassParams::HkpLinFrictionConstraintAtom(map.next_value()?)
                                    },
                                    "hkpLinkedCollidable" => {
                                                        ClassParams::HkpLinkedCollidable(map.next_value()?)
                                    },
                                    "hkpLinLimitConstraintAtom" => {
                                                        ClassParams::HkpLinLimitConstraintAtom(map.next_value()?)
                                    },
                                    "hkpLinMotorConstraintAtom" => {
                                                        ClassParams::HkpLinMotorConstraintAtom(map.next_value()?)
                                    },
                                    "hkpLinSoftConstraintAtom" => {
                                                        ClassParams::HkpLinSoftConstraintAtom(map.next_value()?)
                                    },
                                    "hkpListShapeChildInfo" => {
                                                        ClassParams::HkpListShapeChildInfo(map.next_value()?)
                                    },
                                    "hkpListShape" => {
                                                        ClassParams::HkpListShape(map.next_value()?)
                                    },
                                    "hkpMalleableConstraintData" => {
                                                        ClassParams::HkpMalleableConstraintData(map.next_value()?)
                                    },
                                    "hkpMassChangerModifierConstraintAtom" => {
                                                        ClassParams::HkpMassChangerModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpMassProperties" => {
                                                        ClassParams::HkpMassProperties(map.next_value()?)
                                    },
                                    "hkpMaterial" => {
                                                        ClassParams::HkpMaterial(map.next_value()?)
                                    },
                                    "hkpMaxSizeMotion" => {
                                                        ClassParams::HkpMaxSizeMotion(map.next_value()?)
                                    },
                                    "hkpMeshMaterial" => {
                                                        ClassParams::HkpMeshMaterial(map.next_value()?)
                                    },
                                    "hkpMeshShapeSubpart" => {
                                                        ClassParams::HkpMeshShapeSubpart(map.next_value()?)
                                    },
                                    "hkpMeshShape" => {
                                                        ClassParams::HkpMeshShape(map.next_value()?)
                                    },
                                    "hkpModifierConstraintAtom" => {
                                                        ClassParams::HkpModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpMoppBvTreeShape" => {
                                                        ClassParams::HkpMoppBvTreeShape(map.next_value()?)
                                    },
                                    "hkpMoppCodeCodeInfo" => {
                                                        ClassParams::HkpMoppCodeCodeInfo(map.next_value()?)
                                    },
                                    "hkpMoppCodeReindexedTerminal" => {
                                                        ClassParams::HkpMoppCodeReindexedTerminal(map.next_value()?)
                                    },
                                    "hkpMoppCode" => {
                                                        ClassParams::HkpMoppCode(map.next_value()?)
                                    },
                                    "hkpMotion" => {
                                                        ClassParams::HkpMotion(map.next_value()?)
                                    },
                                    "hkpMotorAction" => {
                                                        ClassParams::HkpMotorAction(map.next_value()?)
                                    },
                                    "hkpMountedBallGun" => {
                                                        ClassParams::HkpMountedBallGun(map.next_value()?)
                                    },
                                    "hkpMouseSpringAction" => {
                                                        ClassParams::HkpMouseSpringAction(map.next_value()?)
                                    },
                                    "hkpMovingSurfaceModifierConstraintAtom" => {
                                                        ClassParams::HkpMovingSurfaceModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpMultiRayShapeRay" => {
                                                        ClassParams::HkpMultiRayShapeRay(map.next_value()?)
                                    },
                                    "hkpMultiRayShape" => {
                                                        ClassParams::HkpMultiRayShape(map.next_value()?)
                                    },
                                    "hkpMultiSphereShape" => {
                                                        ClassParams::HkpMultiSphereShape(map.next_value()?)
                                    },
                                    "hkpMultithreadedVehicleManager" => {
                                                        ClassParams::HkpMultithreadedVehicleManager(map.next_value()?)
                                    },
                                    "hkpNamedMeshMaterial" => {
                                                        ClassParams::HkpNamedMeshMaterial(map.next_value()?)
                                    },
                                    "hkpNullCollisionFilter" => {
                                                        ClassParams::HkpNullCollisionFilter(map.next_value()?)
                                    },
                                    "hkPostFinishAttribute" => {
                                                        ClassParams::HkPostFinishAttribute(map.next_value()?)
                                    },
                                    "hkpOverwritePivotConstraintAtom" => {
                                                        ClassParams::HkpOverwritePivotConstraintAtom(map.next_value()?)
                                    },
                                    "hkpPairCollisionFilterMapPairFilterKeyOverrideType" => {
                                                        ClassParams::HkpPairCollisionFilterMapPairFilterKeyOverrideType(map.next_value()?)
                                    },
                                    "hkpPairCollisionFilter" => {
                                                        ClassParams::HkpPairCollisionFilter(map.next_value()?)
                                    },
                                    "hkpParametricCurve" => {
                                                        ClassParams::HkpParametricCurve(map.next_value()?)
                                    },
                                    "hkpPhantomCallbackShape" => {
                                                        ClassParams::HkpPhantomCallbackShape(map.next_value()?)
                                    },
                                    "hkpPhantom" => {
                                                        ClassParams::HkpPhantom(map.next_value()?)
                                    },
                                    "hkpPhysicsData" => {
                                                        ClassParams::HkpPhysicsData(map.next_value()?)
                                    },
                                    "hkpPhysicsSystemWithContacts" => {
                                                        ClassParams::HkpPhysicsSystemWithContacts(map.next_value()?)
                                    },
                                    "hkpPhysicsSystem" => {
                                                        ClassParams::HkpPhysicsSystem(map.next_value()?)
                                    },
                                    "hkpPlaneShape" => {
                                                        ClassParams::HkpPlaneShape(map.next_value()?)
                                    },
                                    "hkpPointToPathConstraintData" => {
                                                        ClassParams::HkpPointToPathConstraintData(map.next_value()?)
                                    },
                                    "hkpPointToPlaneConstraintDataAtoms" => {
                                                        ClassParams::HkpPointToPlaneConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpPointToPlaneConstraintData" => {
                                                        ClassParams::HkpPointToPlaneConstraintData(map.next_value()?)
                                    },
                                    "hkpPositionConstraintMotor" => {
                                                        ClassParams::HkpPositionConstraintMotor(map.next_value()?)
                                    },
                                    "hkpPoweredChainDataConstraintInfo" => {
                                                        ClassParams::HkpPoweredChainDataConstraintInfo(map.next_value()?)
                                    },
                                    "hkpPoweredChainData" => {
                                                        ClassParams::HkpPoweredChainData(map.next_value()?)
                                    },
                                    "hkpPoweredChainMapperLinkInfo" => {
                                                        ClassParams::HkpPoweredChainMapperLinkInfo(map.next_value()?)
                                    },
                                    "hkpPoweredChainMapperTarget" => {
                                                        ClassParams::HkpPoweredChainMapperTarget(map.next_value()?)
                                    },
                                    "hkpPoweredChainMapper" => {
                                                        ClassParams::HkpPoweredChainMapper(map.next_value()?)
                                    },
                                    "hkpPrismaticConstraintDataAtoms" => {
                                                        ClassParams::HkpPrismaticConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpPrismaticConstraintData" => {
                                                        ClassParams::HkpPrismaticConstraintData(map.next_value()?)
                                    },
                                    "hkpProjectileGun" => {
                                                        ClassParams::HkpProjectileGun(map.next_value()?)
                                    },
                                    "hkpPropertyValue" => {
                                                        ClassParams::HkpPropertyValue(map.next_value()?)
                                    },
                                    "hkpProperty" => {
                                                        ClassParams::HkpProperty(map.next_value()?)
                                    },
                                    "hkpPulleyConstraintAtom" => {
                                                        ClassParams::HkpPulleyConstraintAtom(map.next_value()?)
                                    },
                                    "hkpPulleyConstraintDataAtoms" => {
                                                        ClassParams::HkpPulleyConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpPulleyConstraintData" => {
                                                        ClassParams::HkpPulleyConstraintData(map.next_value()?)
                                    },
                                    "hkpRackAndPinionConstraintAtom" => {
                                                        ClassParams::HkpRackAndPinionConstraintAtom(map.next_value()?)
                                    },
                                    "hkpRackAndPinionConstraintDataAtoms" => {
                                                        ClassParams::HkpRackAndPinionConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpRackAndPinionConstraintData" => {
                                                        ClassParams::HkpRackAndPinionConstraintData(map.next_value()?)
                                    },
                                    "hkpRagdollConstraintDataAtoms" => {
                                                        ClassParams::HkpRagdollConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpRagdollConstraintData" => {
                                                        ClassParams::HkpRagdollConstraintData(map.next_value()?)
                                    },
                                    "hkpRagdollLimitsDataAtoms" => {
                                                        ClassParams::HkpRagdollLimitsDataAtoms(map.next_value()?)
                                    },
                                    "hkpRagdollLimitsData" => {
                                                        ClassParams::HkpRagdollLimitsData(map.next_value()?)
                                    },
                                    "hkpRagdollMotorConstraintAtom" => {
                                                        ClassParams::HkpRagdollMotorConstraintAtom(map.next_value()?)
                                    },
                                    "hkpRayCollidableFilter" => {
                                                        ClassParams::HkpRayCollidableFilter(map.next_value()?)
                                    },
                                    "hkpRayShapeCollectionFilter" => {
                                                        ClassParams::HkpRayShapeCollectionFilter(map.next_value()?)
                                    },
                                    "hkpRejectChassisListener" => {
                                                        ClassParams::HkpRejectChassisListener(map.next_value()?)
                                    },
                                    "hkpRemoveTerminalsMoppModifier" => {
                                                        ClassParams::HkpRemoveTerminalsMoppModifier(map.next_value()?)
                                    },
                                    "hkpReorientAction" => {
                                                        ClassParams::HkpReorientAction(map.next_value()?)
                                    },
                                    "hkpRigidBody" => {
                                                        ClassParams::HkpRigidBody(map.next_value()?)
                                    },
                                    "hkpRotationalConstraintDataAtoms" => {
                                                        ClassParams::HkpRotationalConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpRotationalConstraintData" => {
                                                        ClassParams::HkpRotationalConstraintData(map.next_value()?)
                                    },
                                    "hkpSampledHeightFieldShape" => {
                                                        ClassParams::HkpSampledHeightFieldShape(map.next_value()?)
                                    },
                                    "hkpSerializedAgentNnEntry" => {
                                                        ClassParams::HkpSerializedAgentNnEntry(map.next_value()?)
                                    },
                                    "hkpSerializedDisplayMarkerList" => {
                                                        ClassParams::HkpSerializedDisplayMarkerList(map.next_value()?)
                                    },
                                    "hkpSerializedDisplayMarker" => {
                                                        ClassParams::HkpSerializedDisplayMarker(map.next_value()?)
                                    },
                                    "hkpSerializedDisplayRbTransformsDisplayTransformPair" => {
                                                        ClassParams::HkpSerializedDisplayRbTransformsDisplayTransformPair(map.next_value()?)
                                    },
                                    "hkpSerializedDisplayRbTransforms" => {
                                                        ClassParams::HkpSerializedDisplayRbTransforms(map.next_value()?)
                                    },
                                    "hkpSerializedSubTrack1nInfo" => {
                                                        ClassParams::HkpSerializedSubTrack1NInfo(map.next_value()?)
                                    },
                                    "hkpSerializedTrack1nInfo" => {
                                                        ClassParams::HkpSerializedTrack1NInfo(map.next_value()?)
                                    },
                                    "hkpSetLocalRotationsConstraintAtom" => {
                                                        ClassParams::HkpSetLocalRotationsConstraintAtom(map.next_value()?)
                                    },
                                    "hkpSetLocalTransformsConstraintAtom" => {
                                                        ClassParams::HkpSetLocalTransformsConstraintAtom(map.next_value()?)
                                    },
                                    "hkpSetLocalTranslationsConstraintAtom" => {
                                                        ClassParams::HkpSetLocalTranslationsConstraintAtom(map.next_value()?)
                                    },
                                    "hkpSetupStabilizationAtom" => {
                                                        ClassParams::HkpSetupStabilizationAtom(map.next_value()?)
                                    },
                                    "hkpShapeCollectionFilter" => {
                                                        ClassParams::HkpShapeCollectionFilter(map.next_value()?)
                                    },
                                    "hkpShapeCollection" => {
                                                        ClassParams::HkpShapeCollection(map.next_value()?)
                                    },
                                    "hkpShapeContainer" => {
                                                        ClassParams::HkpShapeContainer(map.next_value()?)
                                    },
                                    "hkpShapeInfo" => {
                                                        ClassParams::HkpShapeInfo(map.next_value()?)
                                    },
                                    "hkpShapeModifier" => {
                                                        ClassParams::HkpShapeModifier(map.next_value()?)
                                    },
                                    "hkpShapePhantom" => {
                                                        ClassParams::HkpShapePhantom(map.next_value()?)
                                    },
                                    "hkpShape" => {
                                                        ClassParams::HkpShape(map.next_value()?)
                                    },
                                    "hkpSimpleContactConstraintAtom" => {
                                                        ClassParams::HkpSimpleContactConstraintAtom(map.next_value()?)
                                    },
                                    "hkpSimpleContactConstraintDataInfo" => {
                                                        ClassParams::HkpSimpleContactConstraintDataInfo(map.next_value()?)
                                    },
                                    "hkpSimpleMeshShapeTriangle" => {
                                                        ClassParams::HkpSimpleMeshShapeTriangle(map.next_value()?)
                                    },
                                    "hkpSimpleMeshShape" => {
                                                        ClassParams::HkpSimpleMeshShape(map.next_value()?)
                                    },
                                    "hkpSimpleShapePhantomCollisionDetail" => {
                                                        ClassParams::HkpSimpleShapePhantomCollisionDetail(map.next_value()?)
                                    },
                                    "hkpSimpleShapePhantom" => {
                                                        ClassParams::HkpSimpleShapePhantom(map.next_value()?)
                                    },
                                    "hkpSimulation" => {
                                                        ClassParams::HkpSimulation(map.next_value()?)
                                    },
                                    "hkpSingleShapeContainer" => {
                                                        ClassParams::HkpSingleShapeContainer(map.next_value()?)
                                    },
                                    "hkpSoftContactModifierConstraintAtom" => {
                                                        ClassParams::HkpSoftContactModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpSphereMotion" => {
                                                        ClassParams::HkpSphereMotion(map.next_value()?)
                                    },
                                    "hkpSphereRepShape" => {
                                                        ClassParams::HkpSphereRepShape(map.next_value()?)
                                    },
                                    "hkpSphereShape" => {
                                                        ClassParams::HkpSphereShape(map.next_value()?)
                                    },
                                    "hkpSpringAction" => {
                                                        ClassParams::HkpSpringAction(map.next_value()?)
                                    },
                                    "hkpSpringDamperConstraintMotor" => {
                                                        ClassParams::HkpSpringDamperConstraintMotor(map.next_value()?)
                                    },
                                    "hkpStiffSpringChainDataConstraintInfo" => {
                                                        ClassParams::HkpStiffSpringChainDataConstraintInfo(map.next_value()?)
                                    },
                                    "hkpStiffSpringChainData" => {
                                                        ClassParams::HkpStiffSpringChainData(map.next_value()?)
                                    },
                                    "hkpStiffSpringConstraintAtom" => {
                                                        ClassParams::HkpStiffSpringConstraintAtom(map.next_value()?)
                                    },
                                    "hkpStiffSpringConstraintDataAtoms" => {
                                                        ClassParams::HkpStiffSpringConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpStiffSpringConstraintData" => {
                                                        ClassParams::HkpStiffSpringConstraintData(map.next_value()?)
                                    },
                                    "hkpStorageExtendedMeshShapeMaterial" => {
                                                        ClassParams::HkpStorageExtendedMeshShapeMaterial(map.next_value()?)
                                    },
                                    "hkpStorageExtendedMeshShapeMeshSubpartStorage" => {
                                                        ClassParams::HkpStorageExtendedMeshShapeMeshSubpartStorage(map.next_value()?)
                                    },
                                    "hkpStorageExtendedMeshShapeShapeSubpartStorage" => {
                                                        ClassParams::HkpStorageExtendedMeshShapeShapeSubpartStorage(map.next_value()?)
                                    },
                                    "hkpStorageExtendedMeshShape" => {
                                                        ClassParams::HkpStorageExtendedMeshShape(map.next_value()?)
                                    },
                                    "hkpStorageMeshShapeSubpartStorage" => {
                                                        ClassParams::HkpStorageMeshShapeSubpartStorage(map.next_value()?)
                                    },
                                    "hkpStorageMeshShape" => {
                                                        ClassParams::HkpStorageMeshShape(map.next_value()?)
                                    },
                                    "hkpStorageSampledHeightFieldShape" => {
                                                        ClassParams::HkpStorageSampledHeightFieldShape(map.next_value()?)
                                    },
                                    "hkpThinBoxMotion" => {
                                                        ClassParams::HkpThinBoxMotion(map.next_value()?)
                                    },
                                    "hkpTransformShape" => {
                                                        ClassParams::HkpTransformShape(map.next_value()?)
                                    },
                                    "hkpTriangleShape" => {
                                                        ClassParams::HkpTriangleShape(map.next_value()?)
                                    },
                                    "hkpTriggerVolumeEventInfo" => {
                                                        ClassParams::HkpTriggerVolumeEventInfo(map.next_value()?)
                                    },
                                    "hkpTriggerVolume" => {
                                                        ClassParams::HkpTriggerVolume(map.next_value()?)
                                    },
                                    "hkpTriSampledHeightFieldBvTreeShape" => {
                                                        ClassParams::HkpTriSampledHeightFieldBvTreeShape(map.next_value()?)
                                    },
                                    "hkpTriSampledHeightFieldCollection" => {
                                                        ClassParams::HkpTriSampledHeightFieldCollection(map.next_value()?)
                                    },
                                    "hkpTwistLimitConstraintAtom" => {
                                                        ClassParams::HkpTwistLimitConstraintAtom(map.next_value()?)
                                    },
                                    "hkpTypedBroadPhaseHandle" => {
                                                        ClassParams::HkpTypedBroadPhaseHandle(map.next_value()?)
                                    },
                                    "hkpTyremarkPoint" => {
                                                        ClassParams::HkpTyremarkPoint(map.next_value()?)
                                    },
                                    "hkpTyremarksInfo" => {
                                                        ClassParams::HkpTyremarksInfo(map.next_value()?)
                                    },
                                    "hkpTyremarksWheel" => {
                                                        ClassParams::HkpTyremarksWheel(map.next_value()?)
                                    },
                                    "hkpUnaryAction" => {
                                                        ClassParams::HkpUnaryAction(map.next_value()?)
                                    },
                                    "hkpVehicleAerodynamics" => {
                                                        ClassParams::HkpVehicleAerodynamics(map.next_value()?)
                                    },
                                    "hkpVehicleBrake" => {
                                                        ClassParams::HkpVehicleBrake(map.next_value()?)
                                    },
                                    "hkpVehicleCastBatchingManager" => {
                                                        ClassParams::HkpVehicleCastBatchingManager(map.next_value()?)
                                    },
                                    "hkpVehicleDataWheelComponentParams" => {
                                                        ClassParams::HkpVehicleDataWheelComponentParams(map.next_value()?)
                                    },
                                    "hkpVehicleData" => {
                                                        ClassParams::HkpVehicleData(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultAerodynamics" => {
                                                        ClassParams::HkpVehicleDefaultAerodynamics(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultAnalogDriverInput" => {
                                                        ClassParams::HkpVehicleDefaultAnalogDriverInput(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultBrakeWheelBrakingProperties" => {
                                                        ClassParams::HkpVehicleDefaultBrakeWheelBrakingProperties(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultBrake" => {
                                                        ClassParams::HkpVehicleDefaultBrake(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultEngine" => {
                                                        ClassParams::HkpVehicleDefaultEngine(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultSteering" => {
                                                        ClassParams::HkpVehicleDefaultSteering(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters" => {
                                                        ClassParams::HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultSuspension" => {
                                                        ClassParams::HkpVehicleDefaultSuspension(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultTransmission" => {
                                                        ClassParams::HkpVehicleDefaultTransmission(map.next_value()?)
                                    },
                                    "hkpVehicleDefaultVelocityDamper" => {
                                                        ClassParams::HkpVehicleDefaultVelocityDamper(map.next_value()?)
                                    },
                                    "hkpVehicleDriverInputAnalogStatus" => {
                                                        ClassParams::HkpVehicleDriverInputAnalogStatus(map.next_value()?)
                                    },
                                    "hkpVehicleDriverInputStatus" => {
                                                        ClassParams::HkpVehicleDriverInputStatus(map.next_value()?)
                                    },
                                    "hkpVehicleDriverInput" => {
                                                        ClassParams::HkpVehicleDriverInput(map.next_value()?)
                                    },
                                    "hkpVehicleEngine" => {
                                                        ClassParams::HkpVehicleEngine(map.next_value()?)
                                    },
                                    "hkpVehicleFrictionDescriptionAxisDescription" => {
                                                        ClassParams::HkpVehicleFrictionDescriptionAxisDescription(map.next_value()?)
                                    },
                                    "hkpVehicleFrictionDescription" => {
                                                        ClassParams::HkpVehicleFrictionDescription(map.next_value()?)
                                    },
                                    "hkpVehicleFrictionStatusAxisStatus" => {
                                                        ClassParams::HkpVehicleFrictionStatusAxisStatus(map.next_value()?)
                                    },
                                    "hkpVehicleFrictionStatus" => {
                                                        ClassParams::HkpVehicleFrictionStatus(map.next_value()?)
                                    },
                                    "hkpVehicleInstanceWheelInfo" => {
                                                        ClassParams::HkpVehicleInstanceWheelInfo(map.next_value()?)
                                    },
                                    "hkpVehicleInstance" => {
                                                        ClassParams::HkpVehicleInstance(map.next_value()?)
                                    },
                                    "hkpVehicleLinearCastBatchingManager" => {
                                                        ClassParams::HkpVehicleLinearCastBatchingManager(map.next_value()?)
                                    },
                                    "hkpVehicleLinearCastWheelCollideWheelState" => {
                                                        ClassParams::HkpVehicleLinearCastWheelCollideWheelState(map.next_value()?)
                                    },
                                    "hkpVehicleLinearCastWheelCollide" => {
                                                        ClassParams::HkpVehicleLinearCastWheelCollide(map.next_value()?)
                                    },
                                    "hkpVehicleManager" => {
                                                        ClassParams::HkpVehicleManager(map.next_value()?)
                                    },
                                    "hkpVehicleRayCastBatchingManager" => {
                                                        ClassParams::HkpVehicleRayCastBatchingManager(map.next_value()?)
                                    },
                                    "hkpVehicleRayCastWheelCollide" => {
                                                        ClassParams::HkpVehicleRayCastWheelCollide(map.next_value()?)
                                    },
                                    "hkpVehicleSteering" => {
                                                        ClassParams::HkpVehicleSteering(map.next_value()?)
                                    },
                                    "hkpVehicleSuspensionSuspensionWheelParameters" => {
                                                        ClassParams::HkpVehicleSuspensionSuspensionWheelParameters(map.next_value()?)
                                    },
                                    "hkpVehicleSuspension" => {
                                                        ClassParams::HkpVehicleSuspension(map.next_value()?)
                                    },
                                    "hkpVehicleTransmission" => {
                                                        ClassParams::HkpVehicleTransmission(map.next_value()?)
                                    },
                                    "hkpVehicleVelocityDamper" => {
                                                        ClassParams::HkpVehicleVelocityDamper(map.next_value()?)
                                    },
                                    "hkpVehicleWheelCollide" => {
                                                        ClassParams::HkpVehicleWheelCollide(map.next_value()?)
                                    },
                                    "hkpVelocityConstraintMotor" => {
                                                        ClassParams::HkpVelocityConstraintMotor(map.next_value()?)
                                    },
                                    "hkpViscousSurfaceModifierConstraintAtom" => {
                                                        ClassParams::HkpViscousSurfaceModifierConstraintAtom(map.next_value()?)
                                    },
                                    "hkpWeldingUtility" => {
                                                        ClassParams::HkpWeldingUtility(map.next_value()?)
                                    },
                                    "hkpWheelConstraintDataAtoms" => {
                                                        ClassParams::HkpWheelConstraintDataAtoms(map.next_value()?)
                                    },
                                    "hkpWheelConstraintData" => {
                                                        ClassParams::HkpWheelConstraintData(map.next_value()?)
                                    },
                                    "hkpWorldCinfo" => {
                                                        ClassParams::HkpWorldCinfo(map.next_value()?)
                                    },
                                    "hkpWorldObject" => {
                                                        ClassParams::HkpWorldObject(map.next_value()?)
                                    },
                                    "hkpWorld" => {
                                                        ClassParams::HkpWorld(map.next_value()?)
                                    },
                                    "hkQTransform" => {
                                                        ClassParams::HkQTransform(map.next_value()?)
                                    },
                                    "hkRangeInt32Attribute" => {
                                                        ClassParams::HkRangeInt32Attribute(map.next_value()?)
                                    },
                                    "hkRangeRealAttribute" => {
                                                        ClassParams::HkRangeRealAttribute(map.next_value()?)
                                    },
                                    "hkReferencedObject" => {
                                                        ClassParams::HkReferencedObject(map.next_value()?)
                                    },
                                    "hkReflectedFileAttribute" => {
                                                        ClassParams::HkReflectedFileAttribute(map.next_value()?)
                                    },
                                    "hkResourceBase" => {
                                                        ClassParams::HkResourceBase(map.next_value()?)
                                    },
                                    "hkResourceContainer" => {
                                                        ClassParams::HkResourceContainer(map.next_value()?)
                                    },
                                    "hkResourceHandle" => {
                                                        ClassParams::HkResourceHandle(map.next_value()?)
                                    },
                                    "hkRootLevelContainerNamedVariant" => {
                                                        ClassParams::HkRootLevelContainerNamedVariant(map.next_value()?)
                                    },
                                    "hkRootLevelContainer" => {
                                                        ClassParams::HkRootLevelContainer(map.next_value()?)
                                    },
                                    "hkSemanticsAttribute" => {
                                                        ClassParams::HkSemanticsAttribute(map.next_value()?)
                                    },
                                    "hkSimpleLocalFrame" => {
                                                        ClassParams::HkSimpleLocalFrame(map.next_value()?)
                                    },
                                    "hkSphere" => {
                                                        ClassParams::HkSphere(map.next_value()?)
                                    },
                                    "hkSweptTransform" => {
                                                        ClassParams::HkSweptTransform(map.next_value()?)
                                    },
                                    "hkTraceStreamTitle" => {
                                                        ClassParams::HkTraceStreamTitle(map.next_value()?)
                                    },
                                    "hkTrackerSerializableScanSnapshotAllocation" => {
                                                        ClassParams::HkTrackerSerializableScanSnapshotAllocation(map.next_value()?)
                                    },
                                    "hkTrackerSerializableScanSnapshotBlock" => {
                                                        ClassParams::HkTrackerSerializableScanSnapshotBlock(map.next_value()?)
                                    },
                                    "hkTrackerSerializableScanSnapshot" => {
                                                        ClassParams::HkTrackerSerializableScanSnapshot(map.next_value()?)
                                    },
                                    "hkUiAttribute" => {
                                                        ClassParams::HkUiAttribute(map.next_value()?)
                                    },
                                    "hkVertexFormatElement" => {
                                                        ClassParams::HkVertexFormatElement(map.next_value()?)
                                    },
                                    "hkVertexFormat" => {
                                                        ClassParams::HkVertexFormat(map.next_value()?)
                                    },
                                    "hkWorldMemoryAvailableWatchDog" => {
                                                        ClassParams::HkWorldMemoryAvailableWatchDog(map.next_value()?)
                                    },
                                    "hkxAnimatedFloat" => {
                                                        ClassParams::HkxAnimatedFloat(map.next_value()?)
                                    },
                                    "hkxAnimatedMatrix" => {
                                                        ClassParams::HkxAnimatedMatrix(map.next_value()?)
                                    },
                                    "hkxAnimatedQuaternion" => {
                                                        ClassParams::HkxAnimatedQuaternion(map.next_value()?)
                                    },
                                    "hkxAnimatedVector" => {
                                                        ClassParams::HkxAnimatedVector(map.next_value()?)
                                    },
                                    "hkxAttributeGroup" => {
                                                        ClassParams::HkxAttributeGroup(map.next_value()?)
                                    },
                                    "hkxAttributeHolder" => {
                                                        ClassParams::HkxAttributeHolder(map.next_value()?)
                                    },
                                    "hkxAttribute" => {
                                                        ClassParams::HkxAttribute(map.next_value()?)
                                    },
                                    "hkxCamera" => {
                                                        ClassParams::HkxCamera(map.next_value()?)
                                    },
                                    "hkxEdgeSelectionChannel" => {
                                                        ClassParams::HkxEdgeSelectionChannel(map.next_value()?)
                                    },
                                    "hkxEnumItem" => {
                                                        ClassParams::HkxEnumItem(map.next_value()?)
                                    },
                                    "hkxEnum" => {
                                                        ClassParams::HkxEnum(map.next_value()?)
                                    },
                                    "hkxEnvironmentVariable" => {
                                                        ClassParams::HkxEnvironmentVariable(map.next_value()?)
                                    },
                                    "hkxEnvironment" => {
                                                        ClassParams::HkxEnvironment(map.next_value()?)
                                    },
                                    "hkxIndexBuffer" => {
                                                        ClassParams::HkxIndexBuffer(map.next_value()?)
                                    },
                                    "hkxLight" => {
                                                        ClassParams::HkxLight(map.next_value()?)
                                    },
                                    "hkxMaterialEffect" => {
                                                        ClassParams::HkxMaterialEffect(map.next_value()?)
                                    },
                                    "hkxMaterialProperty" => {
                                                        ClassParams::HkxMaterialProperty(map.next_value()?)
                                    },
                                    "hkxMaterialShaderSet" => {
                                                        ClassParams::HkxMaterialShaderSet(map.next_value()?)
                                    },
                                    "hkxMaterialShader" => {
                                                        ClassParams::HkxMaterialShader(map.next_value()?)
                                    },
                                    "hkxMaterialTextureStage" => {
                                                        ClassParams::HkxMaterialTextureStage(map.next_value()?)
                                    },
                                    "hkxMaterial" => {
                                                        ClassParams::HkxMaterial(map.next_value()?)
                                    },
                                    "hkxMeshSection" => {
                                                        ClassParams::HkxMeshSection(map.next_value()?)
                                    },
                                    "hkxMeshUserChannelInfo" => {
                                                        ClassParams::HkxMeshUserChannelInfo(map.next_value()?)
                                    },
                                    "hkxMesh" => {
                                                        ClassParams::HkxMesh(map.next_value()?)
                                    },
                                    "hkxNodeAnnotationData" => {
                                                        ClassParams::HkxNodeAnnotationData(map.next_value()?)
                                    },
                                    "hkxNodeSelectionSet" => {
                                                        ClassParams::HkxNodeSelectionSet(map.next_value()?)
                                    },
                                    "hkxNode" => {
                                                        ClassParams::HkxNode(map.next_value()?)
                                    },
                                    "hkxScene" => {
                                                        ClassParams::HkxScene(map.next_value()?)
                                    },
                                    "hkxSkinBinding" => {
                                                        ClassParams::HkxSkinBinding(map.next_value()?)
                                    },
                                    "hkxSparselyAnimatedBool" => {
                                                        ClassParams::HkxSparselyAnimatedBool(map.next_value()?)
                                    },
                                    "hkxSparselyAnimatedEnum" => {
                                                        ClassParams::HkxSparselyAnimatedEnum(map.next_value()?)
                                    },
                                    "hkxSparselyAnimatedInt" => {
                                                        ClassParams::HkxSparselyAnimatedInt(map.next_value()?)
                                    },
                                    "hkxSparselyAnimatedString" => {
                                                        ClassParams::HkxSparselyAnimatedString(map.next_value()?)
                                    },
                                    "hkxTextureFile" => {
                                                        ClassParams::HkxTextureFile(map.next_value()?)
                                    },
                                    "hkxTextureInplace" => {
                                                        ClassParams::HkxTextureInplace(map.next_value()?)
                                    },
                                    "hkxTriangleSelectionChannel" => {
                                                        ClassParams::HkxTriangleSelectionChannel(map.next_value()?)
                                    },
                                    "hkxVertexBufferVertexData" => {
                                                        ClassParams::HkxVertexBufferVertexData(map.next_value()?)
                                    },
                                    "hkxVertexBuffer" => {
                                                        ClassParams::HkxVertexBuffer(map.next_value()?)
                                    },
                                    "hkxVertexDescriptionElementDecl" => {
                                                        ClassParams::HkxVertexDescriptionElementDecl(map.next_value()?)
                                    },
                                    "hkxVertexDescription" => {
                                                        ClassParams::HkxVertexDescription(map.next_value()?)
                                    },
                                    "hkxVertexFloatDataChannel" => {
                                                        ClassParams::HkxVertexFloatDataChannel(map.next_value()?)
                                    },
                                    "hkxVertexIntDataChannel" => {
                                                        ClassParams::HkxVertexIntDataChannel(map.next_value()?)
                                    },
                                    "hkxVertexSelectionChannel" => {
                                                        ClassParams::HkxVertexSelectionChannel(map.next_value()?)
                                    },
                                    "hkxVertexVectorDataChannel" => {
                                                        ClassParams::HkxVertexVectorDataChannel(map.next_value()?)
                                    },

                                    unknown => {
                                        return Err(de::Error::custom(format!(
                                            "Unexpected value {unknown}"
                                        )))
                                    }
                                })?);
                            } else {
                                return Err(de::Error::custom("Processing an array of `hkparam` requires identification by `class` attribute first, but non exist"));
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
