use core::mem::size_of;
use indexmap::IndexMap;
use zerocopy::{ByteOrder, U32};

use super::section_header::SectionHeader;

trait ReadFixup {
    /// Reads `core::mem::size_of::<Self>` size fixup data from the argument byte slice.
    fn from_slice<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self>
    where
        Self: Sized;
}

trait WriteFixup {
    /// Writes `core::mem::size_of::<Self>` size fixup data to the argument byte slice.
    fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalFixup {
    pub src: usize,
    pub dst: usize,
}

impl ReadFixup for LocalFixup {
    fn from_slice<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;

        let src = B::read_u32(&bytes[offset..]) as usize;
        offset += 4;
        let dst = B::read_u32(&bytes[offset..]) as usize;

        Ok(LocalFixup { src, dst })
    }
}

impl WriteFixup for LocalFixup {
    fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        let mut offset = 0;

        B::write_u32(&mut bytes[offset..], self.src as u32);
        offset += 4;
        B::write_u32(&mut bytes[offset..], self.dst as u32);

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
    fn from_slice<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;

        let src = B::read_u32(&bytes[offset..]) as usize;
        offset += 4;
        let dst_section_index = B::read_u32(&bytes[offset..]) as usize;
        offset += 4;
        let dst = B::read_u32(&bytes[offset..]) as usize;

        Ok(GlobalFixup {
            src,
            dst_section_index,
            dst,
        })
    }
}

impl WriteFixup for GlobalFixup {
    fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        let mut offset = 0;

        B::write_u32(&mut bytes[offset..], self.src as u32);
        offset += 4;
        B::write_u32(&mut bytes[offset..], self.dst_section_index as u32);
        offset += 4;
        B::write_u32(&mut bytes[offset..], self.dst as u32);

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualFixup {
    pub src: usize,
    pub dst_section_index: usize,
    pub dst: usize,
}

impl ReadFixup for VirtualFixup {
    fn from_slice<B: ByteOrder>(bytes: &[u8]) -> std::io::Result<Self> {
        let mut offset = 0;

        let src = B::read_u32(&bytes[offset..]) as usize;
        offset += 4;
        let dst_section_index = B::read_u32(&bytes[offset..]) as usize;
        offset += 4;
        let dst = B::read_u32(&bytes[offset..]) as usize;

        Ok(VirtualFixup {
            src,
            dst_section_index,
            dst,
        })
    }
}

impl WriteFixup for VirtualFixup {
    fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<()> {
        let mut offset = 0;

        B::write_u32(&mut bytes[offset..], self.src as u32);
        offset += 4;
        B::write_u32(&mut bytes[offset..], self.dst_section_index as u32);
        offset += 4;
        B::write_u32(&mut bytes[offset..], self.dst as u32);
        Ok(())
    }
}

/// Has fixup maps & section content bytes data
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HkxSection<'a> {
    pub global_map: IndexMap<usize, GlobalFixup>,
    pub local_map: IndexMap<usize, LocalFixup>,
    pub virtual_map: IndexMap<usize, VirtualFixup>,
    pub section_data: &'a [u8],
    /// section index id
    pub section_index: u32,
}

impl<'a> HkxSection<'a> {
    const OFFSET_MAP_IS_NONE: u32 = 0xFFFFFFFF;

    /// Read fixup maps.
    ///
    /// # Assumption
    /// bytes start must set section header's `absolute_data_offset`.
    pub fn from_slice<B: ByteOrder>(
        bytes: &'a [u8],
        section_header: &SectionHeader<B>,
        section_id: u32,
    ) -> std::io::Result<Self> {
        let local_fixups_offset = section_header.local_fixups_offset.get() as usize;
        let global_fixups_offset = section_header.global_fixups_offset.get() as usize;
        let virtual_fixups_offset = section_header.virtual_fixups_offset.get() as usize;
        let exports_offset = section_header.exports_offset.get() as usize;

        let mut offset = 0;

        let section_data = &bytes[..local_fixups_offset];

        // Read local fixups
        offset += local_fixups_offset;
        let mut local_map = IndexMap::new();
        while offset < global_fixups_offset {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let local_fixup = LocalFixup::from_slice::<B>(bytes)?;
                offset += size_of::<LocalFixup>();
                local_map.insert(local_fixup.src, local_fixup);
            }
        }

        // Read global fixups
        let mut global_map = IndexMap::new();
        while offset < virtual_fixups_offset {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let global_fixup = GlobalFixup::from_slice::<B>(bytes)?;
                offset += size_of::<GlobalFixup>();
                global_map.insert(global_fixup.src, global_fixup);
            }
        }

        // Read virtual fixups
        let mut virtual_map = IndexMap::new();
        while offset < exports_offset {
            if B::read_u32(&bytes[offset..]) != Self::OFFSET_MAP_IS_NONE {
                let virtual_fixup = VirtualFixup::from_slice::<B>(bytes)?;
                offset += size_of::<VirtualFixup>();
                virtual_map.insert(virtual_fixup.src, virtual_fixup);
            }
        }

        Ok(HkxSection {
            global_map,
            local_map,
            virtual_map,
            section_data,
            section_index: section_id,
        })
    }

    /// Assuming that the `absolute_data_offset` (the byte u32 position where the section data
    /// actually is) is provided for each section, write the byte position and fixup map from there.
    ///
    /// # Note
    /// The data (e.g., class names) is 16 bytes aligned and is filled with `0xFF` and padded until it reaches 16 bytes alignment.
    pub fn write<B: ByteOrder>(&self, bytes: &mut [u8]) -> std::io::Result<SectionHeader<B>> {
        // next write position
        let mut offset = 0;

        // Write section data
        bytes[offset..offset + self.section_data.len()].copy_from_slice(self.section_data);
        offset += self.section_data.len();

        // pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write local fixups
        let local_offset = offset as u32;
        for (_, local_fixup) in &self.local_map {
            local_fixup.write::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<LocalFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write global fixups
        let global_offset = offset as u32;
        for (_, global_fixup) in &self.global_map {
            global_fixup.write::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<GlobalFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        // Write virtual fixups
        let virtual_offset = offset as u32;
        for (_, virtual_fixup) in &self.virtual_map {
            virtual_fixup.write::<B>(&mut bytes[offset..])?;
            offset += std::mem::size_of::<VirtualFixup>();
        }

        // Pad until 16-byte alignment
        while offset % 16 != 0 {
            bytes[offset] = 0xFF;
            offset += 1;
        }

        Ok(SectionHeader {
            section_tag_separator: 0xFF,
            absolute_data_start: U32::<B>::from(0),
            local_fixups_offset: U32::<B>::from(local_offset),
            global_fixups_offset: U32::<B>::from(global_offset),
            virtual_fixups_offset: U32::<B>::from(virtual_offset),
            exports_offset: U32::<B>::from(offset as u32),
            imports_offset: U32::<B>::from(offset as u32),
            end_offset: U32::<B>::from(offset as u32),
            ..Default::default()
        })
    }
}
