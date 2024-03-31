//! HKX format bytes reader/writer
pub mod hkx_header;
pub mod packfile_deserializer;
pub mod sections;

/// Serde definition trait for HKX binaries for each class
pub use packfile_deserializer::{ByteDeSerialize, PackFileDeserializer};

/// External crates
pub use num_derive::{FromPrimitive, ToPrimitive}; // For enum byte read/write
pub use num_traits::{FromPrimitive, ToPrimitive}; // For enum byte read/write
pub use zerocopy::ByteOrder; // For primitive read/write
