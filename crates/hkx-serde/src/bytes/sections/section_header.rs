//! The 48bytes each HKX section header contains metadata information about the HKX file.
//!
//! For SkyrimSE, the bytes are arranged in the following order.
//! - `__classnames__` 48bytes
//! - `__types__` 48bytes
//! - `__data__` 48bytes
//!
//! # Note
//! This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
//!
//! Depending on the havok version, there may be padding after the section header group.
//! (at least not in SkyrimSE).
use zerocopy::{AsBytes, ByteOrder, FromBytes, FromZeroes, LittleEndian, U32};

/// The 48bytes each HKX section header contains metadata information about the HKX file.
///
/// For SkyrimSE, the bytes are arranged in the following order.
/// - `__classnames__` 48bytes
/// - `__types__` 48bytes
/// - `__data__` 48bytes
///
/// # Note
/// This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
///
/// Depending on the havok version, there may be padding after the section header group.
/// (at least not in SkyrimSE).
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, FromBytes, AsBytes, FromZeroes)]
#[repr(C, packed)]
pub struct SectionHeader<O: ByteOrder> {
    /// Section name.
    ///
    /// For SkyrimSE, the bytes are arranged in the following order.
    /// - `__classnames__`
    /// - `__types__`
    /// - `__data__`
    ///
    /// # Bytes Example
    /// ```rust
    /// assert_eq!(
    ///   *b"__classnames__\0\0\0\0\0",
    ///   [0x5F, 0x5F, 0x63, 0x6C, 0x61, 0x73, 0x73, 0x6E, 0x61, 0x6D, 0x65, 0x73, 0x5F, 0x5F, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00]
    /// );
    /// ```
    pub section_tag: [u8; 19],
    /// Always must be `0xFF`
    pub section_tag_separator: u8,
    /// Section start & fixup base offset.
    ///
    /// # Example of position
    /// `hkx_header.section_count:3` & `hkx_header.section_offset:0` => `0x000000D0` bytes.
    ///
    /// - Calculation formula
    ///
    ///   Hkx header 64bytes + 48bytes * 3 sections = 208bytes == `0xD0`
    pub absolute_data_start: U32<O>,
    /// Offset from absolute offset to local fixup map.
    pub local_fixups_offset: U32<O>,
    /// Offset from absolute offset to global fixup map.
    pub global_fixups_offset: U32<O>,
    /// Offset from absolute offset to virtual class fixup map.
    pub virtual_fixups_offset: U32<O>,
    pub exports_offset: U32<O>,
    pub imports_offset: U32<O>,
    pub end_offset: U32<O>,
}

// Must be 48bytes.
static_assertions::assert_eq_size!(SectionHeader<LittleEndian>, [u8; 48]);
