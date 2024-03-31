pub fn generate_impl_serde(rust_struct_name: &str, life_time: &str, fields_len: usize) -> String {
    let mut rust_code = String::new();

    let de_life_time = life_time.replace("<'a>", "<'de>");
    let ser_life_time = life_time.replace("<'a>", "<'_>");

    let struct_ser_with_life_time = format!("{rust_struct_name}{ser_life_time}");
    let struct_de_with_life_time = format!("{rust_struct_name}{de_life_time}");

    let visitor_ser_with_life_time = format!("{rust_struct_name}Visitor{ser_life_time}");
    let visitor_de_with_life_time = format!("{rust_struct_name}Visitor{de_life_time}");

    // serde only supports up to `[T; 32]`, so use `Vec` if it is larger than that.
    let mut serde_note = "";
    let mut visitor_array_ser_type = format!("[{rust_struct_name}Visitor; {fields_len}]");
    let mut visitor_array_de_type = format!("[{rust_struct_name}Visitor; {fields_len}]");
    if fields_len > 0 {
        serde_note =  "\n        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`." ;
        visitor_array_ser_type = format!("Vec<{visitor_ser_with_life_time}>");
        visitor_array_de_type = format!("Vec<{visitor_de_with_life_time}>");
    };

    rust_code.push_str(&format!(
        r#"
impl Serialize for {struct_ser_with_life_time} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{{serde_note}
        let visitor: {visitor_array_ser_type} = self.into();
        visitor.serialize(serializer)
    }}
}}

impl<'de> Deserialize<'de> for {struct_de_with_life_time} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{{serde_note}
        let de = <{visitor_array_de_type}>::deserialize(deserializer)?;
        Ok(de.into())
    }}
}}
"#
    ));

    rust_code
}
