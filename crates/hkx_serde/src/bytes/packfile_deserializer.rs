use super::hkx_header::HkxHeader;
use super::sections::{
    class_name_section::ClassNames, section_contents::SectionContents,
    section_header::SectionHeader,
};
use crate::classes::class_params::ClassParams;
use crate::error::Result;
use crate::havok_types::HkArrayClass;
use core::mem::size_of;
use std::collections::HashMap;
use std::ffi::CStr;
use std::str::from_utf8;
use zerocopy::{BigEndian, ByteOrder, FromBytes, LittleEndian};

/// Serialize trait for HKX binaries for C++ Havok class.
pub trait ByteSerialize {
    /// As bytes slice(HKX binary) from a instance.
    fn as_bytes(&self) -> Result<&[u8]>;
}

/// Deserialize trait for HKX binaries for C++ Havok class.
pub trait ByteDeSerialize {
    /// Create a new instance from bytes slice(HKX binary).
    fn from_bytes<B>(bytes: &[u8], de: &mut PackFileDeserializer) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]

pub struct PackFileDeserializer<'bytes> {
    /// Pointer byte size. 4 or 8.
    pub ptr_size: u8,

    /// `__classnames__` section content bytes.
    pub class_section: SectionContents<'bytes>,
    /// `__data__` section content bytes. (`hkparam`s data of each class)
    pub data_section: SectionContents<'bytes>,
    /// `__type__` section content bytes.
    pub type_section: SectionContents<'bytes>,

    /// Signature & ClassName pairs from `__class_names__` section content bytes.
    pub class_names: ClassNames<'bytes>,

    /// current bytes data section position.
    ///
    /// This is used to indicate up to which class field binary data was read during deserialization of each C++ Havok Class.
    pub current_position: u32,
    pub deserialized_objects: HashMap<u32, ClassParams<'bytes>>,
}

impl<'bytes> PackFileDeserializer<'bytes> {
    /// Ptr size
    /// - 32 => [`ByteOrder::read_u32`]
    /// - 64 => [`ByteOrder::read_u64`]
    fn read_usize<B>(&self, bytes: &[u8]) -> Result<usize>
    where
        B: ByteOrder,
    {
        Ok(match self.ptr_size {
            32 => B::read_u32(bytes) as usize,
            64 => B::read_u64(bytes) as usize,
            _ => return Err(BytesDeError::InvalidPtrSize(self.ptr_size).into()),
        })
    }

    fn read_class_array<B, T>(&mut self, bytes: &[u8]) -> Result<HkArrayClass<T>>
    where
        B: ByteOrder,
        T: ByteDeSerialize,
    {
        let read_start = self.current_position as usize;

        self.read_usize::<B>(bytes)?; // Consume pointer(u32|u64 == usize) assert 0
        let size = B::read_u32(bytes);
        B::read_u32(bytes); // Capacity and flags
        self.current_position += 16; // ptr(4|8 bytes) + size(4bytes) + cap(4bytes)

        // flag assertion
        let size_cap_and_flags = size | (0x80 << 24);
        if size == size_cap_and_flags {
            return Err(BytesDeError::ParseError {
                expected: size_cap_and_flags.to_string(),
                actual: size.to_string(),
            }
            .into());
        }

        let mut res = Vec::new();
        if size > 0 {
            let local_dst = self.data_section.local_map[read_start].dst as usize;
            for _ in 0..size {
                res.push(T::from_bytes::<B>(&bytes[local_dst..], self)?);
            }
        }

        Ok(res.into())
    }

    fn read_class_ptr<B, T>(&mut self, bytes: &[u8]) -> Result<T>
    where
        B: ByteOrder,
        T: ByteDeSerialize,
    {
        if !self
            .data_section
            .local_map
            .contains_key(&self.current_position)
        {
            self.current_position += 8;
            return T::from_bytes::<B>(bytes, self);
        }

        let global_dst = self.data_section.global_map[&self.current_position].dst;
        // self.deserialize_virtual_class(&bytes, global_dst)
        todo!()
    }

    fn read_string_ptr<'a>(&mut self, bytes: &'a [u8]) -> Result<&'a str> {
        if !self
            .data_section
            .local_map
            .contains_key(&self.current_position)
        {
            self.current_position += 8;
            return Ok("");
        }

        let local_dst = self.data_section.local_map[&self.current_position].dst as usize;
        let c_str = CStr::from_bytes_until_nul(&bytes[local_dst..])?;
        let s = from_utf8(c_str.to_bytes())?;

        self.current_position += s.len() as u32;
        Ok(s)
    }

    /// Create a new instance from hkx class fields.
    fn deserialize_virtual_class<B>(&mut self, bytes: &[u8], offset: u32) -> Result<()>
    where
        B: ByteOrder,
    {
        if self.deserialized_objects.contains_key(&offset) {
            return Ok(());
        };

        let name_offset = self.data_section.virtual_map[&offset].name_offset as usize;
        let hk_class_name = &self.class_names.offset_class_names_map[&name_offset].class_name;
        let class =
            ClassParams::from_class_name_and_bytes::<B>(hk_class_name.to_str()?, bytes, self)?;
        self.deserialized_objects.insert(offset, class);

        Ok(())
    }

    /// Read hkx header, each section header, section fixup offsets maps, and class name offsets map.
    ///
    /// # Note
    /// This method does not yet read the binary data information of the fields of the C++ Havok Class.
    fn deserialize_headers_and_map<B>(bytes: &'bytes [u8]) -> Result<Self>
    where
        B: ByteOrder,
    {
        // current position
        let mut start = 0;

        // 1. Read 64bytes hkx file header.
        let header = HkxHeader::<B>::ref_from(&bytes[start..size_of::<HkxHeader<B>>()]).unwrap();
        start += size_of::<HkxHeader<B>>();
        // 2. Skip padding
        let padding = header.section_offset.get();
        if padding > 0 {
            start += padding as usize;
        }

        // 3. Read 48bytes * 3 section headers
        let section_next_pos = start + size_of::<SectionHeader<B>>();
        let class_header = SectionHeader::<B>::ref_from_bytes(&bytes[start..section_next_pos])?;
        start = section_next_pos;

        let section_next_pos = start + size_of::<SectionHeader<B>>();
        let type_header = SectionHeader::<B>::ref_from_bytes(&bytes[start..section_next_pos])?;
        start = section_next_pos;

        let section_next_pos = start + size_of::<SectionHeader<B>>();
        let data_header = SectionHeader::<B>::ref_from_bytes(&bytes[start..section_next_pos])?;

        tracing::debug!("class_header: {class_header}");
        tracing::debug!(" type_header: {type_header}");
        tracing::debug!(" data_header: {data_header}");

        // 4. Section fixup map by each section header information
        let class_section = SectionContents::from_bytes(bytes, class_header, 0)?;
        let type_section = SectionContents::from_bytes(bytes, type_header, 1)?;
        let data_section = SectionContents::from_bytes(bytes, data_header, 2)?;

        // 5. Read class section content
        let class_names = ClassNames::from_bytes::<B>(class_section.section_data)?;

        tracing::debug!("data_section.local_map: {:#?}", &data_section.local_map);
        tracing::debug!("data_section.global_map: {:#?}", &data_section.global_map);
        tracing::debug!("data_section.virtual_map: {:#?}", &data_section.virtual_map);
        tracing::debug!(" class_names: {:#?}", &class_names);

        Ok(Self {
            ptr_size: header.pointer_size,
            class_section,
            data_section,
            type_section,
            class_names,
            current_position: data_header.absolute_data_start.get(),
            deserialized_objects: HashMap::new(),
        })
    }

    pub fn deserialize(bytes: &'bytes [u8]) -> Result<()> {
        match HkxHeader::is_big_endian(bytes) {
            true => {
                let mut de = Self::deserialize_headers_and_map::<BigEndian>(bytes)?;
                let data_section = de.data_section.section_data;
                de.deserialize_virtual_class::<BigEndian>(data_section, 0)?;
            }
            false => {
                let mut de = Self::deserialize_headers_and_map::<LittleEndian>(bytes)?;
                let data_section = de.data_section.section_data;
                de.deserialize_virtual_class::<LittleEndian>(data_section, 0)?;
            }
        };

        Ok(())
    }
}

/// Error type for binary data deserialization.
#[derive(Debug, thiserror::Error)]
pub enum BytesDeError {
    /// Expected pointer size 4 or 8 byte. But got {0}.
    #[error("Expected pointer size 4 or 8 byte. But got {0}.")]
    InvalidPtrSize(u8),

    /// Failed to parse {actual} as {expected}.
    #[error("Failed to parse {actual} as {expected}.")]
    ParseError { expected: String, actual: String },
}

#[cfg(test)]
mod tests {
    use hkx_serde_tracing::init_tracing;

    use super::*;
    #[allow(unused)]
    use pretty_assertions::assert_eq;
    use tracing::Level;

    #[test]
    fn should_deserialize() {
        let _guard = init_tracing(Some("deserialize_hkx_bytes"), false, Level::DEBUG);
        // let bytes = include_bytes!("../../../../tests/1hm_behavior_x86_64.hkx");
        let bytes = include_bytes!("../../../../tests/defaultmale.hkx");

        if let Err(e) = PackFileDeserializer::deserialize(bytes.as_slice()) {
            print!("{e}");
        };
    }
}
