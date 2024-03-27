use super::hkx_header::HkxHeader;
use super::sections::section_header::SectionHeader;
use super::sections::{class_name_section::HKXClassNames, section_fixup::HkxSection};
use crate::classes::class_params::ClassParams;
use crate::error::Result;
use core::mem::size_of;
use std::collections::{hash_map, HashMap};
use zerocopy::{BigEndian, ByteOrder, FromBytes, LittleEndian};

pub trait HkxSerialize {
    fn as_bytes(&self) -> Result<&[u8]>;
}

pub trait HkxDeSerialize {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackFileDeserializer<'a> {
    pub class_section: HkxSection<'a>,
    pub data_section: HkxSection<'a>,
    pub type_section: HkxSection<'a>,

    pub class_names: HKXClassNames<'a>,
    pub deserialized_objects: HashMap<usize, ClassParams<'a>>,
}

impl<'a> PackFileDeserializer<'a> {
    // fn read_class_array<B, T>(&self, br: &mut (impl Read + Seek)) -> Result<Vec<Vec<T>>>
    // where
    //     B: ByteOrder,
    //     T: HkxDeSerialize,
    // {
    //     br.read_u64::<B>()?; // Consume pointer assert 0
    //     let size = br.read_u32::<B>()?;
    //     br.read_u32::<B>()?; // Capacity and flags

    //     let mut res = Vec::new();
    //     if size > 0 {
    //         let backup_position = br.stream_position()?;

    //         // Do a local fixup lookup
    //         let pos = (br.stream_position()? - 16) as usize;
    //         let dst = self.data_section.local_map[&pos].dst as u64;
    //         br.seek(SeekFrom::Start(dst))?;

    //         for _ in 0..size {
    //             res.push(T::from_bytes::<B>(br)?);
    //         }

    //         br.seek(SeekFrom::Start(backup_position))?;
    //     }

    //     Ok(res)
    // }

    // fn read_class_ptr<B>(&mut self, br: &mut (impl Read + Seek)) -> Result<ClassParams>
    // where
    //     B: ByteOrder,
    // {
    //     br.read_u64::<B>()?;
    //     if !self
    //         .data_section
    //         .global_map
    //         .contains_key(&(br.stream_position()? as usize))
    //     {};

    //     let dst = self.data_section.global_map[&(br.stream_position()? as usize)].dst;
    //     let t = &self.construct_virtual_class::<B>(br, dst).unwrap();
    //     Ok(t.clone())
    // }

    fn construct_virtual_class<B>(
        &self,
        bytes: &[u8],
        deserialized_objects: &mut HashMap<usize, ClassParams<'a>>,
    ) -> Result<()>
    where
        B: ByteOrder,
    {
        let mut offset = 0;

        if let hash_map::Entry::Vacant(e) = deserialized_objects.entry(offset) {
            let fixup = &self.data_section.virtual_map[&offset];
            let hk_class_name = &self.class_names.offset_class_names_map[&fixup.dst].class_name;

            let hk_class =
                ClassParams::from_class_name::<B>(hk_class_name.to_str().unwrap(), bytes)?;
            e.insert(hk_class);
        };

        Ok(())
    }

    /// Read hkx header, each section header and class names bytes
    fn deserialize_partially<B>(bytes: &'a [u8]) -> Result<Self>
    where
        B: ByteOrder,
    {
        let mut offset = 0;

        let _header = HkxHeader::<B>::ref_from(&bytes[offset..size_of::<HkxHeader<B>>()]).unwrap();
        offset += size_of::<HkxHeader<B>>();

        // Read section headers
        let section_end_pos = offset + size_of::<SectionHeader<B>>();
        let classes_header = SectionHeader::<B>::ref_from(&bytes[offset..section_end_pos]).unwrap();
        offset = section_end_pos;

        let section_end_pos = offset + size_of::<SectionHeader<B>>();
        let type_header = SectionHeader::<B>::ref_from(&bytes[offset..section_end_pos]).unwrap();
        offset = section_end_pos;

        let section_end_pos = offset + size_of::<SectionHeader<B>>();
        let data_header = SectionHeader::<B>::ref_from(&bytes[offset..section_end_pos]).unwrap();

        // - Section fixup map
        let abs_offset = classes_header.absolute_data_start.get() as usize;
        let class_section = HkxSection::from_slice(&bytes[abs_offset..], classes_header, 1)?;
        let class_names = HKXClassNames::from_slice::<B>(&bytes[abs_offset..])?;

        let abs_offset = data_header.absolute_data_start.get() as usize;
        dbg!(&data_header); // 1027184

        let data_section = HkxSection::from_slice(&bytes[abs_offset..], data_header, 2)?;

        let abs_offset = type_header.absolute_data_start.get() as usize;
        let type_section = HkxSection::from_slice(&bytes[abs_offset..], type_header, 3)?;

        Ok(Self {
            class_section,
            data_section,
            type_section,
            class_names,
            deserialized_objects: HashMap::new(),
        })
    }

    pub fn deserialize(
        bytes: &'a [u8],
        deserialized_objects: &mut HashMap<usize, ClassParams<'a>>,
    ) -> Result<()> {
        match HkxHeader::is_big_endian(bytes) {
            true => {
                let de = Self::deserialize_partially::<BigEndian>(bytes)?;
                let section_data = de.data_section.section_data;
                de.construct_virtual_class::<BigEndian>(section_data, deserialized_objects)?;
            }
            false => {
                let de = Self::deserialize_partially::<LittleEndian>(bytes)?;
                dbg!(&de);
                let section_data = de.data_section.section_data;
                de.construct_virtual_class::<LittleEndian>(section_data, deserialized_objects)?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_deserialize() {
        let bytes = include_bytes!("../../../../tests/1hm_behavior.hkx");

        let mut de_obj = HashMap::new();
        if let Err(e) = PackFileDeserializer::deserialize(bytes.as_slice(), &mut de_obj) {
            print!("{e}");
        };

        dbg!(de_obj);
    }
}

// [crates\hkx-serde\src\bytes\packfile_deserializer.rs:124:9] &data_header = SectionHeader {
//     section_tag: [
//         95,
//         95,
//         100,
//         97,
//         116,
//         97,
//         95,
//         95,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//         0,
//     ],
//     section_tag_separator: 255,
//     absolute_data_start: U32(
//         1424, // 05 90
//     ),
//     local_fixups_offset: U32(
//         1027184, // 0f ac 70
//     ), abs + local = 0F B2 00 (1028608)
//     global_fixups_offset: U32(
//         1084496, // 10 8c 50
//     ),
//     virtual_fixups_offset: U32(
//         1154848, // 11 9f 20
//     ),
//     exports_offset: U32(
//         1212384, //  12 7f e0
//     ),
//     imports_offset: U32(
//         1212384, //  12 7f e0
//     ),
//     end_offset: U32(
//         1212384, //  12 7f e0
//     ),
// }
