//! The type of enumeration of all C++ havok class fields.
use super::*;
use crate::classes::Class;
use crate::bytes::*;
use crate::error::{HkxError, Result};
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
    #[serde(bound(deserialize = "Box<BgsGamebryoSequenceGenerator<'a>>: Deserialize<'de>"))]
    BgsGamebryoSequenceGenerator(Box<BgsGamebryoSequenceGenerator<'a>>),

    #[serde(rename = "0xc1215be6")]
    #[serde(bound(deserialize = "Box<BsBoneSwitchGeneratorBoneData<'a>>: Deserialize<'de>"))]
    BsBoneSwitchGeneratorBoneData(Box<BsBoneSwitchGeneratorBoneData<'a>>),

    #[serde(rename = "0xf33d3eea")]
    #[serde(bound(deserialize = "Box<BsBoneSwitchGenerator<'a>>: Deserialize<'de>"))]
    BsBoneSwitchGenerator(Box<BsBoneSwitchGenerator<'a>>),

    #[serde(rename = "0xa67f8c46")]
    #[serde(bound(deserialize = "Box<BsComputeAddBoneAnimModifier<'a>>: Deserialize<'de>"))]
    BsComputeAddBoneAnimModifier(Box<BsComputeAddBoneAnimModifier<'a>>),

    #[serde(rename = "0x5119eb06")]
    #[serde(bound(deserialize = "Box<BsCyclicBlendTransitionGenerator<'a>>: Deserialize<'de>"))]
    BsCyclicBlendTransitionGenerator(Box<BsCyclicBlendTransitionGenerator<'a>>),

    #[serde(rename = "0x31f6b8b6")]
    #[serde(bound(deserialize = "Box<BsDecomposeVectorModifier<'a>>: Deserialize<'de>"))]
    BsDecomposeVectorModifier(Box<BsDecomposeVectorModifier<'a>>),

    #[serde(rename = "0x19a005c0")]
    #[serde(bound(deserialize = "Box<BsDirectAtModifier<'a>>: Deserialize<'de>"))]
    BsDirectAtModifier(Box<BsDirectAtModifier<'a>>),

    #[serde(rename = "0xb34d2bbd")]
    #[serde(bound(deserialize = "Box<BsDistTriggerModifier<'a>>: Deserialize<'de>"))]
    BsDistTriggerModifier(Box<BsDistTriggerModifier<'a>>),

    #[serde(rename = "0x6030970c")]
    #[serde(bound(deserialize = "Box<BsEventEveryNEventsModifier<'a>>: Deserialize<'de>"))]
    BsEventEveryNEventsModifier(Box<BsEventEveryNEventsModifier<'a>>),

    #[serde(rename = "0x1062d993")]
    #[serde(bound(deserialize = "Box<BsEventOnDeactivateModifier<'a>>: Deserialize<'de>"))]
    BsEventOnDeactivateModifier(Box<BsEventOnDeactivateModifier<'a>>),

    #[serde(rename = "0x81d0777a")]
    #[serde(bound(deserialize = "Box<BsEventOnFalseToTrueModifier<'a>>: Deserialize<'de>"))]
    BsEventOnFalseToTrueModifier(Box<BsEventOnFalseToTrueModifier<'a>>),

    #[serde(rename = "0xbda33bfe")]
    #[serde(bound(deserialize = "Box<BsGetTimeStepModifier<'a>>: Deserialize<'de>"))]
    BsGetTimeStepModifier(Box<BsGetTimeStepModifier<'a>>),

    #[serde(rename = "0x29adc802")]
    #[serde(bound(deserialize = "Box<BsInterpValueModifier<'a>>: Deserialize<'de>"))]
    BsInterpValueModifier(Box<BsInterpValueModifier<'a>>),

    #[serde(rename = "0xb0fde45a")]
    #[serde(bound(deserialize = "Box<BsIsActiveModifier<'a>>: Deserialize<'de>"))]
    BsIsActiveModifier(Box<BsIsActiveModifier<'a>>),

    #[serde(rename = "0x6b8a15fc")]
    #[serde(bound(deserialize = "Box<BsiStateManagerModifierBSiStateData<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifierBSiStateData(Box<BsiStateManagerModifierBSiStateData<'a>>),

    #[serde(rename = "0x99463586")]
    #[serde(bound(deserialize = "Box<BsiStateManagerModifierBsiStateManagerStateListener<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifierBsiStateManagerStateListener(Box<BsiStateManagerModifierBsiStateManagerStateListener<'a>>),

    #[serde(rename = "0x6cb24f2e")]
    #[serde(bound(deserialize = "Box<BsiStateManagerModifier<'a>>: Deserialize<'de>"))]
    BsiStateManagerModifier(Box<BsiStateManagerModifier<'a>>),

    #[serde(rename = "0xf0826fc1")]
    #[serde(bound(deserialize = "Box<BSiStateTaggingGenerator<'a>>: Deserialize<'de>"))]
    BSiStateTaggingGenerator(Box<BSiStateTaggingGenerator<'a>>),

    #[serde(rename = "0x8ea971e5")]
    #[serde(bound(deserialize = "Box<BsLimbIkModifier<'a>>: Deserialize<'de>"))]
    BsLimbIkModifier(Box<BsLimbIkModifier<'a>>),

    #[serde(rename = "0x29efee59")]
    BsLookAtModifierBoneData(Box<BsLookAtModifierBoneData>),

    #[serde(rename = "0xd756fc25")]
    #[serde(bound(deserialize = "Box<BsLookAtModifier<'a>>: Deserialize<'de>"))]
    BsLookAtModifier(Box<BsLookAtModifier<'a>>),

    #[serde(rename = "0x1e20a97a")]
    #[serde(bound(deserialize = "Box<BsModifyOnceModifier<'a>>: Deserialize<'de>"))]
    BsModifyOnceModifier(Box<BsModifyOnceModifier<'a>>),

    #[serde(rename = "0xb8571122")]
    #[serde(bound(deserialize = "Box<BsOffsetAnimationGenerator<'a>>: Deserialize<'de>"))]
    BsOffsetAnimationGenerator(Box<BsOffsetAnimationGenerator<'a>>),

    #[serde(rename = "0x703d7b66")]
    #[serde(bound(deserialize = "Box<BsPassByTargetTriggerModifier<'a>>: Deserialize<'de>"))]
    BsPassByTargetTriggerModifier(Box<BsPassByTargetTriggerModifier<'a>>),

    #[serde(rename = "0x8003d8ce")]
    #[serde(bound(deserialize = "Box<BsRagdollContactListenerModifier<'a>>: Deserialize<'de>"))]
    BsRagdollContactListenerModifier(Box<BsRagdollContactListenerModifier<'a>>),

    #[serde(rename = "0xd297fda9")]
    #[serde(bound(deserialize = "Box<BsSpeedSamplerModifier<'a>>: Deserialize<'de>"))]
    BsSpeedSamplerModifier(Box<BsSpeedSamplerModifier<'a>>),

    #[serde(rename = "0xd83bea64")]
    #[serde(bound(deserialize = "Box<BsSynchronizedClipGenerator<'a>>: Deserialize<'de>"))]
    BsSynchronizedClipGenerator(Box<BsSynchronizedClipGenerator<'a>>),

    #[serde(rename = "0x531f3292")]
    #[serde(bound(deserialize = "Box<BsTimerModifier<'a>>: Deserialize<'de>"))]
    BsTimerModifier(Box<BsTimerModifier<'a>>),

    #[serde(rename = "0xd2d9a04")]
    #[serde(bound(deserialize = "Box<BsTweenerModifier<'a>>: Deserialize<'de>"))]
    BsTweenerModifier(Box<BsTweenerModifier<'a>>),

    #[serde(rename = "0x1d716a17")]
    HkAabbHalf(Box<HkAabbHalf>),

    #[serde(rename = "0x11e7c11")]
    HkAabbUint32(Box<HkAabbUint32>),

    #[serde(rename = "0x4a948b16")]
    HkAabb(Box<HkAabb>),

    #[serde(rename = "0xda8c7d7d")]
    HkaAnimatedReferenceFrame(Box<HkaAnimatedReferenceFrame>),

    #[serde(rename = "0x66eac971")]
    #[serde(bound(deserialize = "Box<HkaAnimationBinding<'a>>: Deserialize<'de>"))]
    HkaAnimationBinding(Box<HkaAnimationBinding<'a>>),

    #[serde(rename = "0x8dc20333")]
    #[serde(bound(deserialize = "Box<HkaAnimationContainer<'a>>: Deserialize<'de>"))]
    HkaAnimationContainer(Box<HkaAnimationContainer<'a>>),

    #[serde(rename = "0x4bc4c3e0")]
    HkaAnimationPreviewColorContainer(Box<HkaAnimationPreviewColorContainer>),

    #[serde(rename = "0xa6fa7e88")]
    #[serde(bound(deserialize = "Box<HkaAnimation<'a>>: Deserialize<'de>"))]
    HkaAnimation(Box<HkaAnimation<'a>>),

    #[serde(rename = "0x623bf34f")]
    #[serde(bound(deserialize = "Box<HkaAnnotationTrackAnnotation<'a>>: Deserialize<'de>"))]
    HkaAnnotationTrackAnnotation(Box<HkaAnnotationTrackAnnotation<'a>>),

    #[serde(rename = "0xd4114fdd")]
    #[serde(bound(deserialize = "Box<HkaAnnotationTrack<'a>>: Deserialize<'de>"))]
    HkaAnnotationTrack(Box<HkaAnnotationTrack<'a>>),

    #[serde(rename = "0xa8ccd5cf")]
    #[serde(bound(deserialize = "Box<HkaBoneAttachment<'a>>: Deserialize<'de>"))]
    HkaBoneAttachment(Box<HkaBoneAttachment<'a>>),

    #[serde(rename = "0x35912f8a")]
    #[serde(bound(deserialize = "Box<HkaBone<'a>>: Deserialize<'de>"))]
    HkaBone(Box<HkaBone<'a>>),

    #[serde(rename = "0x6d85e445")]
    HkaDefaultAnimatedReferenceFrame(Box<HkaDefaultAnimatedReferenceFrame>),

    #[serde(rename = "0x724a7561")]
    HkaDeltaCompressedAnimationQuantizationFormat(Box<HkaDeltaCompressedAnimationQuantizationFormat>),

    #[serde(rename = "0x90a68d40")]
    #[serde(bound(deserialize = "Box<HkaDeltaCompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaDeltaCompressedAnimation(Box<HkaDeltaCompressedAnimation<'a>>),

    #[serde(rename = "0x1d81207c")]
    #[serde(bound(deserialize = "Box<HkaFootstepAnalysisInfoContainer<'a>>: Deserialize<'de>"))]
    HkaFootstepAnalysisInfoContainer(Box<HkaFootstepAnalysisInfoContainer<'a>>),

    #[serde(rename = "0x824faf75")]
    HkaFootstepAnalysisInfo(Box<HkaFootstepAnalysisInfo>),

    #[serde(rename = "0x930af031")]
    #[serde(bound(deserialize = "Box<HkaInterleavedUncompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaInterleavedUncompressedAnimation(Box<HkaInterleavedUncompressedAnimation<'a>>),

    #[serde(rename = "0xa3d0ac71")]
    HkaKeyFrameHierarchyUtilityControlData(Box<HkaKeyFrameHierarchyUtilityControlData>),

    #[serde(rename = "0x7bd5c66f")]
    HkaKeyFrameHierarchyUtility(Box<HkaKeyFrameHierarchyUtility>),

    #[serde(rename = "0x207cb01")]
    #[serde(bound(deserialize = "Box<HkAlignSceneToNodeOptions<'a>>: Deserialize<'de>"))]
    HkAlignSceneToNodeOptions(Box<HkAlignSceneToNodeOptions<'a>>),

    #[serde(rename = "0x48aceb75")]
    HkaMeshBindingMapping(Box<HkaMeshBindingMapping>),

    #[serde(rename = "0x81d9950b")]
    #[serde(bound(deserialize = "Box<HkaMeshBinding<'a>>: Deserialize<'de>"))]
    HkaMeshBinding(Box<HkaMeshBinding<'a>>),

    #[serde(rename = "0xf7d64649")]
    HkaQuantizedAnimationTrackCompressionParams(Box<HkaQuantizedAnimationTrackCompressionParams>),

    #[serde(rename = "0x3920f053")]
    #[serde(bound(deserialize = "Box<HkaQuantizedAnimation<'a>>: Deserialize<'de>"))]
    HkaQuantizedAnimation(Box<HkaQuantizedAnimation<'a>>),

    #[serde(rename = "0x154948e8")]
    #[serde(bound(deserialize = "Box<HkaRagdollInstance<'a>>: Deserialize<'de>"))]
    HkaRagdollInstance(Box<HkaRagdollInstance<'a>>),

    #[serde(rename = "0xd404a39a")]
    HkArrayTypeAttribute(Box<HkArrayTypeAttribute>),

    #[serde(rename = "0x52e8043")]
    #[serde(bound(deserialize = "Box<HkaSkeletonLocalFrameOnBone<'a>>: Deserialize<'de>"))]
    HkaSkeletonLocalFrameOnBone(Box<HkaSkeletonLocalFrameOnBone<'a>>),

    #[serde(rename = "0xa528f7cf")]
    HkaSkeletonMapperDataChainMapping(Box<HkaSkeletonMapperDataChainMapping>),

    #[serde(rename = "0x3405deca")]
    HkaSkeletonMapperDataSimpleMapping(Box<HkaSkeletonMapperDataSimpleMapping>),

    #[serde(rename = "0x95687ea0")]
    #[serde(bound(deserialize = "Box<HkaSkeletonMapperData<'a>>: Deserialize<'de>"))]
    HkaSkeletonMapperData(Box<HkaSkeletonMapperData<'a>>),

    #[serde(rename = "0x12df42a5")]
    #[serde(bound(deserialize = "Box<HkaSkeletonMapper<'a>>: Deserialize<'de>"))]
    HkaSkeletonMapper(Box<HkaSkeletonMapper<'a>>),

    #[serde(rename = "0x366e8220")]
    #[serde(bound(deserialize = "Box<HkaSkeleton<'a>>: Deserialize<'de>"))]
    HkaSkeleton(Box<HkaSkeleton<'a>>),

    #[serde(rename = "0xde830789")]
    HkaSplineCompressedAnimationAnimationCompressionParams(Box<HkaSplineCompressedAnimationAnimationCompressionParams>),

    #[serde(rename = "0x42e878d3")]
    HkaSplineCompressedAnimationTrackCompressionParams(Box<HkaSplineCompressedAnimationTrackCompressionParams>),

    #[serde(rename = "0x792ee0bb")]
    #[serde(bound(deserialize = "Box<HkaSplineCompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaSplineCompressedAnimation(Box<HkaSplineCompressedAnimation<'a>>),

    #[serde(rename = "0x27c6cafa")]
    HkaWaveletCompressedAnimationCompressionParams(Box<HkaWaveletCompressedAnimationCompressionParams>),

    #[serde(rename = "0x724a7561")]
    HkaWaveletCompressedAnimationQuantizationFormat(Box<HkaWaveletCompressedAnimationQuantizationFormat>),

    #[serde(rename = "0x77cf0962")]
    #[serde(bound(deserialize = "Box<HkaWaveletCompressedAnimation<'a>>: Deserialize<'de>"))]
    HkaWaveletCompressedAnimation(Box<HkaWaveletCompressedAnimation<'a>>),

    #[serde(rename = "0xe0708a00")]
    HkBaseObject(Box<HkBaseObject>),

    #[serde(rename = "0xcc0aab32")]
    #[serde(bound(deserialize = "Box<HkbAttachmentModifier<'a>>: Deserialize<'de>"))]
    HkbAttachmentModifier(Box<HkbAttachmentModifier<'a>>),

    #[serde(rename = "0x774632b")]
    HkbAttachmentSetup(Box<HkbAttachmentSetup>),

    #[serde(rename = "0x48b8ad52")]
    HkbAttributeModifierAssignment(Box<HkbAttributeModifierAssignment>),

    #[serde(rename = "0x1245d97d")]
    #[serde(bound(deserialize = "Box<HkbAttributeModifier<'a>>: Deserialize<'de>"))]
    HkbAttributeModifier(Box<HkbAttributeModifier<'a>>),

    #[serde(rename = "0xca0888ca")]
    #[serde(bound(deserialize = "Box<HkbAuxiliaryNodeInfo<'a>>: Deserialize<'de>"))]
    HkbAuxiliaryNodeInfo(Box<HkbAuxiliaryNodeInfo<'a>>),

    #[serde(rename = "0x66840004")]
    HkbBehaviorEventsInfo(Box<HkbBehaviorEventsInfo>),

    #[serde(rename = "0x95aca5d")]
    #[serde(bound(deserialize = "Box<HkbBehaviorGraphData<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphData(Box<HkbBehaviorGraphData<'a>>),

    #[serde(rename = "0x645f898b")]
    #[serde(bound(deserialize = "Box<HkbBehaviorGraphInternalStateInfo<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphInternalStateInfo(Box<HkbBehaviorGraphInternalStateInfo<'a>>),

    #[serde(rename = "0x8699b6eb")]
    #[serde(bound(deserialize = "Box<HkbBehaviorGraphInternalState<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphInternalState(Box<HkbBehaviorGraphInternalState<'a>>),

    #[serde(rename = "0xc713064e")]
    #[serde(bound(deserialize = "Box<HkbBehaviorGraphStringData<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraphStringData(Box<HkbBehaviorGraphStringData<'a>>),

    #[serde(rename = "0xb1218f86")]
    #[serde(bound(deserialize = "Box<HkbBehaviorGraph<'a>>: Deserialize<'de>"))]
    HkbBehaviorGraph(Box<HkbBehaviorGraph<'a>>),

    #[serde(rename = "0x35a0439a")]
    #[serde(bound(deserialize = "Box<HkbBehaviorInfoIdToNamePair<'a>>: Deserialize<'de>"))]
    HkbBehaviorInfoIdToNamePair(Box<HkbBehaviorInfoIdToNamePair<'a>>),

    #[serde(rename = "0xf7645395")]
    #[serde(bound(deserialize = "Box<HkbBehaviorInfo<'a>>: Deserialize<'de>"))]
    HkbBehaviorInfo(Box<HkbBehaviorInfo<'a>>),

    #[serde(rename = "0xfcb5423")]
    #[serde(bound(deserialize = "Box<HkbBehaviorReferenceGenerator<'a>>: Deserialize<'de>"))]
    HkbBehaviorReferenceGenerator(Box<HkbBehaviorReferenceGenerator<'a>>),

    #[serde(rename = "0x2c1432d7")]
    #[serde(bound(deserialize = "Box<HkbBindable<'a>>: Deserialize<'de>"))]
    HkbBindable(Box<HkbBindable<'a>>),

    #[serde(rename = "0x23041af0")]
    HkbBlendCurveUtils(Box<HkbBlendCurveUtils>),

    #[serde(rename = "0xff7327c0")]
    HkbBlenderGeneratorChildInternalState(Box<HkbBlenderGeneratorChildInternalState>),

    #[serde(rename = "0xe2b384b0")]
    #[serde(bound(deserialize = "Box<HkbBlenderGeneratorChild<'a>>: Deserialize<'de>"))]
    HkbBlenderGeneratorChild(Box<HkbBlenderGeneratorChild<'a>>),

    #[serde(rename = "0x84717488")]
    HkbBlenderGeneratorInternalState(Box<HkbBlenderGeneratorInternalState>),

    #[serde(rename = "0x22df7147")]
    #[serde(bound(deserialize = "Box<HkbBlenderGenerator<'a>>: Deserialize<'de>"))]
    HkbBlenderGenerator(Box<HkbBlenderGenerator<'a>>),

    #[serde(rename = "0xb18c70c2")]
    HkbBlendingTransitionEffectInternalState(Box<HkbBlendingTransitionEffectInternalState>),

    #[serde(rename = "0xfd8584fe")]
    #[serde(bound(deserialize = "Box<HkbBlendingTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbBlendingTransitionEffect(Box<HkbBlendingTransitionEffect<'a>>),

    #[serde(rename = "0xaa8619")]
    #[serde(bound(deserialize = "Box<HkbBoneIndexArray<'a>>: Deserialize<'de>"))]
    HkbBoneIndexArray(Box<HkbBoneIndexArray<'a>>),

    #[serde(rename = "0xcd902b77")]
    #[serde(bound(deserialize = "Box<HkbBoneWeightArray<'a>>: Deserialize<'de>"))]
    HkbBoneWeightArray(Box<HkbBoneWeightArray<'a>>),

    #[serde(rename = "0x514763dc")]
    HkbBoolVariableSequencedDataSample(Box<HkbBoolVariableSequencedDataSample>),

    #[serde(rename = "0x37416fce")]
    HkbBoolVariableSequencedData(Box<HkbBoolVariableSequencedData>),

    #[serde(rename = "0x64136982")]
    HkbCameraShakeEventPayload(Box<HkbCameraShakeEventPayload>),

    #[serde(rename = "0x3544e182")]
    #[serde(bound(deserialize = "Box<HkbCharacterAddedInfo<'a>>: Deserialize<'de>"))]
    HkbCharacterAddedInfo(Box<HkbCharacterAddedInfo<'a>>),

    #[serde(rename = "0x7a195d1d")]
    HkbCharacterControlCommand(Box<HkbCharacterControlCommand>),

    #[serde(rename = "0x5b6c03d9")]
    HkbCharacterControllerControlData(Box<HkbCharacterControllerControlData>),

    #[serde(rename = "0xf8dfec0d")]
    HkbCharacterControllerModifierInternalState(Box<HkbCharacterControllerModifierInternalState>),

    #[serde(rename = "0xf675d6fb")]
    #[serde(bound(deserialize = "Box<HkbCharacterControllerModifier<'a>>: Deserialize<'de>"))]
    HkbCharacterControllerModifier(Box<HkbCharacterControllerModifier<'a>>),

    #[serde(rename = "0xa0f415bf")]
    #[serde(bound(deserialize = "Box<HkbCharacterDataCharacterControllerInfo<'a>>: Deserialize<'de>"))]
    HkbCharacterDataCharacterControllerInfo(Box<HkbCharacterDataCharacterControllerInfo<'a>>),

    #[serde(rename = "0x300d6808")]
    #[serde(bound(deserialize = "Box<HkbCharacterData<'a>>: Deserialize<'de>"))]
    HkbCharacterData(Box<HkbCharacterData<'a>>),

    #[serde(rename = "0xd9709ff2")]
    HkbCharacterInfo(Box<HkbCharacterInfo>),

    #[serde(rename = "0xe5a2a413")]
    #[serde(bound(deserialize = "Box<HkbCharacterSetup<'a>>: Deserialize<'de>"))]
    HkbCharacterSetup(Box<HkbCharacterSetup<'a>>),

    #[serde(rename = "0x180d900d")]
    HkbCharacterSkinInfo(Box<HkbCharacterSkinInfo>),

    #[serde(rename = "0x2eda84f8")]
    HkbCharacterSteppedInfo(Box<HkbCharacterSteppedInfo>),

    #[serde(rename = "0x655b42bc")]
    #[serde(bound(deserialize = "Box<HkbCharacterStringData<'a>>: Deserialize<'de>"))]
    HkbCharacterStringData(Box<HkbCharacterStringData<'a>>),

    #[serde(rename = "0x3088a5c5")]
    #[serde(bound(deserialize = "Box<HkbCharacter<'a>>: Deserialize<'de>"))]
    HkbCharacter(Box<HkbCharacter<'a>>),

    #[serde(rename = "0xa2624c97")]
    #[serde(bound(deserialize = "Box<HkbClientCharacterState<'a>>: Deserialize<'de>"))]
    HkbClientCharacterState(Box<HkbClientCharacterState<'a>>),

    #[serde(rename = "0x750edf40")]
    HkbClipGeneratorEcho(Box<HkbClipGeneratorEcho>),

    #[serde(rename = "0x26ce5bf3")]
    HkbClipGeneratorInternalState(Box<HkbClipGeneratorInternalState>),

    #[serde(rename = "0x333b85b9")]
    #[serde(bound(deserialize = "Box<HkbClipGenerator<'a>>: Deserialize<'de>"))]
    HkbClipGenerator(Box<HkbClipGenerator<'a>>),

    #[serde(rename = "0x59c23a0f")]
    #[serde(bound(deserialize = "Box<HkbClipTriggerArray<'a>>: Deserialize<'de>"))]
    HkbClipTriggerArray(Box<HkbClipTriggerArray<'a>>),

    #[serde(rename = "0x7eb45cea")]
    #[serde(bound(deserialize = "Box<HkbClipTrigger<'a>>: Deserialize<'de>"))]
    HkbClipTrigger(Box<HkbClipTrigger<'a>>),

    #[serde(rename = "0xa92ed39f")]
    HkbCombineTransformsModifierInternalState(Box<HkbCombineTransformsModifierInternalState>),

    #[serde(rename = "0xfd1f0b79")]
    #[serde(bound(deserialize = "Box<HkbCombineTransformsModifier<'a>>: Deserialize<'de>"))]
    HkbCombineTransformsModifier(Box<HkbCombineTransformsModifier<'a>>),

    #[serde(rename = "0xc6aaccc8")]
    HkbCompiledExpressionSetToken(Box<HkbCompiledExpressionSetToken>),

    #[serde(rename = "0x3a7d76cc")]
    HkbCompiledExpressionSet(Box<HkbCompiledExpressionSet>),

    #[serde(rename = "0x6ac054d7")]
    HkbComputeDirectionModifierInternalState(Box<HkbComputeDirectionModifierInternalState>),

    #[serde(rename = "0xdf358bd3")]
    #[serde(bound(deserialize = "Box<HkbComputeDirectionModifier<'a>>: Deserialize<'de>"))]
    HkbComputeDirectionModifier(Box<HkbComputeDirectionModifier<'a>>),

    #[serde(rename = "0x71cd1eb0")]
    HkbComputeRotationFromAxisAngleModifierInternalState(Box<HkbComputeRotationFromAxisAngleModifierInternalState>),

    #[serde(rename = "0x9b3f6936")]
    #[serde(bound(deserialize = "Box<HkbComputeRotationFromAxisAngleModifier<'a>>: Deserialize<'de>"))]
    HkbComputeRotationFromAxisAngleModifier(Box<HkbComputeRotationFromAxisAngleModifier<'a>>),

    #[serde(rename = "0x71cd1eb0")]
    HkbComputeRotationToTargetModifierInternalState(Box<HkbComputeRotationToTargetModifierInternalState>),

    #[serde(rename = "0x47665f1c")]
    #[serde(bound(deserialize = "Box<HkbComputeRotationToTargetModifier<'a>>: Deserialize<'de>"))]
    HkbComputeRotationToTargetModifier(Box<HkbComputeRotationToTargetModifier<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbCondition(Box<HkbCondition>),

    #[serde(rename = "0xe0c4d4a7")]
    #[serde(bound(deserialize = "Box<HkbContext<'a>>: Deserialize<'de>"))]
    HkbContext(Box<HkbContext<'a>>),

    #[serde(rename = "0x508d3b36")]
    HkbDampingModifierInternalState(Box<HkbDampingModifierInternalState>),

    #[serde(rename = "0x9a040f03")]
    #[serde(bound(deserialize = "Box<HkbDampingModifier<'a>>: Deserialize<'de>"))]
    HkbDampingModifier(Box<HkbDampingModifier<'a>>),

    #[serde(rename = "0x7bd5c66f")]
    HkbDefaultMessageLog(Box<HkbDefaultMessageLog>),

    #[serde(rename = "0x85fb0b80")]
    HkbDelayedModifierInternalState(Box<HkbDelayedModifierInternalState>),

    #[serde(rename = "0x8e101a7a")]
    #[serde(bound(deserialize = "Box<HkbDelayedModifier<'a>>: Deserialize<'de>"))]
    HkbDelayedModifier(Box<HkbDelayedModifier<'a>>),

    #[serde(rename = "0x7b32d942")]
    HkbDetectCloseToGroundModifierInternalState(Box<HkbDetectCloseToGroundModifierInternalState>),

    #[serde(rename = "0x981687b2")]
    #[serde(bound(deserialize = "Box<HkbDetectCloseToGroundModifier<'a>>: Deserialize<'de>"))]
    HkbDetectCloseToGroundModifier(Box<HkbDetectCloseToGroundModifier<'a>>),

    #[serde(rename = "0xb8686f6b")]
    HkbEvaluateExpressionModifierInternalExpressionData(Box<HkbEvaluateExpressionModifierInternalExpressionData>),

    #[serde(rename = "0xb414d58e")]
    HkbEvaluateExpressionModifierInternalState(Box<HkbEvaluateExpressionModifierInternalState>),

    #[serde(rename = "0xf900f6be")]
    #[serde(bound(deserialize = "Box<HkbEvaluateExpressionModifier<'a>>: Deserialize<'de>"))]
    HkbEvaluateExpressionModifier(Box<HkbEvaluateExpressionModifier<'a>>),

    #[serde(rename = "0x79757102")]
    #[serde(bound(deserialize = "Box<HkbEvaluateHandleModifier<'a>>: Deserialize<'de>"))]
    HkbEvaluateHandleModifier(Box<HkbEvaluateHandleModifier<'a>>),

    #[serde(rename = "0x76bddb31")]
    #[serde(bound(deserialize = "Box<HkbEventBase<'a>>: Deserialize<'de>"))]
    HkbEventBase(Box<HkbEventBase<'a>>),

    #[serde(rename = "0xd14bf000")]
    HkbEventDrivenModifierInternalState(Box<HkbEventDrivenModifierInternalState>),

    #[serde(rename = "0x7ed3f44e")]
    #[serde(bound(deserialize = "Box<HkbEventDrivenModifier<'a>>: Deserialize<'de>"))]
    HkbEventDrivenModifier(Box<HkbEventDrivenModifier<'a>>),

    #[serde(rename = "0x5874eed4")]
    HkbEventInfo(Box<HkbEventInfo>),

    #[serde(rename = "0x3d2dbd34")]
    #[serde(bound(deserialize = "Box<HkbEventPayloadList<'a>>: Deserialize<'de>"))]
    HkbEventPayloadList(Box<HkbEventPayloadList<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbEventPayload(Box<HkbEventPayload>),

    #[serde(rename = "0xdb38a15")]
    #[serde(bound(deserialize = "Box<HkbEventProperty<'a>>: Deserialize<'de>"))]
    HkbEventProperty(Box<HkbEventProperty<'a>>),

    #[serde(rename = "0xc02da3")]
    #[serde(bound(deserialize = "Box<HkbEventRaisedInfo<'a>>: Deserialize<'de>"))]
    HkbEventRaisedInfo(Box<HkbEventRaisedInfo<'a>>),

    #[serde(rename = "0x330a56ee")]
    #[serde(bound(deserialize = "Box<HkbEventRangeDataArray<'a>>: Deserialize<'de>"))]
    HkbEventRangeDataArray(Box<HkbEventRangeDataArray<'a>>),

    #[serde(rename = "0x6cb92c76")]
    #[serde(bound(deserialize = "Box<HkbEventRangeData<'a>>: Deserialize<'de>"))]
    HkbEventRangeData(Box<HkbEventRangeData<'a>>),

    #[serde(rename = "0x9139b821")]
    #[serde(bound(deserialize = "Box<HkbEventSequencedDataSequencedEvent<'a>>: Deserialize<'de>"))]
    HkbEventSequencedDataSequencedEvent(Box<HkbEventSequencedDataSequencedEvent<'a>>),

    #[serde(rename = "0x76798eb8")]
    #[serde(bound(deserialize = "Box<HkbEventSequencedData<'a>>: Deserialize<'de>"))]
    HkbEventSequencedData(Box<HkbEventSequencedData<'a>>),

    #[serde(rename = "0xcc47b48d")]
    HkbEventsFromRangeModifierInternalState(Box<HkbEventsFromRangeModifierInternalState>),

    #[serde(rename = "0xbc561b6e")]
    #[serde(bound(deserialize = "Box<HkbEventsFromRangeModifier<'a>>: Deserialize<'de>"))]
    HkbEventsFromRangeModifier(Box<HkbEventsFromRangeModifier<'a>>),

    #[serde(rename = "0x3e0fd810")]
    #[serde(bound(deserialize = "Box<HkbEvent<'a>>: Deserialize<'de>"))]
    HkbEvent(Box<HkbEvent<'a>>),

    #[serde(rename = "0x1c3c1045")]
    #[serde(bound(deserialize = "Box<HkbExpressionCondition<'a>>: Deserialize<'de>"))]
    HkbExpressionCondition(Box<HkbExpressionCondition<'a>>),

    #[serde(rename = "0x4b9ee1a2")]
    #[serde(bound(deserialize = "Box<HkbExpressionDataArray<'a>>: Deserialize<'de>"))]
    HkbExpressionDataArray(Box<HkbExpressionDataArray<'a>>),

    #[serde(rename = "0x6740042a")]
    #[serde(bound(deserialize = "Box<HkbExpressionData<'a>>: Deserialize<'de>"))]
    HkbExpressionData(Box<HkbExpressionData<'a>>),

    #[serde(rename = "0x804dcbab")]
    #[serde(bound(deserialize = "Box<HkbExtractRagdollPoseModifier<'a>>: Deserialize<'de>"))]
    HkbExtractRagdollPoseModifier(Box<HkbExtractRagdollPoseModifier<'a>>),

    #[serde(rename = "0xa111b704")]
    HkbFootIkControlData(Box<HkbFootIkControlData>),

    #[serde(rename = "0x9e17091a")]
    #[serde(bound(deserialize = "Box<HkbFootIkControlsModifierLeg<'a>>: Deserialize<'de>"))]
    HkbFootIkControlsModifierLeg(Box<HkbFootIkControlsModifierLeg<'a>>),

    #[serde(rename = "0xe5b6f544")]
    #[serde(bound(deserialize = "Box<HkbFootIkControlsModifier<'a>>: Deserialize<'de>"))]
    HkbFootIkControlsModifier(Box<HkbFootIkControlsModifier<'a>>),

    #[serde(rename = "0x224b18d1")]
    HkbFootIkDriverInfoLeg(Box<HkbFootIkDriverInfoLeg>),

    #[serde(rename = "0xc6a09dbf")]
    HkbFootIkDriverInfo(Box<HkbFootIkDriverInfo>),

    #[serde(rename = "0xa681b7f0")]
    HkbFootIkGains(Box<HkbFootIkGains>),

    #[serde(rename = "0xe5ca3677")]
    #[serde(bound(deserialize = "Box<HkbFootIkModifierInternalLegData<'a>>: Deserialize<'de>"))]
    HkbFootIkModifierInternalLegData(Box<HkbFootIkModifierInternalLegData<'a>>),

    #[serde(rename = "0x9f3e3a04")]
    #[serde(bound(deserialize = "Box<HkbFootIkModifierLeg<'a>>: Deserialize<'de>"))]
    HkbFootIkModifierLeg(Box<HkbFootIkModifierLeg<'a>>),

    #[serde(rename = "0xed8966c0")]
    #[serde(bound(deserialize = "Box<HkbFootIkModifier<'a>>: Deserialize<'de>"))]
    HkbFootIkModifier(Box<HkbFootIkModifier<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbGeneratorOutputListener(Box<HkbGeneratorOutputListener>),

    #[serde(rename = "0xb597cf92")]
    HkbGeneratorSyncInfoSyncPoint(Box<HkbGeneratorSyncInfoSyncPoint>),

    #[serde(rename = "0xa3c341f8")]
    HkbGeneratorSyncInfo(Box<HkbGeneratorSyncInfo>),

    #[serde(rename = "0xd6692b5d")]
    HkbGeneratorTransitionEffectInternalState(Box<HkbGeneratorTransitionEffectInternalState>),

    #[serde(rename = "0x5f771b12")]
    #[serde(bound(deserialize = "Box<HkbGeneratorTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbGeneratorTransitionEffect(Box<HkbGeneratorTransitionEffect<'a>>),

    #[serde(rename = "0xd68aefc")]
    #[serde(bound(deserialize = "Box<HkbGenerator<'a>>: Deserialize<'de>"))]
    HkbGenerator(Box<HkbGenerator<'a>>),

    #[serde(rename = "0x50c34a17")]
    #[serde(bound(deserialize = "Box<HkbGetHandleOnBoneModifier<'a>>: Deserialize<'de>"))]
    HkbGetHandleOnBoneModifier(Box<HkbGetHandleOnBoneModifier<'a>>),

    #[serde(rename = "0xd84cad4a")]
    HkbGetUpModifierInternalState(Box<HkbGetUpModifierInternalState>),

    #[serde(rename = "0x61cb7ac0")]
    #[serde(bound(deserialize = "Box<HkbGetUpModifier<'a>>: Deserialize<'de>"))]
    HkbGetUpModifier(Box<HkbGetUpModifier<'a>>),

    #[serde(rename = "0xa92ed39f")]
    HkbGetWorldFromModelModifierInternalState(Box<HkbGetWorldFromModelModifierInternalState>),

    #[serde(rename = "0x873fc6f7")]
    #[serde(bound(deserialize = "Box<HkbGetWorldFromModelModifier<'a>>: Deserialize<'de>"))]
    HkbGetWorldFromModelModifier(Box<HkbGetWorldFromModelModifier<'a>>),

    #[serde(rename = "0xd72b8d17")]
    #[serde(bound(deserialize = "Box<HkbHandIkControlData<'a>>: Deserialize<'de>"))]
    HkbHandIkControlData(Box<HkbHandIkControlData<'a>>),

    #[serde(rename = "0x9c72e9e3")]
    #[serde(bound(deserialize = "Box<HkbHandIkControlsModifierHand<'a>>: Deserialize<'de>"))]
    HkbHandIkControlsModifierHand(Box<HkbHandIkControlsModifierHand<'a>>),

    #[serde(rename = "0x9f0488bb")]
    #[serde(bound(deserialize = "Box<HkbHandIkControlsModifier<'a>>: Deserialize<'de>"))]
    HkbHandIkControlsModifier(Box<HkbHandIkControlsModifier<'a>>),

    #[serde(rename = "0x14dfe1dd")]
    #[serde(bound(deserialize = "Box<HkbHandIkDriverInfoHand<'a>>: Deserialize<'de>"))]
    HkbHandIkDriverInfoHand(Box<HkbHandIkDriverInfoHand<'a>>),

    #[serde(rename = "0xc299090a")]
    #[serde(bound(deserialize = "Box<HkbHandIkDriverInfo<'a>>: Deserialize<'de>"))]
    HkbHandIkDriverInfo(Box<HkbHandIkDriverInfo<'a>>),

    #[serde(rename = "0x14dfe1dd")]
    #[serde(bound(deserialize = "Box<HkbHandIkModifierHand<'a>>: Deserialize<'de>"))]
    HkbHandIkModifierHand(Box<HkbHandIkModifierHand<'a>>),

    #[serde(rename = "0xef8bc2f7")]
    #[serde(bound(deserialize = "Box<HkbHandIkModifier<'a>>: Deserialize<'de>"))]
    HkbHandIkModifier(Box<HkbHandIkModifier<'a>>),

    #[serde(rename = "0xd8b6401c")]
    #[serde(bound(deserialize = "Box<HkbHandle<'a>>: Deserialize<'de>"))]
    HkbHandle(Box<HkbHandle<'a>>),

    #[serde(rename = "0xebbc1bd3")]
    HkbIntEventPayload(Box<HkbIntEventPayload>),

    #[serde(rename = "0xbe7ac63c")]
    HkbIntVariableSequencedDataSample(Box<HkbIntVariableSequencedDataSample>),

    #[serde(rename = "0x7bfc518a")]
    HkbIntVariableSequencedData(Box<HkbIntVariableSequencedData>),

    #[serde(rename = "0xda41bd9b")]
    HkBitField(Box<HkBitField>),

    #[serde(rename = "0x72deb7a6")]
    HkbKeyframeBonesModifierKeyframeInfo(Box<HkbKeyframeBonesModifierKeyframeInfo>),

    #[serde(rename = "0x95f66629")]
    #[serde(bound(deserialize = "Box<HkbKeyframeBonesModifier<'a>>: Deserialize<'de>"))]
    HkbKeyframeBonesModifier(Box<HkbKeyframeBonesModifier<'a>>),

    #[serde(rename = "0x6a5094e3")]
    #[serde(bound(deserialize = "Box<HkbLinkedSymbolInfo<'a>>: Deserialize<'de>"))]
    HkbLinkedSymbolInfo(Box<HkbLinkedSymbolInfo<'a>>),

    #[serde(rename = "0xa14caba6")]
    HkbLookAtModifierInternalState(Box<HkbLookAtModifierInternalState>),

    #[serde(rename = "0x3d28e066")]
    #[serde(bound(deserialize = "Box<HkbLookAtModifier<'a>>: Deserialize<'de>"))]
    HkbLookAtModifier(Box<HkbLookAtModifier<'a>>),

    #[serde(rename = "0x492c6137")]
    HkbManualSelectorGeneratorInternalState(Box<HkbManualSelectorGeneratorInternalState>),

    #[serde(rename = "0xd932fab8")]
    #[serde(bound(deserialize = "Box<HkbManualSelectorGenerator<'a>>: Deserialize<'de>"))]
    HkbManualSelectorGenerator(Box<HkbManualSelectorGenerator<'a>>),

    #[serde(rename = "0x26a196c5")]
    #[serde(bound(deserialize = "Box<HkbMessageLog<'a>>: Deserialize<'de>"))]
    HkbMessageLog(Box<HkbMessageLog<'a>>),

    #[serde(rename = "0xc6c2da4f")]
    HkbMirroredSkeletonInfo(Box<HkbMirroredSkeletonInfo>),

    #[serde(rename = "0xa9a271ea")]
    #[serde(bound(deserialize = "Box<HkbMirrorModifier<'a>>: Deserialize<'de>"))]
    HkbMirrorModifier(Box<HkbMirrorModifier<'a>>),

    #[serde(rename = "0x1f81fae6")]
    #[serde(bound(deserialize = "Box<HkbModifierGenerator<'a>>: Deserialize<'de>"))]
    HkbModifierGenerator(Box<HkbModifierGenerator<'a>>),

    #[serde(rename = "0xa4180ca1")]
    #[serde(bound(deserialize = "Box<HkbModifierList<'a>>: Deserialize<'de>"))]
    HkbModifierList(Box<HkbModifierList<'a>>),

    #[serde(rename = "0x3697e044")]
    #[serde(bound(deserialize = "Box<HkbModifierWrapper<'a>>: Deserialize<'de>"))]
    HkbModifierWrapper(Box<HkbModifierWrapper<'a>>),

    #[serde(rename = "0x96ec5ced")]
    #[serde(bound(deserialize = "Box<HkbModifier<'a>>: Deserialize<'de>"))]
    HkbModifier(Box<HkbModifier<'a>>),

    #[serde(rename = "0x28f67ba0")]
    HkbMoveCharacterModifierInternalState(Box<HkbMoveCharacterModifierInternalState>),

    #[serde(rename = "0x8f7492a0")]
    #[serde(bound(deserialize = "Box<HkbMoveCharacterModifier<'a>>: Deserialize<'de>"))]
    HkbMoveCharacterModifier(Box<HkbMoveCharacterModifier<'a>>),

    #[serde(rename = "0x65bdd3a0")]
    #[serde(bound(deserialize = "Box<HkbNamedEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedEventPayload(Box<HkbNamedEventPayload<'a>>),

    #[serde(rename = "0x3c99bda4")]
    #[serde(bound(deserialize = "Box<HkbNamedIntEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedIntEventPayload(Box<HkbNamedIntEventPayload<'a>>),

    #[serde(rename = "0x9c99fd70")]
    #[serde(bound(deserialize = "Box<HkbNamedRealEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedRealEventPayload(Box<HkbNamedRealEventPayload<'a>>),

    #[serde(rename = "0x6caa9113")]
    #[serde(bound(deserialize = "Box<HkbNamedStringEventPayload<'a>>: Deserialize<'de>"))]
    HkbNamedStringEventPayload(Box<HkbNamedStringEventPayload<'a>>),

    #[serde(rename = "0x7db9971d")]
    #[serde(bound(deserialize = "Box<HkbNodeInternalStateInfo<'a>>: Deserialize<'de>"))]
    HkbNodeInternalStateInfo(Box<HkbNodeInternalStateInfo<'a>>),

    #[serde(rename = "0x6d26f61d")]
    #[serde(bound(deserialize = "Box<HkbNode<'a>>: Deserialize<'de>"))]
    HkbNode(Box<HkbNode<'a>>),

    #[serde(rename = "0x9df46cd6")]
    HkbParticleSystemEventPayload(Box<HkbParticleSystemEventPayload>),

    #[serde(rename = "0x552d9dd4")]
    HkbPoseMatchingGeneratorInternalState(Box<HkbPoseMatchingGeneratorInternalState>),

    #[serde(rename = "0x29e271b4")]
    #[serde(bound(deserialize = "Box<HkbPoseMatchingGenerator<'a>>: Deserialize<'de>"))]
    HkbPoseMatchingGenerator(Box<HkbPoseMatchingGenerator<'a>>),

    #[serde(rename = "0xf5ba21b")]
    HkbPoweredRagdollControlData(Box<HkbPoweredRagdollControlData>),

    #[serde(rename = "0x7cb54065")]
    #[serde(bound(deserialize = "Box<HkbPoweredRagdollControlsModifier<'a>>: Deserialize<'de>"))]
    HkbPoweredRagdollControlsModifier(Box<HkbPoweredRagdollControlsModifier<'a>>),

    #[serde(rename = "0x13a39ba7")]
    #[serde(bound(deserialize = "Box<HkbProjectData<'a>>: Deserialize<'de>"))]
    HkbProjectData(Box<HkbProjectData<'a>>),

    #[serde(rename = "0x76ad60a")]
    #[serde(bound(deserialize = "Box<HkbProjectStringData<'a>>: Deserialize<'de>"))]
    HkbProjectStringData(Box<HkbProjectStringData<'a>>),

    #[serde(rename = "0x39de637e")]
    HkbProxyModifierProxyInfo(Box<HkbProxyModifierProxyInfo>),

    #[serde(rename = "0x8a41554f")]
    #[serde(bound(deserialize = "Box<HkbProxyModifier<'a>>: Deserialize<'de>"))]
    HkbProxyModifier(Box<HkbProxyModifier<'a>>),

    #[serde(rename = "0xa0a7bf9c")]
    HkbRaiseEventCommand(Box<HkbRaiseEventCommand>),

    #[serde(rename = "0x9416affd")]
    HkbRealEventPayload(Box<HkbRealEventPayload>),

    #[serde(rename = "0xbb708bbd")]
    HkbRealVariableSequencedDataSample(Box<HkbRealVariableSequencedDataSample>),

    #[serde(rename = "0xe2862d02")]
    HkbRealVariableSequencedData(Box<HkbRealVariableSequencedData>),

    #[serde(rename = "0x26a5675a")]
    #[serde(bound(deserialize = "Box<HkbReferencePoseGenerator<'a>>: Deserialize<'de>"))]
    HkbReferencePoseGenerator(Box<HkbReferencePoseGenerator<'a>>),

    #[serde(rename = "0x58b1d082")]
    #[serde(bound(deserialize = "Box<HkbRegisteredGenerator<'a>>: Deserialize<'de>"))]
    HkbRegisteredGenerator(Box<HkbRegisteredGenerator<'a>>),

    #[serde(rename = "0x1e0bc068")]
    HkbRigidBodyRagdollControlData(Box<HkbRigidBodyRagdollControlData>),

    #[serde(rename = "0xaa87d1eb")]
    #[serde(bound(deserialize = "Box<HkbRigidBodyRagdollControlsModifier<'a>>: Deserialize<'de>"))]
    HkbRigidBodyRagdollControlsModifier(Box<HkbRigidBodyRagdollControlsModifier<'a>>),

    #[serde(rename = "0x3eb2e082")]
    HkbRoleAttribute(Box<HkbRoleAttribute>),

    #[serde(rename = "0xdc40bf4a")]
    HkbRotateCharacterModifierInternalState(Box<HkbRotateCharacterModifierInternalState>),

    #[serde(rename = "0x877ebc0b")]
    #[serde(bound(deserialize = "Box<HkbRotateCharacterModifier<'a>>: Deserialize<'de>"))]
    HkbRotateCharacterModifier(Box<HkbRotateCharacterModifier<'a>>),

    #[serde(rename = "0xfb56b692")]
    #[serde(bound(deserialize = "Box<HkbSenseHandleModifierRange<'a>>: Deserialize<'de>"))]
    HkbSenseHandleModifierRange(Box<HkbSenseHandleModifierRange<'a>>),

    #[serde(rename = "0x2a064d99")]
    #[serde(bound(deserialize = "Box<HkbSenseHandleModifier<'a>>: Deserialize<'de>"))]
    HkbSenseHandleModifier(Box<HkbSenseHandleModifier<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkbSequencedData(Box<HkbSequencedData>),

    #[serde(rename = "0x419b9a05")]
    HkbSequenceInternalState(Box<HkbSequenceInternalState>),

    #[serde(rename = "0x6a5094e3")]
    #[serde(bound(deserialize = "Box<HkbSequenceStringData<'a>>: Deserialize<'de>"))]
    HkbSequenceStringData(Box<HkbSequenceStringData<'a>>),

    #[serde(rename = "0x43182ca3")]
    #[serde(bound(deserialize = "Box<HkbSequence<'a>>: Deserialize<'de>"))]
    HkbSequence(Box<HkbSequence<'a>>),

    #[serde(rename = "0xe18b74b9")]
    #[serde(bound(deserialize = "Box<HkbSetBehaviorCommand<'a>>: Deserialize<'de>"))]
    HkbSetBehaviorCommand(Box<HkbSetBehaviorCommand<'a>>),

    #[serde(rename = "0xfab12b45")]
    HkbSetLocalTimeOfClipGeneratorCommand(Box<HkbSetLocalTimeOfClipGeneratorCommand>),

    #[serde(rename = "0xc5160b64")]
    #[serde(bound(deserialize = "Box<HkbSetNodePropertyCommand<'a>>: Deserialize<'de>"))]
    HkbSetNodePropertyCommand(Box<HkbSetNodePropertyCommand<'a>>),

    #[serde(rename = "0xf3ae5fca")]
    HkbSetWordVariableCommand(Box<HkbSetWordVariableCommand>),

    #[serde(rename = "0xafcfa211")]
    #[serde(bound(deserialize = "Box<HkbSetWorldFromModelModifier<'a>>: Deserialize<'de>"))]
    HkbSetWorldFromModelModifier(Box<HkbSetWorldFromModelModifier<'a>>),

    #[serde(rename = "0x2a241367")]
    HkbSimulationControlCommand(Box<HkbSimulationControlCommand>),

    #[serde(rename = "0xa40822b4")]
    HkbSimulationStateInfo(Box<HkbSimulationStateInfo>),

    #[serde(rename = "0xda8c7d7d")]
    HkbStateChooser(Box<HkbStateChooser>),

    #[serde(rename = "0xda8c7d7d")]
    HkbStateListener(Box<HkbStateListener>),

    #[serde(rename = "0xbb90d54f")]
    #[serde(bound(deserialize = "Box<HkbStateMachineActiveTransitionInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineActiveTransitionInfo(Box<HkbStateMachineActiveTransitionInfo<'a>>),

    #[serde(rename = "0x26d5499")]
    HkbStateMachineDelayedTransitionInfo(Box<HkbStateMachineDelayedTransitionInfo>),

    #[serde(rename = "0xb07b4388")]
    #[serde(bound(deserialize = "Box<HkbStateMachineEventPropertyArray<'a>>: Deserialize<'de>"))]
    HkbStateMachineEventPropertyArray(Box<HkbStateMachineEventPropertyArray<'a>>),

    #[serde(rename = "0xbd1a7502")]
    #[serde(bound(deserialize = "Box<HkbStateMachineInternalState<'a>>: Deserialize<'de>"))]
    HkbStateMachineInternalState(Box<HkbStateMachineInternalState<'a>>),

    #[serde(rename = "0x7358f5da")]
    #[serde(bound(deserialize = "Box<HkbStateMachineNestedStateMachineData<'a>>: Deserialize<'de>"))]
    HkbStateMachineNestedStateMachineData(Box<HkbStateMachineNestedStateMachineData<'a>>),

    #[serde(rename = "0x3ab09a2e")]
    HkbStateMachineProspectiveTransitionInfo(Box<HkbStateMachineProspectiveTransitionInfo>),

    #[serde(rename = "0xed7f9d0")]
    #[serde(bound(deserialize = "Box<HkbStateMachineStateInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineStateInfo(Box<HkbStateMachineStateInfo<'a>>),

    #[serde(rename = "0x60a881e5")]
    HkbStateMachineTimeInterval(Box<HkbStateMachineTimeInterval>),

    #[serde(rename = "0xe397b11e")]
    #[serde(bound(deserialize = "Box<HkbStateMachineTransitionInfoArray<'a>>: Deserialize<'de>"))]
    HkbStateMachineTransitionInfoArray(Box<HkbStateMachineTransitionInfoArray<'a>>),

    #[serde(rename = "0x9810c2d0")]
    HkbStateMachineTransitionInfoReference(Box<HkbStateMachineTransitionInfoReference>),

    #[serde(rename = "0xcdec8025")]
    #[serde(bound(deserialize = "Box<HkbStateMachineTransitionInfo<'a>>: Deserialize<'de>"))]
    HkbStateMachineTransitionInfo(Box<HkbStateMachineTransitionInfo<'a>>),

    #[serde(rename = "0x816c1dcb")]
    #[serde(bound(deserialize = "Box<HkbStateMachine<'a>>: Deserialize<'de>"))]
    HkbStateMachine(Box<HkbStateMachine<'a>>),

    #[serde(rename = "0x5ab50487")]
    #[serde(bound(deserialize = "Box<HkbStringCondition<'a>>: Deserialize<'de>"))]
    HkbStringCondition(Box<HkbStringCondition<'a>>),

    #[serde(rename = "0xed04256a")]
    #[serde(bound(deserialize = "Box<HkbStringEventPayload<'a>>: Deserialize<'de>"))]
    HkbStringEventPayload(Box<HkbStringEventPayload<'a>>),

    #[serde(rename = "0xc0fcc436")]
    #[serde(bound(deserialize = "Box<HkbTestStateChooser<'a>>: Deserialize<'de>"))]
    HkbTestStateChooser(Box<HkbTestStateChooser<'a>>),

    #[serde(rename = "0x83ec2d42")]
    HkbTimerModifierInternalState(Box<HkbTimerModifierInternalState>),

    #[serde(rename = "0x338b4879")]
    #[serde(bound(deserialize = "Box<HkbTimerModifier<'a>>: Deserialize<'de>"))]
    HkbTimerModifier(Box<HkbTimerModifier<'a>>),

    #[serde(rename = "0x5ca91c99")]
    HkbTransformVectorModifierInternalState(Box<HkbTransformVectorModifierInternalState>),

    #[serde(rename = "0xf93e0e24")]
    #[serde(bound(deserialize = "Box<HkbTransformVectorModifier<'a>>: Deserialize<'de>"))]
    HkbTransformVectorModifier(Box<HkbTransformVectorModifier<'a>>),

    #[serde(rename = "0x945da157")]
    #[serde(bound(deserialize = "Box<HkbTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbTransitionEffect(Box<HkbTransitionEffect<'a>>),

    #[serde(rename = "0xb6b76b32")]
    #[serde(bound(deserialize = "Box<HkbTwistModifier<'a>>: Deserialize<'de>"))]
    HkbTwistModifier(Box<HkbTwistModifier<'a>>),

    #[serde(rename = "0x4d592f72")]
    #[serde(bound(deserialize = "Box<HkbVariableBindingSetBinding<'a>>: Deserialize<'de>"))]
    HkbVariableBindingSetBinding(Box<HkbVariableBindingSetBinding<'a>>),

    #[serde(rename = "0x338ad4ff")]
    #[serde(bound(deserialize = "Box<HkbVariableBindingSet<'a>>: Deserialize<'de>"))]
    HkbVariableBindingSet(Box<HkbVariableBindingSet<'a>>),

    #[serde(rename = "0x9e746ba2")]
    HkbVariableInfo(Box<HkbVariableInfo>),

    #[serde(rename = "0x27812d8d")]
    #[serde(bound(deserialize = "Box<HkbVariableValueSet<'a>>: Deserialize<'de>"))]
    HkbVariableValueSet(Box<HkbVariableValueSet<'a>>),

    #[serde(rename = "0xb99bd6a")]
    HkbVariableValue(Box<HkbVariableValue>),

    #[serde(rename = "0x25640b46")]
    HkbWorldEnums(Box<HkbWorldEnums>),

    #[serde(rename = "0xa3af8783")]
    HkbWorldFromModelModeData(Box<HkbWorldFromModelModeData>),

    #[serde(rename = "0xce6f8a6c")]
    #[serde(bound(deserialize = "Box<HkClassEnumItem<'a>>: Deserialize<'de>"))]
    HkClassEnumItem(Box<HkClassEnumItem<'a>>),

    #[serde(rename = "0x8a3609cf")]
    #[serde(bound(deserialize = "Box<HkClassEnum<'a>>: Deserialize<'de>"))]
    HkClassEnum(Box<HkClassEnum<'a>>),

    #[serde(rename = "0x4e32287c")]
    HkContactPointMaterial(Box<HkContactPointMaterial>),

    #[serde(rename = "0x91d7dd8e")]
    HkContactPoint(Box<HkContactPoint>),

    #[serde(rename = "0x1388d601")]
    #[serde(bound(deserialize = "Box<HkCustomAttributesAttribute<'a>>: Deserialize<'de>"))]
    HkCustomAttributesAttribute(Box<HkCustomAttributesAttribute<'a>>),

    #[serde(rename = "0xbff19005")]
    #[serde(bound(deserialize = "Box<HkCustomAttributes<'a>>: Deserialize<'de>"))]
    HkCustomAttributes(Box<HkCustomAttributes<'a>>),

    #[serde(rename = "0x1e3857bb")]
    #[serde(bound(deserialize = "Box<HkDataObjectTypeAttribute<'a>>: Deserialize<'de>"))]
    HkDataObjectTypeAttribute(Box<HkDataObjectTypeAttribute<'a>>),

    #[serde(rename = "0xe9f9578a")]
    #[serde(bound(deserialize = "Box<HkDescriptionAttribute<'a>>: Deserialize<'de>"))]
    HkDescriptionAttribute(Box<HkDescriptionAttribute<'a>>),

    #[serde(rename = "0x630edd9e")]
    #[serde(bound(deserialize = "Box<HkDocumentationAttribute<'a>>: Deserialize<'de>"))]
    HkDocumentationAttribute(Box<HkDocumentationAttribute<'a>>),

    #[serde(rename = "0x9687513b")]
    HkGeometryTriangle(Box<HkGeometryTriangle>),

    #[serde(rename = "0x98dd8bdc")]
    HkGeometry(Box<HkGeometry>),

    #[serde(rename = "0x23aadfb6")]
    #[serde(bound(deserialize = "Box<HkGizmoAttribute<'a>>: Deserialize<'de>"))]
    HkGizmoAttribute(Box<HkGizmoAttribute<'a>>),

    #[serde(rename = "0x7684dc80")]
    HkHalf8(Box<HkHalf8>),

    #[serde(rename = "0x87fe6b5c")]
    #[serde(bound(deserialize = "Box<HkIndexedTransformSet<'a>>: Deserialize<'de>"))]
    HkIndexedTransformSet(Box<HkIndexedTransformSet<'a>>),

    #[serde(rename = "0x255d8164")]
    HkLinkAttribute(Box<HkLinkAttribute>),

    #[serde(rename = "0xb1a96c2f")]
    #[serde(bound(deserialize = "Box<HkLocalFrameGroup<'a>>: Deserialize<'de>"))]
    HkLocalFrameGroup(Box<HkLocalFrameGroup<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkLocalFrame(Box<HkLocalFrame>),

    #[serde(rename = "0x94a620a8")]
    #[serde(bound(deserialize = "Box<HkMemoryMeshBody<'a>>: Deserialize<'de>"))]
    HkMemoryMeshBody(Box<HkMemoryMeshBody<'a>>),

    #[serde(rename = "0x12156ee3")]
    #[serde(bound(deserialize = "Box<HkMemoryMeshMaterial<'a>>: Deserialize<'de>"))]
    HkMemoryMeshMaterial(Box<HkMemoryMeshMaterial<'a>>),

    #[serde(rename = "0xb743a578")]
    #[serde(bound(deserialize = "Box<HkMemoryMeshShape<'a>>: Deserialize<'de>"))]
    HkMemoryMeshShape(Box<HkMemoryMeshShape<'a>>),

    #[serde(rename = "0x2db6577c")]
    #[serde(bound(deserialize = "Box<HkMemoryMeshTexture<'a>>: Deserialize<'de>"))]
    HkMemoryMeshTexture(Box<HkMemoryMeshTexture<'a>>),

    #[serde(rename = "0xa2e50753")]
    HkMemoryMeshVertexBuffer(Box<HkMemoryMeshVertexBuffer>),

    #[serde(rename = "0x4762f92a")]
    #[serde(bound(deserialize = "Box<HkMemoryResourceContainer<'a>>: Deserialize<'de>"))]
    HkMemoryResourceContainer(Box<HkMemoryResourceContainer<'a>>),

    #[serde(rename = "0x3144d17c")]
    #[serde(bound(deserialize = "Box<HkMemoryResourceHandleExternalLink<'a>>: Deserialize<'de>"))]
    HkMemoryResourceHandleExternalLink(Box<HkMemoryResourceHandleExternalLink<'a>>),

    #[serde(rename = "0xbffac086")]
    #[serde(bound(deserialize = "Box<HkMemoryResourceHandle<'a>>: Deserialize<'de>"))]
    HkMemoryResourceHandle(Box<HkMemoryResourceHandle<'a>>),

    #[serde(rename = "0x7bd5c66f")]
    HkMemoryTrackerAttribute(Box<HkMemoryTrackerAttribute>),

    #[serde(rename = "0xd0be5d7d")]
    HkMeshBody(Box<HkMeshBody>),

    #[serde(rename = "0x48aceb75")]
    HkMeshBoneIndexMapping(Box<HkMeshBoneIndexMapping>),

    #[serde(rename = "0xda8c7d7d")]
    HkMeshMaterial(Box<HkMeshMaterial>),

    #[serde(rename = "0x6075f3ff")]
    #[serde(bound(deserialize = "Box<HkMeshSectionCinfo<'a>>: Deserialize<'de>"))]
    HkMeshSectionCinfo(Box<HkMeshSectionCinfo<'a>>),

    #[serde(rename = "0x1893c365")]
    #[serde(bound(deserialize = "Box<HkMeshSection<'a>>: Deserialize<'de>"))]
    HkMeshSection(Box<HkMeshSection<'a>>),

    #[serde(rename = "0x9117d62e")]
    HkMeshShape(Box<HkMeshShape>),

    #[serde(rename = "0xc9887918")]
    HkMeshTexture(Box<HkMeshTexture>),

    #[serde(rename = "0x534b08c8")]
    HkMeshVertexBuffer(Box<HkMeshVertexBuffer>),

    #[serde(rename = "0x338c092f")]
    HkModelerNodeTypeAttribute(Box<HkModelerNodeTypeAttribute>),

    #[serde(rename = "0x7798b7db")]
    #[serde(bound(deserialize = "Box<HkMonitorStreamFrameInfo<'a>>: Deserialize<'de>"))]
    HkMonitorStreamFrameInfo(Box<HkMonitorStreamFrameInfo<'a>>),

    #[serde(rename = "0x2c76ce16")]
    #[serde(bound(deserialize = "Box<HkMonitorStreamStringMapStringMap<'a>>: Deserialize<'de>"))]
    HkMonitorStreamStringMapStringMap(Box<HkMonitorStreamStringMapStringMap<'a>>),

    #[serde(rename = "0xc4d3a8b4")]
    #[serde(bound(deserialize = "Box<HkMonitorStreamStringMap<'a>>: Deserialize<'de>"))]
    HkMonitorStreamStringMap(Box<HkMonitorStreamStringMap<'a>>),

    #[serde(rename = "0x7c338c66")]
    #[serde(bound(deserialize = "Box<HkMoppBvTreeShapeBase<'a>>: Deserialize<'de>"))]
    HkMoppBvTreeShapeBase(Box<HkMoppBvTreeShapeBase<'a>>),

    #[serde(rename = "0x5797386e")]
    HkMotionState(Box<HkMotionState>),

    #[serde(rename = "0x4731fb1b")]
    HkMultipleVertexBufferElementInfo(Box<HkMultipleVertexBufferElementInfo>),

    #[serde(rename = "0xa0e22afc")]
    HkMultipleVertexBufferLockedElement(Box<HkMultipleVertexBufferLockedElement>),

    #[serde(rename = "0xdafbe0e6")]
    #[serde(bound(deserialize = "Box<HkMultipleVertexBufferVertexBufferInfo<'a>>: Deserialize<'de>"))]
    HkMultipleVertexBufferVertexBufferInfo(Box<HkMultipleVertexBufferVertexBufferInfo<'a>>),

    #[serde(rename = "0xde3ab602")]
    #[serde(bound(deserialize = "Box<HkMultipleVertexBuffer<'a>>: Deserialize<'de>"))]
    HkMultipleVertexBuffer(Box<HkMultipleVertexBuffer<'a>>),

    #[serde(rename = "0x11e4408b")]
    HkMultiThreadCheck(Box<HkMultiThreadCheck>),

    #[serde(rename = "0xdcdb8b8b")]
    Hkp2DAngConstraintAtom(Box<Hkp2DAngConstraintAtom>),

    #[serde(rename = "0x2c5189dd")]
    #[serde(bound(deserialize = "Box<HkpAabbPhantom<'a>>: Deserialize<'de>"))]
    HkpAabbPhantom(Box<HkpAabbPhantom<'a>>),

    #[serde(rename = "0x9c16df5b")]
    HkPackedVector3(Box<HkPackedVector3>),

    #[serde(rename = "0x79f9ffda")]
    HkPackfileHeader(Box<HkPackfileHeader>),

    #[serde(rename = "0xf2a92154")]
    HkPackfileSectionHeader(Box<HkPackfileSectionHeader>),

    #[serde(rename = "0xbdf70a51")]
    #[serde(bound(deserialize = "Box<HkpAction<'a>>: Deserialize<'de>"))]
    HkpAction(Box<HkpAction<'a>>),

    #[serde(rename = "0x35bb3cd0")]
    HkpAngConstraintAtom(Box<HkpAngConstraintAtom>),

    #[serde(rename = "0xf313aa80")]
    HkpAngFrictionConstraintAtom(Box<HkpAngFrictionConstraintAtom>),

    #[serde(rename = "0x9be0d9d")]
    HkpAngLimitConstraintAtom(Box<HkpAngLimitConstraintAtom>),

    #[serde(rename = "0x81f087ff")]
    #[serde(bound(deserialize = "Box<HkpAngMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpAngMotorConstraintAtom(Box<HkpAngMotorConstraintAtom<'a>>),

    #[serde(rename = "0x35f4c487")]
    #[serde(bound(deserialize = "Box<HkpAngularDashpotAction<'a>>: Deserialize<'de>"))]
    HkpAngularDashpotAction(Box<HkpAngularDashpotAction<'a>>),

    #[serde(rename = "0x674bcd2d")]
    #[serde(bound(deserialize = "Box<HkpArrayAction<'a>>: Deserialize<'de>"))]
    HkpArrayAction(Box<HkpArrayAction<'a>>),

    #[serde(rename = "0xc73dcaf9")]
    HkpBallAndSocketConstraintDataAtoms(Box<HkpBallAndSocketConstraintDataAtoms>),

    #[serde(rename = "0x5a6954d9")]
    HkpBallAndSocketConstraintData(Box<HkpBallAndSocketConstraintData>),

    #[serde(rename = "0x57b06d35")]
    #[serde(bound(deserialize = "Box<HkpBallGun<'a>>: Deserialize<'de>"))]
    HkpBallGun(Box<HkpBallGun<'a>>),

    #[serde(rename = "0xc9cbedf2")]
    HkpBallSocketChainDataConstraintInfo(Box<HkpBallSocketChainDataConstraintInfo>),

    #[serde(rename = "0x102aae9c")]
    #[serde(bound(deserialize = "Box<HkpBallSocketChainData<'a>>: Deserialize<'de>"))]
    HkpBallSocketChainData(Box<HkpBallSocketChainData<'a>>),

    #[serde(rename = "0xe70e4dfa")]
    HkpBallSocketConstraintAtom(Box<HkpBallSocketConstraintAtom>),

    #[serde(rename = "0xc00f3403")]
    #[serde(bound(deserialize = "Box<HkpBinaryAction<'a>>: Deserialize<'de>"))]
    HkpBinaryAction(Box<HkpBinaryAction<'a>>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Box<HkpBoxMotion<'a>>: Deserialize<'de>"))]
    HkpBoxMotion(Box<HkpBoxMotion<'a>>),

    #[serde(rename = "0x3444d2d5")]
    HkpBoxShape(Box<HkpBoxShape>),

    #[serde(rename = "0xda8c7d7d")]
    HkpBreakableBody(Box<HkpBreakableBody>),

    #[serde(rename = "0x7d6310c8")]
    #[serde(bound(deserialize = "Box<HkpBreakableConstraintData<'a>>: Deserialize<'de>"))]
    HkpBreakableConstraintData(Box<HkpBreakableConstraintData<'a>>),

    #[serde(rename = "0xde152a4d")]
    #[serde(bound(deserialize = "Box<HkpBridgeAtoms<'a>>: Deserialize<'de>"))]
    HkpBridgeAtoms(Box<HkpBridgeAtoms<'a>>),

    #[serde(rename = "0x87a4f31b")]
    #[serde(bound(deserialize = "Box<HkpBridgeConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpBridgeConstraintAtom(Box<HkpBridgeConstraintAtom<'a>>),

    #[serde(rename = "0x940569dc")]
    HkpBroadPhaseHandle(Box<HkpBroadPhaseHandle>),

    #[serde(rename = "0x286eb64c")]
    #[serde(bound(deserialize = "Box<HkpBvShape<'a>>: Deserialize<'de>"))]
    HkpBvShape(Box<HkpBvShape<'a>>),

    #[serde(rename = "0xa823d623")]
    HkpBvTreeShape(Box<HkpBvTreeShape>),

    #[serde(rename = "0xcf227f58")]
    #[serde(bound(deserialize = "Box<HkpCachingShapePhantom<'a>>: Deserialize<'de>"))]
    HkpCachingShapePhantom(Box<HkpCachingShapePhantom<'a>>),

    #[serde(rename = "0xafcd79ad")]
    #[serde(bound(deserialize = "Box<HkpCallbackConstraintMotor<'a>>: Deserialize<'de>"))]
    HkpCallbackConstraintMotor(Box<HkpCallbackConstraintMotor<'a>>),

    #[serde(rename = "0xdd0b1fd3")]
    HkpCapsuleShape(Box<HkpCapsuleShape>),

    #[serde(rename = "0x54a4b841")]
    #[serde(bound(deserialize = "Box<HkpCdBody<'a>>: Deserialize<'de>"))]
    HkpCdBody(Box<HkpCdBody<'a>>),

    #[serde(rename = "0x1d7dbdd2")]
    #[serde(bound(deserialize = "Box<HkpCenterOfMassChangerModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpCenterOfMassChangerModifierConstraintAtom(Box<HkpCenterOfMassChangerModifierConstraintAtom<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpCharacterControllerCinfo(Box<HkpCharacterControllerCinfo>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Box<HkpCharacterMotion<'a>>: Deserialize<'de>"))]
    HkpCharacterMotion(Box<HkpCharacterMotion<'a>>),

    #[serde(rename = "0x586d97b2")]
    #[serde(bound(deserialize = "Box<HkpCharacterProxyCinfo<'a>>: Deserialize<'de>"))]
    HkpCharacterProxyCinfo(Box<HkpCharacterProxyCinfo<'a>>),

    #[serde(rename = "0x892f441")]
    #[serde(bound(deserialize = "Box<HkpCharacterRigidBodyCinfo<'a>>: Deserialize<'de>"))]
    HkpCharacterRigidBodyCinfo(Box<HkpCharacterRigidBodyCinfo<'a>>),

    #[serde(rename = "0xf2b1f399")]
    HkpCogWheelConstraintAtom(Box<HkpCogWheelConstraintAtom>),

    #[serde(rename = "0xf855ba44")]
    HkpCogWheelConstraintDataAtoms(Box<HkpCogWheelConstraintDataAtoms>),

    #[serde(rename = "0x7f0e53fc")]
    HkpCogWheelConstraintData(Box<HkpCogWheelConstraintData>),

    #[serde(rename = "0xb5f0e6b1")]
    #[serde(bound(deserialize = "Box<HkpCollidableBoundingVolumeData<'a>>: Deserialize<'de>"))]
    HkpCollidableBoundingVolumeData(Box<HkpCollidableBoundingVolumeData<'a>>),

    #[serde(rename = "0xe0708a00")]
    HkpCollidableCollidableFilter(Box<HkpCollidableCollidableFilter>),

    #[serde(rename = "0x9a0e42a5")]
    #[serde(bound(deserialize = "Box<HkpCollidable<'a>>: Deserialize<'de>"))]
    HkpCollidable(Box<HkpCollidable<'a>>),

    #[serde(rename = "0x2603bf04")]
    #[serde(bound(deserialize = "Box<HkpCollisionFilterList<'a>>: Deserialize<'de>"))]
    HkpCollisionFilterList(Box<HkpCollisionFilterList<'a>>),

    #[serde(rename = "0x60960336")]
    HkpCollisionFilter(Box<HkpCollisionFilter>),

    #[serde(rename = "0xcbfc95a4")]
    HkpCompressedMeshShapeBigTriangle(Box<HkpCompressedMeshShapeBigTriangle>),

    #[serde(rename = "0x5d0d67bd")]
    HkpCompressedMeshShapeChunk(Box<HkpCompressedMeshShapeChunk>),

    #[serde(rename = "0x385bb842")]
    HkpCompressedMeshShapeConvexPiece(Box<HkpCompressedMeshShapeConvexPiece>),

    #[serde(rename = "0xa62d5e6e")]
    #[serde(bound(deserialize = "Box<HkpCompressedMeshShape<'a>>: Deserialize<'de>"))]
    HkpCompressedMeshShape(Box<HkpCompressedMeshShape<'a>>),

    #[serde(rename = "0x97b6e143")]
    HkpCompressedSampledHeightFieldShape(Box<HkpCompressedSampledHeightFieldShape>),

    #[serde(rename = "0xf19443c8")]
    HkpConeLimitConstraintAtom(Box<HkpConeLimitConstraintAtom>),

    #[serde(rename = "0x20a447fe")]
    #[serde(bound(deserialize = "Box<HkpConstrainedSystemFilter<'a>>: Deserialize<'de>"))]
    HkpConstrainedSystemFilter(Box<HkpConstrainedSystemFilter<'a>>),

    #[serde(rename = "0x59d67ef6")]
    HkpConstraintAtom(Box<HkpConstraintAtom>),

    #[serde(rename = "0x5facc7ff")]
    HkpConstraintChainData(Box<HkpConstraintChainData>),

    #[serde(rename = "0xc3971189")]
    #[serde(bound(deserialize = "Box<HkpConstraintChainInstanceAction<'a>>: Deserialize<'de>"))]
    HkpConstraintChainInstanceAction(Box<HkpConstraintChainInstanceAction<'a>>),

    #[serde(rename = "0x7a490753")]
    #[serde(bound(deserialize = "Box<HkpConstraintChainInstance<'a>>: Deserialize<'de>"))]
    HkpConstraintChainInstance(Box<HkpConstraintChainInstance<'a>>),

    #[serde(rename = "0xc3b577b1")]
    #[serde(bound(deserialize = "Box<HkpConstraintCollisionFilter<'a>>: Deserialize<'de>"))]
    HkpConstraintCollisionFilter(Box<HkpConstraintCollisionFilter<'a>>),

    #[serde(rename = "0xee3c2aec")]
    #[serde(bound(deserialize = "Box<HkpConstraintInstanceSmallArraySerializeOverrideType<'a>>: Deserialize<'de>"))]
    HkpConstraintInstanceSmallArraySerializeOverrideType(Box<HkpConstraintInstanceSmallArraySerializeOverrideType<'a>>),

    #[serde(rename = "0x34eba5f")]
    #[serde(bound(deserialize = "Box<HkpConstraintInstance<'a>>: Deserialize<'de>"))]
    HkpConstraintInstance(Box<HkpConstraintInstance<'a>>),

    #[serde(rename = "0x6a44c317")]
    HkpConstraintMotor(Box<HkpConstraintMotor>),

    #[serde(rename = "0x81d074a4")]
    HkpConvexListFilter(Box<HkpConvexListFilter>),

    #[serde(rename = "0x450b26e8")]
    #[serde(bound(deserialize = "Box<HkpConvexListShape<'a>>: Deserialize<'de>"))]
    HkpConvexListShape(Box<HkpConvexListShape<'a>>),

    #[serde(rename = "0x38fd3d97")]
    #[serde(bound(deserialize = "Box<HkpConvexPieceMeshShape<'a>>: Deserialize<'de>"))]
    HkpConvexPieceMeshShape(Box<HkpConvexPieceMeshShape<'a>>),

    #[serde(rename = "0xa5bd1d6e")]
    HkpConvexPieceStreamData(Box<HkpConvexPieceStreamData>),

    #[serde(rename = "0xf8f74f85")]
    HkpConvexShape(Box<HkpConvexShape>),

    #[serde(rename = "0xfbd72f9")]
    #[serde(bound(deserialize = "Box<HkpConvexTransformShapeBase<'a>>: Deserialize<'de>"))]
    HkpConvexTransformShapeBase(Box<HkpConvexTransformShapeBase<'a>>),

    #[serde(rename = "0xae3e5017")]
    #[serde(bound(deserialize = "Box<HkpConvexTransformShape<'a>>: Deserialize<'de>"))]
    HkpConvexTransformShape(Box<HkpConvexTransformShape<'a>>),

    #[serde(rename = "0x5ba0a5f7")]
    #[serde(bound(deserialize = "Box<HkpConvexTranslateShape<'a>>: Deserialize<'de>"))]
    HkpConvexTranslateShape(Box<HkpConvexTranslateShape<'a>>),

    #[serde(rename = "0x63d38e9c")]
    HkpConvexVerticesConnectivity(Box<HkpConvexVerticesConnectivity>),

    #[serde(rename = "0x3d80c5bf")]
    HkpConvexVerticesShapeFourVectors(Box<HkpConvexVerticesShapeFourVectors>),

    #[serde(rename = "0x28726ad8")]
    #[serde(bound(deserialize = "Box<HkpConvexVerticesShape<'a>>: Deserialize<'de>"))]
    HkpConvexVerticesShape(Box<HkpConvexVerticesShape<'a>>),

    #[serde(rename = "0x3e463c3a")]
    HkpCylinderShape(Box<HkpCylinderShape>),

    #[serde(rename = "0x50746c6e")]
    #[serde(bound(deserialize = "Box<HkpDashpotAction<'a>>: Deserialize<'de>"))]
    HkpDashpotAction(Box<HkpDashpotAction<'a>>),

    #[serde(rename = "0xb69c1c02")]
    HkpDefaultConvexListFilter(Box<HkpDefaultConvexListFilter>),

    #[serde(rename = "0x77d6b19f")]
    HkpDefaultWorldMemoryWatchDog(Box<HkpDefaultWorldMemoryWatchDog>),

    #[serde(rename = "0xfac3351c")]
    #[serde(bound(deserialize = "Box<HkpDisableEntityCollisionFilter<'a>>: Deserialize<'de>"))]
    HkpDisableEntityCollisionFilter(Box<HkpDisableEntityCollisionFilter<'a>>),

    #[serde(rename = "0xc8ae86a7")]
    #[serde(bound(deserialize = "Box<HkpDisplayBindingDataPhysicsSystem<'a>>: Deserialize<'de>"))]
    HkpDisplayBindingDataPhysicsSystem(Box<HkpDisplayBindingDataPhysicsSystem<'a>>),

    #[serde(rename = "0xfe16e2a3")]
    #[serde(bound(deserialize = "Box<HkpDisplayBindingDataRigidBody<'a>>: Deserialize<'de>"))]
    HkpDisplayBindingDataRigidBody(Box<HkpDisplayBindingDataRigidBody<'a>>),

    #[serde(rename = "0xdc46c906")]
    #[serde(bound(deserialize = "Box<HkpDisplayBindingData<'a>>: Deserialize<'de>"))]
    HkpDisplayBindingData(Box<HkpDisplayBindingData<'a>>),

    #[serde(rename = "0xf557023c")]
    #[serde(bound(deserialize = "Box<HkpEntityExtendedListeners<'a>>: Deserialize<'de>"))]
    HkpEntityExtendedListeners(Box<HkpEntityExtendedListeners<'a>>),

    #[serde(rename = "0xee3c2aec")]
    #[serde(bound(deserialize = "Box<HkpEntitySmallArraySerializeOverrideType<'a>>: Deserialize<'de>"))]
    HkpEntitySmallArraySerializeOverrideType(Box<HkpEntitySmallArraySerializeOverrideType<'a>>),

    #[serde(rename = "0x81147f05")]
    #[serde(bound(deserialize = "Box<HkpEntitySpuCollisionCallback<'a>>: Deserialize<'de>"))]
    HkpEntitySpuCollisionCallback(Box<HkpEntitySpuCollisionCallback<'a>>),

    #[serde(rename = "0xa03c774b")]
    #[serde(bound(deserialize = "Box<HkpEntity<'a>>: Deserialize<'de>"))]
    HkpEntity(Box<HkpEntity<'a>>),

    #[serde(rename = "0xf204b155")]
    #[serde(bound(deserialize = "Box<HkpExtendedMeshShapeShapesSubpart<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShapeShapesSubpart(Box<HkpExtendedMeshShapeShapesSubpart<'a>>),

    #[serde(rename = "0xf4608207")]
    #[serde(bound(deserialize = "Box<HkpExtendedMeshShapeSubpart<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShapeSubpart(Box<HkpExtendedMeshShapeSubpart<'a>>),

    #[serde(rename = "0x44c32df6")]
    #[serde(bound(deserialize = "Box<HkpExtendedMeshShapeTrianglesSubpart<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShapeTrianglesSubpart(Box<HkpExtendedMeshShapeTrianglesSubpart<'a>>),

    #[serde(rename = "0x177114a2")]
    #[serde(bound(deserialize = "Box<HkpExtendedMeshShape<'a>>: Deserialize<'de>"))]
    HkpExtendedMeshShape(Box<HkpExtendedMeshShape<'a>>),

    #[serde(rename = "0x3d3da311")]
    #[serde(bound(deserialize = "Box<HkpFastMeshShape<'a>>: Deserialize<'de>"))]
    HkpFastMeshShape(Box<HkpFastMeshShape<'a>>),

    #[serde(rename = "0x852ab70b")]
    #[serde(bound(deserialize = "Box<HkpFirstPersonGun<'a>>: Deserialize<'de>"))]
    HkpFirstPersonGun(Box<HkpFirstPersonGun<'a>>),

    #[serde(rename = "0x64abf85c")]
    #[serde(bound(deserialize = "Box<HkpFixedRigidMotion<'a>>: Deserialize<'de>"))]
    HkpFixedRigidMotion(Box<HkpFixedRigidMotion<'a>>),

    #[serde(rename = "0xd6421f19")]
    HkpGenericConstraintDataSchemeConstraintInfo(Box<HkpGenericConstraintDataSchemeConstraintInfo>),

    #[serde(rename = "0x11fd6f6c")]
    #[serde(bound(deserialize = "Box<HkpGenericConstraintDataScheme<'a>>: Deserialize<'de>"))]
    HkpGenericConstraintDataScheme(Box<HkpGenericConstraintDataScheme<'a>>),

    #[serde(rename = "0xfa824640")]
    #[serde(bound(deserialize = "Box<HkpGenericConstraintData<'a>>: Deserialize<'de>"))]
    HkpGenericConstraintData(Box<HkpGenericConstraintData<'a>>),

    #[serde(rename = "0x5e2754cd")]
    #[serde(bound(deserialize = "Box<HkpGravityGun<'a>>: Deserialize<'de>"))]
    HkpGravityGun(Box<HkpGravityGun<'a>>),

    #[serde(rename = "0x5cc01561")]
    HkpGroupCollisionFilter(Box<HkpGroupCollisionFilter>),

    #[serde(rename = "0x65ee88e4")]
    HkpGroupFilter(Box<HkpGroupFilter>),

    #[serde(rename = "0xe7eca7eb")]
    HkpHeightFieldShape(Box<HkpHeightFieldShape>),

    #[serde(rename = "0x6958371c")]
    HkpHingeConstraintDataAtoms(Box<HkpHingeConstraintDataAtoms>),

    #[serde(rename = "0x9590f046")]
    HkpHingeConstraintData(Box<HkpHingeConstraintData>),

    #[serde(rename = "0x555876ff")]
    HkpHingeLimitsDataAtoms(Box<HkpHingeLimitsDataAtoms>),

    #[serde(rename = "0xbd46760a")]
    HkpHingeLimitsData(Box<HkpHingeLimitsData>),

    #[serde(rename = "0x5c6aa14d")]
    #[serde(bound(deserialize = "Box<HkpIgnoreModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpIgnoreModifierConstraintAtom(Box<HkpIgnoreModifierConstraintAtom<'a>>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Box<HkpKeyframedRigidMotion<'a>>: Deserialize<'de>"))]
    HkpKeyframedRigidMotion(Box<HkpKeyframedRigidMotion<'a>>),

    #[serde(rename = "0x3377b0b0")]
    HkpLimitedForceConstraintMotor(Box<HkpLimitedForceConstraintMotor>),

    #[serde(rename = "0x54c7715b")]
    #[serde(bound(deserialize = "Box<HkpLimitedHingeConstraintDataAtoms<'a>>: Deserialize<'de>"))]
    HkpLimitedHingeConstraintDataAtoms(Box<HkpLimitedHingeConstraintDataAtoms<'a>>),

    #[serde(rename = "0x7c15bb6b")]
    #[serde(bound(deserialize = "Box<HkpLimitedHingeConstraintData<'a>>: Deserialize<'de>"))]
    HkpLimitedHingeConstraintData(Box<HkpLimitedHingeConstraintData<'a>>),

    #[serde(rename = "0x7b6b0210")]
    HkpLinConstraintAtom(Box<HkpLinConstraintAtom>),

    #[serde(rename = "0xd7b3be03")]
    HkpLinearParametricCurve(Box<HkpLinearParametricCurve>),

    #[serde(rename = "0x3e94ef7c")]
    HkpLinFrictionConstraintAtom(Box<HkpLinFrictionConstraintAtom>),

    #[serde(rename = "0xe1a81497")]
    #[serde(bound(deserialize = "Box<HkpLinkedCollidable<'a>>: Deserialize<'de>"))]
    HkpLinkedCollidable(Box<HkpLinkedCollidable<'a>>),

    #[serde(rename = "0xa44d1b07")]
    HkpLinLimitConstraintAtom(Box<HkpLinLimitConstraintAtom>),

    #[serde(rename = "0x10312464")]
    #[serde(bound(deserialize = "Box<HkpLinMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpLinMotorConstraintAtom(Box<HkpLinMotorConstraintAtom<'a>>),

    #[serde(rename = "0x52b27d69")]
    HkpLinSoftConstraintAtom(Box<HkpLinSoftConstraintAtom>),

    #[serde(rename = "0x80df0f90")]
    #[serde(bound(deserialize = "Box<HkpListShapeChildInfo<'a>>: Deserialize<'de>"))]
    HkpListShapeChildInfo(Box<HkpListShapeChildInfo<'a>>),

    #[serde(rename = "0xa1937cbd")]
    #[serde(bound(deserialize = "Box<HkpListShape<'a>>: Deserialize<'de>"))]
    HkpListShape(Box<HkpListShape<'a>>),

    #[serde(rename = "0x6748b2cf")]
    #[serde(bound(deserialize = "Box<HkpMalleableConstraintData<'a>>: Deserialize<'de>"))]
    HkpMalleableConstraintData(Box<HkpMalleableConstraintData<'a>>),

    #[serde(rename = "0xb6b28240")]
    #[serde(bound(deserialize = "Box<HkpMassChangerModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpMassChangerModifierConstraintAtom(Box<HkpMassChangerModifierConstraintAtom<'a>>),

    #[serde(rename = "0x68a56834")]
    HkpMassProperties(Box<HkpMassProperties>),

    #[serde(rename = "0x33be6570")]
    HkpMaterial(Box<HkpMaterial>),

    #[serde(rename = "0x64abf85c")]
    #[serde(bound(deserialize = "Box<HkpMaxSizeMotion<'a>>: Deserialize<'de>"))]
    HkpMaxSizeMotion(Box<HkpMaxSizeMotion<'a>>),

    #[serde(rename = "0x886cde0c")]
    HkpMeshMaterial(Box<HkpMeshMaterial>),

    #[serde(rename = "0x27336e5d")]
    #[serde(bound(deserialize = "Box<HkpMeshShapeSubpart<'a>>: Deserialize<'de>"))]
    HkpMeshShapeSubpart(Box<HkpMeshShapeSubpart<'a>>),

    #[serde(rename = "0x3bf12c0f")]
    #[serde(bound(deserialize = "Box<HkpMeshShape<'a>>: Deserialize<'de>"))]
    HkpMeshShape(Box<HkpMeshShape<'a>>),

    #[serde(rename = "0xb13fef1f")]
    #[serde(bound(deserialize = "Box<HkpModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpModifierConstraintAtom(Box<HkpModifierConstraintAtom<'a>>),

    #[serde(rename = "0x90b29d39")]
    #[serde(bound(deserialize = "Box<HkpMoppBvTreeShape<'a>>: Deserialize<'de>"))]
    HkpMoppBvTreeShape(Box<HkpMoppBvTreeShape<'a>>),

    #[serde(rename = "0xd8fdbb08")]
    HkpMoppCodeCodeInfo(Box<HkpMoppCodeCodeInfo>),

    #[serde(rename = "0x6ed8ac06")]
    HkpMoppCodeReindexedTerminal(Box<HkpMoppCodeReindexedTerminal>),

    #[serde(rename = "0x924c2661")]
    HkpMoppCode(Box<HkpMoppCode>),

    #[serde(rename = "0x98aadb4f")]
    #[serde(bound(deserialize = "Box<HkpMotion<'a>>: Deserialize<'de>"))]
    HkpMotion(Box<HkpMotion<'a>>),

    #[serde(rename = "0x8ff131d9")]
    #[serde(bound(deserialize = "Box<HkpMotorAction<'a>>: Deserialize<'de>"))]
    HkpMotorAction(Box<HkpMotorAction<'a>>),

    #[serde(rename = "0x6791ffce")]
    #[serde(bound(deserialize = "Box<HkpMountedBallGun<'a>>: Deserialize<'de>"))]
    HkpMountedBallGun(Box<HkpMountedBallGun<'a>>),

    #[serde(rename = "0x6e087fd6")]
    #[serde(bound(deserialize = "Box<HkpMouseSpringAction<'a>>: Deserialize<'de>"))]
    HkpMouseSpringAction(Box<HkpMouseSpringAction<'a>>),

    #[serde(rename = "0x79ab517d")]
    #[serde(bound(deserialize = "Box<HkpMovingSurfaceModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpMovingSurfaceModifierConstraintAtom(Box<HkpMovingSurfaceModifierConstraintAtom<'a>>),

    #[serde(rename = "0xffdc0b65")]
    HkpMultiRayShapeRay(Box<HkpMultiRayShapeRay>),

    #[serde(rename = "0xea2e7ec9")]
    HkpMultiRayShape(Box<HkpMultiRayShape>),

    #[serde(rename = "0x61a590fc")]
    HkpMultiSphereShape(Box<HkpMultiSphereShape>),

    #[serde(rename = "0xc03af40d")]
    #[serde(bound(deserialize = "Box<HkpMultithreadedVehicleManager<'a>>: Deserialize<'de>"))]
    HkpMultithreadedVehicleManager(Box<HkpMultithreadedVehicleManager<'a>>),

    #[serde(rename = "0x66b42df1")]
    #[serde(bound(deserialize = "Box<HkpNamedMeshMaterial<'a>>: Deserialize<'de>"))]
    HkpNamedMeshMaterial(Box<HkpNamedMeshMaterial<'a>>),

    #[serde(rename = "0xb120a34f")]
    HkpNullCollisionFilter(Box<HkpNullCollisionFilter>),

    #[serde(rename = "0x903abb2c")]
    #[serde(bound(deserialize = "Box<HkPostFinishAttribute<'a>>: Deserialize<'de>"))]
    HkPostFinishAttribute(Box<HkPostFinishAttribute<'a>>),

    #[serde(rename = "0x1f11b467")]
    HkpOverwritePivotConstraintAtom(Box<HkpOverwritePivotConstraintAtom>),

    #[serde(rename = "0x36195969")]
    #[serde(bound(deserialize = "Box<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>: Deserialize<'de>"))]
    HkpPairCollisionFilterMapPairFilterKeyOverrideType(Box<HkpPairCollisionFilterMapPairFilterKeyOverrideType<'a>>),

    #[serde(rename = "0x4abc140e")]
    #[serde(bound(deserialize = "Box<HkpPairCollisionFilter<'a>>: Deserialize<'de>"))]
    HkpPairCollisionFilter(Box<HkpPairCollisionFilter<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpParametricCurve(Box<HkpParametricCurve>),

    #[serde(rename = "0xe7eca7eb")]
    HkpPhantomCallbackShape(Box<HkpPhantomCallbackShape>),

    #[serde(rename = "0x9b7e6f86")]
    #[serde(bound(deserialize = "Box<HkpPhantom<'a>>: Deserialize<'de>"))]
    HkpPhantom(Box<HkpPhantom<'a>>),

    #[serde(rename = "0xc2a461e4")]
    #[serde(bound(deserialize = "Box<HkpPhysicsData<'a>>: Deserialize<'de>"))]
    HkpPhysicsData(Box<HkpPhysicsData<'a>>),

    #[serde(rename = "0xd0fd4bbe")]
    #[serde(bound(deserialize = "Box<HkpPhysicsSystemWithContacts<'a>>: Deserialize<'de>"))]
    HkpPhysicsSystemWithContacts(Box<HkpPhysicsSystemWithContacts<'a>>),

    #[serde(rename = "0xff724c17")]
    #[serde(bound(deserialize = "Box<HkpPhysicsSystem<'a>>: Deserialize<'de>"))]
    HkpPhysicsSystem(Box<HkpPhysicsSystem<'a>>),

    #[serde(rename = "0xc36bbd30")]
    HkpPlaneShape(Box<HkpPlaneShape>),

    #[serde(rename = "0x8e7cb5da")]
    #[serde(bound(deserialize = "Box<HkpPointToPathConstraintData<'a>>: Deserialize<'de>"))]
    HkpPointToPathConstraintData(Box<HkpPointToPathConstraintData<'a>>),

    #[serde(rename = "0x749bc260")]
    HkpPointToPlaneConstraintDataAtoms(Box<HkpPointToPlaneConstraintDataAtoms>),

    #[serde(rename = "0x65c56e17")]
    HkpPointToPlaneConstraintData(Box<HkpPointToPlaneConstraintData>),

    #[serde(rename = "0x748fb303")]
    HkpPositionConstraintMotor(Box<HkpPositionConstraintMotor>),

    #[serde(rename = "0xf88aee25")]
    #[serde(bound(deserialize = "Box<HkpPoweredChainDataConstraintInfo<'a>>: Deserialize<'de>"))]
    HkpPoweredChainDataConstraintInfo(Box<HkpPoweredChainDataConstraintInfo<'a>>),

    #[serde(rename = "0x38aeafc3")]
    #[serde(bound(deserialize = "Box<HkpPoweredChainData<'a>>: Deserialize<'de>"))]
    HkpPoweredChainData(Box<HkpPoweredChainData<'a>>),

    #[serde(rename = "0xcf071a1b")]
    #[serde(bound(deserialize = "Box<HkpPoweredChainMapperLinkInfo<'a>>: Deserialize<'de>"))]
    HkpPoweredChainMapperLinkInfo(Box<HkpPoweredChainMapperLinkInfo<'a>>),

    #[serde(rename = "0xf651c74d")]
    #[serde(bound(deserialize = "Box<HkpPoweredChainMapperTarget<'a>>: Deserialize<'de>"))]
    HkpPoweredChainMapperTarget(Box<HkpPoweredChainMapperTarget<'a>>),

    #[serde(rename = "0x7a77ef5")]
    #[serde(bound(deserialize = "Box<HkpPoweredChainMapper<'a>>: Deserialize<'de>"))]
    HkpPoweredChainMapper(Box<HkpPoweredChainMapper<'a>>),

    #[serde(rename = "0x7f516137")]
    #[serde(bound(deserialize = "Box<HkpPrismaticConstraintDataAtoms<'a>>: Deserialize<'de>"))]
    HkpPrismaticConstraintDataAtoms(Box<HkpPrismaticConstraintDataAtoms<'a>>),

    #[serde(rename = "0x3996c387")]
    #[serde(bound(deserialize = "Box<HkpPrismaticConstraintData<'a>>: Deserialize<'de>"))]
    HkpPrismaticConstraintData(Box<HkpPrismaticConstraintData<'a>>),

    #[serde(rename = "0xb4f30148")]
    #[serde(bound(deserialize = "Box<HkpProjectileGun<'a>>: Deserialize<'de>"))]
    HkpProjectileGun(Box<HkpProjectileGun<'a>>),

    #[serde(rename = "0xc75925aa")]
    HkpPropertyValue(Box<HkpPropertyValue>),

    #[serde(rename = "0x9ce308e9")]
    HkpProperty(Box<HkpProperty>),

    #[serde(rename = "0x94a08848")]
    HkpPulleyConstraintAtom(Box<HkpPulleyConstraintAtom>),

    #[serde(rename = "0xb149e5a")]
    HkpPulleyConstraintDataAtoms(Box<HkpPulleyConstraintDataAtoms>),

    #[serde(rename = "0x972058ed")]
    HkpPulleyConstraintData(Box<HkpPulleyConstraintData>),

    #[serde(rename = "0x30cae006")]
    HkpRackAndPinionConstraintAtom(Box<HkpRackAndPinionConstraintAtom>),

    #[serde(rename = "0xa58a9659")]
    HkpRackAndPinionConstraintDataAtoms(Box<HkpRackAndPinionConstraintDataAtoms>),

    #[serde(rename = "0xd180ebe0")]
    HkpRackAndPinionConstraintData(Box<HkpRackAndPinionConstraintData>),

    #[serde(rename = "0xeed76b00")]
    #[serde(bound(deserialize = "Box<HkpRagdollConstraintDataAtoms<'a>>: Deserialize<'de>"))]
    HkpRagdollConstraintDataAtoms(Box<HkpRagdollConstraintDataAtoms<'a>>),

    #[serde(rename = "0x8fb5dd29")]
    #[serde(bound(deserialize = "Box<HkpRagdollConstraintData<'a>>: Deserialize<'de>"))]
    HkpRagdollConstraintData(Box<HkpRagdollConstraintData<'a>>),

    #[serde(rename = "0x82b894c3")]
    HkpRagdollLimitsDataAtoms(Box<HkpRagdollLimitsDataAtoms>),

    #[serde(rename = "0xcbdb44aa")]
    HkpRagdollLimitsData(Box<HkpRagdollLimitsData>),

    #[serde(rename = "0x71013826")]
    #[serde(bound(deserialize = "Box<HkpRagdollMotorConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpRagdollMotorConstraintAtom(Box<HkpRagdollMotorConstraintAtom<'a>>),

    #[serde(rename = "0xe0708a00")]
    HkpRayCollidableFilter(Box<HkpRayCollidableFilter>),

    #[serde(rename = "0xe0708a00")]
    HkpRayShapeCollectionFilter(Box<HkpRayShapeCollectionFilter>),

    #[serde(rename = "0xc4fa16c9")]
    #[serde(bound(deserialize = "Box<HkpRejectChassisListener<'a>>: Deserialize<'de>"))]
    HkpRejectChassisListener(Box<HkpRejectChassisListener<'a>>),

    #[serde(rename = "0x91367f03")]
    #[serde(bound(deserialize = "Box<HkpRemoveTerminalsMoppModifier<'a>>: Deserialize<'de>"))]
    HkpRemoveTerminalsMoppModifier(Box<HkpRemoveTerminalsMoppModifier<'a>>),

    #[serde(rename = "0x2dc0ec6a")]
    #[serde(bound(deserialize = "Box<HkpReorientAction<'a>>: Deserialize<'de>"))]
    HkpReorientAction(Box<HkpReorientAction<'a>>),

    #[serde(rename = "0x75f8d805")]
    #[serde(bound(deserialize = "Box<HkpRigidBody<'a>>: Deserialize<'de>"))]
    HkpRigidBody(Box<HkpRigidBody<'a>>),

    #[serde(rename = "0xa0c64586")]
    HkpRotationalConstraintDataAtoms(Box<HkpRotationalConstraintDataAtoms>),

    #[serde(rename = "0x74867d9e")]
    HkpRotationalConstraintData(Box<HkpRotationalConstraintData>),

    #[serde(rename = "0x11213421")]
    HkpSampledHeightFieldShape(Box<HkpSampledHeightFieldShape>),

    #[serde(rename = "0x54785c77")]
    #[serde(bound(deserialize = "Box<HkpSerializedDisplayMarkerList<'a>>: Deserialize<'de>"))]
    HkpSerializedDisplayMarkerList(Box<HkpSerializedDisplayMarkerList<'a>>),

    #[serde(rename = "0xd7c8c54f")]
    HkpSerializedDisplayMarker(Box<HkpSerializedDisplayMarker>),

    #[serde(rename = "0x94ac5bec")]
    #[serde(bound(deserialize = "Box<HkpSerializedDisplayRbTransformsDisplayTransformPair<'a>>: Deserialize<'de>"))]
    HkpSerializedDisplayRbTransformsDisplayTransformPair(Box<HkpSerializedDisplayRbTransformsDisplayTransformPair<'a>>),

    #[serde(rename = "0xc18650ac")]
    #[serde(bound(deserialize = "Box<HkpSerializedDisplayRbTransforms<'a>>: Deserialize<'de>"))]
    HkpSerializedDisplayRbTransforms(Box<HkpSerializedDisplayRbTransforms<'a>>),

    #[serde(rename = "0x10155a")]
    #[serde(bound(deserialize = "Box<HkpSerializedSubTrack1NInfo<'a>>: Deserialize<'de>"))]
    HkpSerializedSubTrack1NInfo(Box<HkpSerializedSubTrack1NInfo<'a>>),

    #[serde(rename = "0xf12d48d9")]
    #[serde(bound(deserialize = "Box<HkpSerializedTrack1NInfo<'a>>: Deserialize<'de>"))]
    HkpSerializedTrack1NInfo(Box<HkpSerializedTrack1NInfo<'a>>),

    #[serde(rename = "0xf81db8e")]
    HkpSetLocalRotationsConstraintAtom(Box<HkpSetLocalRotationsConstraintAtom>),

    #[serde(rename = "0x6e2a5198")]
    HkpSetLocalTransformsConstraintAtom(Box<HkpSetLocalTransformsConstraintAtom>),

    #[serde(rename = "0x5cbfcf4a")]
    HkpSetLocalTranslationsConstraintAtom(Box<HkpSetLocalTranslationsConstraintAtom>),

    #[serde(rename = "0xf05d137e")]
    HkpSetupStabilizationAtom(Box<HkpSetupStabilizationAtom>),

    #[serde(rename = "0xe0708a00")]
    HkpShapeCollectionFilter(Box<HkpShapeCollectionFilter>),

    #[serde(rename = "0xe8c3991d")]
    HkpShapeCollection(Box<HkpShapeCollection>),

    #[serde(rename = "0xe0708a00")]
    HkpShapeContainer(Box<HkpShapeContainer>),

    #[serde(rename = "0xea7f1d08")]
    #[serde(bound(deserialize = "Box<HkpShapeInfo<'a>>: Deserialize<'de>"))]
    HkpShapeInfo(Box<HkpShapeInfo<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpShapeModifier(Box<HkpShapeModifier>),

    #[serde(rename = "0xcb22fbcd")]
    #[serde(bound(deserialize = "Box<HkpShapePhantom<'a>>: Deserialize<'de>"))]
    HkpShapePhantom(Box<HkpShapePhantom<'a>>),

    #[serde(rename = "0x666490a1")]
    HkpShape(Box<HkpShape>),

    #[serde(rename = "0x920df11a")]
    HkpSimpleContactConstraintAtom(Box<HkpSimpleContactConstraintAtom>),

    #[serde(rename = "0xb59d1734")]
    HkpSimpleContactConstraintDataInfo(Box<HkpSimpleContactConstraintDataInfo>),

    #[serde(rename = "0xd38738c1")]
    HkpSimpleMeshShapeTriangle(Box<HkpSimpleMeshShapeTriangle>),

    #[serde(rename = "0x16b3c811")]
    HkpSimpleMeshShape(Box<HkpSimpleMeshShape>),

    #[serde(rename = "0x98bfa6ce")]
    #[serde(bound(deserialize = "Box<HkpSimpleShapePhantomCollisionDetail<'a>>: Deserialize<'de>"))]
    HkpSimpleShapePhantomCollisionDetail(Box<HkpSimpleShapePhantomCollisionDetail<'a>>),

    #[serde(rename = "0x32a2a8a8")]
    #[serde(bound(deserialize = "Box<HkpSimpleShapePhantom<'a>>: Deserialize<'de>"))]
    HkpSimpleShapePhantom(Box<HkpSimpleShapePhantom<'a>>),

    #[serde(rename = "0x97aba922")]
    #[serde(bound(deserialize = "Box<HkpSimulation<'a>>: Deserialize<'de>"))]
    HkpSimulation(Box<HkpSimulation<'a>>),

    #[serde(rename = "0x73aa1d38")]
    #[serde(bound(deserialize = "Box<HkpSingleShapeContainer<'a>>: Deserialize<'de>"))]
    HkpSingleShapeContainer(Box<HkpSingleShapeContainer<'a>>),

    #[serde(rename = "0xecb34e27")]
    #[serde(bound(deserialize = "Box<HkpSoftContactModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpSoftContactModifierConstraintAtom(Box<HkpSoftContactModifierConstraintAtom<'a>>),

    #[serde(rename = "0xbafa2bb7")]
    #[serde(bound(deserialize = "Box<HkpSphereMotion<'a>>: Deserialize<'de>"))]
    HkpSphereMotion(Box<HkpSphereMotion<'a>>),

    #[serde(rename = "0xe7eca7eb")]
    HkpSphereRepShape(Box<HkpSphereRepShape>),

    #[serde(rename = "0x795d9fa")]
    HkpSphereShape(Box<HkpSphereShape>),

    #[serde(rename = "0x88fc09fa")]
    #[serde(bound(deserialize = "Box<HkpSpringAction<'a>>: Deserialize<'de>"))]
    HkpSpringAction(Box<HkpSpringAction<'a>>),

    #[serde(rename = "0x7ead26f6")]
    HkpSpringDamperConstraintMotor(Box<HkpSpringDamperConstraintMotor>),

    #[serde(rename = "0xc624a180")]
    HkpStiffSpringChainDataConstraintInfo(Box<HkpStiffSpringChainDataConstraintInfo>),

    #[serde(rename = "0xf170356b")]
    #[serde(bound(deserialize = "Box<HkpStiffSpringChainData<'a>>: Deserialize<'de>"))]
    HkpStiffSpringChainData(Box<HkpStiffSpringChainData<'a>>),

    #[serde(rename = "0x6c128096")]
    HkpStiffSpringConstraintAtom(Box<HkpStiffSpringConstraintAtom>),

    #[serde(rename = "0x207eb376")]
    HkpStiffSpringConstraintDataAtoms(Box<HkpStiffSpringConstraintDataAtoms>),

    #[serde(rename = "0xb98f66f4")]
    HkpStiffSpringConstraintData(Box<HkpStiffSpringConstraintData>),

    #[serde(rename = "0x2ca3e906")]
    HkpStorageExtendedMeshShapeMaterial(Box<HkpStorageExtendedMeshShapeMaterial>),

    #[serde(rename = "0x5aad4de6")]
    #[serde(bound(deserialize = "Box<HkpStorageExtendedMeshShapeMeshSubpartStorage<'a>>: Deserialize<'de>"))]
    HkpStorageExtendedMeshShapeMeshSubpartStorage(Box<HkpStorageExtendedMeshShapeMeshSubpartStorage<'a>>),

    #[serde(rename = "0x3f7d804c")]
    HkpStorageExtendedMeshShapeShapeSubpartStorage(Box<HkpStorageExtendedMeshShapeShapeSubpartStorage>),

    #[serde(rename = "0xb469efbc")]
    #[serde(bound(deserialize = "Box<HkpStorageExtendedMeshShape<'a>>: Deserialize<'de>"))]
    HkpStorageExtendedMeshShape(Box<HkpStorageExtendedMeshShape<'a>>),

    #[serde(rename = "0xbf27438")]
    HkpStorageMeshShapeSubpartStorage(Box<HkpStorageMeshShapeSubpartStorage>),

    #[serde(rename = "0xbefd8b39")]
    #[serde(bound(deserialize = "Box<HkpStorageMeshShape<'a>>: Deserialize<'de>"))]
    HkpStorageMeshShape(Box<HkpStorageMeshShape<'a>>),

    #[serde(rename = "0x15ff414b")]
    HkpStorageSampledHeightFieldShape(Box<HkpStorageSampledHeightFieldShape>),

    #[serde(rename = "0x64abf85c")]
    #[serde(bound(deserialize = "Box<HkpThinBoxMotion<'a>>: Deserialize<'de>"))]
    HkpThinBoxMotion(Box<HkpThinBoxMotion<'a>>),

    #[serde(rename = "0x787ef513")]
    #[serde(bound(deserialize = "Box<HkpTransformShape<'a>>: Deserialize<'de>"))]
    HkpTransformShape(Box<HkpTransformShape<'a>>),

    #[serde(rename = "0x95ad1a25")]
    HkpTriangleShape(Box<HkpTriangleShape>),

    #[serde(rename = "0xeb60f431")]
    #[serde(bound(deserialize = "Box<HkpTriggerVolumeEventInfo<'a>>: Deserialize<'de>"))]
    HkpTriggerVolumeEventInfo(Box<HkpTriggerVolumeEventInfo<'a>>),

    #[serde(rename = "0xa29a8d1a")]
    #[serde(bound(deserialize = "Box<HkpTriggerVolume<'a>>: Deserialize<'de>"))]
    HkpTriggerVolume(Box<HkpTriggerVolume<'a>>),

    #[serde(rename = "0x58e1e585")]
    #[serde(bound(deserialize = "Box<HkpTriSampledHeightFieldBvTreeShape<'a>>: Deserialize<'de>"))]
    HkpTriSampledHeightFieldBvTreeShape(Box<HkpTriSampledHeightFieldBvTreeShape<'a>>),

    #[serde(rename = "0xc291ddde")]
    #[serde(bound(deserialize = "Box<HkpTriSampledHeightFieldCollection<'a>>: Deserialize<'de>"))]
    HkpTriSampledHeightFieldCollection(Box<HkpTriSampledHeightFieldCollection<'a>>),

    #[serde(rename = "0x7c9b1052")]
    HkpTwistLimitConstraintAtom(Box<HkpTwistLimitConstraintAtom>),

    #[serde(rename = "0xf4b0f799")]
    HkpTypedBroadPhaseHandle(Box<HkpTypedBroadPhaseHandle>),

    #[serde(rename = "0x6bb7c5e8")]
    HkpTyremarkPoint(Box<HkpTyremarkPoint>),

    #[serde(rename = "0x3d0433d6")]
    #[serde(bound(deserialize = "Box<HkpTyremarksInfo<'a>>: Deserialize<'de>"))]
    HkpTyremarksInfo(Box<HkpTyremarksInfo<'a>>),

    #[serde(rename = "0x1eaef041")]
    HkpTyremarksWheel(Box<HkpTyremarksWheel>),

    #[serde(rename = "0x895532c0")]
    #[serde(bound(deserialize = "Box<HkpUnaryAction<'a>>: Deserialize<'de>"))]
    HkpUnaryAction(Box<HkpUnaryAction<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleAerodynamics(Box<HkpVehicleAerodynamics>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleBrake(Box<HkpVehicleBrake>),

    #[serde(rename = "0x53340a9")]
    #[serde(bound(deserialize = "Box<HkpVehicleCastBatchingManager<'a>>: Deserialize<'de>"))]
    HkpVehicleCastBatchingManager(Box<HkpVehicleCastBatchingManager<'a>>),

    #[serde(rename = "0x82fe40e0")]
    HkpVehicleDataWheelComponentParams(Box<HkpVehicleDataWheelComponentParams>),

    #[serde(rename = "0x173feb43")]
    HkpVehicleData(Box<HkpVehicleData>),

    #[serde(rename = "0x42fc5bbd")]
    HkpVehicleDefaultAerodynamics(Box<HkpVehicleDefaultAerodynamics>),

    #[serde(rename = "0x123a5d50")]
    HkpVehicleDefaultAnalogDriverInput(Box<HkpVehicleDefaultAnalogDriverInput>),

    #[serde(rename = "0x1ffad971")]
    HkpVehicleDefaultBrakeWheelBrakingProperties(Box<HkpVehicleDefaultBrakeWheelBrakingProperties>),

    #[serde(rename = "0x4b4f8816")]
    HkpVehicleDefaultBrake(Box<HkpVehicleDefaultBrake>),

    #[serde(rename = "0x56f8ca24")]
    HkpVehicleDefaultEngine(Box<HkpVehicleDefaultEngine>),

    #[serde(rename = "0x8f0411c8")]
    HkpVehicleDefaultSteering(Box<HkpVehicleDefaultSteering>),

    #[serde(rename = "0x7be5bed1")]
    HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(Box<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>),

    #[serde(rename = "0x21735a24")]
    HkpVehicleDefaultSuspension(Box<HkpVehicleDefaultSuspension>),

    #[serde(rename = "0x235d5d6b")]
    HkpVehicleDefaultTransmission(Box<HkpVehicleDefaultTransmission>),

    #[serde(rename = "0x741b8d9e")]
    HkpVehicleDefaultVelocityDamper(Box<HkpVehicleDefaultVelocityDamper>),

    #[serde(rename = "0x2b4a5803")]
    HkpVehicleDriverInputAnalogStatus(Box<HkpVehicleDriverInputAnalogStatus>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleDriverInputStatus(Box<HkpVehicleDriverInputStatus>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleDriverInput(Box<HkpVehicleDriverInput>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleEngine(Box<HkpVehicleEngine>),

    #[serde(rename = "0x59ce153f")]
    HkpVehicleFrictionDescriptionAxisDescription(Box<HkpVehicleFrictionDescriptionAxisDescription>),

    #[serde(rename = "0x1034549a")]
    HkpVehicleFrictionDescription(Box<HkpVehicleFrictionDescription>),

    #[serde(rename = "0xe70e2bb4")]
    HkpVehicleFrictionStatusAxisStatus(Box<HkpVehicleFrictionStatusAxisStatus>),

    #[serde(rename = "0x1c076a84")]
    HkpVehicleFrictionStatus(Box<HkpVehicleFrictionStatus>),

    #[serde(rename = "0x99f693f0")]
    #[serde(bound(deserialize = "Box<HkpVehicleInstanceWheelInfo<'a>>: Deserialize<'de>"))]
    HkpVehicleInstanceWheelInfo(Box<HkpVehicleInstanceWheelInfo<'a>>),

    #[serde(rename = "0x877bb579")]
    #[serde(bound(deserialize = "Box<HkpVehicleInstance<'a>>: Deserialize<'de>"))]
    HkpVehicleInstance(Box<HkpVehicleInstance<'a>>),

    #[serde(rename = "0xed529f13")]
    #[serde(bound(deserialize = "Box<HkpVehicleLinearCastBatchingManager<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastBatchingManager(Box<HkpVehicleLinearCastBatchingManager<'a>>),

    #[serde(rename = "0x2a9acf98")]
    #[serde(bound(deserialize = "Box<HkpVehicleLinearCastWheelCollideWheelState<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastWheelCollideWheelState(Box<HkpVehicleLinearCastWheelCollideWheelState<'a>>),

    #[serde(rename = "0xc59399d0")]
    #[serde(bound(deserialize = "Box<HkpVehicleLinearCastWheelCollide<'a>>: Deserialize<'de>"))]
    HkpVehicleLinearCastWheelCollide(Box<HkpVehicleLinearCastWheelCollide<'a>>),

    #[serde(rename = "0xe2f7d6a7")]
    #[serde(bound(deserialize = "Box<HkpVehicleManager<'a>>: Deserialize<'de>"))]
    HkpVehicleManager(Box<HkpVehicleManager<'a>>),

    #[serde(rename = "0xed529f13")]
    #[serde(bound(deserialize = "Box<HkpVehicleRayCastBatchingManager<'a>>: Deserialize<'de>"))]
    HkpVehicleRayCastBatchingManager(Box<HkpVehicleRayCastBatchingManager<'a>>),

    #[serde(rename = "0x41efd9e3")]
    #[serde(bound(deserialize = "Box<HkpVehicleRayCastWheelCollide<'a>>: Deserialize<'de>"))]
    HkpVehicleRayCastWheelCollide(Box<HkpVehicleRayCastWheelCollide<'a>>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleSteering(Box<HkpVehicleSteering>),

    #[serde(rename = "0x358bfe9c")]
    HkpVehicleSuspensionSuspensionWheelParameters(Box<HkpVehicleSuspensionSuspensionWheelParameters>),

    #[serde(rename = "0xaf5056fa")]
    HkpVehicleSuspension(Box<HkpVehicleSuspension>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleTransmission(Box<HkpVehicleTransmission>),

    #[serde(rename = "0xda8c7d7d")]
    HkpVehicleVelocityDamper(Box<HkpVehicleVelocityDamper>),

    #[serde(rename = "0x4a50fcb")]
    HkpVehicleWheelCollide(Box<HkpVehicleWheelCollide>),

    #[serde(rename = "0xfca2fcc3")]
    HkpVelocityConstraintMotor(Box<HkpVelocityConstraintMotor>),

    #[serde(rename = "0x5c6aa14d")]
    #[serde(bound(deserialize = "Box<HkpViscousSurfaceModifierConstraintAtom<'a>>: Deserialize<'de>"))]
    HkpViscousSurfaceModifierConstraintAtom(Box<HkpViscousSurfaceModifierConstraintAtom<'a>>),

    #[serde(rename = "0xb2b41feb")]
    HkpWeldingUtility(Box<HkpWeldingUtility>),

    #[serde(rename = "0xa5255445")]
    #[serde(bound(deserialize = "Box<HkpWorldCinfo<'a>>: Deserialize<'de>"))]
    HkpWorldCinfo(Box<HkpWorldCinfo<'a>>),

    #[serde(rename = "0x49fb6f2e")]
    #[serde(bound(deserialize = "Box<HkpWorldObject<'a>>: Deserialize<'de>"))]
    HkpWorldObject(Box<HkpWorldObject<'a>>),

    #[serde(rename = "0xaadcec37")]
    #[serde(bound(deserialize = "Box<HkpWorld<'a>>: Deserialize<'de>"))]
    HkpWorld(Box<HkpWorld<'a>>),

    #[serde(rename = "0x471a21ee")]
    HkQTransform(Box<HkQTransform>),

    #[serde(rename = "0x4846be29")]
    HkRangeInt32Attribute(Box<HkRangeInt32Attribute>),

    #[serde(rename = "0x949db24f")]
    HkRangeRealAttribute(Box<HkRangeRealAttribute>),

    #[serde(rename = "0x3b1c1113")]
    HkReferencedObject(Box<HkReferencedObject>),

    #[serde(rename = "0xedb6b8f7")]
    #[serde(bound(deserialize = "Box<HkReflectedFileAttribute<'a>>: Deserialize<'de>"))]
    HkReflectedFileAttribute(Box<HkReflectedFileAttribute<'a>>),

    #[serde(rename = "0x660d7cac")]
    HkResourceBase(Box<HkResourceBase>),

    #[serde(rename = "0x4e94146")]
    HkResourceContainer(Box<HkResourceContainer>),

    #[serde(rename = "0x4e94146")]
    HkResourceHandle(Box<HkResourceHandle>),

    #[serde(rename = "0xb103a2cd")]
    #[serde(bound(deserialize = "Box<HkRootLevelContainerNamedVariant<'a>>: Deserialize<'de>"))]
    HkRootLevelContainerNamedVariant(Box<HkRootLevelContainerNamedVariant<'a>>),

    #[serde(rename = "0x2772c11e")]
    #[serde(bound(deserialize = "Box<HkRootLevelContainer<'a>>: Deserialize<'de>"))]
    HkRootLevelContainer(Box<HkRootLevelContainer<'a>>),

    #[serde(rename = "0x837099c3")]
    HkSemanticsAttribute(Box<HkSemanticsAttribute>),

    #[serde(rename = "0xe758f63c")]
    #[serde(bound(deserialize = "Box<HkSimpleLocalFrame<'a>>: Deserialize<'de>"))]
    HkSimpleLocalFrame(Box<HkSimpleLocalFrame<'a>>),

    #[serde(rename = "0x143dff99")]
    HkSphere(Box<HkSphere>),

    #[serde(rename = "0xb4e5770")]
    HkSweptTransform(Box<HkSweptTransform>),

    #[serde(rename = "0x6a4ca82c")]
    HkTraceStreamTitle(Box<HkTraceStreamTitle>),

    #[serde(rename = "0x9ab3a6ac")]
    HkTrackerSerializableScanSnapshotAllocation(Box<HkTrackerSerializableScanSnapshotAllocation>),

    #[serde(rename = "0xe7f23e6d")]
    HkTrackerSerializableScanSnapshotBlock(Box<HkTrackerSerializableScanSnapshotBlock>),

    #[serde(rename = "0x875af1d9")]
    HkTrackerSerializableScanSnapshot(Box<HkTrackerSerializableScanSnapshot>),

    #[serde(rename = "0xeb6e96e3")]
    #[serde(bound(deserialize = "Box<HkUiAttribute<'a>>: Deserialize<'de>"))]
    HkUiAttribute(Box<HkUiAttribute<'a>>),

    #[serde(rename = "0x54867cbf")]
    HkVertexFormatElement(Box<HkVertexFormatElement>),

    #[serde(rename = "0xf11e3ff7")]
    HkVertexFormat(Box<HkVertexFormat>),

    #[serde(rename = "0xda8c7d7d")]
    HkWorldMemoryAvailableWatchDog(Box<HkWorldMemoryAvailableWatchDog>),

    #[serde(rename = "0xce8b2fbd")]
    HkxAnimatedFloat(Box<HkxAnimatedFloat>),

    #[serde(rename = "0x5838e337")]
    HkxAnimatedMatrix(Box<HkxAnimatedMatrix>),

    #[serde(rename = "0xb4f01baa")]
    HkxAnimatedQuaternion(Box<HkxAnimatedQuaternion>),

    #[serde(rename = "0x34b1a197")]
    HkxAnimatedVector(Box<HkxAnimatedVector>),

    #[serde(rename = "0x345ca95d")]
    #[serde(bound(deserialize = "Box<HkxAttributeGroup<'a>>: Deserialize<'de>"))]
    HkxAttributeGroup(Box<HkxAttributeGroup<'a>>),

    #[serde(rename = "0x7468cc44")]
    #[serde(bound(deserialize = "Box<HkxAttributeHolder<'a>>: Deserialize<'de>"))]
    HkxAttributeHolder(Box<HkxAttributeHolder<'a>>),

    #[serde(rename = "0x7375cae3")]
    #[serde(bound(deserialize = "Box<HkxAttribute<'a>>: Deserialize<'de>"))]
    HkxAttribute(Box<HkxAttribute<'a>>),

    #[serde(rename = "0xe3597b02")]
    HkxCamera(Box<HkxCamera>),

    #[serde(rename = "0x9ad32a5e")]
    HkxEdgeSelectionChannel(Box<HkxEdgeSelectionChannel>),

    #[serde(rename = "0xdf4cf1e9")]
    #[serde(bound(deserialize = "Box<HkxEnumItem<'a>>: Deserialize<'de>"))]
    HkxEnumItem(Box<HkxEnumItem<'a>>),

    #[serde(rename = "0xc4e1211")]
    #[serde(bound(deserialize = "Box<HkxEnum<'a>>: Deserialize<'de>"))]
    HkxEnum(Box<HkxEnum<'a>>),

    #[serde(rename = "0xa6815115")]
    #[serde(bound(deserialize = "Box<HkxEnvironmentVariable<'a>>: Deserialize<'de>"))]
    HkxEnvironmentVariable(Box<HkxEnvironmentVariable<'a>>),

    #[serde(rename = "0x41e1aa5")]
    #[serde(bound(deserialize = "Box<HkxEnvironment<'a>>: Deserialize<'de>"))]
    HkxEnvironment(Box<HkxEnvironment<'a>>),

    #[serde(rename = "0xc12c8197")]
    HkxIndexBuffer(Box<HkxIndexBuffer>),

    #[serde(rename = "0x81c86d42")]
    HkxLight(Box<HkxLight>),

    #[serde(rename = "0x1d39f925")]
    #[serde(bound(deserialize = "Box<HkxMaterialEffect<'a>>: Deserialize<'de>"))]
    HkxMaterialEffect(Box<HkxMaterialEffect<'a>>),

    #[serde(rename = "0xd295234d")]
    HkxMaterialProperty(Box<HkxMaterialProperty>),

    #[serde(rename = "0x154650f3")]
    #[serde(bound(deserialize = "Box<HkxMaterialShaderSet<'a>>: Deserialize<'de>"))]
    HkxMaterialShaderSet(Box<HkxMaterialShaderSet<'a>>),

    #[serde(rename = "0x28515eff")]
    #[serde(bound(deserialize = "Box<HkxMaterialShader<'a>>: Deserialize<'de>"))]
    HkxMaterialShader(Box<HkxMaterialShader<'a>>),

    #[serde(rename = "0xfa6facb2")]
    #[serde(bound(deserialize = "Box<HkxMaterialTextureStage<'a>>: Deserialize<'de>"))]
    HkxMaterialTextureStage(Box<HkxMaterialTextureStage<'a>>),

    #[serde(rename = "0x2954537a")]
    #[serde(bound(deserialize = "Box<HkxMaterial<'a>>: Deserialize<'de>"))]
    HkxMaterial(Box<HkxMaterial<'a>>),

    #[serde(rename = "0xe2286cf8")]
    #[serde(bound(deserialize = "Box<HkxMeshSection<'a>>: Deserialize<'de>"))]
    HkxMeshSection(Box<HkxMeshSection<'a>>),

    #[serde(rename = "0x270724a5")]
    #[serde(bound(deserialize = "Box<HkxMeshUserChannelInfo<'a>>: Deserialize<'de>"))]
    HkxMeshUserChannelInfo(Box<HkxMeshUserChannelInfo<'a>>),

    #[serde(rename = "0xf2edcc5f")]
    #[serde(bound(deserialize = "Box<HkxMesh<'a>>: Deserialize<'de>"))]
    HkxMesh(Box<HkxMesh<'a>>),

    #[serde(rename = "0x433dee92")]
    #[serde(bound(deserialize = "Box<HkxNodeAnnotationData<'a>>: Deserialize<'de>"))]
    HkxNodeAnnotationData(Box<HkxNodeAnnotationData<'a>>),

    #[serde(rename = "0xd753fc4d")]
    #[serde(bound(deserialize = "Box<HkxNodeSelectionSet<'a>>: Deserialize<'de>"))]
    HkxNodeSelectionSet(Box<HkxNodeSelectionSet<'a>>),

    #[serde(rename = "0x5a218502")]
    #[serde(bound(deserialize = "Box<HkxNode<'a>>: Deserialize<'de>"))]
    HkxNode(Box<HkxNode<'a>>),

    #[serde(rename = "0x5f673ddd")]
    #[serde(bound(deserialize = "Box<HkxScene<'a>>: Deserialize<'de>"))]
    HkxScene(Box<HkxScene<'a>>),

    #[serde(rename = "0x5a93f338")]
    #[serde(bound(deserialize = "Box<HkxSkinBinding<'a>>: Deserialize<'de>"))]
    HkxSkinBinding(Box<HkxSkinBinding<'a>>),

    #[serde(rename = "0x7a894596")]
    HkxSparselyAnimatedBool(Box<HkxSparselyAnimatedBool>),

    #[serde(rename = "0x68a47b64")]
    #[serde(bound(deserialize = "Box<HkxSparselyAnimatedEnum<'a>>: Deserialize<'de>"))]
    HkxSparselyAnimatedEnum(Box<HkxSparselyAnimatedEnum<'a>>),

    #[serde(rename = "0xca961951")]
    HkxSparselyAnimatedInt(Box<HkxSparselyAnimatedInt>),

    #[serde(rename = "0x185da6fd")]
    #[serde(bound(deserialize = "Box<HkxSparselyAnimatedString<'a>>: Deserialize<'de>"))]
    HkxSparselyAnimatedString(Box<HkxSparselyAnimatedString<'a>>),

    #[serde(rename = "0x1e289259")]
    #[serde(bound(deserialize = "Box<HkxTextureFile<'a>>: Deserialize<'de>"))]
    HkxTextureFile(Box<HkxTextureFile<'a>>),

    #[serde(rename = "0xd45841d6")]
    #[serde(bound(deserialize = "Box<HkxTextureInplace<'a>>: Deserialize<'de>"))]
    HkxTextureInplace(Box<HkxTextureInplace<'a>>),

    #[serde(rename = "0xa02cfca9")]
    HkxTriangleSelectionChannel(Box<HkxTriangleSelectionChannel>),

    #[serde(rename = "0xd72b6fd0")]
    HkxVertexBufferVertexData(Box<HkxVertexBufferVertexData>),

    #[serde(rename = "0x4ab10615")]
    HkxVertexBuffer(Box<HkxVertexBuffer>),

    #[serde(rename = "0x483a429b")]
    HkxVertexDescriptionElementDecl(Box<HkxVertexDescriptionElementDecl>),

    #[serde(rename = "0x2df6313d")]
    HkxVertexDescription(Box<HkxVertexDescription>),

    #[serde(rename = "0xbeeb397c")]
    HkxVertexFloatDataChannel(Box<HkxVertexFloatDataChannel>),

    #[serde(rename = "0x5a50e673")]
    HkxVertexIntDataChannel(Box<HkxVertexIntDataChannel>),

    #[serde(rename = "0x866ec6d0")]
    HkxVertexSelectionChannel(Box<HkxVertexSelectionChannel>),

    #[serde(rename = "0x2ea63179")]
    HkxVertexVectorDataChannel(Box<HkxVertexVectorDataChannel>),

}
impl<'a> Serialize for Class<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Class", 668)?;
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
                let hkparams = hkparam.ok_or_else(|| de::Error::missing_field("hkparam"))?;

                Ok(Class {
                    name,
                    class,
                    signature,
                    hkparams,
                })
            }
        }

        deserializer.deserialize_map(ClassVisitor)
    }
}

impl<'a> ClassParams<'a> {
    /// Read the contents of hkparam by C++ havok Class name and create a data structure for rust.
    ///
    /// # Assumptions
    /// - The starting point of `bytes` must be the binary data position of the fields
    ///   of the class(`class_name`) to be deserialized.
    pub fn from_class_name_and_bytes<B>(
        class_name: &str,
        bytes: &[u8],
        de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
    {
        Ok(match class_name {
            "BGSGamebryoSequenceGenerator" => ClassParams::BgsGamebryoSequenceGenerator(Box::new(
                BgsGamebryoSequenceGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "BSBoneSwitchGeneratorBoneData" => ClassParams::BsBoneSwitchGeneratorBoneData(Box::new(
                BsBoneSwitchGeneratorBoneData::from_bytes::<B>(bytes, de)?,
            )),
            "BSBoneSwitchGenerator" => ClassParams::BsBoneSwitchGenerator(Box::new(
                BsBoneSwitchGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "BSComputeAddBoneAnimModifier" => ClassParams::BsComputeAddBoneAnimModifier(Box::new(
                BsComputeAddBoneAnimModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSCyclicBlendTransitionGenerator" => ClassParams::BsCyclicBlendTransitionGenerator(Box::new(
                BsCyclicBlendTransitionGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "BSDecomposeVectorModifier" => ClassParams::BsDecomposeVectorModifier(Box::new(
                BsDecomposeVectorModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSDirectAtModifier" => ClassParams::BsDirectAtModifier(Box::new(
                BsDirectAtModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSDistTriggerModifier" => ClassParams::BsDistTriggerModifier(Box::new(
                BsDistTriggerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSEventEveryNEventsModifier" => ClassParams::BsEventEveryNEventsModifier(Box::new(
                BsEventEveryNEventsModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSEventOnDeactivateModifier" => ClassParams::BsEventOnDeactivateModifier(Box::new(
                BsEventOnDeactivateModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSEventOnFalseToTrueModifier" => ClassParams::BsEventOnFalseToTrueModifier(Box::new(
                BsEventOnFalseToTrueModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSGetTimeStepModifier" => ClassParams::BsGetTimeStepModifier(Box::new(
                BsGetTimeStepModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSInterpValueModifier" => ClassParams::BsInterpValueModifier(Box::new(
                BsInterpValueModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSIsActiveModifier" => ClassParams::BsIsActiveModifier(Box::new(
                BsIsActiveModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSIStateManagerModifierBSiStateData" => ClassParams::BsiStateManagerModifierBSiStateData(Box::new(
                BsiStateManagerModifierBSiStateData::from_bytes::<B>(bytes, de)?,
            )),
            "BSIStateManagerModifierBSIStateManagerStateListener" => ClassParams::BsiStateManagerModifierBsiStateManagerStateListener(Box::new(
                BsiStateManagerModifierBsiStateManagerStateListener::from_bytes::<B>(bytes, de)?,
            )),
            "BSIStateManagerModifier" => ClassParams::BsiStateManagerModifier(Box::new(
                BsiStateManagerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSiStateTaggingGenerator" => ClassParams::BSiStateTaggingGenerator(Box::new(
                BSiStateTaggingGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "BSLimbIKModifier" => ClassParams::BsLimbIkModifier(Box::new(
                BsLimbIkModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSLookAtModifierBoneData" => ClassParams::BsLookAtModifierBoneData(Box::new(
                BsLookAtModifierBoneData::from_bytes::<B>(bytes, de)?,
            )),
            "BSLookAtModifier" => ClassParams::BsLookAtModifier(Box::new(
                BsLookAtModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSModifyOnceModifier" => ClassParams::BsModifyOnceModifier(Box::new(
                BsModifyOnceModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSOffsetAnimationGenerator" => ClassParams::BsOffsetAnimationGenerator(Box::new(
                BsOffsetAnimationGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "BSPassByTargetTriggerModifier" => ClassParams::BsPassByTargetTriggerModifier(Box::new(
                BsPassByTargetTriggerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSRagdollContactListenerModifier" => ClassParams::BsRagdollContactListenerModifier(Box::new(
                BsRagdollContactListenerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSSpeedSamplerModifier" => ClassParams::BsSpeedSamplerModifier(Box::new(
                BsSpeedSamplerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSSynchronizedClipGenerator" => ClassParams::BsSynchronizedClipGenerator(Box::new(
                BsSynchronizedClipGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "BSTimerModifier" => ClassParams::BsTimerModifier(Box::new(
                BsTimerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "BSTweenerModifier" => ClassParams::BsTweenerModifier(Box::new(
                BsTweenerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkAabbHalf" => ClassParams::HkAabbHalf(Box::new(
                HkAabbHalf::from_bytes::<B>(bytes, de)?,
            )),
            "hkAabbUint32" => ClassParams::HkAabbUint32(Box::new(
                HkAabbUint32::from_bytes::<B>(bytes, de)?,
            )),
            "hkAabb" => ClassParams::HkAabb(Box::new(
                HkAabb::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnimatedReferenceFrame" => ClassParams::HkaAnimatedReferenceFrame(Box::new(
                HkaAnimatedReferenceFrame::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnimationBinding" => ClassParams::HkaAnimationBinding(Box::new(
                HkaAnimationBinding::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnimationContainer" => ClassParams::HkaAnimationContainer(Box::new(
                HkaAnimationContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnimationPreviewColorContainer" => ClassParams::HkaAnimationPreviewColorContainer(Box::new(
                HkaAnimationPreviewColorContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnimation" => ClassParams::HkaAnimation(Box::new(
                HkaAnimation::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnnotationTrackAnnotation" => ClassParams::HkaAnnotationTrackAnnotation(Box::new(
                HkaAnnotationTrackAnnotation::from_bytes::<B>(bytes, de)?,
            )),
            "hkaAnnotationTrack" => ClassParams::HkaAnnotationTrack(Box::new(
                HkaAnnotationTrack::from_bytes::<B>(bytes, de)?,
            )),
            "hkaBoneAttachment" => ClassParams::HkaBoneAttachment(Box::new(
                HkaBoneAttachment::from_bytes::<B>(bytes, de)?,
            )),
            "hkaBone" => ClassParams::HkaBone(Box::new(
                HkaBone::from_bytes::<B>(bytes, de)?,
            )),
            "hkaDefaultAnimatedReferenceFrame" => ClassParams::HkaDefaultAnimatedReferenceFrame(Box::new(
                HkaDefaultAnimatedReferenceFrame::from_bytes::<B>(bytes, de)?,
            )),
            "hkaDeltaCompressedAnimationQuantizationFormat" => ClassParams::HkaDeltaCompressedAnimationQuantizationFormat(Box::new(
                HkaDeltaCompressedAnimationQuantizationFormat::from_bytes::<B>(bytes, de)?,
            )),
            "hkaDeltaCompressedAnimation" => ClassParams::HkaDeltaCompressedAnimation(Box::new(
                HkaDeltaCompressedAnimation::from_bytes::<B>(bytes, de)?,
            )),
            "hkaFootstepAnalysisInfoContainer" => ClassParams::HkaFootstepAnalysisInfoContainer(Box::new(
                HkaFootstepAnalysisInfoContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkaFootstepAnalysisInfo" => ClassParams::HkaFootstepAnalysisInfo(Box::new(
                HkaFootstepAnalysisInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkaInterleavedUncompressedAnimation" => ClassParams::HkaInterleavedUncompressedAnimation(Box::new(
                HkaInterleavedUncompressedAnimation::from_bytes::<B>(bytes, de)?,
            )),
            "hkaKeyFrameHierarchyUtilityControlData" => ClassParams::HkaKeyFrameHierarchyUtilityControlData(Box::new(
                HkaKeyFrameHierarchyUtilityControlData::from_bytes::<B>(bytes, de)?,
            )),
            "hkaKeyFrameHierarchyUtility" => ClassParams::HkaKeyFrameHierarchyUtility(Box::new(
                HkaKeyFrameHierarchyUtility::from_bytes::<B>(bytes, de)?,
            )),
            "hkAlignSceneToNodeOptions" => ClassParams::HkAlignSceneToNodeOptions(Box::new(
                HkAlignSceneToNodeOptions::from_bytes::<B>(bytes, de)?,
            )),
            "hkaMeshBindingMapping" => ClassParams::HkaMeshBindingMapping(Box::new(
                HkaMeshBindingMapping::from_bytes::<B>(bytes, de)?,
            )),
            "hkaMeshBinding" => ClassParams::HkaMeshBinding(Box::new(
                HkaMeshBinding::from_bytes::<B>(bytes, de)?,
            )),
            "hkaQuantizedAnimationTrackCompressionParams" => ClassParams::HkaQuantizedAnimationTrackCompressionParams(Box::new(
                HkaQuantizedAnimationTrackCompressionParams::from_bytes::<B>(bytes, de)?,
            )),
            "hkaQuantizedAnimation" => ClassParams::HkaQuantizedAnimation(Box::new(
                HkaQuantizedAnimation::from_bytes::<B>(bytes, de)?,
            )),
            "hkaRagdollInstance" => ClassParams::HkaRagdollInstance(Box::new(
                HkaRagdollInstance::from_bytes::<B>(bytes, de)?,
            )),
            "hkArrayTypeAttribute" => ClassParams::HkArrayTypeAttribute(Box::new(
                HkArrayTypeAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSkeletonLocalFrameOnBone" => ClassParams::HkaSkeletonLocalFrameOnBone(Box::new(
                HkaSkeletonLocalFrameOnBone::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSkeletonMapperDataChainMapping" => ClassParams::HkaSkeletonMapperDataChainMapping(Box::new(
                HkaSkeletonMapperDataChainMapping::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSkeletonMapperDataSimpleMapping" => ClassParams::HkaSkeletonMapperDataSimpleMapping(Box::new(
                HkaSkeletonMapperDataSimpleMapping::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSkeletonMapperData" => ClassParams::HkaSkeletonMapperData(Box::new(
                HkaSkeletonMapperData::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSkeletonMapper" => ClassParams::HkaSkeletonMapper(Box::new(
                HkaSkeletonMapper::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSkeleton" => ClassParams::HkaSkeleton(Box::new(
                HkaSkeleton::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSplineCompressedAnimationAnimationCompressionParams" => ClassParams::HkaSplineCompressedAnimationAnimationCompressionParams(Box::new(
                HkaSplineCompressedAnimationAnimationCompressionParams::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSplineCompressedAnimationTrackCompressionParams" => ClassParams::HkaSplineCompressedAnimationTrackCompressionParams(Box::new(
                HkaSplineCompressedAnimationTrackCompressionParams::from_bytes::<B>(bytes, de)?,
            )),
            "hkaSplineCompressedAnimation" => ClassParams::HkaSplineCompressedAnimation(Box::new(
                HkaSplineCompressedAnimation::from_bytes::<B>(bytes, de)?,
            )),
            "hkaWaveletCompressedAnimationCompressionParams" => ClassParams::HkaWaveletCompressedAnimationCompressionParams(Box::new(
                HkaWaveletCompressedAnimationCompressionParams::from_bytes::<B>(bytes, de)?,
            )),
            "hkaWaveletCompressedAnimationQuantizationFormat" => ClassParams::HkaWaveletCompressedAnimationQuantizationFormat(Box::new(
                HkaWaveletCompressedAnimationQuantizationFormat::from_bytes::<B>(bytes, de)?,
            )),
            "hkaWaveletCompressedAnimation" => ClassParams::HkaWaveletCompressedAnimation(Box::new(
                HkaWaveletCompressedAnimation::from_bytes::<B>(bytes, de)?,
            )),
            "hkBaseObject" => ClassParams::HkBaseObject(Box::new(
                HkBaseObject::from_bytes::<B>(bytes, de)?,
            )),
            "hkbAttachmentModifier" => ClassParams::HkbAttachmentModifier(Box::new(
                HkbAttachmentModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbAttachmentSetup" => ClassParams::HkbAttachmentSetup(Box::new(
                HkbAttachmentSetup::from_bytes::<B>(bytes, de)?,
            )),
            "hkbAttributeModifierAssignment" => ClassParams::HkbAttributeModifierAssignment(Box::new(
                HkbAttributeModifierAssignment::from_bytes::<B>(bytes, de)?,
            )),
            "hkbAttributeModifier" => ClassParams::HkbAttributeModifier(Box::new(
                HkbAttributeModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbAuxiliaryNodeInfo" => ClassParams::HkbAuxiliaryNodeInfo(Box::new(
                HkbAuxiliaryNodeInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorEventsInfo" => ClassParams::HkbBehaviorEventsInfo(Box::new(
                HkbBehaviorEventsInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorGraphData" => ClassParams::HkbBehaviorGraphData(Box::new(
                HkbBehaviorGraphData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorGraphInternalStateInfo" => ClassParams::HkbBehaviorGraphInternalStateInfo(Box::new(
                HkbBehaviorGraphInternalStateInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorGraphInternalState" => ClassParams::HkbBehaviorGraphInternalState(Box::new(
                HkbBehaviorGraphInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorGraphStringData" => ClassParams::HkbBehaviorGraphStringData(Box::new(
                HkbBehaviorGraphStringData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorGraph" => ClassParams::HkbBehaviorGraph(Box::new(
                HkbBehaviorGraph::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorInfoIdToNamePair" => ClassParams::HkbBehaviorInfoIdToNamePair(Box::new(
                HkbBehaviorInfoIdToNamePair::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorInfo" => ClassParams::HkbBehaviorInfo(Box::new(
                HkbBehaviorInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBehaviorReferenceGenerator" => ClassParams::HkbBehaviorReferenceGenerator(Box::new(
                HkbBehaviorReferenceGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBindable" => ClassParams::HkbBindable(Box::new(
                HkbBindable::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlendCurveUtils" => ClassParams::HkbBlendCurveUtils(Box::new(
                HkbBlendCurveUtils::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlenderGeneratorChildInternalState" => ClassParams::HkbBlenderGeneratorChildInternalState(Box::new(
                HkbBlenderGeneratorChildInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlenderGeneratorChild" => ClassParams::HkbBlenderGeneratorChild(Box::new(
                HkbBlenderGeneratorChild::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlenderGeneratorInternalState" => ClassParams::HkbBlenderGeneratorInternalState(Box::new(
                HkbBlenderGeneratorInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlenderGenerator" => ClassParams::HkbBlenderGenerator(Box::new(
                HkbBlenderGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlendingTransitionEffectInternalState" => ClassParams::HkbBlendingTransitionEffectInternalState(Box::new(
                HkbBlendingTransitionEffectInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBlendingTransitionEffect" => ClassParams::HkbBlendingTransitionEffect(Box::new(
                HkbBlendingTransitionEffect::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBoneIndexArray" => ClassParams::HkbBoneIndexArray(Box::new(
                HkbBoneIndexArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBoneWeightArray" => ClassParams::HkbBoneWeightArray(Box::new(
                HkbBoneWeightArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBoolVariableSequencedDataSample" => ClassParams::HkbBoolVariableSequencedDataSample(Box::new(
                HkbBoolVariableSequencedDataSample::from_bytes::<B>(bytes, de)?,
            )),
            "hkbBoolVariableSequencedData" => ClassParams::HkbBoolVariableSequencedData(Box::new(
                HkbBoolVariableSequencedData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCameraShakeEventPayload" => ClassParams::HkbCameraShakeEventPayload(Box::new(
                HkbCameraShakeEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterAddedInfo" => ClassParams::HkbCharacterAddedInfo(Box::new(
                HkbCharacterAddedInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterControlCommand" => ClassParams::HkbCharacterControlCommand(Box::new(
                HkbCharacterControlCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterControllerControlData" => ClassParams::HkbCharacterControllerControlData(Box::new(
                HkbCharacterControllerControlData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterControllerModifierInternalState" => ClassParams::HkbCharacterControllerModifierInternalState(Box::new(
                HkbCharacterControllerModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterControllerModifier" => ClassParams::HkbCharacterControllerModifier(Box::new(
                HkbCharacterControllerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterDataCharacterControllerInfo" => ClassParams::HkbCharacterDataCharacterControllerInfo(Box::new(
                HkbCharacterDataCharacterControllerInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterData" => ClassParams::HkbCharacterData(Box::new(
                HkbCharacterData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterInfo" => ClassParams::HkbCharacterInfo(Box::new(
                HkbCharacterInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterSetup" => ClassParams::HkbCharacterSetup(Box::new(
                HkbCharacterSetup::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterSkinInfo" => ClassParams::HkbCharacterSkinInfo(Box::new(
                HkbCharacterSkinInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterSteppedInfo" => ClassParams::HkbCharacterSteppedInfo(Box::new(
                HkbCharacterSteppedInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacterStringData" => ClassParams::HkbCharacterStringData(Box::new(
                HkbCharacterStringData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCharacter" => ClassParams::HkbCharacter(Box::new(
                HkbCharacter::from_bytes::<B>(bytes, de)?,
            )),
            "hkbClientCharacterState" => ClassParams::HkbClientCharacterState(Box::new(
                HkbClientCharacterState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbClipGeneratorEcho" => ClassParams::HkbClipGeneratorEcho(Box::new(
                HkbClipGeneratorEcho::from_bytes::<B>(bytes, de)?,
            )),
            "hkbClipGeneratorInternalState" => ClassParams::HkbClipGeneratorInternalState(Box::new(
                HkbClipGeneratorInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbClipGenerator" => ClassParams::HkbClipGenerator(Box::new(
                HkbClipGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbClipTriggerArray" => ClassParams::HkbClipTriggerArray(Box::new(
                HkbClipTriggerArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbClipTrigger" => ClassParams::HkbClipTrigger(Box::new(
                HkbClipTrigger::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCombineTransformsModifierInternalState" => ClassParams::HkbCombineTransformsModifierInternalState(Box::new(
                HkbCombineTransformsModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCombineTransformsModifier" => ClassParams::HkbCombineTransformsModifier(Box::new(
                HkbCombineTransformsModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCompiledExpressionSetToken" => ClassParams::HkbCompiledExpressionSetToken(Box::new(
                HkbCompiledExpressionSetToken::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCompiledExpressionSet" => ClassParams::HkbCompiledExpressionSet(Box::new(
                HkbCompiledExpressionSet::from_bytes::<B>(bytes, de)?,
            )),
            "hkbComputeDirectionModifierInternalState" => ClassParams::HkbComputeDirectionModifierInternalState(Box::new(
                HkbComputeDirectionModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbComputeDirectionModifier" => ClassParams::HkbComputeDirectionModifier(Box::new(
                HkbComputeDirectionModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbComputeRotationFromAxisAngleModifierInternalState" => ClassParams::HkbComputeRotationFromAxisAngleModifierInternalState(Box::new(
                HkbComputeRotationFromAxisAngleModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbComputeRotationFromAxisAngleModifier" => ClassParams::HkbComputeRotationFromAxisAngleModifier(Box::new(
                HkbComputeRotationFromAxisAngleModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbComputeRotationToTargetModifierInternalState" => ClassParams::HkbComputeRotationToTargetModifierInternalState(Box::new(
                HkbComputeRotationToTargetModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbComputeRotationToTargetModifier" => ClassParams::HkbComputeRotationToTargetModifier(Box::new(
                HkbComputeRotationToTargetModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbCondition" => ClassParams::HkbCondition(Box::new(
                HkbCondition::from_bytes::<B>(bytes, de)?,
            )),
            "hkbContext" => ClassParams::HkbContext(Box::new(
                HkbContext::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDampingModifierInternalState" => ClassParams::HkbDampingModifierInternalState(Box::new(
                HkbDampingModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDampingModifier" => ClassParams::HkbDampingModifier(Box::new(
                HkbDampingModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDefaultMessageLog" => ClassParams::HkbDefaultMessageLog(Box::new(
                HkbDefaultMessageLog::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDelayedModifierInternalState" => ClassParams::HkbDelayedModifierInternalState(Box::new(
                HkbDelayedModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDelayedModifier" => ClassParams::HkbDelayedModifier(Box::new(
                HkbDelayedModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDetectCloseToGroundModifierInternalState" => ClassParams::HkbDetectCloseToGroundModifierInternalState(Box::new(
                HkbDetectCloseToGroundModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbDetectCloseToGroundModifier" => ClassParams::HkbDetectCloseToGroundModifier(Box::new(
                HkbDetectCloseToGroundModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEvaluateExpressionModifierInternalExpressionData" => ClassParams::HkbEvaluateExpressionModifierInternalExpressionData(Box::new(
                HkbEvaluateExpressionModifierInternalExpressionData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEvaluateExpressionModifierInternalState" => ClassParams::HkbEvaluateExpressionModifierInternalState(Box::new(
                HkbEvaluateExpressionModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEvaluateExpressionModifier" => ClassParams::HkbEvaluateExpressionModifier(Box::new(
                HkbEvaluateExpressionModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEvaluateHandleModifier" => ClassParams::HkbEvaluateHandleModifier(Box::new(
                HkbEvaluateHandleModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventBase" => ClassParams::HkbEventBase(Box::new(
                HkbEventBase::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventDrivenModifierInternalState" => ClassParams::HkbEventDrivenModifierInternalState(Box::new(
                HkbEventDrivenModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventDrivenModifier" => ClassParams::HkbEventDrivenModifier(Box::new(
                HkbEventDrivenModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventInfo" => ClassParams::HkbEventInfo(Box::new(
                HkbEventInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventPayloadList" => ClassParams::HkbEventPayloadList(Box::new(
                HkbEventPayloadList::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventPayload" => ClassParams::HkbEventPayload(Box::new(
                HkbEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventProperty" => ClassParams::HkbEventProperty(Box::new(
                HkbEventProperty::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventRaisedInfo" => ClassParams::HkbEventRaisedInfo(Box::new(
                HkbEventRaisedInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventRangeDataArray" => ClassParams::HkbEventRangeDataArray(Box::new(
                HkbEventRangeDataArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventRangeData" => ClassParams::HkbEventRangeData(Box::new(
                HkbEventRangeData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventSequencedDataSequencedEvent" => ClassParams::HkbEventSequencedDataSequencedEvent(Box::new(
                HkbEventSequencedDataSequencedEvent::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventSequencedData" => ClassParams::HkbEventSequencedData(Box::new(
                HkbEventSequencedData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventsFromRangeModifierInternalState" => ClassParams::HkbEventsFromRangeModifierInternalState(Box::new(
                HkbEventsFromRangeModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEventsFromRangeModifier" => ClassParams::HkbEventsFromRangeModifier(Box::new(
                HkbEventsFromRangeModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbEvent" => ClassParams::HkbEvent(Box::new(
                HkbEvent::from_bytes::<B>(bytes, de)?,
            )),
            "hkbExpressionCondition" => ClassParams::HkbExpressionCondition(Box::new(
                HkbExpressionCondition::from_bytes::<B>(bytes, de)?,
            )),
            "hkbExpressionDataArray" => ClassParams::HkbExpressionDataArray(Box::new(
                HkbExpressionDataArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbExpressionData" => ClassParams::HkbExpressionData(Box::new(
                HkbExpressionData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbExtractRagdollPoseModifier" => ClassParams::HkbExtractRagdollPoseModifier(Box::new(
                HkbExtractRagdollPoseModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkControlData" => ClassParams::HkbFootIkControlData(Box::new(
                HkbFootIkControlData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkControlsModifierLeg" => ClassParams::HkbFootIkControlsModifierLeg(Box::new(
                HkbFootIkControlsModifierLeg::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkControlsModifier" => ClassParams::HkbFootIkControlsModifier(Box::new(
                HkbFootIkControlsModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkDriverInfoLeg" => ClassParams::HkbFootIkDriverInfoLeg(Box::new(
                HkbFootIkDriverInfoLeg::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkDriverInfo" => ClassParams::HkbFootIkDriverInfo(Box::new(
                HkbFootIkDriverInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkGains" => ClassParams::HkbFootIkGains(Box::new(
                HkbFootIkGains::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkModifierInternalLegData" => ClassParams::HkbFootIkModifierInternalLegData(Box::new(
                HkbFootIkModifierInternalLegData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkModifierLeg" => ClassParams::HkbFootIkModifierLeg(Box::new(
                HkbFootIkModifierLeg::from_bytes::<B>(bytes, de)?,
            )),
            "hkbFootIkModifier" => ClassParams::HkbFootIkModifier(Box::new(
                HkbFootIkModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGeneratorOutputListener" => ClassParams::HkbGeneratorOutputListener(Box::new(
                HkbGeneratorOutputListener::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGeneratorSyncInfoSyncPoint" => ClassParams::HkbGeneratorSyncInfoSyncPoint(Box::new(
                HkbGeneratorSyncInfoSyncPoint::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGeneratorSyncInfo" => ClassParams::HkbGeneratorSyncInfo(Box::new(
                HkbGeneratorSyncInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGeneratorTransitionEffectInternalState" => ClassParams::HkbGeneratorTransitionEffectInternalState(Box::new(
                HkbGeneratorTransitionEffectInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGeneratorTransitionEffect" => ClassParams::HkbGeneratorTransitionEffect(Box::new(
                HkbGeneratorTransitionEffect::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGenerator" => ClassParams::HkbGenerator(Box::new(
                HkbGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGetHandleOnBoneModifier" => ClassParams::HkbGetHandleOnBoneModifier(Box::new(
                HkbGetHandleOnBoneModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGetUpModifierInternalState" => ClassParams::HkbGetUpModifierInternalState(Box::new(
                HkbGetUpModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGetUpModifier" => ClassParams::HkbGetUpModifier(Box::new(
                HkbGetUpModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGetWorldFromModelModifierInternalState" => ClassParams::HkbGetWorldFromModelModifierInternalState(Box::new(
                HkbGetWorldFromModelModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbGetWorldFromModelModifier" => ClassParams::HkbGetWorldFromModelModifier(Box::new(
                HkbGetWorldFromModelModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkControlData" => ClassParams::HkbHandIkControlData(Box::new(
                HkbHandIkControlData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkControlsModifierHand" => ClassParams::HkbHandIkControlsModifierHand(Box::new(
                HkbHandIkControlsModifierHand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkControlsModifier" => ClassParams::HkbHandIkControlsModifier(Box::new(
                HkbHandIkControlsModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkDriverInfoHand" => ClassParams::HkbHandIkDriverInfoHand(Box::new(
                HkbHandIkDriverInfoHand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkDriverInfo" => ClassParams::HkbHandIkDriverInfo(Box::new(
                HkbHandIkDriverInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkModifierHand" => ClassParams::HkbHandIkModifierHand(Box::new(
                HkbHandIkModifierHand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandIkModifier" => ClassParams::HkbHandIkModifier(Box::new(
                HkbHandIkModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbHandle" => ClassParams::HkbHandle(Box::new(
                HkbHandle::from_bytes::<B>(bytes, de)?,
            )),
            "hkbIntEventPayload" => ClassParams::HkbIntEventPayload(Box::new(
                HkbIntEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbIntVariableSequencedDataSample" => ClassParams::HkbIntVariableSequencedDataSample(Box::new(
                HkbIntVariableSequencedDataSample::from_bytes::<B>(bytes, de)?,
            )),
            "hkbIntVariableSequencedData" => ClassParams::HkbIntVariableSequencedData(Box::new(
                HkbIntVariableSequencedData::from_bytes::<B>(bytes, de)?,
            )),
            "hkBitField" => ClassParams::HkBitField(Box::new(
                HkBitField::from_bytes::<B>(bytes, de)?,
            )),
            "hkbKeyframeBonesModifierKeyframeInfo" => ClassParams::HkbKeyframeBonesModifierKeyframeInfo(Box::new(
                HkbKeyframeBonesModifierKeyframeInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbKeyframeBonesModifier" => ClassParams::HkbKeyframeBonesModifier(Box::new(
                HkbKeyframeBonesModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbLinkedSymbolInfo" => ClassParams::HkbLinkedSymbolInfo(Box::new(
                HkbLinkedSymbolInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbLookAtModifierInternalState" => ClassParams::HkbLookAtModifierInternalState(Box::new(
                HkbLookAtModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbLookAtModifier" => ClassParams::HkbLookAtModifier(Box::new(
                HkbLookAtModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbManualSelectorGeneratorInternalState" => ClassParams::HkbManualSelectorGeneratorInternalState(Box::new(
                HkbManualSelectorGeneratorInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbManualSelectorGenerator" => ClassParams::HkbManualSelectorGenerator(Box::new(
                HkbManualSelectorGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbMessageLog" => ClassParams::HkbMessageLog(Box::new(
                HkbMessageLog::from_bytes::<B>(bytes, de)?,
            )),
            "hkbMirroredSkeletonInfo" => ClassParams::HkbMirroredSkeletonInfo(Box::new(
                HkbMirroredSkeletonInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbMirrorModifier" => ClassParams::HkbMirrorModifier(Box::new(
                HkbMirrorModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbModifierGenerator" => ClassParams::HkbModifierGenerator(Box::new(
                HkbModifierGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbModifierList" => ClassParams::HkbModifierList(Box::new(
                HkbModifierList::from_bytes::<B>(bytes, de)?,
            )),
            "hkbModifierWrapper" => ClassParams::HkbModifierWrapper(Box::new(
                HkbModifierWrapper::from_bytes::<B>(bytes, de)?,
            )),
            "hkbModifier" => ClassParams::HkbModifier(Box::new(
                HkbModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbMoveCharacterModifierInternalState" => ClassParams::HkbMoveCharacterModifierInternalState(Box::new(
                HkbMoveCharacterModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbMoveCharacterModifier" => ClassParams::HkbMoveCharacterModifier(Box::new(
                HkbMoveCharacterModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbNamedEventPayload" => ClassParams::HkbNamedEventPayload(Box::new(
                HkbNamedEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbNamedIntEventPayload" => ClassParams::HkbNamedIntEventPayload(Box::new(
                HkbNamedIntEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbNamedRealEventPayload" => ClassParams::HkbNamedRealEventPayload(Box::new(
                HkbNamedRealEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbNamedStringEventPayload" => ClassParams::HkbNamedStringEventPayload(Box::new(
                HkbNamedStringEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbNodeInternalStateInfo" => ClassParams::HkbNodeInternalStateInfo(Box::new(
                HkbNodeInternalStateInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbNode" => ClassParams::HkbNode(Box::new(
                HkbNode::from_bytes::<B>(bytes, de)?,
            )),
            "hkbParticleSystemEventPayload" => ClassParams::HkbParticleSystemEventPayload(Box::new(
                HkbParticleSystemEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbPoseMatchingGeneratorInternalState" => ClassParams::HkbPoseMatchingGeneratorInternalState(Box::new(
                HkbPoseMatchingGeneratorInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbPoseMatchingGenerator" => ClassParams::HkbPoseMatchingGenerator(Box::new(
                HkbPoseMatchingGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbPoweredRagdollControlData" => ClassParams::HkbPoweredRagdollControlData(Box::new(
                HkbPoweredRagdollControlData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbPoweredRagdollControlsModifier" => ClassParams::HkbPoweredRagdollControlsModifier(Box::new(
                HkbPoweredRagdollControlsModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbProjectData" => ClassParams::HkbProjectData(Box::new(
                HkbProjectData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbProjectStringData" => ClassParams::HkbProjectStringData(Box::new(
                HkbProjectStringData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbProxyModifierProxyInfo" => ClassParams::HkbProxyModifierProxyInfo(Box::new(
                HkbProxyModifierProxyInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbProxyModifier" => ClassParams::HkbProxyModifier(Box::new(
                HkbProxyModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRaiseEventCommand" => ClassParams::HkbRaiseEventCommand(Box::new(
                HkbRaiseEventCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRealEventPayload" => ClassParams::HkbRealEventPayload(Box::new(
                HkbRealEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRealVariableSequencedDataSample" => ClassParams::HkbRealVariableSequencedDataSample(Box::new(
                HkbRealVariableSequencedDataSample::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRealVariableSequencedData" => ClassParams::HkbRealVariableSequencedData(Box::new(
                HkbRealVariableSequencedData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbReferencePoseGenerator" => ClassParams::HkbReferencePoseGenerator(Box::new(
                HkbReferencePoseGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRegisteredGenerator" => ClassParams::HkbRegisteredGenerator(Box::new(
                HkbRegisteredGenerator::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRigidBodyRagdollControlData" => ClassParams::HkbRigidBodyRagdollControlData(Box::new(
                HkbRigidBodyRagdollControlData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRigidBodyRagdollControlsModifier" => ClassParams::HkbRigidBodyRagdollControlsModifier(Box::new(
                HkbRigidBodyRagdollControlsModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRoleAttribute" => ClassParams::HkbRoleAttribute(Box::new(
                HkbRoleAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRotateCharacterModifierInternalState" => ClassParams::HkbRotateCharacterModifierInternalState(Box::new(
                HkbRotateCharacterModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbRotateCharacterModifier" => ClassParams::HkbRotateCharacterModifier(Box::new(
                HkbRotateCharacterModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSenseHandleModifierRange" => ClassParams::HkbSenseHandleModifierRange(Box::new(
                HkbSenseHandleModifierRange::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSenseHandleModifier" => ClassParams::HkbSenseHandleModifier(Box::new(
                HkbSenseHandleModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSequencedData" => ClassParams::HkbSequencedData(Box::new(
                HkbSequencedData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSequenceInternalState" => ClassParams::HkbSequenceInternalState(Box::new(
                HkbSequenceInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSequenceStringData" => ClassParams::HkbSequenceStringData(Box::new(
                HkbSequenceStringData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSequence" => ClassParams::HkbSequence(Box::new(
                HkbSequence::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSetBehaviorCommand" => ClassParams::HkbSetBehaviorCommand(Box::new(
                HkbSetBehaviorCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSetLocalTimeOfClipGeneratorCommand" => ClassParams::HkbSetLocalTimeOfClipGeneratorCommand(Box::new(
                HkbSetLocalTimeOfClipGeneratorCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSetNodePropertyCommand" => ClassParams::HkbSetNodePropertyCommand(Box::new(
                HkbSetNodePropertyCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSetWordVariableCommand" => ClassParams::HkbSetWordVariableCommand(Box::new(
                HkbSetWordVariableCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSetWorldFromModelModifier" => ClassParams::HkbSetWorldFromModelModifier(Box::new(
                HkbSetWorldFromModelModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSimulationControlCommand" => ClassParams::HkbSimulationControlCommand(Box::new(
                HkbSimulationControlCommand::from_bytes::<B>(bytes, de)?,
            )),
            "hkbSimulationStateInfo" => ClassParams::HkbSimulationStateInfo(Box::new(
                HkbSimulationStateInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateChooser" => ClassParams::HkbStateChooser(Box::new(
                HkbStateChooser::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateListener" => ClassParams::HkbStateListener(Box::new(
                HkbStateListener::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineActiveTransitionInfo" => ClassParams::HkbStateMachineActiveTransitionInfo(Box::new(
                HkbStateMachineActiveTransitionInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineDelayedTransitionInfo" => ClassParams::HkbStateMachineDelayedTransitionInfo(Box::new(
                HkbStateMachineDelayedTransitionInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineEventPropertyArray" => ClassParams::HkbStateMachineEventPropertyArray(Box::new(
                HkbStateMachineEventPropertyArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineInternalState" => ClassParams::HkbStateMachineInternalState(Box::new(
                HkbStateMachineInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineNestedStateMachineData" => ClassParams::HkbStateMachineNestedStateMachineData(Box::new(
                HkbStateMachineNestedStateMachineData::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineProspectiveTransitionInfo" => ClassParams::HkbStateMachineProspectiveTransitionInfo(Box::new(
                HkbStateMachineProspectiveTransitionInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineStateInfo" => ClassParams::HkbStateMachineStateInfo(Box::new(
                HkbStateMachineStateInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineTimeInterval" => ClassParams::HkbStateMachineTimeInterval(Box::new(
                HkbStateMachineTimeInterval::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineTransitionInfoArray" => ClassParams::HkbStateMachineTransitionInfoArray(Box::new(
                HkbStateMachineTransitionInfoArray::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineTransitionInfoReference" => ClassParams::HkbStateMachineTransitionInfoReference(Box::new(
                HkbStateMachineTransitionInfoReference::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachineTransitionInfo" => ClassParams::HkbStateMachineTransitionInfo(Box::new(
                HkbStateMachineTransitionInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStateMachine" => ClassParams::HkbStateMachine(Box::new(
                HkbStateMachine::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStringCondition" => ClassParams::HkbStringCondition(Box::new(
                HkbStringCondition::from_bytes::<B>(bytes, de)?,
            )),
            "hkbStringEventPayload" => ClassParams::HkbStringEventPayload(Box::new(
                HkbStringEventPayload::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTestStateChooser" => ClassParams::HkbTestStateChooser(Box::new(
                HkbTestStateChooser::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTimerModifierInternalState" => ClassParams::HkbTimerModifierInternalState(Box::new(
                HkbTimerModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTimerModifier" => ClassParams::HkbTimerModifier(Box::new(
                HkbTimerModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTransformVectorModifierInternalState" => ClassParams::HkbTransformVectorModifierInternalState(Box::new(
                HkbTransformVectorModifierInternalState::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTransformVectorModifier" => ClassParams::HkbTransformVectorModifier(Box::new(
                HkbTransformVectorModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTransitionEffect" => ClassParams::HkbTransitionEffect(Box::new(
                HkbTransitionEffect::from_bytes::<B>(bytes, de)?,
            )),
            "hkbTwistModifier" => ClassParams::HkbTwistModifier(Box::new(
                HkbTwistModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkbVariableBindingSetBinding" => ClassParams::HkbVariableBindingSetBinding(Box::new(
                HkbVariableBindingSetBinding::from_bytes::<B>(bytes, de)?,
            )),
            "hkbVariableBindingSet" => ClassParams::HkbVariableBindingSet(Box::new(
                HkbVariableBindingSet::from_bytes::<B>(bytes, de)?,
            )),
            "hkbVariableInfo" => ClassParams::HkbVariableInfo(Box::new(
                HkbVariableInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkbVariableValueSet" => ClassParams::HkbVariableValueSet(Box::new(
                HkbVariableValueSet::from_bytes::<B>(bytes, de)?,
            )),
            "hkbVariableValue" => ClassParams::HkbVariableValue(Box::new(
                HkbVariableValue::from_bytes::<B>(bytes, de)?,
            )),
            "hkbWorldEnums" => ClassParams::HkbWorldEnums(Box::new(
                HkbWorldEnums::from_bytes::<B>(bytes, de)?,
            )),
            "hkbWorldFromModelModeData" => ClassParams::HkbWorldFromModelModeData(Box::new(
                HkbWorldFromModelModeData::from_bytes::<B>(bytes, de)?,
            )),
            "hkClassEnumItem" => ClassParams::HkClassEnumItem(Box::new(
                HkClassEnumItem::from_bytes::<B>(bytes, de)?,
            )),
            "hkClassEnum" => ClassParams::HkClassEnum(Box::new(
                HkClassEnum::from_bytes::<B>(bytes, de)?,
            )),
            "hkContactPointMaterial" => ClassParams::HkContactPointMaterial(Box::new(
                HkContactPointMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkContactPoint" => ClassParams::HkContactPoint(Box::new(
                HkContactPoint::from_bytes::<B>(bytes, de)?,
            )),
            "hkCustomAttributesAttribute" => ClassParams::HkCustomAttributesAttribute(Box::new(
                HkCustomAttributesAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkCustomAttributes" => ClassParams::HkCustomAttributes(Box::new(
                HkCustomAttributes::from_bytes::<B>(bytes, de)?,
            )),
            "hkDataObjectTypeAttribute" => ClassParams::HkDataObjectTypeAttribute(Box::new(
                HkDataObjectTypeAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkDescriptionAttribute" => ClassParams::HkDescriptionAttribute(Box::new(
                HkDescriptionAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkDocumentationAttribute" => ClassParams::HkDocumentationAttribute(Box::new(
                HkDocumentationAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkGeometryTriangle" => ClassParams::HkGeometryTriangle(Box::new(
                HkGeometryTriangle::from_bytes::<B>(bytes, de)?,
            )),
            "hkGeometry" => ClassParams::HkGeometry(Box::new(
                HkGeometry::from_bytes::<B>(bytes, de)?,
            )),
            "hkGizmoAttribute" => ClassParams::HkGizmoAttribute(Box::new(
                HkGizmoAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkHalf8" => ClassParams::HkHalf8(Box::new(
                HkHalf8::from_bytes::<B>(bytes, de)?,
            )),
            "hkIndexedTransformSet" => ClassParams::HkIndexedTransformSet(Box::new(
                HkIndexedTransformSet::from_bytes::<B>(bytes, de)?,
            )),
            "hkLinkAttribute" => ClassParams::HkLinkAttribute(Box::new(
                HkLinkAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkLocalFrameGroup" => ClassParams::HkLocalFrameGroup(Box::new(
                HkLocalFrameGroup::from_bytes::<B>(bytes, de)?,
            )),
            "hkLocalFrame" => ClassParams::HkLocalFrame(Box::new(
                HkLocalFrame::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryMeshBody" => ClassParams::HkMemoryMeshBody(Box::new(
                HkMemoryMeshBody::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryMeshMaterial" => ClassParams::HkMemoryMeshMaterial(Box::new(
                HkMemoryMeshMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryMeshShape" => ClassParams::HkMemoryMeshShape(Box::new(
                HkMemoryMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryMeshTexture" => ClassParams::HkMemoryMeshTexture(Box::new(
                HkMemoryMeshTexture::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryMeshVertexBuffer" => ClassParams::HkMemoryMeshVertexBuffer(Box::new(
                HkMemoryMeshVertexBuffer::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryResourceContainer" => ClassParams::HkMemoryResourceContainer(Box::new(
                HkMemoryResourceContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryResourceHandleExternalLink" => ClassParams::HkMemoryResourceHandleExternalLink(Box::new(
                HkMemoryResourceHandleExternalLink::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryResourceHandle" => ClassParams::HkMemoryResourceHandle(Box::new(
                HkMemoryResourceHandle::from_bytes::<B>(bytes, de)?,
            )),
            "hkMemoryTrackerAttribute" => ClassParams::HkMemoryTrackerAttribute(Box::new(
                HkMemoryTrackerAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshBody" => ClassParams::HkMeshBody(Box::new(
                HkMeshBody::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshBoneIndexMapping" => ClassParams::HkMeshBoneIndexMapping(Box::new(
                HkMeshBoneIndexMapping::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshMaterial" => ClassParams::HkMeshMaterial(Box::new(
                HkMeshMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshSectionCinfo" => ClassParams::HkMeshSectionCinfo(Box::new(
                HkMeshSectionCinfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshSection" => ClassParams::HkMeshSection(Box::new(
                HkMeshSection::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshShape" => ClassParams::HkMeshShape(Box::new(
                HkMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshTexture" => ClassParams::HkMeshTexture(Box::new(
                HkMeshTexture::from_bytes::<B>(bytes, de)?,
            )),
            "hkMeshVertexBuffer" => ClassParams::HkMeshVertexBuffer(Box::new(
                HkMeshVertexBuffer::from_bytes::<B>(bytes, de)?,
            )),
            "hkModelerNodeTypeAttribute" => ClassParams::HkModelerNodeTypeAttribute(Box::new(
                HkModelerNodeTypeAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkMonitorStreamFrameInfo" => ClassParams::HkMonitorStreamFrameInfo(Box::new(
                HkMonitorStreamFrameInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkMonitorStreamStringMapStringMap" => ClassParams::HkMonitorStreamStringMapStringMap(Box::new(
                HkMonitorStreamStringMapStringMap::from_bytes::<B>(bytes, de)?,
            )),
            "hkMonitorStreamStringMap" => ClassParams::HkMonitorStreamStringMap(Box::new(
                HkMonitorStreamStringMap::from_bytes::<B>(bytes, de)?,
            )),
            "hkMoppBvTreeShapeBase" => ClassParams::HkMoppBvTreeShapeBase(Box::new(
                HkMoppBvTreeShapeBase::from_bytes::<B>(bytes, de)?,
            )),
            "hkMotionState" => ClassParams::HkMotionState(Box::new(
                HkMotionState::from_bytes::<B>(bytes, de)?,
            )),
            "hkMultipleVertexBufferElementInfo" => ClassParams::HkMultipleVertexBufferElementInfo(Box::new(
                HkMultipleVertexBufferElementInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkMultipleVertexBufferLockedElement" => ClassParams::HkMultipleVertexBufferLockedElement(Box::new(
                HkMultipleVertexBufferLockedElement::from_bytes::<B>(bytes, de)?,
            )),
            "hkMultipleVertexBufferVertexBufferInfo" => ClassParams::HkMultipleVertexBufferVertexBufferInfo(Box::new(
                HkMultipleVertexBufferVertexBufferInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkMultipleVertexBuffer" => ClassParams::HkMultipleVertexBuffer(Box::new(
                HkMultipleVertexBuffer::from_bytes::<B>(bytes, de)?,
            )),
            "hkMultiThreadCheck" => ClassParams::HkMultiThreadCheck(Box::new(
                HkMultiThreadCheck::from_bytes::<B>(bytes, de)?,
            )),
            "hkp2dAngConstraintAtom" => ClassParams::Hkp2DAngConstraintAtom(Box::new(
                Hkp2DAngConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAabbPhantom" => ClassParams::HkpAabbPhantom(Box::new(
                HkpAabbPhantom::from_bytes::<B>(bytes, de)?,
            )),
            "hkPackedVector3" => ClassParams::HkPackedVector3(Box::new(
                HkPackedVector3::from_bytes::<B>(bytes, de)?,
            )),
            "hkPackfileHeader" => ClassParams::HkPackfileHeader(Box::new(
                HkPackfileHeader::from_bytes::<B>(bytes, de)?,
            )),
            "hkPackfileSectionHeader" => ClassParams::HkPackfileSectionHeader(Box::new(
                HkPackfileSectionHeader::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAction" => ClassParams::HkpAction(Box::new(
                HkpAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAngConstraintAtom" => ClassParams::HkpAngConstraintAtom(Box::new(
                HkpAngConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAngFrictionConstraintAtom" => ClassParams::HkpAngFrictionConstraintAtom(Box::new(
                HkpAngFrictionConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAngLimitConstraintAtom" => ClassParams::HkpAngLimitConstraintAtom(Box::new(
                HkpAngLimitConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAngMotorConstraintAtom" => ClassParams::HkpAngMotorConstraintAtom(Box::new(
                HkpAngMotorConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpAngularDashpotAction" => ClassParams::HkpAngularDashpotAction(Box::new(
                HkpAngularDashpotAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpArrayAction" => ClassParams::HkpArrayAction(Box::new(
                HkpArrayAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBallAndSocketConstraintDataAtoms" => ClassParams::HkpBallAndSocketConstraintDataAtoms(Box::new(
                HkpBallAndSocketConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBallAndSocketConstraintData" => ClassParams::HkpBallAndSocketConstraintData(Box::new(
                HkpBallAndSocketConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBallGun" => ClassParams::HkpBallGun(Box::new(
                HkpBallGun::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBallSocketChainDataConstraintInfo" => ClassParams::HkpBallSocketChainDataConstraintInfo(Box::new(
                HkpBallSocketChainDataConstraintInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBallSocketChainData" => ClassParams::HkpBallSocketChainData(Box::new(
                HkpBallSocketChainData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBallSocketConstraintAtom" => ClassParams::HkpBallSocketConstraintAtom(Box::new(
                HkpBallSocketConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBinaryAction" => ClassParams::HkpBinaryAction(Box::new(
                HkpBinaryAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBoxMotion" => ClassParams::HkpBoxMotion(Box::new(
                HkpBoxMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBoxShape" => ClassParams::HkpBoxShape(Box::new(
                HkpBoxShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBreakableBody" => ClassParams::HkpBreakableBody(Box::new(
                HkpBreakableBody::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBreakableConstraintData" => ClassParams::HkpBreakableConstraintData(Box::new(
                HkpBreakableConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBridgeAtoms" => ClassParams::HkpBridgeAtoms(Box::new(
                HkpBridgeAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBridgeConstraintAtom" => ClassParams::HkpBridgeConstraintAtom(Box::new(
                HkpBridgeConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBroadPhaseHandle" => ClassParams::HkpBroadPhaseHandle(Box::new(
                HkpBroadPhaseHandle::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBvShape" => ClassParams::HkpBvShape(Box::new(
                HkpBvShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpBvTreeShape" => ClassParams::HkpBvTreeShape(Box::new(
                HkpBvTreeShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCachingShapePhantom" => ClassParams::HkpCachingShapePhantom(Box::new(
                HkpCachingShapePhantom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCallbackConstraintMotor" => ClassParams::HkpCallbackConstraintMotor(Box::new(
                HkpCallbackConstraintMotor::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCapsuleShape" => ClassParams::HkpCapsuleShape(Box::new(
                HkpCapsuleShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCdBody" => ClassParams::HkpCdBody(Box::new(
                HkpCdBody::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCenterOfMassChangerModifierConstraintAtom" => ClassParams::HkpCenterOfMassChangerModifierConstraintAtom(Box::new(
                HkpCenterOfMassChangerModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCharacterControllerCinfo" => ClassParams::HkpCharacterControllerCinfo(Box::new(
                HkpCharacterControllerCinfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCharacterMotion" => ClassParams::HkpCharacterMotion(Box::new(
                HkpCharacterMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCharacterProxyCinfo" => ClassParams::HkpCharacterProxyCinfo(Box::new(
                HkpCharacterProxyCinfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCharacterRigidBodyCinfo" => ClassParams::HkpCharacterRigidBodyCinfo(Box::new(
                HkpCharacterRigidBodyCinfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCogWheelConstraintAtom" => ClassParams::HkpCogWheelConstraintAtom(Box::new(
                HkpCogWheelConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCogWheelConstraintDataAtoms" => ClassParams::HkpCogWheelConstraintDataAtoms(Box::new(
                HkpCogWheelConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCogWheelConstraintData" => ClassParams::HkpCogWheelConstraintData(Box::new(
                HkpCogWheelConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCollidableBoundingVolumeData" => ClassParams::HkpCollidableBoundingVolumeData(Box::new(
                HkpCollidableBoundingVolumeData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCollidableCollidableFilter" => ClassParams::HkpCollidableCollidableFilter(Box::new(
                HkpCollidableCollidableFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCollidable" => ClassParams::HkpCollidable(Box::new(
                HkpCollidable::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCollisionFilterList" => ClassParams::HkpCollisionFilterList(Box::new(
                HkpCollisionFilterList::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCollisionFilter" => ClassParams::HkpCollisionFilter(Box::new(
                HkpCollisionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCompressedMeshShapeBigTriangle" => ClassParams::HkpCompressedMeshShapeBigTriangle(Box::new(
                HkpCompressedMeshShapeBigTriangle::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCompressedMeshShapeChunk" => ClassParams::HkpCompressedMeshShapeChunk(Box::new(
                HkpCompressedMeshShapeChunk::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCompressedMeshShapeConvexPiece" => ClassParams::HkpCompressedMeshShapeConvexPiece(Box::new(
                HkpCompressedMeshShapeConvexPiece::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCompressedMeshShape" => ClassParams::HkpCompressedMeshShape(Box::new(
                HkpCompressedMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCompressedSampledHeightFieldShape" => ClassParams::HkpCompressedSampledHeightFieldShape(Box::new(
                HkpCompressedSampledHeightFieldShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConeLimitConstraintAtom" => ClassParams::HkpConeLimitConstraintAtom(Box::new(
                HkpConeLimitConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstrainedSystemFilter" => ClassParams::HkpConstrainedSystemFilter(Box::new(
                HkpConstrainedSystemFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintAtom" => ClassParams::HkpConstraintAtom(Box::new(
                HkpConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintChainData" => ClassParams::HkpConstraintChainData(Box::new(
                HkpConstraintChainData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintChainInstanceAction" => ClassParams::HkpConstraintChainInstanceAction(Box::new(
                HkpConstraintChainInstanceAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintChainInstance" => ClassParams::HkpConstraintChainInstance(Box::new(
                HkpConstraintChainInstance::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintCollisionFilter" => ClassParams::HkpConstraintCollisionFilter(Box::new(
                HkpConstraintCollisionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintInstanceSmallArraySerializeOverrideType" => ClassParams::HkpConstraintInstanceSmallArraySerializeOverrideType(Box::new(
                HkpConstraintInstanceSmallArraySerializeOverrideType::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintInstance" => ClassParams::HkpConstraintInstance(Box::new(
                HkpConstraintInstance::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConstraintMotor" => ClassParams::HkpConstraintMotor(Box::new(
                HkpConstraintMotor::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexListFilter" => ClassParams::HkpConvexListFilter(Box::new(
                HkpConvexListFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexListShape" => ClassParams::HkpConvexListShape(Box::new(
                HkpConvexListShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexPieceMeshShape" => ClassParams::HkpConvexPieceMeshShape(Box::new(
                HkpConvexPieceMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexPieceStreamData" => ClassParams::HkpConvexPieceStreamData(Box::new(
                HkpConvexPieceStreamData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexShape" => ClassParams::HkpConvexShape(Box::new(
                HkpConvexShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexTransformShapeBase" => ClassParams::HkpConvexTransformShapeBase(Box::new(
                HkpConvexTransformShapeBase::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexTransformShape" => ClassParams::HkpConvexTransformShape(Box::new(
                HkpConvexTransformShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexTranslateShape" => ClassParams::HkpConvexTranslateShape(Box::new(
                HkpConvexTranslateShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexVerticesConnectivity" => ClassParams::HkpConvexVerticesConnectivity(Box::new(
                HkpConvexVerticesConnectivity::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexVerticesShapeFourVectors" => ClassParams::HkpConvexVerticesShapeFourVectors(Box::new(
                HkpConvexVerticesShapeFourVectors::from_bytes::<B>(bytes, de)?,
            )),
            "hkpConvexVerticesShape" => ClassParams::HkpConvexVerticesShape(Box::new(
                HkpConvexVerticesShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpCylinderShape" => ClassParams::HkpCylinderShape(Box::new(
                HkpCylinderShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDashpotAction" => ClassParams::HkpDashpotAction(Box::new(
                HkpDashpotAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDefaultConvexListFilter" => ClassParams::HkpDefaultConvexListFilter(Box::new(
                HkpDefaultConvexListFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDefaultWorldMemoryWatchDog" => ClassParams::HkpDefaultWorldMemoryWatchDog(Box::new(
                HkpDefaultWorldMemoryWatchDog::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDisableEntityCollisionFilter" => ClassParams::HkpDisableEntityCollisionFilter(Box::new(
                HkpDisableEntityCollisionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDisplayBindingDataPhysicsSystem" => ClassParams::HkpDisplayBindingDataPhysicsSystem(Box::new(
                HkpDisplayBindingDataPhysicsSystem::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDisplayBindingDataRigidBody" => ClassParams::HkpDisplayBindingDataRigidBody(Box::new(
                HkpDisplayBindingDataRigidBody::from_bytes::<B>(bytes, de)?,
            )),
            "hkpDisplayBindingData" => ClassParams::HkpDisplayBindingData(Box::new(
                HkpDisplayBindingData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpEntityExtendedListeners" => ClassParams::HkpEntityExtendedListeners(Box::new(
                HkpEntityExtendedListeners::from_bytes::<B>(bytes, de)?,
            )),
            "hkpEntitySmallArraySerializeOverrideType" => ClassParams::HkpEntitySmallArraySerializeOverrideType(Box::new(
                HkpEntitySmallArraySerializeOverrideType::from_bytes::<B>(bytes, de)?,
            )),
            "hkpEntitySpuCollisionCallback" => ClassParams::HkpEntitySpuCollisionCallback(Box::new(
                HkpEntitySpuCollisionCallback::from_bytes::<B>(bytes, de)?,
            )),
            "hkpEntity" => ClassParams::HkpEntity(Box::new(
                HkpEntity::from_bytes::<B>(bytes, de)?,
            )),
            "hkpExtendedMeshShapeShapesSubpart" => ClassParams::HkpExtendedMeshShapeShapesSubpart(Box::new(
                HkpExtendedMeshShapeShapesSubpart::from_bytes::<B>(bytes, de)?,
            )),
            "hkpExtendedMeshShapeSubpart" => ClassParams::HkpExtendedMeshShapeSubpart(Box::new(
                HkpExtendedMeshShapeSubpart::from_bytes::<B>(bytes, de)?,
            )),
            "hkpExtendedMeshShapeTrianglesSubpart" => ClassParams::HkpExtendedMeshShapeTrianglesSubpart(Box::new(
                HkpExtendedMeshShapeTrianglesSubpart::from_bytes::<B>(bytes, de)?,
            )),
            "hkpExtendedMeshShape" => ClassParams::HkpExtendedMeshShape(Box::new(
                HkpExtendedMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpFastMeshShape" => ClassParams::HkpFastMeshShape(Box::new(
                HkpFastMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpFirstPersonGun" => ClassParams::HkpFirstPersonGun(Box::new(
                HkpFirstPersonGun::from_bytes::<B>(bytes, de)?,
            )),
            "hkpFixedRigidMotion" => ClassParams::HkpFixedRigidMotion(Box::new(
                HkpFixedRigidMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpGenericConstraintDataSchemeConstraintInfo" => ClassParams::HkpGenericConstraintDataSchemeConstraintInfo(Box::new(
                HkpGenericConstraintDataSchemeConstraintInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpGenericConstraintDataScheme" => ClassParams::HkpGenericConstraintDataScheme(Box::new(
                HkpGenericConstraintDataScheme::from_bytes::<B>(bytes, de)?,
            )),
            "hkpGenericConstraintData" => ClassParams::HkpGenericConstraintData(Box::new(
                HkpGenericConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpGravityGun" => ClassParams::HkpGravityGun(Box::new(
                HkpGravityGun::from_bytes::<B>(bytes, de)?,
            )),
            "hkpGroupCollisionFilter" => ClassParams::HkpGroupCollisionFilter(Box::new(
                HkpGroupCollisionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpGroupFilter" => ClassParams::HkpGroupFilter(Box::new(
                HkpGroupFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpHeightFieldShape" => ClassParams::HkpHeightFieldShape(Box::new(
                HkpHeightFieldShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpHingeConstraintDataAtoms" => ClassParams::HkpHingeConstraintDataAtoms(Box::new(
                HkpHingeConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpHingeConstraintData" => ClassParams::HkpHingeConstraintData(Box::new(
                HkpHingeConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpHingeLimitsDataAtoms" => ClassParams::HkpHingeLimitsDataAtoms(Box::new(
                HkpHingeLimitsDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpHingeLimitsData" => ClassParams::HkpHingeLimitsData(Box::new(
                HkpHingeLimitsData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpIgnoreModifierConstraintAtom" => ClassParams::HkpIgnoreModifierConstraintAtom(Box::new(
                HkpIgnoreModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpKeyframedRigidMotion" => ClassParams::HkpKeyframedRigidMotion(Box::new(
                HkpKeyframedRigidMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLimitedForceConstraintMotor" => ClassParams::HkpLimitedForceConstraintMotor(Box::new(
                HkpLimitedForceConstraintMotor::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLimitedHingeConstraintDataAtoms" => ClassParams::HkpLimitedHingeConstraintDataAtoms(Box::new(
                HkpLimitedHingeConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLimitedHingeConstraintData" => ClassParams::HkpLimitedHingeConstraintData(Box::new(
                HkpLimitedHingeConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinConstraintAtom" => ClassParams::HkpLinConstraintAtom(Box::new(
                HkpLinConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinearParametricCurve" => ClassParams::HkpLinearParametricCurve(Box::new(
                HkpLinearParametricCurve::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinFrictionConstraintAtom" => ClassParams::HkpLinFrictionConstraintAtom(Box::new(
                HkpLinFrictionConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinkedCollidable" => ClassParams::HkpLinkedCollidable(Box::new(
                HkpLinkedCollidable::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinLimitConstraintAtom" => ClassParams::HkpLinLimitConstraintAtom(Box::new(
                HkpLinLimitConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinMotorConstraintAtom" => ClassParams::HkpLinMotorConstraintAtom(Box::new(
                HkpLinMotorConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpLinSoftConstraintAtom" => ClassParams::HkpLinSoftConstraintAtom(Box::new(
                HkpLinSoftConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpListShapeChildInfo" => ClassParams::HkpListShapeChildInfo(Box::new(
                HkpListShapeChildInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpListShape" => ClassParams::HkpListShape(Box::new(
                HkpListShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMalleableConstraintData" => ClassParams::HkpMalleableConstraintData(Box::new(
                HkpMalleableConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMassChangerModifierConstraintAtom" => ClassParams::HkpMassChangerModifierConstraintAtom(Box::new(
                HkpMassChangerModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMassProperties" => ClassParams::HkpMassProperties(Box::new(
                HkpMassProperties::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMaterial" => ClassParams::HkpMaterial(Box::new(
                HkpMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMaxSizeMotion" => ClassParams::HkpMaxSizeMotion(Box::new(
                HkpMaxSizeMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMeshMaterial" => ClassParams::HkpMeshMaterial(Box::new(
                HkpMeshMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMeshShapeSubpart" => ClassParams::HkpMeshShapeSubpart(Box::new(
                HkpMeshShapeSubpart::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMeshShape" => ClassParams::HkpMeshShape(Box::new(
                HkpMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpModifierConstraintAtom" => ClassParams::HkpModifierConstraintAtom(Box::new(
                HkpModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMoppBvTreeShape" => ClassParams::HkpMoppBvTreeShape(Box::new(
                HkpMoppBvTreeShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMoppCodeCodeInfo" => ClassParams::HkpMoppCodeCodeInfo(Box::new(
                HkpMoppCodeCodeInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMoppCodeReindexedTerminal" => ClassParams::HkpMoppCodeReindexedTerminal(Box::new(
                HkpMoppCodeReindexedTerminal::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMoppCode" => ClassParams::HkpMoppCode(Box::new(
                HkpMoppCode::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMotion" => ClassParams::HkpMotion(Box::new(
                HkpMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMotorAction" => ClassParams::HkpMotorAction(Box::new(
                HkpMotorAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMountedBallGun" => ClassParams::HkpMountedBallGun(Box::new(
                HkpMountedBallGun::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMouseSpringAction" => ClassParams::HkpMouseSpringAction(Box::new(
                HkpMouseSpringAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMovingSurfaceModifierConstraintAtom" => ClassParams::HkpMovingSurfaceModifierConstraintAtom(Box::new(
                HkpMovingSurfaceModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMultiRayShapeRay" => ClassParams::HkpMultiRayShapeRay(Box::new(
                HkpMultiRayShapeRay::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMultiRayShape" => ClassParams::HkpMultiRayShape(Box::new(
                HkpMultiRayShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMultiSphereShape" => ClassParams::HkpMultiSphereShape(Box::new(
                HkpMultiSphereShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpMultithreadedVehicleManager" => ClassParams::HkpMultithreadedVehicleManager(Box::new(
                HkpMultithreadedVehicleManager::from_bytes::<B>(bytes, de)?,
            )),
            "hkpNamedMeshMaterial" => ClassParams::HkpNamedMeshMaterial(Box::new(
                HkpNamedMeshMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkpNullCollisionFilter" => ClassParams::HkpNullCollisionFilter(Box::new(
                HkpNullCollisionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkPostFinishAttribute" => ClassParams::HkPostFinishAttribute(Box::new(
                HkPostFinishAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkpOverwritePivotConstraintAtom" => ClassParams::HkpOverwritePivotConstraintAtom(Box::new(
                HkpOverwritePivotConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPairCollisionFilterMapPairFilterKeyOverrideType" => ClassParams::HkpPairCollisionFilterMapPairFilterKeyOverrideType(Box::new(
                HkpPairCollisionFilterMapPairFilterKeyOverrideType::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPairCollisionFilter" => ClassParams::HkpPairCollisionFilter(Box::new(
                HkpPairCollisionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpParametricCurve" => ClassParams::HkpParametricCurve(Box::new(
                HkpParametricCurve::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPhantomCallbackShape" => ClassParams::HkpPhantomCallbackShape(Box::new(
                HkpPhantomCallbackShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPhantom" => ClassParams::HkpPhantom(Box::new(
                HkpPhantom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPhysicsData" => ClassParams::HkpPhysicsData(Box::new(
                HkpPhysicsData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPhysicsSystemWithContacts" => ClassParams::HkpPhysicsSystemWithContacts(Box::new(
                HkpPhysicsSystemWithContacts::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPhysicsSystem" => ClassParams::HkpPhysicsSystem(Box::new(
                HkpPhysicsSystem::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPlaneShape" => ClassParams::HkpPlaneShape(Box::new(
                HkpPlaneShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPointToPathConstraintData" => ClassParams::HkpPointToPathConstraintData(Box::new(
                HkpPointToPathConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPointToPlaneConstraintDataAtoms" => ClassParams::HkpPointToPlaneConstraintDataAtoms(Box::new(
                HkpPointToPlaneConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPointToPlaneConstraintData" => ClassParams::HkpPointToPlaneConstraintData(Box::new(
                HkpPointToPlaneConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPositionConstraintMotor" => ClassParams::HkpPositionConstraintMotor(Box::new(
                HkpPositionConstraintMotor::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPoweredChainDataConstraintInfo" => ClassParams::HkpPoweredChainDataConstraintInfo(Box::new(
                HkpPoweredChainDataConstraintInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPoweredChainData" => ClassParams::HkpPoweredChainData(Box::new(
                HkpPoweredChainData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPoweredChainMapperLinkInfo" => ClassParams::HkpPoweredChainMapperLinkInfo(Box::new(
                HkpPoweredChainMapperLinkInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPoweredChainMapperTarget" => ClassParams::HkpPoweredChainMapperTarget(Box::new(
                HkpPoweredChainMapperTarget::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPoweredChainMapper" => ClassParams::HkpPoweredChainMapper(Box::new(
                HkpPoweredChainMapper::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPrismaticConstraintDataAtoms" => ClassParams::HkpPrismaticConstraintDataAtoms(Box::new(
                HkpPrismaticConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPrismaticConstraintData" => ClassParams::HkpPrismaticConstraintData(Box::new(
                HkpPrismaticConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpProjectileGun" => ClassParams::HkpProjectileGun(Box::new(
                HkpProjectileGun::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPropertyValue" => ClassParams::HkpPropertyValue(Box::new(
                HkpPropertyValue::from_bytes::<B>(bytes, de)?,
            )),
            "hkpProperty" => ClassParams::HkpProperty(Box::new(
                HkpProperty::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPulleyConstraintAtom" => ClassParams::HkpPulleyConstraintAtom(Box::new(
                HkpPulleyConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPulleyConstraintDataAtoms" => ClassParams::HkpPulleyConstraintDataAtoms(Box::new(
                HkpPulleyConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpPulleyConstraintData" => ClassParams::HkpPulleyConstraintData(Box::new(
                HkpPulleyConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRackAndPinionConstraintAtom" => ClassParams::HkpRackAndPinionConstraintAtom(Box::new(
                HkpRackAndPinionConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRackAndPinionConstraintDataAtoms" => ClassParams::HkpRackAndPinionConstraintDataAtoms(Box::new(
                HkpRackAndPinionConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRackAndPinionConstraintData" => ClassParams::HkpRackAndPinionConstraintData(Box::new(
                HkpRackAndPinionConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRagdollConstraintDataAtoms" => ClassParams::HkpRagdollConstraintDataAtoms(Box::new(
                HkpRagdollConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRagdollConstraintData" => ClassParams::HkpRagdollConstraintData(Box::new(
                HkpRagdollConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRagdollLimitsDataAtoms" => ClassParams::HkpRagdollLimitsDataAtoms(Box::new(
                HkpRagdollLimitsDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRagdollLimitsData" => ClassParams::HkpRagdollLimitsData(Box::new(
                HkpRagdollLimitsData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRagdollMotorConstraintAtom" => ClassParams::HkpRagdollMotorConstraintAtom(Box::new(
                HkpRagdollMotorConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRayCollidableFilter" => ClassParams::HkpRayCollidableFilter(Box::new(
                HkpRayCollidableFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRayShapeCollectionFilter" => ClassParams::HkpRayShapeCollectionFilter(Box::new(
                HkpRayShapeCollectionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRejectChassisListener" => ClassParams::HkpRejectChassisListener(Box::new(
                HkpRejectChassisListener::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRemoveTerminalsMoppModifier" => ClassParams::HkpRemoveTerminalsMoppModifier(Box::new(
                HkpRemoveTerminalsMoppModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkpReorientAction" => ClassParams::HkpReorientAction(Box::new(
                HkpReorientAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRigidBody" => ClassParams::HkpRigidBody(Box::new(
                HkpRigidBody::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRotationalConstraintDataAtoms" => ClassParams::HkpRotationalConstraintDataAtoms(Box::new(
                HkpRotationalConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpRotationalConstraintData" => ClassParams::HkpRotationalConstraintData(Box::new(
                HkpRotationalConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSampledHeightFieldShape" => ClassParams::HkpSampledHeightFieldShape(Box::new(
                HkpSampledHeightFieldShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSerializedDisplayMarkerList" => ClassParams::HkpSerializedDisplayMarkerList(Box::new(
                HkpSerializedDisplayMarkerList::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSerializedDisplayMarker" => ClassParams::HkpSerializedDisplayMarker(Box::new(
                HkpSerializedDisplayMarker::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSerializedDisplayRbTransformsDisplayTransformPair" => ClassParams::HkpSerializedDisplayRbTransformsDisplayTransformPair(Box::new(
                HkpSerializedDisplayRbTransformsDisplayTransformPair::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSerializedDisplayRbTransforms" => ClassParams::HkpSerializedDisplayRbTransforms(Box::new(
                HkpSerializedDisplayRbTransforms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSerializedSubTrack1nInfo" => ClassParams::HkpSerializedSubTrack1NInfo(Box::new(
                HkpSerializedSubTrack1NInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSerializedTrack1nInfo" => ClassParams::HkpSerializedTrack1NInfo(Box::new(
                HkpSerializedTrack1NInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSetLocalRotationsConstraintAtom" => ClassParams::HkpSetLocalRotationsConstraintAtom(Box::new(
                HkpSetLocalRotationsConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSetLocalTransformsConstraintAtom" => ClassParams::HkpSetLocalTransformsConstraintAtom(Box::new(
                HkpSetLocalTransformsConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSetLocalTranslationsConstraintAtom" => ClassParams::HkpSetLocalTranslationsConstraintAtom(Box::new(
                HkpSetLocalTranslationsConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSetupStabilizationAtom" => ClassParams::HkpSetupStabilizationAtom(Box::new(
                HkpSetupStabilizationAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShapeCollectionFilter" => ClassParams::HkpShapeCollectionFilter(Box::new(
                HkpShapeCollectionFilter::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShapeCollection" => ClassParams::HkpShapeCollection(Box::new(
                HkpShapeCollection::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShapeContainer" => ClassParams::HkpShapeContainer(Box::new(
                HkpShapeContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShapeInfo" => ClassParams::HkpShapeInfo(Box::new(
                HkpShapeInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShapeModifier" => ClassParams::HkpShapeModifier(Box::new(
                HkpShapeModifier::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShapePhantom" => ClassParams::HkpShapePhantom(Box::new(
                HkpShapePhantom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpShape" => ClassParams::HkpShape(Box::new(
                HkpShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimpleContactConstraintAtom" => ClassParams::HkpSimpleContactConstraintAtom(Box::new(
                HkpSimpleContactConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimpleContactConstraintDataInfo" => ClassParams::HkpSimpleContactConstraintDataInfo(Box::new(
                HkpSimpleContactConstraintDataInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimpleMeshShapeTriangle" => ClassParams::HkpSimpleMeshShapeTriangle(Box::new(
                HkpSimpleMeshShapeTriangle::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimpleMeshShape" => ClassParams::HkpSimpleMeshShape(Box::new(
                HkpSimpleMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimpleShapePhantomCollisionDetail" => ClassParams::HkpSimpleShapePhantomCollisionDetail(Box::new(
                HkpSimpleShapePhantomCollisionDetail::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimpleShapePhantom" => ClassParams::HkpSimpleShapePhantom(Box::new(
                HkpSimpleShapePhantom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSimulation" => ClassParams::HkpSimulation(Box::new(
                HkpSimulation::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSingleShapeContainer" => ClassParams::HkpSingleShapeContainer(Box::new(
                HkpSingleShapeContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSoftContactModifierConstraintAtom" => ClassParams::HkpSoftContactModifierConstraintAtom(Box::new(
                HkpSoftContactModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSphereMotion" => ClassParams::HkpSphereMotion(Box::new(
                HkpSphereMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSphereRepShape" => ClassParams::HkpSphereRepShape(Box::new(
                HkpSphereRepShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSphereShape" => ClassParams::HkpSphereShape(Box::new(
                HkpSphereShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSpringAction" => ClassParams::HkpSpringAction(Box::new(
                HkpSpringAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpSpringDamperConstraintMotor" => ClassParams::HkpSpringDamperConstraintMotor(Box::new(
                HkpSpringDamperConstraintMotor::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStiffSpringChainDataConstraintInfo" => ClassParams::HkpStiffSpringChainDataConstraintInfo(Box::new(
                HkpStiffSpringChainDataConstraintInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStiffSpringChainData" => ClassParams::HkpStiffSpringChainData(Box::new(
                HkpStiffSpringChainData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStiffSpringConstraintAtom" => ClassParams::HkpStiffSpringConstraintAtom(Box::new(
                HkpStiffSpringConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStiffSpringConstraintDataAtoms" => ClassParams::HkpStiffSpringConstraintDataAtoms(Box::new(
                HkpStiffSpringConstraintDataAtoms::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStiffSpringConstraintData" => ClassParams::HkpStiffSpringConstraintData(Box::new(
                HkpStiffSpringConstraintData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageExtendedMeshShapeMaterial" => ClassParams::HkpStorageExtendedMeshShapeMaterial(Box::new(
                HkpStorageExtendedMeshShapeMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageExtendedMeshShapeMeshSubpartStorage" => ClassParams::HkpStorageExtendedMeshShapeMeshSubpartStorage(Box::new(
                HkpStorageExtendedMeshShapeMeshSubpartStorage::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageExtendedMeshShapeShapeSubpartStorage" => ClassParams::HkpStorageExtendedMeshShapeShapeSubpartStorage(Box::new(
                HkpStorageExtendedMeshShapeShapeSubpartStorage::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageExtendedMeshShape" => ClassParams::HkpStorageExtendedMeshShape(Box::new(
                HkpStorageExtendedMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageMeshShapeSubpartStorage" => ClassParams::HkpStorageMeshShapeSubpartStorage(Box::new(
                HkpStorageMeshShapeSubpartStorage::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageMeshShape" => ClassParams::HkpStorageMeshShape(Box::new(
                HkpStorageMeshShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpStorageSampledHeightFieldShape" => ClassParams::HkpStorageSampledHeightFieldShape(Box::new(
                HkpStorageSampledHeightFieldShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpThinBoxMotion" => ClassParams::HkpThinBoxMotion(Box::new(
                HkpThinBoxMotion::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTransformShape" => ClassParams::HkpTransformShape(Box::new(
                HkpTransformShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTriangleShape" => ClassParams::HkpTriangleShape(Box::new(
                HkpTriangleShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTriggerVolumeEventInfo" => ClassParams::HkpTriggerVolumeEventInfo(Box::new(
                HkpTriggerVolumeEventInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTriggerVolume" => ClassParams::HkpTriggerVolume(Box::new(
                HkpTriggerVolume::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTriSampledHeightFieldBvTreeShape" => ClassParams::HkpTriSampledHeightFieldBvTreeShape(Box::new(
                HkpTriSampledHeightFieldBvTreeShape::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTriSampledHeightFieldCollection" => ClassParams::HkpTriSampledHeightFieldCollection(Box::new(
                HkpTriSampledHeightFieldCollection::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTwistLimitConstraintAtom" => ClassParams::HkpTwistLimitConstraintAtom(Box::new(
                HkpTwistLimitConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTypedBroadPhaseHandle" => ClassParams::HkpTypedBroadPhaseHandle(Box::new(
                HkpTypedBroadPhaseHandle::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTyremarkPoint" => ClassParams::HkpTyremarkPoint(Box::new(
                HkpTyremarkPoint::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTyremarksInfo" => ClassParams::HkpTyremarksInfo(Box::new(
                HkpTyremarksInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpTyremarksWheel" => ClassParams::HkpTyremarksWheel(Box::new(
                HkpTyremarksWheel::from_bytes::<B>(bytes, de)?,
            )),
            "hkpUnaryAction" => ClassParams::HkpUnaryAction(Box::new(
                HkpUnaryAction::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleAerodynamics" => ClassParams::HkpVehicleAerodynamics(Box::new(
                HkpVehicleAerodynamics::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleBrake" => ClassParams::HkpVehicleBrake(Box::new(
                HkpVehicleBrake::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleCastBatchingManager" => ClassParams::HkpVehicleCastBatchingManager(Box::new(
                HkpVehicleCastBatchingManager::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDataWheelComponentParams" => ClassParams::HkpVehicleDataWheelComponentParams(Box::new(
                HkpVehicleDataWheelComponentParams::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleData" => ClassParams::HkpVehicleData(Box::new(
                HkpVehicleData::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultAerodynamics" => ClassParams::HkpVehicleDefaultAerodynamics(Box::new(
                HkpVehicleDefaultAerodynamics::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultAnalogDriverInput" => ClassParams::HkpVehicleDefaultAnalogDriverInput(Box::new(
                HkpVehicleDefaultAnalogDriverInput::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultBrakeWheelBrakingProperties" => ClassParams::HkpVehicleDefaultBrakeWheelBrakingProperties(Box::new(
                HkpVehicleDefaultBrakeWheelBrakingProperties::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultBrake" => ClassParams::HkpVehicleDefaultBrake(Box::new(
                HkpVehicleDefaultBrake::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultEngine" => ClassParams::HkpVehicleDefaultEngine(Box::new(
                HkpVehicleDefaultEngine::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultSteering" => ClassParams::HkpVehicleDefaultSteering(Box::new(
                HkpVehicleDefaultSteering::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters" => ClassParams::HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(Box::new(
                HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultSuspension" => ClassParams::HkpVehicleDefaultSuspension(Box::new(
                HkpVehicleDefaultSuspension::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultTransmission" => ClassParams::HkpVehicleDefaultTransmission(Box::new(
                HkpVehicleDefaultTransmission::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDefaultVelocityDamper" => ClassParams::HkpVehicleDefaultVelocityDamper(Box::new(
                HkpVehicleDefaultVelocityDamper::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDriverInputAnalogStatus" => ClassParams::HkpVehicleDriverInputAnalogStatus(Box::new(
                HkpVehicleDriverInputAnalogStatus::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDriverInputStatus" => ClassParams::HkpVehicleDriverInputStatus(Box::new(
                HkpVehicleDriverInputStatus::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleDriverInput" => ClassParams::HkpVehicleDriverInput(Box::new(
                HkpVehicleDriverInput::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleEngine" => ClassParams::HkpVehicleEngine(Box::new(
                HkpVehicleEngine::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleFrictionDescriptionAxisDescription" => ClassParams::HkpVehicleFrictionDescriptionAxisDescription(Box::new(
                HkpVehicleFrictionDescriptionAxisDescription::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleFrictionDescription" => ClassParams::HkpVehicleFrictionDescription(Box::new(
                HkpVehicleFrictionDescription::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleFrictionStatusAxisStatus" => ClassParams::HkpVehicleFrictionStatusAxisStatus(Box::new(
                HkpVehicleFrictionStatusAxisStatus::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleFrictionStatus" => ClassParams::HkpVehicleFrictionStatus(Box::new(
                HkpVehicleFrictionStatus::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleInstanceWheelInfo" => ClassParams::HkpVehicleInstanceWheelInfo(Box::new(
                HkpVehicleInstanceWheelInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleInstance" => ClassParams::HkpVehicleInstance(Box::new(
                HkpVehicleInstance::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleLinearCastBatchingManager" => ClassParams::HkpVehicleLinearCastBatchingManager(Box::new(
                HkpVehicleLinearCastBatchingManager::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleLinearCastWheelCollideWheelState" => ClassParams::HkpVehicleLinearCastWheelCollideWheelState(Box::new(
                HkpVehicleLinearCastWheelCollideWheelState::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleLinearCastWheelCollide" => ClassParams::HkpVehicleLinearCastWheelCollide(Box::new(
                HkpVehicleLinearCastWheelCollide::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleManager" => ClassParams::HkpVehicleManager(Box::new(
                HkpVehicleManager::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleRayCastBatchingManager" => ClassParams::HkpVehicleRayCastBatchingManager(Box::new(
                HkpVehicleRayCastBatchingManager::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleRayCastWheelCollide" => ClassParams::HkpVehicleRayCastWheelCollide(Box::new(
                HkpVehicleRayCastWheelCollide::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleSteering" => ClassParams::HkpVehicleSteering(Box::new(
                HkpVehicleSteering::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleSuspensionSuspensionWheelParameters" => ClassParams::HkpVehicleSuspensionSuspensionWheelParameters(Box::new(
                HkpVehicleSuspensionSuspensionWheelParameters::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleSuspension" => ClassParams::HkpVehicleSuspension(Box::new(
                HkpVehicleSuspension::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleTransmission" => ClassParams::HkpVehicleTransmission(Box::new(
                HkpVehicleTransmission::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleVelocityDamper" => ClassParams::HkpVehicleVelocityDamper(Box::new(
                HkpVehicleVelocityDamper::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVehicleWheelCollide" => ClassParams::HkpVehicleWheelCollide(Box::new(
                HkpVehicleWheelCollide::from_bytes::<B>(bytes, de)?,
            )),
            "hkpVelocityConstraintMotor" => ClassParams::HkpVelocityConstraintMotor(Box::new(
                HkpVelocityConstraintMotor::from_bytes::<B>(bytes, de)?,
            )),
            "hkpViscousSurfaceModifierConstraintAtom" => ClassParams::HkpViscousSurfaceModifierConstraintAtom(Box::new(
                HkpViscousSurfaceModifierConstraintAtom::from_bytes::<B>(bytes, de)?,
            )),
            "hkpWeldingUtility" => ClassParams::HkpWeldingUtility(Box::new(
                HkpWeldingUtility::from_bytes::<B>(bytes, de)?,
            )),
            "hkpWorldCinfo" => ClassParams::HkpWorldCinfo(Box::new(
                HkpWorldCinfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkpWorldObject" => ClassParams::HkpWorldObject(Box::new(
                HkpWorldObject::from_bytes::<B>(bytes, de)?,
            )),
            "hkpWorld" => ClassParams::HkpWorld(Box::new(
                HkpWorld::from_bytes::<B>(bytes, de)?,
            )),
            "hkQTransform" => ClassParams::HkQTransform(Box::new(
                HkQTransform::from_bytes::<B>(bytes, de)?,
            )),
            "hkRangeInt32Attribute" => ClassParams::HkRangeInt32Attribute(Box::new(
                HkRangeInt32Attribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkRangeRealAttribute" => ClassParams::HkRangeRealAttribute(Box::new(
                HkRangeRealAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkReferencedObject" => ClassParams::HkReferencedObject(Box::new(
                HkReferencedObject::from_bytes::<B>(bytes, de)?,
            )),
            "hkReflectedFileAttribute" => ClassParams::HkReflectedFileAttribute(Box::new(
                HkReflectedFileAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkResourceBase" => ClassParams::HkResourceBase(Box::new(
                HkResourceBase::from_bytes::<B>(bytes, de)?,
            )),
            "hkResourceContainer" => ClassParams::HkResourceContainer(Box::new(
                HkResourceContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkResourceHandle" => ClassParams::HkResourceHandle(Box::new(
                HkResourceHandle::from_bytes::<B>(bytes, de)?,
            )),
            "hkRootLevelContainerNamedVariant" => ClassParams::HkRootLevelContainerNamedVariant(Box::new(
                HkRootLevelContainerNamedVariant::from_bytes::<B>(bytes, de)?,
            )),
            "hkRootLevelContainer" => ClassParams::HkRootLevelContainer(Box::new(
                HkRootLevelContainer::from_bytes::<B>(bytes, de)?,
            )),
            "hkSemanticsAttribute" => ClassParams::HkSemanticsAttribute(Box::new(
                HkSemanticsAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkSimpleLocalFrame" => ClassParams::HkSimpleLocalFrame(Box::new(
                HkSimpleLocalFrame::from_bytes::<B>(bytes, de)?,
            )),
            "hkSphere" => ClassParams::HkSphere(Box::new(
                HkSphere::from_bytes::<B>(bytes, de)?,
            )),
            "hkSweptTransform" => ClassParams::HkSweptTransform(Box::new(
                HkSweptTransform::from_bytes::<B>(bytes, de)?,
            )),
            "hkTraceStreamTitle" => ClassParams::HkTraceStreamTitle(Box::new(
                HkTraceStreamTitle::from_bytes::<B>(bytes, de)?,
            )),
            "hkTrackerSerializableScanSnapshotAllocation" => ClassParams::HkTrackerSerializableScanSnapshotAllocation(Box::new(
                HkTrackerSerializableScanSnapshotAllocation::from_bytes::<B>(bytes, de)?,
            )),
            "hkTrackerSerializableScanSnapshotBlock" => ClassParams::HkTrackerSerializableScanSnapshotBlock(Box::new(
                HkTrackerSerializableScanSnapshotBlock::from_bytes::<B>(bytes, de)?,
            )),
            "hkTrackerSerializableScanSnapshot" => ClassParams::HkTrackerSerializableScanSnapshot(Box::new(
                HkTrackerSerializableScanSnapshot::from_bytes::<B>(bytes, de)?,
            )),
            "hkUiAttribute" => ClassParams::HkUiAttribute(Box::new(
                HkUiAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkVertexFormatElement" => ClassParams::HkVertexFormatElement(Box::new(
                HkVertexFormatElement::from_bytes::<B>(bytes, de)?,
            )),
            "hkVertexFormat" => ClassParams::HkVertexFormat(Box::new(
                HkVertexFormat::from_bytes::<B>(bytes, de)?,
            )),
            "hkWorldMemoryAvailableWatchDog" => ClassParams::HkWorldMemoryAvailableWatchDog(Box::new(
                HkWorldMemoryAvailableWatchDog::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAnimatedFloat" => ClassParams::HkxAnimatedFloat(Box::new(
                HkxAnimatedFloat::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAnimatedMatrix" => ClassParams::HkxAnimatedMatrix(Box::new(
                HkxAnimatedMatrix::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAnimatedQuaternion" => ClassParams::HkxAnimatedQuaternion(Box::new(
                HkxAnimatedQuaternion::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAnimatedVector" => ClassParams::HkxAnimatedVector(Box::new(
                HkxAnimatedVector::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAttributeGroup" => ClassParams::HkxAttributeGroup(Box::new(
                HkxAttributeGroup::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAttributeHolder" => ClassParams::HkxAttributeHolder(Box::new(
                HkxAttributeHolder::from_bytes::<B>(bytes, de)?,
            )),
            "hkxAttribute" => ClassParams::HkxAttribute(Box::new(
                HkxAttribute::from_bytes::<B>(bytes, de)?,
            )),
            "hkxCamera" => ClassParams::HkxCamera(Box::new(
                HkxCamera::from_bytes::<B>(bytes, de)?,
            )),
            "hkxEdgeSelectionChannel" => ClassParams::HkxEdgeSelectionChannel(Box::new(
                HkxEdgeSelectionChannel::from_bytes::<B>(bytes, de)?,
            )),
            "hkxEnumItem" => ClassParams::HkxEnumItem(Box::new(
                HkxEnumItem::from_bytes::<B>(bytes, de)?,
            )),
            "hkxEnum" => ClassParams::HkxEnum(Box::new(
                HkxEnum::from_bytes::<B>(bytes, de)?,
            )),
            "hkxEnvironmentVariable" => ClassParams::HkxEnvironmentVariable(Box::new(
                HkxEnvironmentVariable::from_bytes::<B>(bytes, de)?,
            )),
            "hkxEnvironment" => ClassParams::HkxEnvironment(Box::new(
                HkxEnvironment::from_bytes::<B>(bytes, de)?,
            )),
            "hkxIndexBuffer" => ClassParams::HkxIndexBuffer(Box::new(
                HkxIndexBuffer::from_bytes::<B>(bytes, de)?,
            )),
            "hkxLight" => ClassParams::HkxLight(Box::new(
                HkxLight::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMaterialEffect" => ClassParams::HkxMaterialEffect(Box::new(
                HkxMaterialEffect::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMaterialProperty" => ClassParams::HkxMaterialProperty(Box::new(
                HkxMaterialProperty::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMaterialShaderSet" => ClassParams::HkxMaterialShaderSet(Box::new(
                HkxMaterialShaderSet::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMaterialShader" => ClassParams::HkxMaterialShader(Box::new(
                HkxMaterialShader::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMaterialTextureStage" => ClassParams::HkxMaterialTextureStage(Box::new(
                HkxMaterialTextureStage::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMaterial" => ClassParams::HkxMaterial(Box::new(
                HkxMaterial::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMeshSection" => ClassParams::HkxMeshSection(Box::new(
                HkxMeshSection::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMeshUserChannelInfo" => ClassParams::HkxMeshUserChannelInfo(Box::new(
                HkxMeshUserChannelInfo::from_bytes::<B>(bytes, de)?,
            )),
            "hkxMesh" => ClassParams::HkxMesh(Box::new(
                HkxMesh::from_bytes::<B>(bytes, de)?,
            )),
            "hkxNodeAnnotationData" => ClassParams::HkxNodeAnnotationData(Box::new(
                HkxNodeAnnotationData::from_bytes::<B>(bytes, de)?,
            )),
            "hkxNodeSelectionSet" => ClassParams::HkxNodeSelectionSet(Box::new(
                HkxNodeSelectionSet::from_bytes::<B>(bytes, de)?,
            )),
            "hkxNode" => ClassParams::HkxNode(Box::new(
                HkxNode::from_bytes::<B>(bytes, de)?,
            )),
            "hkxScene" => ClassParams::HkxScene(Box::new(
                HkxScene::from_bytes::<B>(bytes, de)?,
            )),
            "hkxSkinBinding" => ClassParams::HkxSkinBinding(Box::new(
                HkxSkinBinding::from_bytes::<B>(bytes, de)?,
            )),
            "hkxSparselyAnimatedBool" => ClassParams::HkxSparselyAnimatedBool(Box::new(
                HkxSparselyAnimatedBool::from_bytes::<B>(bytes, de)?,
            )),
            "hkxSparselyAnimatedEnum" => ClassParams::HkxSparselyAnimatedEnum(Box::new(
                HkxSparselyAnimatedEnum::from_bytes::<B>(bytes, de)?,
            )),
            "hkxSparselyAnimatedInt" => ClassParams::HkxSparselyAnimatedInt(Box::new(
                HkxSparselyAnimatedInt::from_bytes::<B>(bytes, de)?,
            )),
            "hkxSparselyAnimatedString" => ClassParams::HkxSparselyAnimatedString(Box::new(
                HkxSparselyAnimatedString::from_bytes::<B>(bytes, de)?,
            )),
            "hkxTextureFile" => ClassParams::HkxTextureFile(Box::new(
                HkxTextureFile::from_bytes::<B>(bytes, de)?,
            )),
            "hkxTextureInplace" => ClassParams::HkxTextureInplace(Box::new(
                HkxTextureInplace::from_bytes::<B>(bytes, de)?,
            )),
            "hkxTriangleSelectionChannel" => ClassParams::HkxTriangleSelectionChannel(Box::new(
                HkxTriangleSelectionChannel::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexBufferVertexData" => ClassParams::HkxVertexBufferVertexData(Box::new(
                HkxVertexBufferVertexData::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexBuffer" => ClassParams::HkxVertexBuffer(Box::new(
                HkxVertexBuffer::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexDescriptionElementDecl" => ClassParams::HkxVertexDescriptionElementDecl(Box::new(
                HkxVertexDescriptionElementDecl::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexDescription" => ClassParams::HkxVertexDescription(Box::new(
                HkxVertexDescription::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexFloatDataChannel" => ClassParams::HkxVertexFloatDataChannel(Box::new(
                HkxVertexFloatDataChannel::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexIntDataChannel" => ClassParams::HkxVertexIntDataChannel(Box::new(
                HkxVertexIntDataChannel::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexSelectionChannel" => ClassParams::HkxVertexSelectionChannel(Box::new(
                HkxVertexSelectionChannel::from_bytes::<B>(bytes, de)?,
            )),
            "hkxVertexVectorDataChannel" => ClassParams::HkxVertexVectorDataChannel(Box::new(
                HkxVertexVectorDataChannel::from_bytes::<B>(bytes, de)?,
            )),

            unknown => return Err(HkxError::UnknownHavokClass(unknown.into())),
        })
    }
}
