pub fn generate_impl_bytes_deserialize(rust_class_name_with_life_time: &str) -> String {
    let mut rust_code = String::new();
    let rust_class_name_with_life_time = rust_class_name_with_life_time.replace("<'a>", "<'de>");

    rust_code.push_str(&format!(
        r#"
impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for {rust_class_name_with_life_time} {{
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {{
        todo!()
    }}
}}
"#
    ));

    rust_code
}
