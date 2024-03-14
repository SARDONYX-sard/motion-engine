//! Rust [`Serializer`]/[`Deserializer`] corresponding to C++ class `hkpConstraintAtom`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkpConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 2
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0x59d67ef6`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintAtom {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(AtomType),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintAtom, "@name",
    ("type" => Type(AtomType)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AtomType {
    #[serde(rename = "TYPE_INVALID")]
    TypeInvalid = 0,
    #[serde(rename = "TYPE_BRIDGE")]
    TypeBridge = 1,
    #[serde(rename = "TYPE_SET_LOCAL_TRANSFORMS")]
    TypeSetLocalTransforms = 2,
    #[serde(rename = "TYPE_SET_LOCAL_TRANSLATIONS")]
    TypeSetLocalTranslations = 3,
    #[serde(rename = "TYPE_SET_LOCAL_ROTATIONS")]
    TypeSetLocalRotations = 4,
    #[serde(rename = "TYPE_BALL_SOCKET")]
    TypeBallSocket = 5,
    #[serde(rename = "TYPE_STIFF_SPRING")]
    TypeStiffSpring = 6,
    #[serde(rename = "TYPE_LIN")]
    TypeLin = 7,
    #[serde(rename = "TYPE_LIN_SOFT")]
    TypeLinSoft = 8,
    #[serde(rename = "TYPE_LIN_LIMIT")]
    TypeLinLimit = 9,
    #[serde(rename = "TYPE_LIN_FRICTION")]
    TypeLinFriction = 10,
    #[serde(rename = "TYPE_LIN_MOTOR")]
    TypeLinMotor = 11,
    #[serde(rename = "TYPE_2D_ANG")]
    Type2DAng = 12,
    #[serde(rename = "TYPE_ANG")]
    TypeAng = 13,
    #[serde(rename = "TYPE_ANG_LIMIT")]
    TypeAngLimit = 14,
    #[serde(rename = "TYPE_TWIST_LIMIT")]
    TypeTwistLimit = 15,
    #[serde(rename = "TYPE_CONE_LIMIT")]
    TypeConeLimit = 16,
    #[serde(rename = "TYPE_ANG_FRICTION")]
    TypeAngFriction = 17,
    #[serde(rename = "TYPE_ANG_MOTOR")]
    TypeAngMotor = 18,
    #[serde(rename = "TYPE_RAGDOLL_MOTOR")]
    TypeRagdollMotor = 19,
    #[serde(rename = "TYPE_PULLEY")]
    TypePulley = 20,
    #[serde(rename = "TYPE_RACK_AND_PINION")]
    TypeRackAndPinion = 21,
    #[serde(rename = "TYPE_COG_WHEEL")]
    TypeCogWheel = 22,
    #[serde(rename = "TYPE_SETUP_STABILIZATION")]
    TypeSetupStabilization = 23,
    #[serde(rename = "TYPE_OVERWRITE_PIVOT")]
    TypeOverwritePivot = 24,
    #[serde(rename = "TYPE_CONTACT")]
    TypeContact = 25,
    #[serde(rename = "TYPE_MODIFIER_SOFT_CONTACT")]
    TypeModifierSoftContact = 26,
    #[serde(rename = "TYPE_MODIFIER_MASS_CHANGER")]
    TypeModifierMassChanger = 27,
    #[serde(rename = "TYPE_MODIFIER_VISCOUS_SURFACE")]
    TypeModifierViscousSurface = 28,
    #[serde(rename = "TYPE_MODIFIER_MOVING_SURFACE")]
    TypeModifierMovingSurface = 29,
    #[serde(rename = "TYPE_MODIFIER_IGNORE_CONSTRAINT")]
    TypeModifierIgnoreConstraint = 30,
    #[serde(rename = "TYPE_MODIFIER_CENTER_OF_MASS_CHANGER")]
    TypeModifierCenterOfMassChanger = 31,
    #[serde(rename = "TYPE_MAX")]
    TypeMax = 32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CallbackRequest {
    #[serde(rename = "CALLBACK_REQUEST_NONE")]
    CallbackRequestNone = 0,
    #[serde(rename = "CALLBACK_REQUEST_NEW_CONTACT_POINT")]
    CallbackRequestNewContactPoint = 1,
    #[serde(rename = "CALLBACK_REQUEST_SETUP_PPU_ONLY")]
    CallbackRequestSetupPpuOnly = 2,
    #[serde(rename = "CALLBACK_REQUEST_SETUP_CALLBACK")]
    CallbackRequestSetupCallback = 4,
    #[serde(rename = "CALLBACK_REQUEST_CONTACT_POINT_CALLBACK")]
    CallbackRequestContactPointCallback = 8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SolvingMethod {
    #[serde(rename = "METHOD_STABILIZED")]
    MethodStabilized = 0,
    #[serde(rename = "METHOD_OLD")]
    MethodOld = 1,
}
