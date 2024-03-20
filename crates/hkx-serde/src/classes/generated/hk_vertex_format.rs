//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkVertexFormat`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkVertexFormat`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 260
/// -    vtable: false
/// - signature: `0xf11e3ff7`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkVertexFormat {
    /// # C++ Class Fields Info
    /// -   name:`"elements"`
    /// -   type: `struct hkVertexFormatElement[32]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "elements")]
    Elements(CStyleArrayClass<HkVertexFormatElement, 32>),
    /// # C++ Class Fields Info
    /// -   name:`"numElements"`
    /// -   type: `hkInt32`
    /// - offset: 256
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numElements")]
    NumElements(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkVertexFormat, "@name",
    ("elements" => Elements(CStyleArrayClass<HkVertexFormatElement, 32>)),
    ("numElements" => NumElements(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "TYPE_NONE")]
    TypeNone = 0,
    #[serde(rename = "TYPE_INT8")]
    TypeInt8 = 1,
    #[serde(rename = "TYPE_UINT8")]
    TypeUint8 = 2,
    #[serde(rename = "TYPE_INT16")]
    TypeInt16 = 3,
    #[serde(rename = "TYPE_UINT16")]
    TypeUint16 = 4,
    #[serde(rename = "TYPE_INT32")]
    TypeInt32 = 5,
    #[serde(rename = "TYPE_UINT32")]
    TypeUint32 = 6,
    #[serde(rename = "TYPE_UINT8_DWORD")]
    TypeUint8Dword = 7,
    #[serde(rename = "TYPE_ARGB32")]
    TypeArgb32 = 8,
    #[serde(rename = "TYPE_FLOAT16")]
    TypeFloat16 = 9,
    #[serde(rename = "TYPE_FLOAT32")]
    TypeFloat32 = 10,
    #[serde(rename = "TYPE_VECTOR4")]
    TypeVector4 = 11,
    #[serde(rename = "TYPE_LAST")]
    TypeLast = 12,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentUsage {
    #[serde(rename = "USAGE_NONE")]
    UsageNone = 0,
    #[serde(rename = "USAGE_POSITION")]
    UsagePosition = 1,
    #[serde(rename = "USAGE_NORMAL")]
    UsageNormal = 2,
    #[serde(rename = "USAGE_COLOR")]
    UsageColor = 3,
    #[serde(rename = "USAGE_TANGENT")]
    UsageTangent = 4,
    #[serde(rename = "USAGE_BINORMAL")]
    UsageBinormal = 5,
    #[serde(rename = "USAGE_BLEND_MATRIX_INDEX")]
    UsageBlendMatrixIndex = 6,
    #[serde(rename = "USAGE_BLEND_WEIGHTS")]
    UsageBlendWeights = 7,
    #[serde(rename = "USAGE_BLEND_WEIGHTS_LAST_IMPLIED")]
    UsageBlendWeightsLastImplied = 8,
    #[serde(rename = "USAGE_TEX_COORD")]
    UsageTexCoord = 9,
    #[serde(rename = "USAGE_POINT_SIZE")]
    UsagePointSize = 10,
    #[serde(rename = "USAGE_USER")]
    UsageUser = 11,
    #[serde(rename = "USAGE_LAST")]
    UsageLast = 12,
}
bitflags::bitflags! {
    /// # Bit flags that represented enum.
    ///
    /// # Note
    /// The 0 flag is always enabled when the trailing bit is 0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HintFlags: u16 {
        /// Special flags." 0" is used when the character "0" is received.
        /// - only "0" is also entered during serialization.
        /// - This flag is the default value that exists for all flags.
        const NULL= !0;
        const FLAG_READ = 1;
        const FLAG_WRITE = 2;
        const FLAG_DYNAMIC = 4;
        const FLAG_NOT_SHARED = 8;
    }
}

impl Default for HintFlags {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for HintFlags {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bits(value as u16).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl TryFrom<u16> for HintFlags {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or_else(|| format!("Set invalid value: {value}"))
    }
}

impl serde::Serialize for HintFlags {
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

impl<'de> serde::Deserialize<'de> for HintFlags {
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
                        "FLAG_READ" => flags |= Self::FLAG_READ,
                        "FLAG_WRITE" => flags |= Self::FLAG_WRITE,
                        "FLAG_DYNAMIC" => flags |= Self::FLAG_DYNAMIC,
                        "FLAG_NOT_SHARED" => flags |= Self::FLAG_NOT_SHARED,
                        unknown => match parse_int::parse(unknown) {
                            Ok(int) => {
                                if let Some(bits) = Self::from_bits(int) {
                                    flags |= bits
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Expected HintFlags flags but got '{unknown}'",
                                    )));
                                };
                            }
                            Err(_err) => {
                                return Err(serde::de::Error::custom(format!(
                                    "Expected HintFlags flags or integer, but got '{unknown}'"
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

impl core::fmt::Display for HintFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl HintFlags {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(Self::NULL) {
            return "0".into();
        };

        if self.contains(Self::FLAG_READ) {
            flags.push("FLAG_READ");
        }
        if self.contains(Self::FLAG_WRITE) {
            flags.push("FLAG_WRITE");
        }
        if self.contains(Self::FLAG_DYNAMIC) {
            flags.push("FLAG_DYNAMIC");
        }
        if self.contains(Self::FLAG_NOT_SHARED) {
            flags.push("FLAG_NOT_SHARED");
        }

        flags.join("|").into()
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SharingType {
    #[serde(rename = "SHARING_ALL_SHARED")]
    SharingAllShared = 0,
    #[serde(rename = "SHARING_ALL_NOT_SHARED")]
    SharingAllNotShared = 1,
    #[serde(rename = "SHARING_MIXTURE")]
    SharingMixture = 2,
}
