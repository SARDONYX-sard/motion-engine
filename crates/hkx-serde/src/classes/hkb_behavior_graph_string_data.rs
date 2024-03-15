//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbBehaviorGraphStringData`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};

/// `hkbBehaviorGraphStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xc713064e`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphStringData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"eventNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventNames")]
    EventNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"attributeNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeNames")]
    AttributeNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"variableNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableNames")]
    VariableNames(HkArrayStringPtr<'a>),
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyNames")]
    CharacterPropertyNames(HkArrayStringPtr<'a>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphStringData<'de>, "@name",
    ("eventNames" => EventNames(HkArrayStringPtr<'de>)),
    ("attributeNames" => AttributeNames(HkArrayStringPtr<'de>)),
    ("variableNames" => VariableNames(HkArrayStringPtr<'de>)),
    ("characterPropertyNames" => CharacterPropertyNames(HkArrayStringPtr<'de>)),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::{Class, ClassParams};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data = Class {
            name: "#0085".into(),
            class: "hkbBehaviorGraphStringData".into(),
            signature: "0xc713064e".into(),
            hkparams: ClassParams::HkbBehaviorGraphStringData(vec![
                HkbBehaviorGraphStringData::EventNames(
                    vec![
                        "cannedTurnRight90Flee",
                        "cannedTurnRight180Flee",
                        "cannedTurnLeft90Flee",
                        "cannedTurnLeft180Flee",
                    ]
                    .into(),
                ),
                HkbBehaviorGraphStringData::AttributeNames(Default::default()),
                HkbBehaviorGraphStringData::VariableNames(
                    vec![
                        "blendDefault",
                        "blendFast",
                        "blendSlow",
                        "Direction",
                        "IsBlocking",
                        "Speed",
                    ]
                    .into(),
                ),
                HkbBehaviorGraphStringData::CharacterPropertyNames(Default::default()),
            ]),
        };
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected = "\
<hkobject name=\"#0085\" class=\"hkbBehaviorGraphStringData\" signature=\"0xc713064e\">\
    <hkparam name=\"eventNames\" numelements=\"4\">\
      <hkcstring>cannedTurnRight90Flee</hkcstring>\
      <hkcstring>cannedTurnRight180Flee</hkcstring>\
      <hkcstring>cannedTurnLeft90Flee</hkcstring>\
      <hkcstring>cannedTurnLeft180Flee</hkcstring>\
    </hkparam>\
    <hkparam name=\"attributeNames\" numelements=\"0\"/>\
    <hkparam name=\"variableNames\" numelements=\"6\">\
      <hkcstring>blendDefault</hkcstring>\
      <hkcstring>blendFast</hkcstring>\
        <hkcstring>blendSlow</hkcstring>\
        <hkcstring>Direction</hkcstring>\
        <hkcstring>IsBlocking</hkcstring>\
        <hkcstring>Speed</hkcstring>\
      </hkparam>\
    <hkparam name=\"characterPropertyNames\" numelements=\"0\"/>\
</hkobject>\
";
        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize() {
        let xml_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("dummy.xml");
        let xml_str = std::fs::read_to_string(xml_path).unwrap();
        let deserialized: Class = quick_xml::de::from_str(&xml_str).unwrap();

        let expected = Class {
            name: "#0085".into(),
            class: "hkbBehaviorGraphStringData".into(),
            signature: "0xc713064e".into(),
            hkparams: ClassParams::HkbBehaviorGraphStringData(vec![
                HkbBehaviorGraphStringData::EventNames(
                    vec![
                        "cannedTurnRight90Flee",
                        "cannedTurnRight180Flee",
                        "cannedTurnLeft90Flee",
                        "cannedTurnLeft180Flee",
                    ]
                    .into(),
                ),
                HkbBehaviorGraphStringData::AttributeNames(Default::default()),
                HkbBehaviorGraphStringData::VariableNames(
                    vec![
                        "blendDefault",
                        "blendFast",
                        "blendSlow",
                        "Direction",
                        "IsBlocking",
                        "Speed",
                    ]
                    .into(),
                ),
                HkbBehaviorGraphStringData::CharacterPropertyNames(Default::default()),
            ]),
        };
        assert_eq!(deserialized, expected);
    }
}
