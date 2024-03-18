//! Flags for field alignment needs, skipping serialization, etc.(The original C++ code is u16 bit flags)
//! - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
use crate::parse_rpt::Enum;

/// Generate u16 bit flags Rust code
pub fn generate_bitflags(enum_info: &Enum) -> String {
    let mut rust_code = String::new();
    let (enum_name, enum_pair_info) = enum_info;

    rust_code.push_str(&format!(
        r#"bitflags::bitflags! {{
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct {enum_name}: u16 {{
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
"#
    ));

    for (enum_tag, enum_value) in enum_pair_info {
        rust_code.push_str(&format!("        const {enum_tag} = {enum_value};\n"));
    }

    rust_code.push_str(&format!(
        r#"    }}
}}

impl Default for {enum_name} {{
    fn default() -> Self {{
        Self::NULL
    }}
}}

impl TryFrom<usize> for {enum_name} {{
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {{
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {{value}}"))
    }}
}}

impl TryFrom<u16> for {enum_name} {{
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {{
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {{value}}"))
    }}
}}

impl serde::Serialize for {enum_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        if self.contains(Self::NULL) {{
            serializer.serialize_str("0")
        }} else {{
            serializer.serialize_str(&self.human_readable())
        }}
    }}
}}

impl<'de> serde::Deserialize<'de> for {enum_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {{
            Some(s) => {{
                if s.as_ref() == "0" {{
                    return Ok(Self::NULL);
                }};
                let mut flags = Self::empty();
                for token in s.split('|') {{
                    match token.trim() {{
"#
    ));

    for (enum_tag, _enum_value) in enum_pair_info {
        rust_code.push_str(&format!(
            "                        \"{enum_tag}\" => flags |= Self::{enum_tag},\n"
        ));
    }

    rust_code.push_str(
        r#"                        _ => return Err(serde::de::Error::custom("Invalid flag")),
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}"#,
    );

    rust_code.push_str(&format!(
        r#"

impl core::fmt::Display for {enum_name} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        write!(f, "{{}}", self.bits())
    }}
}}

impl {enum_name} {{
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {{
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {{
            return "0".into();
        }};
"#
    ));

    for (enum_tag, _enum_value) in enum_pair_info {
        rust_code.push_str(&format!(
            r#"
        if self.contains(Self::{enum_tag}) {{
            flags.push("{enum_tag}");
        }}"#
        ));
    }

    rust_code.push_str(
        r#"

        flags.join("|").into()
    }
}
"#,
    );

    rust_code
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_generate_bitflags() {
        // Define an example enum info
        let enum_info = (
            "FlagValues".into(),
            vec![
                ("FLAGS_NONE", 0),
                ("ALIGN8", 1 << 7),
                ("ALIGN16", 1 << 8),
                ("NOT_OWNED", 1 << 9),
                ("SERIALIZE_IGNORED", 1 << 10),
            ]
            .into_iter()
            .map(|(tag, value)| (tag.into(), value))
            .collect(),
        );
        let generated_code = generate_bitflags(&enum_info);

        // let expected = include_str!("../flag_values.rs");
        // let expected = &expected[..expected.find("#[cfg(test").unwrap()];
        // assert_eq!(generated_code, expected);
        tracing::debug!("{generated_code}");
    }
}
