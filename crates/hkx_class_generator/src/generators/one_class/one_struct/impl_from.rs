use crate::generators::{
    aliases::FieldMap,
    lifetime_manager::get_lifetime_from_fields,
    utils::{is_copyable, trim_primitive},
};
use convert_case::Casing;

/// Generate code to define the conversion for `Visitor` <-> `Struct`.
pub fn generate_impl_from(rust_struct_name: &str, fields: &FieldMap) -> String {
    let mut return_rust_code = String::new();

    let fields_len = fields.len();
    let lifetime = get_lifetime_from_fields(fields);

    let mut fields_values_code = String::new();
    let mut enum_match_inner = String::new();
    let mut visitor_to_struct_fields = String::new();
    let mut struct_ref_to_visitor_fields = String::new();

    for (_, (struct_field, field_type)) in fields {
        let enum_field = struct_field.to_case(convert_case::Case::Pascal);
        let enum_field = match enum_field.chars().next().map_or(false, |c| c.is_numeric()) {
            true => format!("_{enum_field}"),
            false => enum_field,
        };

        fields_values_code.push_str(&format!(
            r#"            let mut {struct_field} = None;
"#,
        ));

        enum_match_inner.push_str(&format!(
        r#"                {rust_struct_name}Visitor::{enum_field}(m) => {struct_field} = Some(m),
"#
    ));

        let cast_method = if field_type.starts_with("Primitive") {
            ".into_inner()"
        } else {
            ""
        };
        visitor_to_struct_fields.push_str(&format!(
            r#"            {struct_field}: {struct_field}.unwrap_or_default(){cast_method},
"#
        ));

        let cast_method = if field_type.starts_with("Primitive") {
            if is_copyable(trim_primitive(field_type)) {
                ".into()"
            } else {
                ".clone().into()"
            }
        } else {
            ".clone()"
        };
        struct_ref_to_visitor_fields.push_str(&format!(
            r#"            {rust_struct_name}Visitor::{enum_field}(data.{struct_field}{cast_method}),
"#
        ));
    }

    // serde only supports up to `[T; 32]`, so use `Vec` if it is larger than that.
    let visitor_array_type = if fields_len > 0 {
        format!("Vec<{rust_struct_name}Visitor{lifetime}>")
    } else {
        format!("[{rust_struct_name}Visitor{lifetime}; {fields_len}]")
    };
    let vec_macro = if fields_len > 0 { "vec!" } else { "" };
    // Visitor to Struct Code
    return_rust_code.push_str(&format!(
        r#"
impl{lifetime} From<{visitor_array_type}> for {rust_struct_name}{lifetime} {{
    fn from(_values: {visitor_array_type}) -> Self {{
{fields_values_code}

        for _value in _values {{
            match _value {{
{enum_match_inner}
            }}
        }}

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {{
{visitor_to_struct_fields}
        }}
    }}
}}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl{lifetime} From<&{rust_struct_name}{lifetime}> for {visitor_array_type} {{
    fn from(data: &{rust_struct_name}{lifetime}) -> Self {{
        {vec_macro}[
{struct_ref_to_visitor_fields}
        ]
    }}
}}
"#
    ));

    return_rust_code
}
