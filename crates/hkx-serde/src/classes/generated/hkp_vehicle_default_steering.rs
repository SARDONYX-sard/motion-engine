//! A Rust structure that implements a serializer/deserializer corresponding to `hkpVehicleDefaultSteering`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 28
/// -  vtable: true
/// -  parent: hkpVehicleSteering/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpVehicleDefaultSteering<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpVehicleDefaultSteering"`: The original C++ class name.
    #[serde(default = "HkpVehicleDefaultSteering::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8f0411c8`: Unique value of this class.
    #[serde(default = "HkpVehicleDefaultSteering::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpVehicleDefaultSteeringHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpVehicleDefaultSteeringHkParam<'a>>
}

impl HkpVehicleDefaultSteering<'_> {
    /// Return `"hkpVehicleDefaultSteering"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpVehicleDefaultSteering".into()
    }

    /// Return `"0x8f0411c8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8f0411c8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleDefaultSteeringHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"maxSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSteeringAngle")]
    MaxSteeringAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSpeedFullSteeringAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSpeedFullSteeringAngle")]
    MaxSpeedFullSteeringAngle(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"doesWheelSteer"`
    /// -   type: `hkArray&lt;hkBool&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "doesWheelSteer")]
    DoesWheelSteer(Vec<Primitive<bool>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSteeringHkParam<'de>, "@name",
    ("maxSteeringAngle" => MaxSteeringAngle(Primitive<f32>)),
    ("maxSpeedFullSteeringAngle" => MaxSpeedFullSteeringAngle(Primitive<f32>)),
    ("doesWheelSteer" => DoesWheelSteer(Vec<Primitive<bool>>)),
}
