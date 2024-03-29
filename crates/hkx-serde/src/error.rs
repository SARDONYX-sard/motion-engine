use crate::bytes::hkx_header::HkxHeaderError;
use crate::bytes::sections::class_name_section::ClassNamesSectionHeaderError;
use crate::bytes::sections::section_header::SectionHeaderError;

/// All Error [`Result`] of hkx_serde crate
pub type Result<T, E = HkxError> = core::result::Result<T, E>;

/// All Error of hkx_serde crate
#[derive(Debug, thiserror::Error)]
pub enum HkxError {
    /// Failed to parse {actual} as {expected}.
    #[error("Failed to parse {actual} as {expected}.")]
    ParseError { expected: String, actual: String },

    /// Unknown havok class {0}
    #[error("Unknown havok class {0}")]
    UnknownHavokClass(String),

    #[error(transparent)]
    HkxHeaderError(#[from] HkxHeaderError),

    #[error(transparent)]
    SectionHeaderError(#[from] SectionHeaderError),

    #[error(transparent)]
    ClassNamesSectionHeaderError(#[from] ClassNamesSectionHeaderError),

    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
