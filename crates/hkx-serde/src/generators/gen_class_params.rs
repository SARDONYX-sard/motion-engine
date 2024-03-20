use super::{
    generate_all_fields,
    generate_code::{ClassMap, LifeTimeMap},
    get_lifetime_from_fields,
};
use convert_case::{Case, Casing as _};

/// Generate all havok C++ class enums.
///
/// Pass `{ key: class name, value: class information }` mapping as information.
///
/// The reason we don't take a signature is that this is not a unique value.
///
/// (**It may inherit the signature of the parent class**. e.g. `hkpShapeContainer`, `baseObject`, etc.)
pub fn generate_class_params(class_map: &ClassMap, life_time_map: &LifeTimeMap) -> String {
    let mut class_params = String::new();

    class_params.push_str(
        r#"
use super::*;
use crate::classes::Class;
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

"#,
    );

    for (_cpp_class_name, class) in class_map {
        let sig = class.signature;
        let struct_name = class.name.to_case(Case::Pascal);

        let (_rust_fields_code, fields) =
            generate_all_fields(class, class_map, Some(life_time_map));
        let life_time = get_lifetime_from_fields(&fields);
        let rust_class_name_with_life_time = format!("{struct_name}{life_time}");

        let life_time_bound = if rust_class_name_with_life_time.ends_with("<'a>") {
            format!("\n    #[serde(bound(deserialize = \"Vec<{rust_class_name_with_life_time}>: Deserialize<'de>\"))]")
        } else {
            "".into()
        };

        class_params.push_str(&format!(
            r#"    #[serde(rename = "0x{sig:x}")]{life_time_bound}
    {struct_name}(Vec<{rust_class_name_with_life_time}>),

"#
        ));
    }

    class_params.push_str(&format!(
        r#"}}

impl<'a> Serialize for Class<'a> {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Class", {})?;
        state.serialize_field("@name", &self.name)?;
        state.serialize_field("@class", &self.class)?;
        state.serialize_field("@signature", &self.signature)?;

        // Serialize hkparam based on class(C++ class name)
        match self.class.as_ref() {{
"#,
        class_map.len()
    ));

    for (cpp_class_name, class) in class_map {
        let struct_name = class.name.to_case(Case::Pascal);

        class_params.push_str(&format!(
            r#"
            "{cpp_class_name}" => {{
                if let ClassParams::{struct_name}(ref params) = self.hkparams {{
                    state.serialize_field("hkparam", params)?;
                }}
            }}
"#
        ));
    }

    class_params.push_str(
        r#"
            _ => {}
        }
        state.end()
    }
}
"#,
    );

    class_params.push_str(
        r#"
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
"#,
    );

    for (cpp_class_name, class) in class_map {
        let struct_name = class.name.to_case(Case::Pascal);

        class_params.push_str(&format!(
            r#"                                    "{cpp_class_name}" => {{
                                                        ClassParams::{struct_name}(map.next_value()?)
                                    }},
"#
        ));
    }

    class_params.push_str(
        r#"
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
"#,
    );

    class_params
}
