//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableValueSet`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::hkb_variable_value::HkbVariableValue;
use crate::havok_types::{HkArrayClass, HkArrayRef, HkArrayVector, Vector4};
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableValueSet<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"wordVariableValues"`
    /// -   type: `hkArray&lt;struct hkbVariableValue&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "wordVariableValues")]
    Word(HkArrayClass<HkbVariableValue>),
    /// # Information on fields in the original C++ class
    /// -   name:`"quadVariableValues"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quadVariableValues")]
    Quad(HkArrayVector<Vector4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"variantVariableValues"`
    /// -   type: `hkArray&lt;hkReferencedObject*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variantVariableValues")]
    Variant(HkArrayRef<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableValueSet<'de>, "@name",
    ("wordVariableValues" => Word(HkArrayClass<HkbVariableValue>)),
    ("quadVariableValues" => Quad(HkArrayVector<Vector4<f32>>)),
    ("variantVariableValues" => Variant(HkArrayRef<Cow<'_, str>>)),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        classes::{Class, ClassParams},
        havok_types::HkArrayClassParam,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let result = quick_xml::se::to_string(&Class {
            name: "#0064".into(),
            class: "hkbVariableValueSet".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValueSet(vec![
                HkbVariableValueSet::Word(HkArrayClass {
                    numelements: 3,
                    classes: vec![
                        HkArrayClassParam {
                            hkparam: 1045220557.into(),
                        },
                        HkArrayClassParam { hkparam: 0.into() },
                        HkArrayClassParam { hkparam: 0.into() },
                    ],
                }),
                HkbVariableValueSet::Quad(HkArrayVector {
                    numelements: 2,
                    value: vec![
                        (63.0, 64.0, 65.0, 66.0).into(),
                        (63.0, 64.0, 65.0, 66.0).into(),
                    ],
                }),
                HkbVariableValueSet::Variant(HkArrayRef {
                    numelements: 2,
                    value: vec!["#0063".into(), "#0064".into()],
                }),
            ]),
        })
        .unwrap();

        dbg!(&result);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
<hkobject name="#0064" class="hkbVariableValueSet" signature="0x27812d8d">
    <hkparam name="wordVariableValues" numelements="3">
        <hkobject>
            <hkparam name="value">1045220557</hkparam>
        </hkobject>
        <hkobject>
            <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
            <hkparam name="value">0</hkparam>
        </hkobject>
    </hkparam>
    <hkparam name="quadVariableValues" numelements="2">
        (0.000000 1.000000 0.000000 0.000000)
        (0.000000 0.000000 -1.000000 0.000000)
    </hkparam>
    <hkparam name="variantVariableValues" numelements="2">
        #0063 #0064
    </hkparam>
</hkobject>
"###;

        let result: Class = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            result,
            Class {
                name: "#0064".into(),
                class: "hkbVariableValueSet".into(),
                signature: "0x27812d8d".into(),
                hkparams: ClassParams::HkbVariableValueSet(vec![
                    HkbVariableValueSet::Word(HkArrayClass {
                        numelements: 3,
                        classes: vec![
                            HkArrayClassParam {
                                hkparam: 1045220557.into(),
                            },
                            HkArrayClassParam { hkparam: 0.into() },
                            HkArrayClassParam { hkparam: 0.into() },
                        ]
                    },),
                    HkbVariableValueSet::Quad(HkArrayVector {
                        numelements: 2,
                        value: vec![
                            (0.000000, 1.000000, 0.000000, 0.000000).into(),
                            (0.000000, 0.000000, -1.000000, 0.000000).into()
                        ],
                    },),
                    HkbVariableValueSet::Variant(HkArrayRef {
                        numelements: 2,
                        value: vec!["#0063".into(), "#0064".into()],
                    },),
                ]),
            }
        );
    }
}
