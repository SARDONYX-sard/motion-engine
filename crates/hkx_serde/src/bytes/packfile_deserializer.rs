use super::hkx_header::HkxHeader;
use super::sections::{
    class_name_section::ClassNames, section_contents::SectionContents,
    section_header::SectionHeader,
};
use crate::classes::class_params::ClassParams;
use crate::error::{HkxError, Result};
use core::mem::size_of;
use std::collections::{hash_map, HashMap};
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackFileDeserializer<'bytes> {
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
    pub current_position: usize,
}

/// C++ Havok class == has hkparams
type HkClassArray<T> = Vec<T>;

impl<'bytes> PackFileDeserializer<'bytes> {
    fn read_class_array<B, T>(&mut self, bytes: &[u8]) -> Result<HkClassArray<T>>
    where
        B: ByteOrder,
        T: ByteDeSerialize,
    {
        // Byte position
        let mut offset = 0;

        B::read_u64(bytes); // Consume pointer assert 0
        let size = B::read_u32(bytes);
        B::read_u32(bytes); // Capacity and flags
        offset += 16; // ptr(4|8 bytes) + size(4bytes) + cap(4bytes)

        // flag assertion
        let size_cap_and_flags = size | (0x80 << 24);
        if size == size_cap_and_flags {
            return Err(HkxError::ParseError {
                expected: size_cap_and_flags.to_string(),
                actual: size.to_string(),
            });
        }

        let mut res = Vec::new();
        if size > 0 {
            let local_dst = self.data_section.local_map[&offset - 16].dst as usize;

            for _ in 0..size {
                res.push(T::from_bytes::<B>(&bytes[local_dst..], self)?);
            }
        }

        Ok(res)
    }

    // fn read_class_ptr<T: ByteDeSerialize>(&self, bytes: &[u8], prev_position: u32) -> Result<T> {
    //     let mut current_position = prev_position;
    //     if !self.data_section.global_map.contains_key(&prev_position) {
    //         current_position += 8;
    //         return Ok((T::from_bytes(bytes), current_position));
    //     }

    //     let global_dst = self.data_section.global_map[&current_position].dst;
    //     self.deserialize_virtual_class(&bytes, global_dst, deserialized_objects)
    // }

    fn read_string_ptr<'a>(&self, bytes: &'a [u8], prev_position: u32) -> Result<(&'a str, u32)> {
        let mut current_position = prev_position;
        if !self.data_section.local_map.contains_key(&prev_position) {
            current_position += 8;
            return Ok(("", current_position));
        }

        let local_dst = self.data_section.local_map[&current_position].dst as usize;
        let c_str = CStr::from_bytes_until_nul(&bytes[local_dst..])?;

        Ok((from_utf8(c_str.to_bytes())?, current_position))
    }

    /// Create a new instance from hkx class fields.
    fn deserialize_virtual_class<B>(
        &mut self,
        bytes: &[u8],
        offset: u32,
        deserialized_objects: &mut HashMap<u32, ClassParams<'bytes>>,
    ) -> Result<()>
    where
        B: ByteOrder,
    {
        if let hash_map::Entry::Vacant(entry) = deserialized_objects.entry(offset) {
            let fixup = self.data_section.virtual_map[&offset].name_offset as usize;
            let hk_class_name = &self.class_names.offset_class_names_map[&fixup].class_name;
            let hk_class =
                ClassParams::from_class_name_and_bytes::<B>(hk_class_name.to_str()?, bytes, self)?;

            entry.insert(hk_class);
        };

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

        tracing::debug!("class_header: {:#?}", &class_header);
        tracing::debug!(" type_header: {:#?}", &type_header);
        tracing::debug!(" data_header: {:#?}", &data_header);

        // 4. Section fixup map by each section header information
        let class_section = SectionContents::from_bytes(bytes, class_header, 1)?;
        let type_section = SectionContents::from_bytes(bytes, type_header, 2)?;
        let data_section = SectionContents::from_bytes(bytes, data_header, 3)?;

        // 5. Read class section content
        let class_names = ClassNames::from_bytes::<B>(class_section.section_data)?;

        tracing::debug!("data_section.local_map: {:#?}", &data_section.local_map);
        tracing::debug!("data_section.global_map: {:#?}", &data_section.global_map);
        tracing::debug!("data_section.virtual_map: {:#?}", &data_section.virtual_map);
        tracing::debug!(" class_names: {:#?}", &class_names);

        Ok(Self {
            class_section,
            data_section,
            type_section,
            class_names,
            current_position: 0,
        })
    }

    pub fn deserialize(
        bytes: &'bytes [u8],
        deserialized_map: &mut HashMap<u32, ClassParams<'bytes>>,
    ) -> Result<()> {
        match HkxHeader::is_big_endian(bytes) {
            true => {
                let mut de = Self::deserialize_headers_and_map::<BigEndian>(bytes)?;
                let data_section = de.data_section.section_data;
                de.deserialize_virtual_class::<BigEndian>(data_section, 0, deserialized_map)?;
            }
            false => {
                let mut de = Self::deserialize_headers_and_map::<LittleEndian>(bytes)?;
                let data_section = de.data_section.section_data;
                de.deserialize_virtual_class::<LittleEndian>(data_section, 0, deserialized_map)?;
            }
        };

        Ok(())
    }
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

        let mut de_obj = HashMap::new();
        if let Err(e) = PackFileDeserializer::deserialize(bytes.as_slice(), &mut de_obj) {
            print!("{e}");
        };

        dbg!(de_obj);
    }
}
