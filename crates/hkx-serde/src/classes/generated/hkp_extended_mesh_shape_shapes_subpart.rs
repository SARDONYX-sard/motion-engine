//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShapeShapesSubpart`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpExtendedMeshShapeShapesSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpExtendedMeshShapeSubpart`/`0xf4608207`
/// - signature: `0xf204b155`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeShapesSubpart<'a> {
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum SubpartType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<SubpartType>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MaterialIndexStridingType`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MaterialIndexStridingType>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialStriding", skip_serializing)]
    MaterialStriding(Primitive<i16>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<u16>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),

    /// # C++ Class Fields Info
    /// -   name:`"childShapes"`
    /// -   type: `hkArray<hkpConvexShape*>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShapes")]
    ChildShapes(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translation")]
    Translation(Vector4<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeShapesSubpart<'de>, "@name",
    ("type" => Type(Primitive<SubpartType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MaterialIndexStridingType>)),
    ("materialStriding" => MaterialStriding(Primitive<i16>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("childShapes" => ChildShapes(HkArrayRef<Cow<'de, str>>)),
    ("rotation" => Rotation(Quaternion<f32>)),
    ("translation" => Translation(Vector4<f32>)),
}
