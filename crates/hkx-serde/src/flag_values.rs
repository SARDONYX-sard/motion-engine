//! Flags for field alignment needs, skipping serialization, etc.(The original C++ code is u16 bit flags)
//! - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, fmt};

bitflags::bitflags! {
    /// Flags for field alignment needs, skipping serialization, etc.
    /// - ref: havok_2010_2_0/Source/Common/Base/Refection/hkClassMember.h#L112
    ///
    /// # Note
    /// Note that the zero th `FLAGS_NONE` is forced to be included due to the bit flag specification.
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FlagValues: usize {
        /// Special flags." 0" is used when the character "0" is received,
        /// and only "0" is also entered during serialization.
        /// This flag is the default value that exists for all flags.
        const NULL= !0;

        const FLAGS_NONE = 0;
        /// Member has forced 8 byte alignment. 1 << 7
        const ALIGN8 = 128;
        /// Member has forced 16 byte alignment. 1 << 8
        const ALIGN16 = 256;
        /// The members memory contents is not owned by this object 1 << 9
        const NOT_OWNED = 512;
        /// This member should not be written when serializing 1 << 10
        const SERIALIZE_IGNORED = 1024;
    }
}

impl Default for FlagValues {
    fn default() -> Self {
        Self::NULL
    }
}

impl TryFrom<usize> for FlagValues {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        FlagValues::from_bits(value).ok_or_else(|| format!("Set invalid value: {}", value))
    }
}

impl FlagValues {
    /// Creates a `FlagValues` instance from a `usize` value.
    ///
    /// This method allows converting a `usize` value to a `FlagValues` instance. Only bits that
    /// correspond to defined flags are considered.
    ///
    /// # Examples
    ///
    /// ```
    /// use hkx_serde::flag_values::FlagValues;
    ///
    /// let flags = FlagValues::from_u16(0b1010);
    /// assert_eq!(flags, FlagValues::ALIGN8 | FlagValues::NOT_OWNED);
    /// ```
    pub fn from_u16(value: u16) -> Self {
        FlagValues::from_bits_truncate(value as usize)
    }

    /// Returns the `FlagValues` instance as a `usize` value.
    ///
    /// This method converts the `FlagValues` instance to a `usize` value, preserving all bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use hkx_serde::flag_values::FlagValues;
    ///
    /// let flags = FlagValues::ALIGN8 | FlagValues::SERIALIZE_IGNORED;
    /// assert_eq!(flags.as_u16(), 0b100000001000);
    /// ```
    pub fn as_u16(&self) -> u16 {
        self.bits() as u16
    }
}

impl Serialize for FlagValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.contains(Self::NULL) {
            serializer.serialize_str("0")
        } else {
            serializer.serialize_str(&self.human_readable())
        }
    }
}

impl<'de> Deserialize<'de> for FlagValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<Cow<'de, str>>::deserialize(deserializer)?;

        match value {
            Some(s) => {
                if s.as_ref() == "0" {
                    return Ok(FlagValues::NULL);
                };
                let mut flags = FlagValues::FLAGS_NONE;
                for token in s.split('|') {
                    match token.trim() {
                        "FLAGS_NONE" => flags |= FlagValues::FLAGS_NONE,
                        "ALIGN8" => flags |= FlagValues::ALIGN8,
                        "ALIGN16" => flags |= FlagValues::ALIGN16,
                        "NOT_OWNED" => flags |= FlagValues::NOT_OWNED,
                        "SERIALIZE_IGNORED" => flags |= FlagValues::SERIALIZE_IGNORED,
                        _ => return Err(serde::de::Error::custom("Invalid flag")),
                    }
                }
                Ok(flags)
            }
            None => Ok(FlagValues::NULL),
        }
    }
}

impl fmt::Debug for FlagValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("FlagValues");

        if self.contains(FlagValues::NULL) {
            debug_struct.field("NULL", &true);
        }

        if self.contains(FlagValues::FLAGS_NONE) {
            debug_struct.field("FLAGS_NONE", &true);
        }
        if self.contains(FlagValues::ALIGN8) {
            debug_struct.field("ALIGN8", &true);
        }
        if self.contains(FlagValues::ALIGN16) {
            debug_struct.field("ALIGN16", &true);
        }
        if self.contains(FlagValues::NOT_OWNED) {
            debug_struct.field("NOT_OWNED", &true);
        }
        if self.contains(FlagValues::SERIALIZE_IGNORED) {
            debug_struct.field("SERIALIZE_IGNORED", &true);
        }

        debug_struct.finish()
    }
}

impl fmt::Display for FlagValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits())
    }
}

impl FlagValues {
    /// Use a string format that is easy for humans to read.
    ///
    /// Like this.
    /// `"FLAGS_NONE | SERIALIZE_IGNORED"`
    pub fn human_readable(&self) -> Cow<'_, str> {
        let mut flags = Vec::new();

        if self.contains(FlagValues::NULL) {
            return "0".into();
        };

        if self.contains(FlagValues::FLAGS_NONE) {
            flags.push("FLAGS_NONE");
        }
        if self.contains(FlagValues::ALIGN8) {
            flags.push("ALIGN8");
        }
        if self.contains(FlagValues::ALIGN16) {
            flags.push("ALIGN16");
        }
        if self.contains(FlagValues::NOT_OWNED) {
            flags.push("NOT_OWNED");
        }
        if self.contains(FlagValues::SERIALIZE_IGNORED) {
            flags.push("SERIALIZE_IGNORED");
        }

        flags.join("|").into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "hkparam")]
    struct TestRoot<T>(T);

    #[test]
    fn should_serialize_flags_null_to_0() {
        let flags = FlagValues::NULL;
        let serialized = to_string(&TestRoot(flags)).unwrap();
        assert_eq!(serialized, "<hkparam>0</hkparam>");
    }

    #[test]
    fn should_serialize_flags_none() {
        let flags = FlagValues::FLAGS_NONE;
        let serialized = to_string(&TestRoot(flags)).unwrap();
        assert_eq!(serialized, "<hkparam>FLAGS_NONE</hkparam>");
    }

    #[test]
    fn should_serialize_multiple_flags() {
        let flags = FlagValues::ALIGN8 | FlagValues::ALIGN16 | FlagValues::SERIALIZE_IGNORED;
        let serialized = to_string(&TestRoot(flags)).unwrap();
        assert_eq!(
            serialized,
            "<hkparam>ALIGN8|ALIGN16|SERIALIZE_IGNORED</hkparam>"
        );
    }

    #[test]
    fn should_deserialize_0_to_flags_null() {
        let deserialized: FlagValues = from_str("<hkparam>0</hkparam>").unwrap();
        assert_eq!(deserialized, FlagValues::NULL);
    }

    #[test]
    fn should_deserialize_single_str_to_flags_none() {
        let deserialized: FlagValues = from_str("<hkparam>FLAGS_NONE</hkparam>").unwrap();
        assert_eq!(deserialized, FlagValues::FLAGS_NONE);
    }

    #[test]
    fn should_deserialize_str_flags_some() {
        let deserialized: FlagValues =
            from_str("<hkparam>ALIGN8|ALIGN16|SERIALIZE_IGNORED</hkparam>").unwrap();
        let expected = FlagValues::ALIGN8 | FlagValues::ALIGN16 | FlagValues::SERIALIZE_IGNORED;
        assert_eq!(deserialized, expected);
    }
}
