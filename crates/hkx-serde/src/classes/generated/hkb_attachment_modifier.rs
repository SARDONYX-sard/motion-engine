//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAttachmentModifier`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbAttachmentModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 108
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0xcc0aab32`
/// -   version: 1
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAttachmentModifier<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacherOnAttach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacherOnAttach")]
    SendToAttacherOnAttach(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacheeOnAttach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacheeOnAttach")]
    SendToAttacheeOnAttach(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacherOnDetach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacherOnDetach")]
    SendToAttacherOnDetach(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"sendToAttacheeOnDetach"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sendToAttacheeOnDetach")]
    SendToAttacheeOnDetach(HkbEventProperty),
    /// # C++ Class Fields Info
    /// -   name:`"attachmentSetup"`
    /// -   type: `struct hkbAttachmentSetup*`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachmentSetup")]
    AttachmentSetup(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"attacherHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attacherHandle")]
    AttacherHandle(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"attacheeHandle"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attacheeHandle")]
    AttacheeHandle(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"attacheeLayer"`
    /// -   type: `hkInt32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attacheeLayer")]
    AttacheeLayer(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"attacheeRB"`
    /// -   type: `void*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attacheeRB", skip_serializing)]
    AttacheeRb(Cow<'a, str>),
    /// # C++ Class Fields Info
    /// -   name:`"oldMotionType"`
    /// -   type: `enum unknown`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldMotionType", skip_serializing)]
    OldMotionType(Primitive<Unknown>),
    /// # C++ Class Fields Info
    /// -   name:`"oldFilterInfo"`
    /// -   type: `hkInt32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "oldFilterInfo", skip_serializing)]
    OldFilterInfo(Primitive<i32>),
    /// # C++ Class Fields Info
    /// -   name:`"attachment"`
    /// -   type: `void*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attachment", skip_serializing)]
    Attachment(Cow<'a, str>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttachmentModifier<'de>, "@name",
    ("sendToAttacherOnAttach" => SendToAttacherOnAttach(HkbEventProperty)),
    ("sendToAttacheeOnAttach" => SendToAttacheeOnAttach(HkbEventProperty)),
    ("sendToAttacherOnDetach" => SendToAttacherOnDetach(HkbEventProperty)),
    ("sendToAttacheeOnDetach" => SendToAttacheeOnDetach(HkbEventProperty)),
    ("attachmentSetup" => AttachmentSetup(Cow<'de, str>)),
    ("attacherHandle" => AttacherHandle(Cow<'de, str>)),
    ("attacheeHandle" => AttacheeHandle(Cow<'de, str>)),
    ("attacheeLayer" => AttacheeLayer(Primitive<i32>)),
    ("attacheeRB" => AttacheeRb(Cow<'de, str>)),
    ("oldMotionType" => OldMotionType(Primitive<Unknown>)),
    ("oldFilterInfo" => OldFilterInfo(Primitive<i32>)),
    ("attachment" => Attachment(Cow<'de, str>)),
}
