use super::Enum;

pub fn serialize_enum<S>(enums: &[Enum], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    use serde::Serialize;

    #[derive(Serialize)]
    struct EnumInfo {
        name: String,
        flags: &'static str, // Change flags type to &'static str
        enum_item: Vec<EnumItem>,
    }

    #[derive(Serialize)]
    struct EnumItem {
        name: String,
        value: i32,
    }

    impl From<Enum> for EnumInfo {
        fn from(value: Enum) -> Self {
            let enum_item = value
                .1
                .iter()
                .map(|enum_pair| EnumItem {
                    name: enum_pair.0.clone(), // Cloning the string
                    value: enum_pair.1,
                })
                .collect();

            Self {
                name: value.0.clone(), // Cloning the string
                flags: "00000000",     // Example value, replace with actual flags
                enum_item,
            }
        }
    }

    let mut seq = serializer.serialize_seq(Some(enums.len()))?;
    for value in enums {
        seq.serialize_element(&EnumInfo::from(value.clone()))?;
    }
    seq.end()
}

pub fn serialize_first_element_of_option_tuple<S>(
    value: &Option<(String, u32)>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize as _;

    pub struct OptionFirstElementSerializer<T>(pub Option<(T, u32)>);
    impl<T> serde::Serialize for OptionFirstElementSerializer<T>
    where
        T: serde::Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match &self.0 {
                Some((value, _)) => value.serialize(serializer),
                None => serializer.serialize_none(),
            }
        }
    }
    OptionFirstElementSerializer(value.clone()).serialize(serializer)
}

pub fn serialize_option_i32<S>(value: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(val) => serializer.serialize_i32(*val),
        None => serializer.serialize_i32(0),
    }
}

pub mod serde_signature {
    use serde::Deserialize as _;

    pub fn serialize<S>(signature: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{signature:#08x}",))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(s) = s.strip_prefix("0x") {
            u32::from_str_radix(s, 16).map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("Invalid signature format"))
        }
    }
}
