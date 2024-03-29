//! Each section fixup map reader/writer
use core::mem::size_of;
use indexmap::IndexMap;
use zerocopy::{ByteOrder, U32};

use super::section_header::SectionHeader;

trait ReadFixup {
    /// Reads `core::mem::size_of::<Self>` size fixup data from the argument byte slice.
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self>
    where
        Self: Sized;
}

trait WriteFixup {
    /// Writes `core::mem::size_of::<Self>` size fixup data to the argument byte slice.
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalFixup {
    pub src: usize,
    pub dst: usize,
}

impl ReadFixup for LocalFixup {
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self> {
        let src = B::read_u32(bytes) as usize;
        let dst = B::read_u32(&bytes[4..]) as usize;

        Ok(LocalFixup { src, dst })
    }
}

impl WriteFixup for LocalFixup {
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        B::write_u32(bytes, self.src as u32);
        B::write_u32(&mut bytes[4..], self.dst as u32);

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalFixup {
    pub src: usize,
    pub dst_section_index: usize,
    pub dst: usize,
}

impl ReadFixup for GlobalFixup {
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self> {
        let src = B::read_u32(bytes) as usize;
        let dst_section_index = B::read_u32(&bytes[4..]) as usize;
        let dst = B::read_u32(&bytes[8..]) as usize;

        Ok(GlobalFixup {
            src,
            dst_section_index,
            dst,
        })
    }
}

impl WriteFixup for GlobalFixup {
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        B::write_u32(bytes, self.src as u32);
        B::write_u32(&mut bytes[4..], self.dst_section_index as u32);
        B::write_u32(&mut bytes[8..], self.dst as u32);

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualFixup {
    pub src: usize,
    pub section_index: usize,
    /// Havok Class name offset.
    ///
    /// # How do we use this?
    /// The value is the same as the starting position of each class name
    /// in the `__classnames__` section.
    ///
    /// This means that we can use this offset in `virtual_fixup_map`
    /// to find out which class name constructor is needed.
    pub name_offset: usize,
}

impl ReadFixup for VirtualFixup {
    fn from_bytes<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self> {
        let src = B::read_u32(bytes) as usize;
        let dst_section_index = B::read_u32(&bytes[4..]) as usize;
        let dst = B::read_u32(&bytes[8..]) as usize;

        Ok(VirtualFixup {
            src,
            section_index: dst_section_index,
            name_offset: dst,
        })
    }
}

impl WriteFixup for VirtualFixup {
    fn write_bytes<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        B::write_u32(bytes, self.src as u32);
        B::write_u32(&mut bytes[4..], self.section_index as u32);
        B::write_u32(&mut bytes[8..], self.name_offset as u32);

        Ok(())
    }
}

/// Has fixup maps & section content bytes ref data.
///
/// Normally, the 0th `classNameOffset` of this map will contain the starting position
/// of the string `hkRootLevelContainer` in the `__classnames__` section.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SectionContents<'bytes> {
    pub global_map: IndexMap<usize, GlobalFixup>,
    pub local_map: IndexMap<usize, LocalFixup>,
    pub virtual_map: IndexMap<usize, VirtualFixup>,
    /// Each section bytes data
    pub section_data: &'bytes [u8],
    ///  Index Section ID
    ///
    /// # Examples
    /// - `__class_names__`: 1
    /// - `__type__`: 2
    /// - `__data__`: 3
    pub section_index: u32,
}

impl<'a> SectionContents<'a> {
    const OFFSET_MAP_IS_NONE: u32 = 0xFFFFFFFF;

    /// Read the fixup map from bytes by relying on the section header.
    ///
    /// # Assumption
    /// - Bytes position is 0 start(== HKX Header start). **Not section header start**.
    pub fn from_bytes<B: ByteOrder>(
        bytes: &'a [u8],
        section_header: &SectionHeader<B>,
        section_id: u32,
    ) -> std::io::Result<Self> {
        let absolute_data_start = section_header.absolute_data_start.get() as usize;
        let local_fixups_offset = section_header.local_fixups_offset.get() as usize;
        let global_fixups_offset = section_header.global_fixups_offset.get() as usize;
        let virtual_fixups_offset = section_header.virtual_fixups_offset.get() as usize;
        let exports_offset = section_header.exports_offset.get() as usize;

        let section_data = &bytes[absolute_data_start..absolute_data_start + local_fixups_offset];

        // Read local fixups
        let mut offset = absolute_data_start + local_fixups_offset;
        let mut local_map = IndexMap::new();

        for _ in 0..(global_fixups_offset - local_fixups_offset) / 8 {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let local_fixup = LocalFixup::from_bytes::<B>(&bytes[offset..])?;
                offset += size_of::<LocalFixup>();
                local_map.insert(local_fixup.src, local_fixup);
            }
        }

        // Read global fixups
        let mut offset = absolute_data_start + global_fixups_offset;
        let mut global_map = IndexMap::new();

        for _ in 0..(virtual_fixups_offset - global_fixups_offset) / 12 {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let global_fixup = GlobalFixup::from_bytes::<B>(&bytes[offset..])?;
                offset += size_of::<GlobalFixup>();
                global_map.insert(global_fixup.src, global_fixup);
            }
        }

        // Read virtual fixups
        let mut offset = absolute_data_start + virtual_fixups_offset;
        let mut virtual_map = IndexMap::new();
        for _ in 0..(exports_offset - virtual_fixups_offset) / 12 {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let virtual_fixup = VirtualFixup::from_bytes::<B>(&bytes[offset..])?;
                offset += size_of::<VirtualFixup>();
                virtual_map.insert(virtual_fixup.src, virtual_fixup);
            }
        }

        Ok(SectionContents {
            global_map,
            local_map,
            virtual_map,
            section_data,
            section_index: section_id,
        })
    }

    /// Write All fixup map & Set fixup's offsets to section header.
    ///
    /// And fills with `0xFF` until 16 bytes alignment.
    ///
    /// # Assumption
    /// - The `absolute_data_offset` of `section_header` is a valid value.
    /// - Bytes position is 0 start(== HKX Header start). **Not section header start**.
    ///
    /// # Note
    /// This is for the convenience of updating the `section header` to the correct fixup offset,
    /// so the byte write of the `section header` is called after this method.
    pub fn write_bytes<B: ByteOrder>(
        &self,
        bytes: &mut [u8],
        section_header: &mut SectionHeader<B>,
    ) -> std::io::Result<()> {
        let absolute_data_start = section_header.absolute_data_start.get();

        // Next write position. (At first, set a section start position.)
        let mut offset = absolute_data_start as usize;

        // Write section data
        bytes[offset..offset + self.section_data.len()].copy_from_slice(self.section_data);
        offset += self.section_data.len();

        // pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write local fixups
        let local_offset = offset as u32 - absolute_data_start;
        for (_, local_fixup) in &self.local_map {
            local_fixup.write_bytes::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<LocalFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write global fixups
        let global_offset = offset as u32 - absolute_data_start;
        for (_, global_fixup) in &self.global_map {
            global_fixup.write_bytes::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<GlobalFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write virtual fixups
        let virtual_offset = offset as u32 - absolute_data_start;
        for (_, virtual_fixup) in &self.virtual_map {
            virtual_fixup.write_bytes::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<VirtualFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        let export_offset = offset as u32 - absolute_data_start;

        // Update section header
        section_header.section_tag_separator = 0xFF;
        section_header.local_fixups_offset = U32::<B>::from(local_offset);
        section_header.global_fixups_offset = U32::<B>::from(global_offset);
        section_header.virtual_fixups_offset = U32::<B>::from(virtual_offset);
        section_header.exports_offset = U32::<B>::from(export_offset);
        section_header.imports_offset = U32::<B>::from(export_offset);
        section_header.end_offset = U32::<B>::from(export_offset);

        Ok(())
    }
}
