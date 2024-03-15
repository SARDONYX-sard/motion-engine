//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxMaterial`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkxMaterial`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkxAttributeHolder`/`0x7468cc44`
/// - signature: `0x2954537a`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterial<'a> {
    /// # C++ Parent class(`hkxAttributeHolder`, parent: `hkReferencedObject`) field Info
    /// -   name:`"attributeGroups"`
    /// -   type: `hkArray&lt;struct hkxAttributeGroup&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeGroups", default)]
    AttributeGroups(HkArrayClass<HkxAttributeGroup>),

    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "memSizeAndFlags", default, skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// # C++ Parent class(`hkReferencedObject`, parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "referenceCount", default, skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // `hkBaseObject`(Parent class) has no fields

    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name", default)]
    Name(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"stages"`
    /// -   type: `hkArray&lt;struct hkxMaterialTextureStage&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stages", default)]
    Stages(HkArrayClass<HkxMaterialTextureStage>),
    /// # C++ Class Fields Info
    /// -   name:`"diffuseColor"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "diffuseColor", default)]
    DiffuseColor(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"ambientColor"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ambientColor", default)]
    AmbientColor(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"specularColor"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "specularColor", default)]
    SpecularColor(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"emissiveColor"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "emissiveColor", default)]
    EmissiveColor(Vector4<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"subMaterials"`
    /// -   type: `hkArray&lt;hkxMaterial*&gt;`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subMaterials", default)]
    SubMaterials(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"extraData"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraData", default)]
    ExtraData(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"properties"`
    /// -   type: `hkArray&lt;struct hkxMaterialProperty&gt;`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "properties", default)]
    Properties(HkArrayClass<HkxMaterialProperty>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterial<'de>, "@name",
    ("attributeGroups" => AttributeGroups(HkArrayClass<HkxAttributeGroup>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("stages" => Stages(HkArrayClass<HkxMaterialTextureStage>)),
    ("diffuseColor" => DiffuseColor(Vector4<f32>)),
    ("ambientColor" => AmbientColor(Vector4<f32>)),
    ("specularColor" => SpecularColor(Vector4<f32>)),
    ("emissiveColor" => EmissiveColor(Vector4<f32>)),
    ("subMaterials" => SubMaterials(HkArrayRef<Cow<'de, str>>)),
    ("extraData" => ExtraData(Primitive<Cow<'de, str>>)),
    ("properties" => Properties(HkArrayClass<HkxMaterialProperty>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextureType {
    #[serde(rename = "TEX_UNKNOWN")]
    TexUnknown = 0,
    #[serde(rename = "TEX_DIFFUSE")]
    TexDiffuse = 1,
    #[serde(rename = "TEX_REFLECTION")]
    TexReflection = 2,
    #[serde(rename = "TEX_BUMP")]
    TexBump = 3,
    #[serde(rename = "TEX_NORMAL")]
    TexNormal = 4,
    #[serde(rename = "TEX_DISPLACEMENT")]
    TexDisplacement = 5,
    #[serde(rename = "TEX_SPECULAR")]
    TexSpecular = 6,
    #[serde(rename = "TEX_SPECULARANDGLOSS")]
    TexSpecularandgloss = 7,
    #[serde(rename = "TEX_OPACITY")]
    TexOpacity = 8,
    #[serde(rename = "TEX_EMISSIVE")]
    TexEmissive = 9,
    #[serde(rename = "TEX_REFRACTION")]
    TexRefraction = 10,
    #[serde(rename = "TEX_GLOSS")]
    TexGloss = 11,
    #[serde(rename = "TEX_NOTEXPORTED")]
    TexNotexported = 12,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyKey {
    #[serde(rename = "PROPERTY_MTL_TYPE_BLEND")]
    PropertyMtlTypeBlend = 1,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE0")]
    PropertyMtlUvIdStage0 = 256,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE1")]
    PropertyMtlUvIdStage1 = 257,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE2")]
    PropertyMtlUvIdStage2 = 258,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE3")]
    PropertyMtlUvIdStage3 = 259,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE4")]
    PropertyMtlUvIdStage4 = 260,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE5")]
    PropertyMtlUvIdStage5 = 261,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE6")]
    PropertyMtlUvIdStage6 = 262,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE7")]
    PropertyMtlUvIdStage7 = 263,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE8")]
    PropertyMtlUvIdStage8 = 264,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE9")]
    PropertyMtlUvIdStage9 = 265,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE10")]
    PropertyMtlUvIdStage10 = 266,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE11")]
    PropertyMtlUvIdStage11 = 267,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE12")]
    PropertyMtlUvIdStage12 = 268,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE13")]
    PropertyMtlUvIdStage13 = 269,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE14")]
    PropertyMtlUvIdStage14 = 270,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE15")]
    PropertyMtlUvIdStage15 = 271,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE_MAX")]
    PropertyMtlUvIdStageMax = 272,
}
