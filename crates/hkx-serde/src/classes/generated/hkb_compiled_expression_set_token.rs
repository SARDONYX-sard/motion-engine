//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCompiledExpressionSetToken`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::havok_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// `hkbCompiledExpressionSetToken`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `None`/`0x0`
/// - signature: `0xc6aaccc8`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCompiledExpressionSetToken {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum TokenType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(Primitive<TokenType>),
    /// # C++ Class Fields Info
    /// -   name:`"operator"`
    /// -   type: `enum Operator`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "operator")]
    Operator(Primitive<Operator>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCompiledExpressionSetToken, "@name",
    ("data" => Data(Primitive<f32>)),
    ("type" => Type(Primitive<TokenType>)),
    ("operator" => Operator(Primitive<Operator>)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "TOKEN_TYPE_NONE")]
    TokenTypeNone = 0,
    #[serde(rename = "TOKEN_TYPE_OPERATOR")]
    TokenTypeOperator = 1,
    #[serde(rename = "TOKEN_TYPE_NUMBER")]
    TokenTypeNumber = 2,
    #[serde(rename = "TOKEN_TYPE_VARIABLE_INDEX")]
    TokenTypeVariableIndex = 3,
    #[serde(rename = "TOKEN_TYPE_OPENING_PAREN")]
    TokenTypeOpeningParen = 4,
    #[serde(rename = "TOKEN_TYPE_CLOSING_PAREN")]
    TokenTypeClosingParen = 5,
    #[serde(rename = "TOKEN_TYPE_COMMA")]
    TokenTypeComma = 6,
    #[serde(rename = "TOKEN_TYPE_CHARACTER_PROPERTY_INDEX")]
    TokenTypeCharacterPropertyIndex = 7,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "OP_NOP")]
    OpNop = 0,
    #[serde(rename = "OP_RAND01")]
    OpRand01 = 1,
    #[serde(rename = "OP_LOGICAL_NOT")]
    OpLogicalNot = 2,
    #[serde(rename = "OP_UNARY_MINUS")]
    OpUnaryMinus = 3,
    #[serde(rename = "OP_UNARY_PLUS")]
    OpUnaryPlus = 4,
    #[serde(rename = "OP_SIN")]
    OpSin = 5,
    #[serde(rename = "OP_COS")]
    OpCos = 6,
    #[serde(rename = "OP_ASIN")]
    OpAsin = 7,
    #[serde(rename = "OP_ACOS")]
    OpAcos = 8,
    #[serde(rename = "OP_SQRT")]
    OpSqrt = 9,
    #[serde(rename = "OP_FABS")]
    OpFabs = 10,
    #[serde(rename = "OP_CEIL")]
    OpCeil = 11,
    #[serde(rename = "OP_FLOOR")]
    OpFloor = 12,
    #[serde(rename = "OP_SQRTINV")]
    OpSqrtinv = 13,
    #[serde(rename = "OP_MUL")]
    OpMul = 14,
    #[serde(rename = "OP_DIV")]
    OpDiv = 15,
    #[serde(rename = "OP_ADD")]
    OpAdd = 16,
    #[serde(rename = "OP_SUB")]
    OpSub = 17,
    #[serde(rename = "OP_LOGICAL_OR")]
    OpLogicalOr = 18,
    #[serde(rename = "OP_LOGICAL_AND")]
    OpLogicalAnd = 19,
    #[serde(rename = "OP_EQ")]
    OpEq = 20,
    #[serde(rename = "OP_NEQ")]
    OpNeq = 21,
    #[serde(rename = "OP_LT")]
    OpLt = 22,
    #[serde(rename = "OP_GT")]
    OpGt = 23,
    #[serde(rename = "OP_LEQ")]
    OpLeq = 24,
    #[serde(rename = "OP_GEQ")]
    OpGeq = 25,
    #[serde(rename = "OP_POW")]
    OpPow = 26,
    #[serde(rename = "OP_MAX2")]
    OpMax2 = 27,
    #[serde(rename = "OP_MIN2")]
    OpMin2 = 28,
    #[serde(rename = "OP_RANDRANGE")]
    OpRandrange = 29,
    #[serde(rename = "OP_ATAN2APPROX")]
    OpAtan2Approx = 30,
    #[serde(rename = "OP_CLAMP")]
    OpClamp = 31,
    #[serde(rename = "OP_MOD")]
    OpMod = 32,
    #[serde(rename = "OP_DEG2RAD")]
    OpDeg2Rad = 33,
    #[serde(rename = "OP_RAD2DEG")]
    OpRad2Deg = 34,
    #[serde(rename = "OP_COSD")]
    OpCosd = 35,
    #[serde(rename = "OP_SIND")]
    OpSind = 36,
    #[serde(rename = "OP_ACOSD")]
    OpAcosd = 37,
    #[serde(rename = "OP_ASIND")]
    OpAsind = 38,
    #[serde(rename = "OP_ATAN2APPROXD")]
    OpAtan2Approxd = 39,
    #[serde(rename = "OP_SIGN")]
    OpSign = 40,
    #[serde(rename = "OP_LERP")]
    OpLerp = 41,
    #[serde(rename = "OP_CLERP")]
    OpClerp = 42,
    #[serde(rename = "OP_COND")]
    OpCond = 43,
}
