//! A Rust structure that implements a serializer/deserializer corresponding to `hkbProxyModifier`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 256
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbProxyModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbProxyModifier"`: Name of this class.
    #[serde(default = "HkbProxyModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8a41554f`: Unique value of this class.
    #[serde(default = "HkbProxyModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbProxyModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbProxyModifierHkParam<'a>>
}

impl HkbProxyModifier<'_> {
    /// Return `"hkbProxyModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbProxyModifier".into()
    }

    /// Return `"0x8a41554f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8a41554f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProxyModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"proxyInfo"`
    /// -   type: `struct hkbProxyModifierProxyInfo`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "proxyInfo")]
    ProxyInfo(HkbProxyModifierProxyInfo),
    /// # Information on fields in the original C++ class
    /// -   name:`"linearVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocity")]
    LinearVelocity(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"horizontalGain"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "horizontalGain")]
    HorizontalGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalGain")]
    VerticalGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxHorizontalSeparation")]
    MaxHorizontalSeparation(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalSeparation")]
    MaxVerticalSeparation(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalDisplacementError"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalDisplacementError")]
    VerticalDisplacementError(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalDisplacementErrorGain"`
    /// -   type: `hkReal`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalDisplacementErrorGain")]
    VerticalDisplacementErrorGain(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxVerticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalDisplacement")]
    MaxVerticalDisplacement(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"minVerticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minVerticalDisplacement")]
    MinVerticalDisplacement(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"capsuleHeight"`
    /// -   type: `hkReal`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleHeight")]
    CapsuleHeight(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"capsuleRadius"`
    /// -   type: `hkReal`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "capsuleRadius")]
    CapsuleRadius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSlopeForRotation"`
    /// -   type: `hkReal`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlopeForRotation")]
    MaxSlopeForRotation(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"phantomType"`
    /// -   type: `enum PhantomType`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "phantomType")]
    PhantomType(PhantomType),
    /// # Information on fields in the original C++ class
    /// -   name:`"linearVelocityMode"`
    /// -   type: `enum LinearVelocityMode`
    /// - offset: 193
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearVelocityMode")]
    LinearVelocityMode(LinearVelocityMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"ignoreIncomingRotation"`
    /// -   type: `hkBool`
    /// - offset: 194
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreIncomingRotation")]
    IgnoreIncomingRotation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"ignoreCollisionDuringRotation"`
    /// -   type: `hkBool`
    /// - offset: 195
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreCollisionDuringRotation")]
    IgnoreCollisionDuringRotation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"ignoreIncomingTranslation"`
    /// -   type: `hkBool`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ignoreIncomingTranslation")]
    IgnoreIncomingTranslation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"includeDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 197
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "includeDownwardMomentum")]
    IncludeDownwardMomentum(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"followWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 198
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "followWorldFromModel")]
    FollowWorldFromModel(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 199
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterProxy"`
    /// -   type: `void*`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterProxy", skip_serializing)]
    CharacterProxy(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"phantom"`
    /// -   type: `void*`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantom", skip_serializing)]
    Phantom(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"phantomShape"`
    /// -   type: `void*`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomShape", skip_serializing)]
    PhantomShape(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"horizontalDisplacement"`
    /// -   type: `hkVector4`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "horizontalDisplacement", skip_serializing)]
    HorizontalDisplacement(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalDisplacement"`
    /// -   type: `hkReal`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "verticalDisplacement", skip_serializing)]
    VerticalDisplacement(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 244
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timestep", skip_serializing)]
    Timestep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"previousFrameFollowWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 248
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "previousFrameFollowWorldFromModel", skip_serializing)]
    PreviousFrameFollowWorldFromModel(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbProxyModifierHkParam<'de>, "@name",
    ("proxyInfo" => ProxyInfo(HkbProxyModifierProxyInfo)),
    ("linearVelocity" => LinearVelocity(cgmath::Vector4<f32>)),
    ("horizontalGain" => HorizontalGain(f64)),
    ("verticalGain" => VerticalGain(f64)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(f64)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(f64)),
    ("verticalDisplacementError" => VerticalDisplacementError(f64)),
    ("verticalDisplacementErrorGain" => VerticalDisplacementErrorGain(f64)),
    ("maxVerticalDisplacement" => MaxVerticalDisplacement(f64)),
    ("minVerticalDisplacement" => MinVerticalDisplacement(f64)),
    ("capsuleHeight" => CapsuleHeight(f64)),
    ("capsuleRadius" => CapsuleRadius(f64)),
    ("maxSlopeForRotation" => MaxSlopeForRotation(f64)),
    ("collisionFilterInfo" => CollisionFilterInfo(u32)),
    ("phantomType" => PhantomType(PhantomType)),
    ("linearVelocityMode" => LinearVelocityMode(LinearVelocityMode)),
    ("ignoreIncomingRotation" => IgnoreIncomingRotation(bool)),
    ("ignoreCollisionDuringRotation" => IgnoreCollisionDuringRotation(bool)),
    ("ignoreIncomingTranslation" => IgnoreIncomingTranslation(bool)),
    ("includeDownwardMomentum" => IncludeDownwardMomentum(bool)),
    ("followWorldFromModel" => FollowWorldFromModel(bool)),
    ("isTouchingGround" => IsTouchingGround(bool)),
    ("characterProxy" => CharacterProxy(())),
    ("phantom" => Phantom(())),
    ("phantomShape" => PhantomShape(())),
    ("horizontalDisplacement" => HorizontalDisplacement(cgmath::Vector4<f32>)),
    ("verticalDisplacement" => VerticalDisplacement(f64)),
    ("timestep" => Timestep(f64)),
    ("previousFrameFollowWorldFromModel" => PreviousFrameFollowWorldFromModel(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PhantomType {
    #[serde(rename = "PHANTOM_TYPE_SIMPLE")]
    PhantomTypeSimple = 0,
    #[serde(rename = "PHANTOM_TYPE_CACHING")]
    PhantomTypeCaching = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LinearVelocityMode {
    #[serde(rename = "LINEAR_VELOCITY_MODE_WORLD")]
    LinearVelocityModeWorld = 0,
    #[serde(rename = "LINEAR_VELOCITY_MODE_MODEL")]
    LinearVelocityModeModel = 1,
}