//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAttachmentSetup`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbAttachmentSetup`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x774632b`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAttachmentSetup {
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
    /// -   name:`"blendInTime"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendInTime")]
    BlendInTime(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"moveAttacherFraction"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "moveAttacherFraction")]
    MoveAttacherFraction(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"fixUpGain"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixUpGain")]
    FixUpGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxLinearDistance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearDistance")]
    MaxLinearDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxAngularDistance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngularDistance")]
    MaxAngularDistance(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"attachmentType"`
    /// -   type: `enum AttachmentType`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachmentType")]
    AttachmentType(Primitive<AttachmentType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttachmentSetup, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("blendInTime" => BlendInTime(Primitive<f32>)),
    ("moveAttacherFraction" => MoveAttacherFraction(Primitive<f32>)),
    ("gain" => Gain(Primitive<f32>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("fixUpGain" => FixUpGain(Primitive<f32>)),
    ("maxLinearDistance" => MaxLinearDistance(Primitive<f32>)),
    ("maxAngularDistance" => MaxAngularDistance(Primitive<f32>)),
    ("attachmentType" => AttachmentType(Primitive<AttachmentType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttachmentType {
    #[serde(rename = "ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY")]
    AttachmentTypeKeyframeRigidBody = 0,
    #[serde(rename = "ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT")]
    AttachmentTypeBallSocketConstraint = 1,
    #[serde(rename = "ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT")]
    AttachmentTypeRagdollConstraint = 2,
    #[serde(rename = "ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL")]
    AttachmentTypeSetWorldFromModel = 3,
    #[serde(rename = "ATTACHMENT_TYPE_NONE")]
    AttachmentTypeNone = 4,
}
