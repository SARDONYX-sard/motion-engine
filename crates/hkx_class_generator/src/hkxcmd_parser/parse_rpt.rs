//! - Rpt(Report file) Format
//!
//! ref: [XML EBNF Syntax](https://www.w3.org/TR/REC-xml/#sec-notation)
//!
//! ```ebnf
//! rpt_file ::= signature_line vtable_line name_line parent_line size_line interface_line enums_line members_line version_line
//!
//! signature_line ::= "Signature:" HEX_VALUE NEWLINE
//! vtable_line ::= "VTable:" BOOL_VALUE NEWLINE
//! name_line ::= "Name:" NAME NEWLINE
//! parent_line ::= "Parent:" ( PARENT_NAME "(" HEX_VALUE ")" | "HK_NULL" ) NEWLINE
//! size_line ::= "Size:" DECIMAL_VALUE NEWLINE
//! interface_line ::= "#IFace:" DECIMAL_VALUE NEWLINE
//! enums_line ::= "#Enums:" DECIMAL_VALUE NEWLINE enum_def*
//! enum_def ::= DECIMAL_VALUE ENUM_NAME ENUM_VALUE_LIST ENUM_FLAGS NEWLINE
//! members_line ::= "#Members:" DECIMAL_VALUE NEWLINE member_def*
//! member_def ::= DECIMAL_VALUE MEMBER_NAME MEMBER_CLASSREF MEMBER_ENUM_REF MEMBER_TYPENAME HEX_VALUE HEX_VALUE DECIMAL_VALUE FLAG_VALUES DECIMAL_VALUE MEMBER_DEFAULT NEWLINE
//! version_line ::= "Version:" DECIMAL_VALUE NEWLINE
//!
//! HEX_VALUE ::= [0-9a-fA-F]+
//! DECIMAL_VALUE ::= [0-9]+
//! BOOL_VALUE ::= "TRUE" | "FALSE"
//! NAME ::= [^\n]+
//! PARENT_NAME ::= [^\s]+
//! ENUM_NAME ::= [^\s]+
//! ENUM_VALUE_LIST ::= "(" (ENUM_PAIR ";")* ENUM_PAIR ")"
//! ENUM_PAIR ::= ENUM_TAG "=" DECIMAL_VALUE
//! ENUM_TAG ::= [^\=]+
//! ENUM_FLAGS ::= "{" DECIMAL_VALUE "}"
//!
//! MEMBER_NAME ::= [^\,]+
//! MEMBER_CLASSREF ::= [^\,]+
//! MEMBER_ENUM_REF ::= [^\,]+
//! MEMBER_TYPENAME ::= [^\,]+
//! FLAG_VALUES ::= HEX_VALUE
//! MEMBER_DEFAULT ::= DECIMAL_VALUE | "HK_NULL"
//!
//! NEWLINE ::= '\n'
//! ```
use super::{flag_values::FlagValues, hk_types::Type, serde_helper::*};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, hex_digit1, multispace0, space0, space1},
    combinator::{map, map_res, opt, recognize},
    error::context,
    sequence::tuple,
};
use num_traits::Num;
use std::borrow::Cow;

/// Verbose Error Message
type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

/// C++ class information from `hkxcmd Report`.
///
/// ref: https://github.com/figment/hkxcmd/blob/dc4c75bf44303d874cc2656f56f107527f79ac29/Addins/Report.cpp#L144
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClassInfo {
    /// Class name(e.g. `"BGSGamebryoSequenceGenerator"`)
    pub name: String,

    /// Havok engine revision version(Maybe)
    pub version: u32,

    #[serde(with = "serde_signature")]
    /// Class signature
    pub signature: u32,

    /// Class size for x86
    pub size_x86: u32,

    /// Class size for x86_64
    pub size_x86_64: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_first_element_of_option_tuple")]
    /// Super class name & signature
    pub parent: Option<(String, u32)>,

    /// Does the parent class in `class_ref` contain a `CString` or `StringPtr`, or `struct containing them` type?
    ///
    /// This information is needed for the lifetime annotation (life of the reference) calculation.
    pub parent_has_string: bool,

    /// Is virtual table C++ class?
    pub vtable: bool,

    /// Does the class in `class_ref` contain a `CString` or `StringPtr`, or `struct containing them` type?
    ///
    /// This information is needed for the lifetime annotation (life of the reference) calculation.
    pub has_string: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Vector of enum names & enum fields
    pub enums: Vec<Enum>,

    /// C++ Class member Information
    pub members: Vec<MemberInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Enum {
    /// enum identifier
    pub name: String,

    #[serde(rename = "vtype")]
    /// Havok Type enumeration (Rough category of `Self::type_name`.)
    pub hk_type: Type,
    #[serde(rename = "vsubtype")]
    /// Type in generics when arrays, etc. come in.
    pub sub_type: Type,

    /// Unknown flags . Always `00000000` in hk2010 version.
    pub flags: String,

    pub enum_item: Vec<EnumItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnumItem {
    pub name: String,
    pub value: i128,
}

/// C++ Class member Information
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberInfo {
    /// Member(Field) name
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Used class name
    pub class_ref: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Used enum name
    pub enum_ref: Option<String>,

    /// When type or subtype is `struct`, does it contain a `CString` or `StringPtr`, or "struct containing them" type?
    ///
    /// This information is needed for the lifetime annotation (life of the reference) calculation.
    pub has_string: bool,

    #[serde(rename = "ctype")]
    /// C++ Type
    pub type_name: String,

    #[serde(rename = "vtype")]
    /// Havok Type enumeration (Rough category of `Self::type_name`.)
    pub hk_type: Type,

    #[serde(rename = "vsubtype")]
    /// Type in generics when arrays, etc. come in.
    pub sub_type: Type,

    /// Member offset for x86
    pub offset_x86: u32,
    /// Member offset for x86_64
    pub offset_x86_64: u32,

    /// Havok Type size
    pub type_size_x86: u32,
    /// Havok Type size
    pub type_size_x86_64: u32,

    #[serde(rename = "arrsize")]
    /// If an array is used, its size .
    pub c_style_array_size: usize,

    /// Flags for field alignment needs, skipping serialization, etc.
    pub flags: FlagValues,

    #[serde(rename = "default", default)]
    #[serde(serialize_with = "serialize_option_i32")]
    /// default member value
    pub default_value: Option<i32>,
}

impl MemberInfo {
    /// Byte size that must be read. Used to calculate the padding of the structure representing the Havok class.
    ///
    /// # Panics
    /// If entered `Type::Void | Type::Zero | Type::FnPtr | Type::InplaceArray | Type::HomogeneousArray | Type::RelArray | Type::Max`.
    ///
    /// These are not used inside the 2010 Havok Class.
    ///
    /// `Type::Array` > 8: Unsupported
    ///
    /// `Type::Struct`: This cannot be calculated here. It is calculated externally using `HashMap` or similar.
    pub fn type_size(&self, hk_type: &Type, ptr_size: u32) -> u32 {
        match hk_type {
            Type::Bool => 1,
            Type::Char => 1,
            Type::Int8 => 1,
            Type::Uint8 => 1,
            Type::Int16 => 2,
            Type::Uint16 => 2,
            Type::Int32 => 4,
            Type::Uint32 => 4,
            Type::Int64 => 8,
            Type::Uint64 => 8,
            Type::Real => 4,

            Type::Vector4 => 16,
            Type::Quaternion => 16,  // Vector3<f32>(12) + Scalar<f32>(4) = 16
            Type::QsTransform => 48, // Vector4<f32>(16) + Quaternion<f32>(16) + Vector4<f32>(16)
            Type::Rotation => 48,    // Vector4<f32>(12) * 3
            Type::Matrix3 => 48,     // Vector4<f32> * 3
            Type::Matrix4 => 64,     // Vector4<f32>(16) * 4
            Type::Transform => 64,   // Matrix3<f32>(48) + Vector4<f32>(16)

            Type::Array => match ptr_size {
                4 => 12,
                8 => 16,
                _ => panic!("Supported array ptr size is 4 or 8 bytes"),
            },
            Type::Struct => todo!(),
            Type::SimpleArray => match ptr_size {
                4 => 8,
                8 => 12,
                _ => panic!("Supported SimpleArray ptr size is 4 or 8 bytes"),
            },
            Type::Enum | Type::Flags => self.type_size(&self.sub_type, ptr_size),
            Type::Half => 2,

            Type::Pointer | Type::Ulong | Type::CString | Type::StringPtr => ptr_size,
            Type::Variant => ptr_size * 2,

            Type::Void
            | Type::Zero
            | Type::FnPtr
            | Type::InplaceArray
            | Type::HomogeneousArray
            | Type::RelArray
            | Type::Max => unimplemented!("Unsupported {:?}", self.sub_type),
        }
    }

    /// Returns the alignment size of the type.
    ///
    /// This returns information about the internal maximum size needed for offset calculations and tail alignment of structures.
    ///
    /// # Note
    /// Most types are the same as the size type, but composite types are the size of the inner type.
    ///
    /// - You might think that `Matrix3` and `Vector4` would be 4 bytes because they use f32 internally,
    ///   but in fact they are 16 bytes aligned for SIMD and alignment. `Vector4<f32>` -> 16
    /// - `Variant` -> ptr size
    ///
    /// # Panics
    /// If entered `Type::Void | Type::Zero | Type::FnPtr | Type::InplaceArray | Type::HomogeneousArray | Type::RelArray | Type::Max`.
    ///
    /// These are not used inside the 2010 Havok Class.
    ///
    /// `Type::Struct`: This cannot be calculated here. It is calculated externally using `HashMap` or similar.
    pub fn size_of_align(&self, hk_type: &Type, ptr_size: u32) -> u32 {
        match hk_type {
            Type::Bool => 1,
            Type::Char => 1,
            Type::Int8 => 1,
            Type::Uint8 => 1,
            Type::Int16 => 2,
            Type::Uint16 => 2,
            Type::Int32 => 4,
            Type::Uint32 => 4,
            Type::Int64 => 8,
            Type::Uint64 => 8,
            Type::Real => 4,

            // Normally, f32 => 4bytes is considered, but in the C++ definition, `align(16)` exists in Vector4, so it needs to be aligned at 16 bytes.
            Type::Vector4
            | Type::Quaternion
            | Type::QsTransform
            | Type::Rotation
            | Type::Matrix3
            | Type::Matrix4
            | Type::Transform => 16,

            Type::Array | Type::SimpleArray => match ptr_size > 4 {
                true => ptr_size, // T* m_data size
                false => 4,       // flag is int
            },
            Type::Struct => panic!("Unsupported Struct."),
            Type::Enum | Type::Flags => self.type_size(&self.sub_type, ptr_size),
            Type::Half => 2,

            Type::Pointer | Type::Ulong | Type::CString | Type::StringPtr | Type::Variant => {
                ptr_size
            }

            Type::Void
            | Type::Zero
            | Type::FnPtr
            | Type::InplaceArray
            | Type::HomogeneousArray
            | Type::RelArray
            | Type::Max => unimplemented!("Unsupported {:?}", self.sub_type),
        }
    }
}

/// Parser that parses strings in rpt files obtained by `hkxcmd Report` and obtains C++ living information.
pub fn parse_class(input: &str) -> IResult<&str, ClassInfo> {
    let (input, signature) = parse_value_number("Signature:", 16)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, vtable) = parse_vtable(input)?;
    let (input, _) = multispace0(input)?;

    let (input, name) = parse_map("Name:")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, parent) = parse_parent(input)?;
    let (input, _) = multispace0(input)?;

    let (input, size) = parse_value_number("Size:", 10)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, interface_count) = parse_value_number("#IFace:", 10)(input)?;
    let (input, _) = multispace0(input)?;

    let mut _interfaces = Vec::new();
    let mut input_outer = input;
    if interface_count > 0 {
        let (input, parsed_interfaces) =
            parse_lines(interface_count, parse_interface)(input_outer)?;
        let (input, _) = multispace0(input)?;
        _interfaces = parsed_interfaces;
        input_outer = input;
    }
    let input = input_outer;

    // Parse enum lines
    let (input, enum_count) = parse_value_number("#Enums:", 10)(input)?;
    let (input, _) = multispace0(input)?;

    let mut enums = Vec::new();
    let mut input_outer = input;
    if enum_count > 0 {
        let (input, parsed_enums) = parse_lines(enum_count, parse_enum)(input_outer)?;
        let (input, _) = multispace0(input)?;
        for (enum_name, enum_item, enum_flags) in parsed_enums {
            enums.push(Enum {
                name: enum_name.into(),
                // NOTE:
                // The ENUM type cannot be determined here because it is found in MEMBER.
                // Therefore, the `Enum &`Void` type, is temporarily inserted
                hk_type: Type::Enum,
                sub_type: Type::Void,
                enum_item,
                flags: enum_flags.into(),
            });
        }
        input_outer = input
    }
    let input = input_outer;

    // Parse class members
    let (input, member_count) = parse_value_number("#Members:", 10)(input)?;
    let (input, _) = multispace0(input)?;

    let mut members = Vec::new();
    let mut input_outer = input;
    if member_count > 0 {
        let (input, parsed_members) = parse_lines(member_count, parse_member)(input_outer)?;
        let (input, _) = multispace0(input)?;
        members = parsed_members;
        input_outer = input;
    }
    let input = input_outer;

    let (input, version) = parse_value_number("Version:", 10)(input)?;

    Ok((
        input,
        ClassInfo {
            signature,
            vtable,
            name: name.into(),
            parent: parent.map(|(s, i)| (s.into(), i)),
            parent_has_string: false,
            size_x86: size,
            size_x86_64: 0,
            has_string: false,
            enums,
            members,
            version,
        },
    ))
}

/// parse value with key
///
/// # Examples
/// Signature: c8df2d7 => c8df2d7
fn parse_map<'a>(key: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input| {
        let (input, _) = tag(key)(input)?;
        let (input, _) = space1(input)?; // remove tab
        let (input, value) = take_while1(|c| c != '\n')(input)?;
        Ok((input, value))
    }
}

/// Parse non prefix
fn parse_integer<T: Num>(input: &str, radix: u32) -> IResult<&str, T> {
    let parse_int = |src| T::from_str_radix(src, radix);
    map_res(recognize(tuple((opt(char('-')), hex_digit1))), parse_int)(input)
}

fn parse_usize_hex(input: &str) -> IResult<&str, usize> {
    parse_integer::<usize>(input, 16)
}

fn parse_value_number<'a>(key: &'a str, radix: u32) -> impl Fn(&'a str) -> IResult<&'a str, u32> {
    move |input| {
        let (input, value) = parse_map(key)(input)?;
        let (_, index) = parse_integer(value, radix)?;
        Ok((input, index))
    }
}

fn parse_lines<'a, F, T>(count: u32, line_parser: F) -> impl Fn(&'a str) -> IResult<&'a str, Vec<T>>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    move |input: &str| {
        let mut vector = Vec::new();
        let mut rest_input = input;

        for _ in 0..count {
            let (input, line) = take_while1(|s| s != '\n')(rest_input)?;
            let (input, _) = char('\n')(input)?;
            rest_input = input;

            let (_, value) = line_parser(line)?;
            vector.push(value);
        }

        Ok((rest_input, vector))
    }
}

/// This class is virtual table class?
fn parse_vtable(input: &str) -> IResult<&str, bool> {
    let (input, _) = tag("VTable:")(input)?;
    let (input, _) = space1(input)?;
    let (input, vtable) = alt((map(tag("TRUE"), |_| true), map(tag("FALSE"), |_| false)))(input)?;
    let (input, _) = char('\n')(input)?;
    Ok((input, vtable))
}

/// Parse parent class and it's signature.
fn parse_parent(input: &str) -> IResult<&str, Option<(Cow<'_, str>, u32)>> {
    let (input, _) = tag("Parent:")(input)?;
    let (input, _) = space1(input)?;

    let (input, line) = take_while1(|c| c != '\n')(input)?;
    let parent = match line.contains("HK_NULL") {
        true => None,
        false => {
            let (input, parent) = take_while1(|c| !matches!(c, '(' | ' '))(line)?;
            let (input, _) = space0(input)?;
            let (input, _) = char('(')(input)?;
            let (input, parent_signature) = take_while1(|c| c != ')')(input)?;
            let (_, parent_signature) = parse_integer(parent_signature, 16)?;
            let _ = char(')')(input)?;
            Some((parent.into(), parent_signature))
        }
    };

    Ok((input, parent))
}

/// Only "HK_NULL" is supported now.
fn parse_interface(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;

    let (input, _iface_index) = digit1(input)?;
    let (input, _) = space1(input)?;

    let (input, field_name) = tag("HK_NULL")(input)?;

    Ok((input, field_name))
}

/// Parse enum one line
///
/// 000 BlendModeFunction (BMF_NONE=0;BMF_PERCENT=1;BMF_ONE_MINUS_PERCENT=2) {00000000}
fn parse_enum(input: &str) -> IResult<&str, (&str, Vec<EnumItem>, &str)> {
    let (input, _) = space0(input)?;

    // 000
    let (input, _enum_index) = digit1(input)?;
    let (input, _) = space1(input)?;

    // BlendModeFunction
    let (input, enum_name) = take_while1(|s| s != ' ')(input)?;
    let (input, _) = space0(input)?;

    // (BMF_NONE=0;BMF_PERCENT=1;BMF_ONE_MINUS_PERCENT=2)
    let (input, _) = char('(')(input)?;
    let mut enum_tags: Vec<EnumItem> = Vec::new();
    let mut input_outer = input;
    loop {
        // BMF_NONE=0
        let (input, enum_tag) = take_while1(|s| s != '=')(input_outer)?;
        let (input, _) = tag("=")(input)?;
        let (input, enum_value) = parse_integer(input, 10)?;
        let (input, have_next) = opt(tag(";"))(input)?;

        input_outer = input;
        enum_tags.push(EnumItem {
            name: enum_tag.into(),
            value: enum_value,
        });
        if have_next.is_some() {
            continue;
        } else {
            break;
        }
    }
    let input = input_outer;

    let (input, _) = char(')')(input)?;
    let (input, _) = space0(input)?;

    // hkxcmd report seems to indicate that nothing but 0 exists, so this information is discarded.
    // Regular expression tried to find the enum flags in the report: \{(?!0{8})[0-9]{8}\}
    // ref: https://github.com/figment/hkxcmd/blob/dc4c75bf44303d874cc2656f56f107527f79ac29/Addins/Report.cpp#L142
    //
    // {00000000}
    let (input, _) = char('{')(input)?;
    // let (input, _enum_flags) = digit1(input)?;
    // dbg!(_enum_flags);
    let (input, enum_flags) = tag("00000000")(input)?;
    let (input, _) = char('}')(input)?;

    Ok((input, (enum_name, enum_tags, enum_flags)))
}

/// ref: https://github.com/figment/hkxcmd/blob/dc4c75bf44303d874cc2656f56f107527f79ac29/Addins/Report.cpp#L144
fn parse_member(input: &str) -> IResult<&str, MemberInfo> {
    fn parse_type(input: &str) -> IResult<&str, Type> {
        context("Type", map_res(parse_usize_hex, Type::try_from))(input)
    }
    fn parse_flag(input: &str) -> IResult<&str, FlagValues> {
        context("FlagValue", map_res(parse_usize_hex, FlagValues::try_from))(input)
    }

    /// `HK_NULL` to [`Option::None`]
    fn parse_null_str<'a, T: From<&'a str>>(input: &'a str) -> IResult<&'a str, Option<T>> {
        fn null_to_option<'a, T: From<&'a str>>(input: &'a str) -> Option<T> {
            match input == "HK_NULL" {
                true => None,
                false => Some(input.into()),
            }
        }
        map(take_while1(|c| c != ','), null_to_option)(input)
    }

    let (input, _) = space0(input)?;

    // 1.If index is put in [`Vec`], this discard because it can be inferred by the index of [`Vec`].
    let (input, _member_index) = digit1(input)?;
    let (input, _) = space1(input)?;

    // 2.Name
    // https://github.com/figment/hkxcmd/blob/dc4c75bf44303d874cc2656f56f107527f79ac29/Addins/Report.cpp#L148
    let (input, field_name) = take_while1(|c| c != ',')(input)?;
    let (input, _) = char(',')(input)?;

    // 3.Class ref
    //https://github.com/figment/hkxcmd/blob/dc4c75bf44303d874cc2656f56f107527f79ac29/Addins/Report.cpp#L149
    let (input, class_ref) = parse_null_str(input)?;
    let (input, _) = char(',')(input)?;

    // 4.Enum ref
    // https://github.com/figment/hkxcmd/blob/dc4c75bf44303d874cc2656f56f107527f79ac29/Addins/Report.cpp#L152
    let (input, enum_ref) = parse_null_str(input)?;
    let (input, _) = char(',')(input)?;

    // 5. C++ Type
    let (input, type_name) = take_while1(|c| c != ',')(input)?;
    let (input, _) = char(',')(input)?;

    // 6. Havok Type
    let (input, hk_type) = parse_type(input)?;
    let (input, _) = char(',')(input)?;

    // 7. Type in generics when arrays, etc. come in.
    let (input, sub_type) = parse_type(input)?;
    let (input, _) = char(',')(input)?;

    // 8.
    let (input, cstyle_array_size) = parse_integer(input, 10)?;
    let (input, _) = char(',')(input)?;

    // 9. Serialize ignore flag, others
    let (input, flags) = parse_flag(input)?;
    let (input, _) = char(',')(input)?;

    // 10.
    let (input, offset) = parse_integer(input, 10)?;
    let (input, _) = char(',')(input)?;

    // 11. Unused
    let (input, _) = tag("HK_NULL")(input)?;
    let (input, _) = char(',')(input)?;

    // 12.
    let (input, default_value) = map(take_while1(|c| c != '\n'), |s| match s == "HK_NULL" {
        true => None,
        false => parse_integer(s, 10).ok().map(|n| n.1),
    })(input)?;

    let member = MemberInfo {
        name: field_name.into(),
        class_ref,
        enum_ref,
        has_string: false,
        type_name: type_name.replace("&lt;", "<").replace("&gt;", ">"),
        hk_type,
        sub_type,
        c_style_array_size: cstyle_array_size,
        flags,
        offset_x86: offset,
        offset_x86_64: 0,
        type_size_x86: 0,
        type_size_x86_64: 0,
        default_value,
    };

    Ok((input, member))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_hkx_cmd_report(input: &str) -> ClassInfo {
        match parse_class(input) {
            Ok((_, class_info)) => class_info,
            Err(e) => {
                let e = match e {
                    nom::Err::Incomplete(e) => panic!("{:?}", e),
                    nom::Err::Error(err) | nom::Err::Failure(err) => err,
                };
                panic!("Error: {}", nom::error::convert_error(input, e));
            }
        }
    }

    #[test]
    #[quick_tracing::init]
    fn should_parse_one_class() {
        let input = r#"Signature:    c8df2d77
VTable:    TRUE
Name:    BGSGamebryoSequenceGenerator
Parent:    hkbGenerator (0d68aefc)
Size:    72
#IFace:    0
#Enums:    1
 000    BlendModeFunction (BMF_NONE=0;BMF_PERCENT=1;BMF_ONE_MINUS_PERCENT=2) {00000000}
#Members:    7
 000    pSequence,HK_NULL,HK_NULL,char*,0000001d,00000000,0,00000000,40,HK_NULL,HK_NULL
 001    eBlendModeFunction,HK_NULL,BlendModeFunction,enum BlendModeFunction,00000018,00000003,0,00000000,44,HK_NULL,HK_NULL
 002    fPercent,HK_NULL,HK_NULL,hkReal,0000000b,00000000,0,00000000,48,HK_NULL,HK_NULL
 003    events,HK_NULL,HK_NULL,hkArray&lt;void&gt;,00000016,00000000,0,00000400,52,HK_NULL,HK_NULL
 004    fTime,HK_NULL,HK_NULL,hkReal,0000000b,00000000,0,00000400,64,HK_NULL,HK_NULL
 005    bDelayedActivate,HK_NULL,HK_NULL,hkBool,00000001,00000000,0,00000400,68,HK_NULL,HK_NULL
 006    bLooping,HK_NULL,HK_NULL,hkBool,00000001,00000000,0,00000400,69,HK_NULL,HK_NULL
Version:    2"#;

        tracing::debug!("{:#?}", parse_hkx_cmd_report(input));
    }

    #[test]
    #[quick_tracing::init(test = "should_parse_all_class", level = "Debug")]
    fn should_parse_all_class() {
        let rpt_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("hkxcmd_help")
            .join("rpt");

        for entry in jwalk::WalkDir::new(rpt_dir).into_iter() {
            let path = entry.unwrap().path();
            let path = path.as_path();
            if !path.is_file() && path.extension() != Some(std::ffi::OsStr::new("xml")) {
                continue;
            }

            // Exclude some problematic classes that aren't needed
            let file_name = path.file_stem().unwrap().to_str().unwrap();

            // Remove tailing version(e.g. _1)
            let file = file_name.rsplit('_').collect::<Vec<_>>();
            let rpt_file_name = *file.last().unwrap();

            let content = std::fs::read_to_string(path).unwrap();
            let class_info = parse_hkx_cmd_report(&content);
            let json = serde_json::to_string_pretty(&class_info).unwrap();

            let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("assets")
                .join("classes");
            std::fs::create_dir_all(&output_dir).unwrap();
            let mut output_file = output_dir.join(rpt_file_name);
            output_file.set_extension("json");

            std::fs::write(output_file, json).unwrap();
        }
    }
}

// Vector4, Quaternion, QsTransform, Rotation, Matrix3, Matrix4, Transform
