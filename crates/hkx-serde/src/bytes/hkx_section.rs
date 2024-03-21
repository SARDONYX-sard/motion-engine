use byteorder::{ByteOrder, ReadBytesExt as _, WriteBytesExt as _};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};

use super::fix_str_io::{read_fix_str, write_fix_str};

trait Fixup {
    /// Read bytes from byte reader.
    fn read<B: ByteOrder>(reader: impl Read) -> std::io::Result<Self>
    where
        Self: Sized;

    /// Write bytes to writer.
    fn write<B: ByteOrder>(&self, writer: impl Write) -> std::io::Result<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalFixup {
    pub src: u32,
    pub dst: u32,
}

impl Fixup for LocalFixup {
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let src = br.read_u32::<B>()?;
        let dst = br.read_u32::<B>()?;
        Ok(LocalFixup { src, dst })
    }

    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.src)?;
        bw.write_u32::<B>(self.dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalFixup {
    pub src: u32,
    pub dst_section_index: u32,
    pub dst: u32,
}

impl Fixup for GlobalFixup {
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let src = br.read_u32::<B>()?;
        let dst_section_index = br.read_u32::<B>()?;
        let dst = br.read_u32::<B>()?;
        Ok(GlobalFixup {
            src,
            dst_section_index,
            dst,
        })
    }

    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.src)?;
        bw.write_u32::<B>(self.dst_section_index)?;
        bw.write_u32::<B>(self.dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualFixup {
    pub src: u32,
    pub dst_section_index: u32,
    pub dst: u32,
}

impl Fixup for VirtualFixup {
    fn read<B: ByteOrder>(mut br: impl Read) -> std::io::Result<Self> {
        let src = br.read_u32::<B>()?;
        let dst_section_index = br.read_u32::<B>()?;
        let dst = br.read_u32::<B>()?;
        Ok(VirtualFixup {
            src,
            dst_section_index,
            dst,
        })
    }

    fn write<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        bw.write_u32::<B>(self.src)?;
        bw.write_u32::<B>(self.dst_section_index)?;
        bw.write_u32::<B>(self.dst)?;
        Ok(())
    }
}

/// There are three types of Hkx sections that we know of.
///
/// The bytes are arranged in the following order.
/// - `__classnames__`
/// - `__types__`
/// - `__data__`
#[derive(Debug, Clone, PartialEq)]
pub struct HkxSection {
    pub global_map: HashMap<u32, GlobalFixup>,
    pub local_map: HashMap<u32, LocalFixup>,
    pub virtual_map: HashMap<u32, VirtualFixup>,
    pub global_fixups: Vec<GlobalFixup>,
    pub local_fixups: Vec<LocalFixup>,
    pub virtual_fixups: Vec<VirtualFixup>,
    pub section_data: Vec<u8>,
    /// Start byte position of each offsets information.
    pub section_id: u32,
    /// `[u8;19]`(19bytes) section tag
    /// # Bytes Examples
    /// ```
    /// // 5F 5F 63 6C 61 73 73 6E 61 6D 65 73 5F 5F 00 00
    /// // 00 00 00
    /// // 14bytes + space 5bytes
    /// "__classnames__     "
    ///
    /// // Another pattern
    /// "__types__"
    /// "__data__"
    /// ```
    ///
    /// - This is followed by one `0xFF`. Probably means the end.
    pub section_tag: String,
    /// # Examples
    /// ```rust
    /// "hk_2010.2.0-r1"
    /// ```
    pub contents_version_string: String,
}

impl HkxSection {
    const OFFSET_MAP_IS_NONE: u32 = 0xFFFFFFFF;

    #[doc(alias = "Self::read")]
    /// Same [`Self::read`]
    pub fn new<B: ByteOrder>(
        br: impl Read + Seek,
        contents_version_string: &str,
    ) -> std::io::Result<Self> {
        Self::read_header::<B>(br, contents_version_string)
    }

    /// Read 48bytes a section header information(e.g. start `__classnames__`)
    /// and fix up information derived from that information..
    ///
    /// # Assumption
    /// [`Seek`] position must set after hkx header bytes.
    /// That is, this function is called after reading hkx header.
    pub fn read_header<B: ByteOrder>(
        mut br: impl Read + Seek,
        contents_version_string: &str,
    ) -> std::io::Result<Self> {
        // section tag 19bytes
        // # Bytes Examples(14bytes name + 5 spaces)
        // 5F 5F 63 6C 61 73 73 6E 61 6D 65 73 5F 5F 00 00
        // 00 00 00
        // "__classnames__     "
        let section_tag = read_fix_str(&mut br, 19)?;
        let separator = br.read_u8()?;
        if separator != 0xFF {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid separator byte: HkxSection expected `0xFF` after 19bytes([u8;19]) section tag(`{section_tag}`), but got `0x{separator:2x}`.")
            ));
        }

        // - offsets information 32bit(4bytes) * 7

        // The data in absolute data start is the position of the consecutive byte
        // sequence of data section(e.g. signature and class).
        //
        // # Example("__classnames__     " next 0xFF next "0D 00 00 00")
        // LittleEndian: 0D 00 00 00
        // This means that ClassNames information data is stored from `0000000D`.
        let absolute_data_start = br.read_u32::<B>()?;

        let local_fixups_offset = br.read_u32::<B>()?;
        let global_fixups_offset = br.read_u32::<B>()?;
        let virtual_fixups_offset = br.read_u32::<B>()?;
        let exports_offset = br.read_u32::<B>()?;
        let _imports_offset = br.read_u32::<B>()?;
        let _end_offset = br.read_u32::<B>()?;

        br.seek(SeekFrom::Start(absolute_data_start.into()))?;

        // Use `with_capacity` to avoid the cost of filling with 0
        let mut section_data = Vec::with_capacity((local_fixups_offset) as usize);
        br.read_exact(&mut section_data)?;

        let mut local_fixups = Vec::new();
        br.seek(SeekFrom::Start(
            (absolute_data_start + local_fixups_offset).into(),
        ))?;
        while br.stream_position()? < (absolute_data_start + global_fixups_offset).into() {
            if br.read_u32::<B>()? != Self::OFFSET_MAP_IS_NONE {
                local_fixups.push(LocalFixup::read::<B>(&mut br)?);
            }
        }

        let mut global_fixups = Vec::new();
        br.seek(SeekFrom::Start(
            (absolute_data_start + global_fixups_offset).into(),
        ))?;
        while br.stream_position()? < (absolute_data_start + virtual_fixups_offset).into() {
            if br.read_u32::<B>()? != Self::OFFSET_MAP_IS_NONE {
                global_fixups.push(GlobalFixup::read::<B>(&mut br)?);
            }
        }

        let mut virtual_fixups = Vec::new();
        br.seek(std::io::SeekFrom::Start(
            (absolute_data_start + virtual_fixups_offset).into(),
        ))?;
        while br.stream_position()? < (absolute_data_start + exports_offset).into() {
            if br.read_u32::<B>()? != Self::OFFSET_MAP_IS_NONE {
                virtual_fixups.push(VirtualFixup::read::<B>(&mut br)?);
            }
        }

        br.seek(std::io::SeekFrom::Start(absolute_data_start.into()))?;

        // Our review of the hkx byte sequence shows that there is no padding, at least in SkyrimSE.
        if contents_version_string != "hk_2010.2.0-r1" {
            br.read_u32::<B>()?; // Padding
            br.read_u32::<B>()?; // Padding
            br.read_u32::<B>()?; // Padding
            br.read_u32::<B>()?; // Padding
        }

        Ok(HkxSection {
            global_map: global_fixups
                .iter()
                .map(|fixup| (fixup.src, fixup.clone()))
                .collect(),
            local_map: local_fixups
                .iter()
                .map(|fixup| (fixup.src, fixup.clone()))
                .collect(),
            virtual_map: virtual_fixups
                .iter()
                .map(|fixup| (fixup.src, fixup.clone()))
                .collect(),
            global_fixups,
            local_fixups,
            virtual_fixups,
            section_data,
            section_id: absolute_data_start,
            section_tag,
            contents_version_string: contents_version_string.to_owned(),
        })
    }

    /// Write 48bytes a section header information(e.g. start `__classnames__`)
    ///
    /// # Note
    /// The fixup information performs byte allocation, but actual valid data writing is not performed by this method.
    ///
    /// # Assumption
    /// [`Seek`] position must set after hkx header bytes.
    /// That is, this function is called after writing hkx header.
    fn write_header<B: ByteOrder>(&self, mut bw: impl Write) -> std::io::Result<()> {
        write_fix_str(&mut bw, &self.section_tag, 19)?;
        bw.write_u8(0xFF)?; // Unused separator

        // 32bit(4bytes) * 7
        bw.write_u32::<B>(0)?; // Placeholder for absolute offset
        bw.write_u32::<B>(0)?; // Placeholder for local offset
        bw.write_u32::<B>(0)?; // Placeholder for global offset
        bw.write_u32::<B>(0)?; // Placeholder for virtual offset
        bw.write_u32::<B>(0)?; // Placeholder for exports offset
        bw.write_u32::<B>(0)?; // Placeholder for imports offset
        bw.write_u32::<B>(0)?; // Placeholder for end offset

        if self.contents_version_string != "hk_2010.2.0-r1" {
            bw.write_u32::<B>(0xFFFFFFFF)?;
            bw.write_u32::<B>(0xFFFFFFFF)?;
            bw.write_u32::<B>(0xFFFFFFFF)?;
            bw.write_u32::<B>(0xFFFFFFFF)?;
        }

        Ok(())
    }

    /// Assuming that the `Seek` position points to the `absolute_data_offset` (the byte u32 position where the section data
    /// actually is) for each section, write the byte position and fixup map from there.
    ///
    /// # Note
    /// The data(e.g. class names) is 16bytes aligned and is filled with `0xFF` and padded until it reaches 16bytes alignment.
    fn write_data<B: ByteOrder>(&self, mut bw: impl Write + Seek) -> std::io::Result<()> {
        let absolute_offset = bw.stream_position()? as u32;
        bw.write_all(&self.section_data)?;

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let local_offset = bw.stream_position()? as u32 - absolute_offset;
        for local_fixup in &self.local_fixups {
            local_fixup.write::<B>(&mut bw)?;
        }

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let global_offset = bw.stream_position()? as u32 - absolute_offset;
        for global_fixup in &self.global_fixups {
            global_fixup.write::<B>(&mut bw)?;
        }

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let virtual_offset = bw.stream_position()? as u32 - absolute_offset;
        for virtual_fixup in &self.virtual_fixups {
            virtual_fixup.write::<B>(&mut bw)?;
        }

        while bw.stream_position()? % 16 != 0 {
            bw.write_u8(0xFF)?;
        } // 16-byte alignment

        let exports_offset = bw.stream_position()? as u32;
        let imports_offset = exports_offset;
        let end_offset = exports_offset;

        bw.seek(SeekFrom::Start(self.section_id as u64))?;
        bw.write_u32::<B>(absolute_offset)?;
        bw.write_u32::<B>(local_offset)?;
        bw.write_u32::<B>(global_offset)?;
        bw.write_u32::<B>(virtual_offset)?;
        bw.write_u32::<B>(exports_offset)?;
        bw.write_u32::<B>(imports_offset)?;
        bw.write_u32::<B>(end_offset)?;

        Ok(())
    }
}
