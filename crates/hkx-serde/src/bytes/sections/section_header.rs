//! The 48bytes each HKX section header contains metadata information about the HKX file.
//!
//! This information is placed immediately after the Hkx header. (In some cases, padding is inserted in between.)
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
#[derive(Clone, Default, Eq, PartialEq, Hash, FromBytes, AsBytes, FromZeroes)]
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

impl<O: ByteOrder> SectionHeader<O> {
    /// Interprets the given `bytes` as a `&Self` without copying.
    ///
    /// If `bytes.len() != size_of::<Self>()` or `bytes` is not aligned to
    /// `align_of::<Self>()`, this returns `Result::Err`.
    #[inline]
    pub fn ref_from_bytes(bytes: &[u8]) -> Result<&Self> {
        if bytes.len() < core::mem::size_of::<Self>() {
            return Err(SectionHeaderError::InsufficientLength);
        }
        let ref_header = Self::ref_from(bytes).ok_or(SectionHeaderError::UnAlignment)?;

        // Separator must set `0xFF`.
        let separator = ref_header.section_tag_separator;
        if separator != 0xFF {
            return Err(SectionHeaderError::InvalidSeparatorByte(separator));
        };

        Ok(ref_header)
    }
}

// Must be 48bytes.
static_assertions::assert_eq_size!(SectionHeader<LittleEndian>, [u8; 48]);

// Manually implement Debug to improve visualization of hex dump.
impl<O: ByteOrder> core::fmt::Debug for SectionHeader<O> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            local_fixups_offset,
            global_fixups_offset,
            virtual_fixups_offset,
            exports_offset,
            imports_offset,
            end_offset,
            ..
        } = *self;

        let abs_hex = &format!("{:#02X}", self.absolute_data_start);
        let local_hex = &format!("{:#02X}", local_fixups_offset);
        let global_hex = &format!("{:#02X}", global_fixups_offset);
        let virtual_hex = &format!("{:#02X}", virtual_fixups_offset);

        let l_offset = self.absolute_data_start + self.local_fixups_offset;
        let g_offset = self.absolute_data_start + self.global_fixups_offset;
        let v_offset = self.absolute_data_start + self.virtual_fixups_offset;
        let e_offset = self.absolute_data_start + self.exports_offset;
        let i_offset = self.absolute_data_start + self.imports_offset;
        let end_off = self.absolute_data_start + self.end_offset;

        f.debug_struct("SectionHeader")
            .field("section_tag", &self.section_tag)
            .field("section_tag_separator", &self.section_tag_separator)
            .field("absolute_data_start", abs_hex)
            .field("local_fixups_offset", local_hex)
            .field("global_fixups_offset", global_hex)
            .field("virtual_fixups_offset", virtual_hex)
            .field("exports_offset", &format!("{:#02X}", exports_offset))
            .field("imports_offset", &format!("{:#02X}", imports_offset))
            .field("end_offset", &format!("{:#02X}", end_offset))
            //
            // Include calculated offsets for convenience
            .field("abs_plus_local", &format!("{:#02X}", l_offset))
            .field("abs_plus_global", &format!("{:#02X}", g_offset))
            .field("abs_plus_virtual", &format!("{:#02X}", v_offset))
            .field("abs_plus_exports", &format!("{:#02X}", e_offset))
            .field("abs_plus_imports", &format!("{:#02X}", i_offset))
            .field("abs_plus_end", &format!("{:#02X}", end_off)) // Include abs + end info
            .finish()
    }
}

/// Result for [`SectionHeader`]
type Result<T, E = SectionHeaderError> = core::result::Result<T, E>;

/// HKX Section header Error
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SectionHeaderError {
    /// Binary data is interpreted as a section header, but it was less than 48bytes.
    #[error("Binary data is interpreted as a section header, but it was less than 48bytes.")]
    InsufficientLength,

    /// The next byte after section_tag (e.g. `__classnames`) in section header should be `0xFF`, but got `{0:#2X}` came.
    #[error("The next byte after section_tag (e.g. `__classnames`) in section header should be `0xFF`, but got `{0:#2X}`.")]
    InvalidSeparatorByte(u8),

    /// Binary data is interpreted as a section header, but has an alignment violation.
    #[error("Binary data is interpreted as a section header, but has an alignment violation.")]
    UnAlignment,
}
