//! HKX `__classnames__` section binary Reader / Writer
use indexmap::IndexMap;
use std::ffi::{CStr, FromBytesUntilNulError};
use zerocopy::ByteOrder;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassPair<'a> {
    /// e.g. `0x2772c11e`
    pub signature: u32,
    /// e.g. `hkRootLevelContainer\0`
    pub class_name: &'a CStr,
}

impl<'a> ClassPair<'a> {
    /// Signature separator byte
    pub const SIGNATURE_SEPARATOR: u8 = 0x09;

    /// Read one class info(signature & class name) from bytes reader.
    ///
    /// # Expected bytes format
    /// signature(4bytes) + separator(0x09) + ClassName(Null terminated ASCII string)
    ///
    /// # Returns
    /// (Class name & signature, read bytes offset)
    pub fn from_slice<B: ByteOrder>(bytes: &'a [u8]) -> Result<(Self, usize)> {
        let mut offset = 0;

        // LE Example: F6 5E 58 75
        let signature = B::read_u32(&bytes[offset..offset + 4]);
        offset += 4;

        let separator = bytes[offset];
        offset += 1;

        if separator != Self::SIGNATURE_SEPARATOR {
            return Err(ClassNamesSectionHeaderError::InvalidSignatureSeparator {
                signature,
                wrong_separator: separator,
            });
        }

        let class_name = CStr::from_bytes_until_nul(&bytes[offset..])?;

        let class_name_len = class_name.to_str()?.len() + 1; // Include null terminator
        offset += class_name_len;

        let hkx_class_name = ClassPair {
            class_name,
            signature,
        };

        Ok((hkx_class_name, offset))
    }

    /// Write one class info(signature & class name) to bytes slice.
    ///
    /// # Return
    /// Written end offset.
    pub fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> Result<usize> {
        // Calculate need buffer size
        let class_name_bytes = self.class_name.to_bytes_with_nul();
        let class_name_len = class_name_bytes.len();
        // signature(4 bytes) + separator(1 bytes) + class name
        let class_info_bytes_len = 4 + 1 + class_name_len;

        if bytes.len() < 5 + class_info_bytes_len {
            return Err(ClassNamesSectionHeaderError::InsufficientWriteBufferSize);
        }

        B::write_u32(&mut bytes[0..4], self.signature);
        bytes[4] = Self::SIGNATURE_SEPARATOR;
        bytes[5..5 + class_name_len].copy_from_slice(class_name_bytes);

        Ok(class_info_bytes_len) // Return the written end offset
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HKXClassNames<'a> {
    /// - key: Start position of bytes where value was stored.
    /// - value: (signature, class name)
    pub offset_class_names_map: IndexMap<usize, ClassPair<'a>>,
}

impl<'a> HKXClassNames<'a> {
    /// For 16 bytes alignment
    const CLASSNAMES_END_PADDING: u8 = 0xFF;

    /// Read classes information(signature & className pair) from bytes reader.
    ///
    /// # Assumption
    /// The position of `Read` must be `absolute data start`.
    ///
    /// That is, this method would normally be called after getting `absolute_data_start` with `HkxSection::read` and Seek it.
    /// - `absolute_data_start`: signature & className pair bytes array
    pub fn from_slice<B: ByteOrder>(bytes: &'a [u8]) -> Result<Self> {
        let mut offset_class_names_map = IndexMap::new();
        let mut offset = 0;

        while bytes[offset + 4] != Self::CLASSNAMES_END_PADDING {
            let (class_pair, read_offset) = ClassPair::from_slice::<B>(&bytes[offset..])?;
            let str_start = offset + 4; // after 4bytes signature
            offset_class_names_map.insert(str_start, class_pair);
            offset += read_offset;

            if offset > bytes.len() {
                break;
            }
        }

        Ok(HKXClassNames {
            offset_class_names_map,
        })
    }

    pub fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> Result<()> {
        let mut offset = 0;

        for (_, class) in &self.offset_class_names_map {
            let next = class.write::<B>(&mut bytes[offset..])?;
            offset += next;
        }

        while (offset % 16) != 0 {
            bytes[offset] = Self::CLASSNAMES_END_PADDING;
            offset += 1;
        }

        Ok(())
    }

    /// Converts the offset-based HashMap into a Vec.
    pub fn as_vec_ref<'this: 'a>(&'this self) -> Vec<&'a ClassPair<'a>> {
        self.offset_class_names_map.values().collect()
    }
}

/// Hkx ClassNames section header Error Result
type Result<T, E = ClassNamesSectionHeaderError> = core::result::Result<T, E>;

/// Hkx ClassNames section header Error
#[derive(Debug, thiserror::Error)]
pub enum ClassNamesSectionHeaderError {
    /// Insufficient buffer size to write class info
    #[error("Insufficient buffer size to write class info")]
    InsufficientWriteBufferSize,

    /// Binary data is interpreted as a header, but it was less than 64bytes.
    #[error("Invalid separator byte: ClassName expected `0x09` after u32 signature (`0x{signature:x}`), but got `0x{wrong_separator:2x}`.")]
    InvalidSignatureSeparator { signature: u32, wrong_separator: u8 },

    /// Invalid str.
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),

    /// External io error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Cstr non terminated string.
    #[error(transparent)]
    FromBytesUntilNulError(#[from] FromBytesUntilNulError),
}
