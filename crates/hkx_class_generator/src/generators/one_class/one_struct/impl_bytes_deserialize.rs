pub fn generate_impl_bytes_deserialize(rust_class_name_with_life_time: &str) -> String {
    let mut rust_code = String::new();
    let rust_class_name_with_life_time = rust_class_name_with_life_time.replace("<'a>", "<'_>");

    rust_code.push_str(&format!(
        r#"
impl ByteDeSerialize for {rust_class_name_with_life_time} {{
    fn from_bytes<B>(
        _bytes: &[u8],
        _de: &mut packfile_deserializer::PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized,
    {{
        todo!()
    }}
}}
"#
    ));

    rust_code
}
