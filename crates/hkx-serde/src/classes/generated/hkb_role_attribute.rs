//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRoleAttribute`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkbRoleAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// - signature: `0x3eb2e082`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRoleAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"role"`
    /// -   type: `enum Role`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "role")]
    Role(Primitive<Role>),
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags RoleFlags`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<RoleFlags>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRoleAttribute, "@name",
    ("role" => Role(Primitive<Role>)),
    ("flags" => Flags(Primitive<RoleFlags>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "ROLE_DEFAULT")]
    RoleDefault = 0,
    #[serde(rename = "ROLE_FILE_NAME")]
    RoleFileName = 1,
    #[serde(rename = "ROLE_BONE_INDEX")]
    RoleBoneIndex = 2,
    #[serde(rename = "ROLE_BONE_INDEX_MAP")]
    RoleBoneIndexMap = 3,
    #[serde(rename = "ROLE_EVENT_ID")]
    RoleEventId = 4,
    #[serde(rename = "ROLE_VARIABLE_INDEX")]
    RoleVariableIndex = 5,
    #[serde(rename = "ROLE_ATTRIBUTE_INDEX")]
    RoleAttributeIndex = 6,
    #[serde(rename = "ROLE_TIME")]
    RoleTime = 7,
}
bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RoleFlags: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
        const FLAG_NONE = 0;
        const FLAG_RAGDOLL = 1;
        const FLAG_NORMALIZED = 2;
        const FLAG_NOT_VARIABLE = 4;
        const FLAG_HIDDEN = 8;
        const FLAG_OUTPUT = 16;
        const FLAG_NOT_CHARACTER_PROPERTY = 32;
    }
}

impl Default for RoleFlags {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for RoleFlags {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for RoleFlags {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for RoleFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.contains(Self::NULL) {
            serializer.serialize_str("0")
        } else {
            serializer.serialize_str(&self.human_readable())
        }
    }
}

impl<'de> serde::Deserialize<'de> for RoleFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<std::borrow::Cow<'de, str>>::deserialize(deserializer)?;

        match value {
            Some(s) => {
                if s.as_ref() == "0" {
                    return Ok(Self::NULL);
                };
                let mut flags = Self::empty();
                for token in s.split('|') {
                    match token.trim() {
                        "FLAG_NONE" => flags |= Self::FLAG_NONE,
                        "FLAG_RAGDOLL" => flags |= Self::FLAG_RAGDOLL,
                        "FLAG_NORMALIZED" => flags |= Self::FLAG_NORMALIZED,
                        "FLAG_NOT_VARIABLE" => flags |= Self::FLAG_NOT_VARIABLE,
                        "FLAG_HIDDEN" => flags |= Self::FLAG_HIDDEN,
                        "FLAG_OUTPUT" => flags |= Self::FLAG_OUTPUT,
                        "FLAG_NOT_CHARACTER_PROPERTY" => flags |= Self::FLAG_NOT_CHARACTER_PROPERTY,
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected RoleFlags flags but got \"{unknown}\"",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected RoleFlags flags or integer, but got \"{unknown}\""
                                )))
                            }
                        },
                    }
                }
                Ok(flags)
            }
            None => Ok(Self::NULL),
        }
    }
}

impl core::fmt::Display for RoleFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl RoleFlags {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAG_NONE) {
            flags.push("FLAG_NONE");
        }
        if self.contains(Self::FLAG_RAGDOLL) {
            flags.push("FLAG_RAGDOLL");
        }
        if self.contains(Self::FLAG_NORMALIZED) {
            flags.push("FLAG_NORMALIZED");
        }
        if self.contains(Self::FLAG_NOT_VARIABLE) {
            flags.push("FLAG_NOT_VARIABLE");
        }
        if self.contains(Self::FLAG_HIDDEN) {
            flags.push("FLAG_HIDDEN");
        }
        if self.contains(Self::FLAG_OUTPUT) {
            flags.push("FLAG_OUTPUT");
        }
        if self.contains(Self::FLAG_NOT_CHARACTER_PROPERTY) {
            flags.push("FLAG_NOT_CHARACTER_PROPERTY");
        }

        flags.join("|").into()
    }
}
