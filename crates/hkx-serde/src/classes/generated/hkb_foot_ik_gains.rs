//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkGains`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbFootIkGains`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xa681b7f0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkGains {
    /// # C++ Class Fields Info
    /// -   name:`"onOffGain"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onOffGain")]
    OnOffGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"groundAscendingGain"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundAscendingGain")]
    GroundAscendingGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"groundDescendingGain"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundDescendingGain")]
    GroundDescendingGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footPlantedGain"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footPlantedGain")]
    FootPlantedGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footRaisedGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footRaisedGain")]
    FootRaisedGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"footUnlockGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footUnlockGain")]
    FootUnlockGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModelFeedbackGain"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelFeedbackGain")]
    WorldFromModelFeedbackGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"errorUpDownBias"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorUpDownBias")]
    ErrorUpDownBias(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"alignWorldFromModelGain"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWorldFromModelGain")]
    AlignWorldFromModelGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"hipOrientationGain"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hipOrientationGain")]
    HipOrientationGain(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"maxKneeAngleDifference"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxKneeAngleDifference")]
    MaxKneeAngleDifference(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ankleOrientationGain"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ankleOrientationGain")]
    AnkleOrientationGain(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkGains, "@name",
    ("onOffGain" => OnOffGain(Primitive<f32>)),
    ("groundAscendingGain" => GroundAscendingGain(Primitive<f32>)),
    ("groundDescendingGain" => GroundDescendingGain(Primitive<f32>)),
    ("footPlantedGain" => FootPlantedGain(Primitive<f32>)),
    ("footRaisedGain" => FootRaisedGain(Primitive<f32>)),
    ("footUnlockGain" => FootUnlockGain(Primitive<f32>)),
    ("worldFromModelFeedbackGain" => WorldFromModelFeedbackGain(Primitive<f32>)),
    ("errorUpDownBias" => ErrorUpDownBias(Primitive<f32>)),
    ("alignWorldFromModelGain" => AlignWorldFromModelGain(Primitive<f32>)),
    ("hipOrientationGain" => HipOrientationGain(Primitive<f32>)),
    ("maxKneeAngleDifference" => MaxKneeAngleDifference(Primitive<f32>)),
    ("ankleOrientationGain" => AnkleOrientationGain(Primitive<f32>)),
}
