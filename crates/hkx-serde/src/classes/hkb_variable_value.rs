//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableValue`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use crate::havok_types::Primitive;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableValue {
    /// # Information on fields in the original C++ class
    /// -   name:`"value"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Primitive<i32>),
}

impl Default for HkbVariableValue {
    fn default() -> Self {
        Self::Value(0.into())
    }
}

impl From<i32> for HkbVariableValue {
    fn from(value: i32) -> Self {
        Self::Value(value.into())
    }
}

impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValue, "@name",
    ("value"=> Value(Primitive<i32>))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::{Class, ClassParams};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = Class {
            name: "#0060".into(),
            class: "hkbVariableValue".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValue(vec![HkbVariableValue::Value(
                1045220557.into(),
            )]),
        };

        let result = quick_xml::se::to_string(&class).unwrap();

        let expected_xml = "\
<hkobject name=\"#0060\" class=\"hkbVariableValue\" signature=\"0x27812d8d\">\
    <hkparam name=\"value\">1045220557</hkparam>\
</hkobject>";

        assert_eq!(result, expected_xml);
        dbg!(result);
    }

    #[test]
    fn should_deserialize() {
        let xml = "\
<hkobject name=\"#0060\" class=\"hkbVariableValue\" signature=\"0x27812d8d\">\
    <hkparam name=\"value\">1045220557</hkparam>\
</hkobject>";

        let result: Class = quick_xml::de::from_str(xml).unwrap();
        let expected = Class {
            name: "#0060".into(),
            class: "hkbVariableValue".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValue(vec![HkbVariableValue::Value(
                1045220557.into(),
            )]),
        };
        assert_eq!(result, expected);
    }
}
