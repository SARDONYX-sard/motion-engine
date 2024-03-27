use crate::bytes::{
    hkx_header::HkxHeaderError, sections::class_name_section::ClassNamesSectionHeaderError,
};

pub type Result<T, E = HkxError> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum HkxError {
    #[error("Failed to parse {actual} as {expected}.")]
    ParseError { expected: String, actual: String },

    #[error(transparent)]
    ClassNamesSectionHeaderError(#[from] ClassNamesSectionHeaderError),

    #[error(transparent)]
    HkxHeaderError(#[from] HkxHeaderError),

    #[error("Unknown havok class {0}")]
    UnknownHavokClass(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
