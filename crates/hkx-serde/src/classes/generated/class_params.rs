//! The type of enumeration of all C++ havok class fields.
use super::*;
use crate::classes::Class;
use crate::bytes::*;
use crate::error::{HkxError, Result};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Pattern enumeration of all C++ havok class fields.
///
/// In XML, these are the fields of the attribute `hkparam`
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(tag = "@signature")]
pub enum ClassParams<'a> {
    #[default]
    Unknown,

    #[serde(rename = "0x13a39ba7")]
    #[serde(bound(deserialize = "Vec<HkbProjectData<'a>>: Deserialize<'de>"))]
    HkbProjectData(Vec<HkbProjectData<'a>>),

    #[serde(rename = "0x76ad60a")]
    #[serde(bound(deserialize = "Vec<HkbProjectStringData<'a>>: Deserialize<'de>"))]
    HkbProjectStringData(Vec<HkbProjectStringData<'a>>),

    #[serde(rename = "0x945da157")]
    #[serde(bound(deserialize = "Vec<HkbTransitionEffect<'a>>: Deserialize<'de>"))]
    HkbTransitionEffect(Vec<HkbTransitionEffect<'a>>),

    #[serde(rename = "0xb103a2cd")]
    #[serde(bound(deserialize = "Vec<HkRootLevelContainerNamedVariant<'a>>: Deserialize<'de>"))]
    HkRootLevelContainerNamedVariant(Vec<HkRootLevelContainerNamedVariant<'a>>),

    #[serde(rename = "0x2772c11e")]
    #[serde(bound(deserialize = "Vec<HkRootLevelContainer<'a>>: Deserialize<'de>"))]
    HkRootLevelContainer(Vec<HkRootLevelContainer<'a>>),

}
impl<'a> Serialize for Class<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Class", 668)?;
        state.serialize_field("@name", &self.name)?;
        state.serialize_field("@class", &self.class)?;
        state.serialize_field("@signature", &self.signature)?;

        // Serialize hkparam based on class(C++ class name)
        match self.class.as_ref() {

            "hkbProjectData" => {
                if let ClassParams::HkbProjectData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbProjectStringData" => {
                if let ClassParams::HkbProjectStringData(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkbTransitionEffect" => {
                if let ClassParams::HkbTransitionEffect(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkRootLevelContainerNamedVariant" => {
                if let ClassParams::HkRootLevelContainerNamedVariant(ref params) = self.hkparams {
                    state.serialize_field("hkparam", params)?;
                }
            }

            "hkRootLevelContainer" => {
                if let ClassParams::HkRootLevelContainer(ref params) = self.hkparams {
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
                            if let Some(ref class_name) = class {
                                hkparam = Some(Ok(match class_name.as_ref() {
                                    "hkbProjectData" => {
                                                        ClassParams::HkbProjectData(map.next_value()?)
                                    },
                                    "hkbProjectStringData" => {
                                                        ClassParams::HkbProjectStringData(map.next_value()?)
                                    },
                                    "hkbTransitionEffect" => {
                                                        ClassParams::HkbTransitionEffect(map.next_value()?)
                                    },
                                    "hkRootLevelContainerNamedVariant" => {
                                                        ClassParams::HkRootLevelContainerNamedVariant(map.next_value()?)
                                    },
                                    "hkRootLevelContainer" => {
                                                        ClassParams::HkRootLevelContainer(map.next_value()?)
                                    },

                                    unknown => {
                                        return Err(de::Error::custom(format!(
                                            "Unexpected value {unknown}"
                                        )))
                                    }
                                })?);
                            } else {
                                return Err(de::Error::custom("Processing an array of `hkparam` requires identification by `class` attribute first, but non exist"));
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
                let hkparams = hkparam.ok_or_else(|| de::Error::missing_field("hkparam"))?;

                Ok(Class {
                    name,
                    class,
                    signature,
                    hkparams,
                })
            }
        }

        deserializer.deserialize_map(ClassVisitor)
    }
}

impl<'a> ClassParams<'a> {
    /// Read the contents of hkparam by C++ havok Class name and create a data structure for rust.
    ///
    /// # Assumptions
    /// - The starting point of `bytes` must be the binary data position of the fields
    ///   of the class(`class_name`) to be deserialized.
    pub fn from_class_name_and_bytes<B>(class_name: &str, bytes: &[u8]) -> Result<Self>
    where
        B: ByteOrder,
    {
        Ok(match class_name {
            "hkbProjectData" => ClassParams::HkbProjectData(
                HkbProjectData::from_bytes::<B>(bytes)?,
            ),
            "hkbProjectStringData" => ClassParams::HkbProjectStringData(
                HkbProjectStringData::from_bytes::<B>(bytes)?,
            ),
            "hkbTransitionEffect" => ClassParams::HkbTransitionEffect(
                HkbTransitionEffect::from_bytes::<B>(bytes)?,
            ),
            "hkRootLevelContainerNamedVariant" => ClassParams::HkRootLevelContainerNamedVariant(
                HkRootLevelContainerNamedVariant::from_bytes::<B>(bytes)?,
            ),
            "hkRootLevelContainer" => ClassParams::HkRootLevelContainer(
                HkRootLevelContainer::from_bytes::<B>(bytes)?,
            ),

            unknown => return Err(HkxError::UnknownHavokClass(unknown.into())),
        })
    }
}
