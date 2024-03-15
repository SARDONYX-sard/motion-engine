// mod generated;
mod hkb_behavior_graph_string_data;
mod hkb_variable_value;
mod hkb_variable_value_set;
mod root;

use self::hkb_behavior_graph_string_data::HkbBehaviorGraphStringData;
use self::hkb_variable_value::HkbVariableValue;
use self::hkb_variable_value_set::HkbVariableValueSet;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class name
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Class<'a> {
    /// e.g. `#0106`
    ///
    /// In XML, these names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    pub name: Cow<'a, str>,

    /// Name of each C++ class.
    ///
    /// e.g. `"hkbBehaviorGraphStringData"`
    pub class: Cow<'a, str>,

    /// Unique value of each class.
    ///
    /// e.g. `0xc713064e`
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ fields) vector
    pub hkparams: ClassParams<'a>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(tag = "@signature")]
pub enum ClassParams<'a> {
    #[default]
    Unknown,

    #[serde(rename = "0x27812d8d")]
    HkbVariableValueSet(Vec<HkbVariableValueSet<'a>>),
    #[serde(rename = "0xc713064e")]
    HkbVariableValue(Vec<HkbVariableValue>),
    #[serde(rename = "0xb99bd6a")]
    HkbBehaviorGraphStringData(Vec<HkbBehaviorGraphStringData<'a>>),
}

impl<'a> Serialize for Class<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Class", 4)?;
        state.serialize_field("@name", &self.name)?;
        state.serialize_field("@class", &self.class)?;
        state.serialize_field("@signature", &self.signature)?;

        // Serialize hkparam based on signature
        match self.signature.as_ref() {
            "0x27812d8d" => {
                if let ClassParams::HkbVariableValueSet(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }
            "0xb99bd6a" => {
                if let ClassParams::HkbVariableValue(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }
            "0xc713064e" => {
                if let ClassParams::HkbBehaviorGraphStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }
            _ => {}
        }

        state.end()
    }
}

// # Note
// In [`quick_xml::impl_deserialize_for_internally_tagged_enum`], only the first attribute can be deserialized by tag.
// What we need this time is the third attribute, "signature". Therefore, we need to deserialize it on our own initiative.
impl<'de> Deserialize<'de> for Class<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClassVisitor;

        impl<'de> Visitor<'de> for ClassVisitor {
            type Value = Class<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Class")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name: Option<Cow<'de, str>> = None;
                let mut class: Option<Cow<'de, str>> = None;
                let mut signature: Option<Cow<'de, str>> = None;
                let mut hkparam: Option<ClassParams<'de>> = None;

                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_ref() {
                        "@name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("@name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "@class" => {
                            if class.is_some() {
                                return Err(de::Error::duplicate_field("@class"));
                            }
                            class = Some(map.next_value()?);
                        }
                        "@signature" => {
                            if signature.is_some() {
                                return Err(de::Error::duplicate_field("@signature"));
                            }
                            signature = Some(map.next_value()?);
                        }
                        "hkparam" => {
                            if let Some(ref signature) = signature {
                                hkparam = Some(Ok(match signature.as_ref() {
                                    "0x27812d8d" => {
                                        ClassParams::HkbVariableValueSet(map.next_value()?)
                                    }
                                    "0xb99bd6a" => ClassParams::HkbVariableValue(map.next_value()?),
                                    "0xc713064e" => {
                                        ClassParams::HkbBehaviorGraphStringData(map.next_value()?)
                                    }
                                    unknown => {
                                        return Err(de::Error::custom(format!(
                                            "Unexpected value {unknown}"
                                        )))
                                    }
                                })?);
                            } else {
                                return Err(de::Error::custom("Processing an array of `hkparam` requires identification by `signature` first, but the `signature` attribute did not exist"));
                            }
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("@name"))?;
                let class = class.ok_or_else(|| de::Error::missing_field("@class"))?;
                let signature = signature.ok_or_else(|| de::Error::missing_field("@signature"))?;
                let hkparam = hkparam.ok_or_else(|| de::Error::missing_field("hkparam"))?;

                Ok(Class {
                    name,
                    class,
                    signature,
                    hkparams: hkparam,
                })
            }
        }

        deserializer.deserialize_map(ClassVisitor)
    }
}
