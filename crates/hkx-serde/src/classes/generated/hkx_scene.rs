//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxScene`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkxScene`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5f673ddd`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxScene<'a> {
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
    //
    /// # C++ Class Fields Info
    /// -   name:`"modeller"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modeller")]
    Modeller(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"asset"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "asset")]
    Asset(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"sceneLength"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sceneLength")]
    SceneLength(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"rootNode"`
    /// -   type: `struct hkxNode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootNode")]
    RootNode(Primitive<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"selectionSets"`
    /// -   type: `hkArray<hkxNodeSelectionSet*>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectionSets")]
    SelectionSets(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"cameras"`
    /// -   type: `hkArray<hkxCamera*>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cameras")]
    Cameras(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"lights"`
    /// -   type: `hkArray<hkxLight*>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lights")]
    Lights(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"meshes"`
    /// -   type: `hkArray<hkxMesh*>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "meshes")]
    Meshes(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<hkxMaterial*>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"inplaceTextures"`
    /// -   type: `hkArray<hkxTextureInplace*>`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inplaceTextures")]
    InplaceTextures(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"externalTextures"`
    /// -   type: `hkArray<hkxTextureFile*>`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "externalTextures")]
    ExternalTextures(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"skinBindings"`
    /// -   type: `hkArray<hkxSkinBinding*>`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skinBindings")]
    SkinBindings(HkArrayRef<Cow<'a, str>>),
    /// # C++ Class Fields Info
    /// -   name:`"appliedTransform"`
    /// -   type: `hkMatrix3`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "appliedTransform")]
    AppliedTransform(Primitive<Matrix3<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxScene<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("modeller" => Modeller(Primitive<Cow<'de, str>>)),
    ("asset" => Asset(Primitive<Cow<'de, str>>)),
    ("sceneLength" => SceneLength(Primitive<f32>)),
    ("rootNode" => RootNode(Primitive<Cow<'de, str>>)),
    ("selectionSets" => SelectionSets(HkArrayRef<Cow<'de, str>>)),
    ("cameras" => Cameras(HkArrayRef<Cow<'de, str>>)),
    ("lights" => Lights(HkArrayRef<Cow<'de, str>>)),
    ("meshes" => Meshes(HkArrayRef<Cow<'de, str>>)),
    ("materials" => Materials(HkArrayRef<Cow<'de, str>>)),
    ("inplaceTextures" => InplaceTextures(HkArrayRef<Cow<'de, str>>)),
    ("externalTextures" => ExternalTextures(HkArrayRef<Cow<'de, str>>)),
    ("skinBindings" => SkinBindings(HkArrayRef<Cow<'de, str>>)),
    ("appliedTransform" => AppliedTransform(Primitive<Matrix3<f32>>)),
}
