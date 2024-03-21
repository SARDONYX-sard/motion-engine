use byteorder::{ByteOrder, ReadBytesExt as _, WriteBytesExt as _};
use num_traits::Zero;
use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom, Write};

const SIGNATURE_SEPARATOR: u8 = 0x09;

#[derive(Debug, Clone, PartialEq)]
pub struct HkxClassName {
    pub class_name: String,
    pub signature: u32,
}

impl HkxClassName {
    /// Read one class info(signature & class name) from bytes reader.
    ///
    /// # Expected bytes format
    /// signature(4bytes) + separator(0x09) + ClassName(Null terminated ASCII string)
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        // LE Example
        // F6 5E 58 75
        let signature = br.read_u32::<B>()?;
        let separator = br.read_u8()?;
        if separator != SIGNATURE_SEPARATOR {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid separator byte: ClassName expected `0x09` after u32 signature (`0x{signature:x}`), but got `0x{separator:2x}`.")
            ));
        }

        // Read ASCII characters until null terminated(0x00) character.
        let mut class_name_bytes = Vec::new();
        loop {
            let byte = br.read_u8()?;
            if !byte.is_zero() {
                class_name_bytes.push(byte);
            } else {
                break;
            }
        }

        let class_name = String::from_utf8_lossy(&class_name_bytes).into();
        Ok(HkxClassName {
            class_name,
            signature,
        })
    }

    /// Write one class to bytes writer.
    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.signature)?;
        bw.write_u8(SIGNATURE_SEPARATOR)?; // Unused separator

        let c_string = std::ffi::CString::new(self.class_name.clone())?;
        //? Since ASCII strings are stored according to the order of character occurrence, endianness is not a concern.
        bw.write_all(c_string.as_bytes_with_nul())?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HKXClassNames {
    /// Same values of [`Self::offset_class_names_map`]
    pub class_names: Vec<HkxClassName>,
    /// - key: Start position of bytes where value was stored.
    /// - value: class name
    pub offset_class_names_map: HashMap<u32, HkxClassName>,
}

impl HKXClassNames {
    /// Read classes information from bytes reader.
    ///
    /// # Assumption
    /// The position of `Read` must be `absolute data offset`.
    ///
    /// That is, this method would normally be called after getting `absolute_data_offset` with `HkxSection::read` and Seek it.
    pub fn read<B: ByteOrder>(mut br: impl Read + Seek) -> std::io::Result<Self> {
        let mut class_names = Vec::new();
        let mut offset_class_names_map = HashMap::new();

        loop {
            let mut signature_buf = [0; 4];
            // The 0x09 serves as a separator,
            // and binaries such as `<class_of_signature>0x09` are assumed.
            if br.read_exact(&mut signature_buf).is_err() || br.read_u8()? != SIGNATURE_SEPARATOR {
                break;
            }

            // 4 bytes signature + 1 byte separator = 5 bytes Seek worked.
            // [`HkxClassName::read`] method read signature and Name, so we back to the byte position of the signature information.
            let string_start = br.stream_position()? as u32 - 5;
            br.seek(SeekFrom::Current(-5))?;

            let class_name = HkxClassName::read::<B>(&mut br)?;
            class_names.push(class_name.clone());
            offset_class_names_map.insert(string_start, class_name);
        }
        Ok(HKXClassNames {
            class_names,
            offset_class_names_map,
        })
    }
}
