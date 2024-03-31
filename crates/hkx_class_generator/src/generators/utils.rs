pub fn is_copyable(input: &str) -> bool {
    matches!(
        input,
        "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "usize"
            | "f32"
            | "f64"
            | "()"
            | "Vector4<f32>"
    )
}

pub fn trim_primitive(input: &str) -> &str {
    let start_index = input.find('<').unwrap() + 1;
    let end_index = input.rfind('>').unwrap();
    &input[start_index..end_index]
}
