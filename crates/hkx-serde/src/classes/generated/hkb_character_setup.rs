//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterSetup`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterSetup<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterSetup"`: The original C++ class name.
    #[serde(default = "HkbCharacterSetup::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe5a2a413`: Unique value of this class.
    #[serde(default = "HkbCharacterSetup::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterSetupHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterSetupHkParam<'a>>
}

impl HkbCharacterSetup<'_> {
    /// Return `"hkbCharacterSetup"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterSetup".into()
    }

    /// Return `"0xe5a2a413"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe5a2a413".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterSetupHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"retargetingSkeletonMappers"`
    /// -   type: `hkArray&lt;hkaSkeletonMapper*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "retargetingSkeletonMappers")]
    RetargetingSkeletonMappers(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"animationSkeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationSkeleton")]
    AnimationSkeleton(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"ragdollToAnimationSkeletonMapper"`
    /// -   type: `struct hkaSkeletonMapper*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollToAnimationSkeletonMapper")]
    RagdollToAnimationSkeletonMapper(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"animationToRagdollSkeletonMapper"`
    /// -   type: `struct hkaSkeletonMapper*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationToRagdollSkeletonMapper")]
    AnimationToRagdollSkeletonMapper(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationBindingSet", skip_serializing)]
    AnimationBindingSet(()),
    /// # Field information in the original C++ class
    /// -   name:`"data"`
    /// -   type: `struct hkbCharacterData*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"mirroredSkeleton"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "mirroredSkeleton", skip_serializing)]
    MirroredSkeleton(()),
    /// # Field information in the original C++ class
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "characterPropertyIdMap", skip_serializing)]
    CharacterPropertyIdMap(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterSetupHkParam<'de>, "@name",
    ("retargetingSkeletonMappers" => RetargetingSkeletonMappers(Vec<Cow<'a, str>>)),
    ("animationSkeleton" => AnimationSkeleton(Cow<'a, str>)),
    ("ragdollToAnimationSkeletonMapper" => RagdollToAnimationSkeletonMapper(Cow<'a, str>)),
    ("animationToRagdollSkeletonMapper" => AnimationToRagdollSkeletonMapper(Cow<'a, str>)),
    ("animationBindingSet" => AnimationBindingSet(())),
    ("data" => Data(Cow<'a, str>)),
    ("mirroredSkeleton" => MirroredSkeleton(())),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(())),
}
