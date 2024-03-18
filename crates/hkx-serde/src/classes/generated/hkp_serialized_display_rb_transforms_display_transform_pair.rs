//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSerializedDisplayRbTransformsDisplayTransformPair`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpSerializedDisplayRbTransformsDisplayTransformPair`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x94ac5bec`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSerializedDisplayRbTransformsDisplayTransformPair<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"rb"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rb")]
    Rb(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"localToDisplay"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "localToDisplay")]
    LocalToDisplay(Transform<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSerializedDisplayRbTransformsDisplayTransformPair<'de>, "@name",
    ("rb" => Rb(Primitive<Cow<'de, str>>)),
    ("localToDisplay" => LocalToDisplay(Transform<f32>)),
}
