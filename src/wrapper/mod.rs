use std::{fs::File, io::{BufReader, Cursor}, error::Error};

use oem_cp::decode_string_complete_table;
use oem_cp::code_table::DECODING_TABLE_CP437;
use zip_structs::{zip_eocd::ZipEOCD, zip_central_directory::ZipCDEntry, zip_local_file_header::ZipLocalFileHeader};

pub struct ZipWrapper<'a> {
    pub(crate) files: Vec<ZipCDEntry>,
    pub(crate) reader: Cursor<&'a [u8]>,
    pub(crate) eocd: Option<ZipEOCD>
}

impl ZipWrapper<'_> {
    pub const fn new() -> Self {
        let buf_reader = Cursor::new(include_bytes!(concat!(env!("JAVA_HOME"), "lib\\", "src.zip")).as_slice());
        ZipWrapper { files: vec![], reader: buf_reader, eocd: None }
    }

    pub fn by_name(&mut self, name: String) -> ZipLocalFileHeader {
        let cd = self.files.iter().filter(|cd| if cd.is_encoded_in_utf8() {
            String::from_utf8_lossy(&cd.file_name_raw).to_string()
        } else {
            decode_string_complete_table(&cd.file_name_raw, &DECODING_TABLE_CP437)
        }.split("/").last().unwrap() == name).nth(0).unwrap();
        ZipLocalFileHeader::from_central_directory(&mut self.reader, &cd).unwrap()
    }

    pub fn names(&self) -> Vec<String> {
        self.files.iter().map(|cd| if cd.is_encoded_in_utf8() {
            String::from_utf8_lossy(&cd.file_name_raw).to_string()
        } else {
            decode_string_complete_table(&cd.file_name_raw, &DECODING_TABLE_CP437)
        }.split("/").last().unwrap().to_string()).collect()
    }

    pub fn is_not_load(&self) -> bool {
        self.eocd.is_none()
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let eocd = ZipEOCD::from_reader(&mut self.reader)?;
        let cd_list = ZipCDEntry::all_from_eocd(&mut self.reader, &eocd)?;
        self.files = cd_list;
        self.eocd = Some(eocd);
        Ok(())
    }
}

impl Default for ZipWrapper<'_> {
    fn default() -> Self {
        ZipWrapper { files: vec![], reader: Cursor::default(), eocd: None }
    }
}

// impl Debug for ZipWrapper {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_list().entries(self.files.iter().map(|z| {
//             let name: String = if z.is_encoded_in_utf8() {
//                 String::from_utf8_lossy(&z.file_name_raw).to_string()
//             } else {
//                 decode_string_complete_table(&z.file_name_raw, &DECODING_TABLE_CP437)
//             };

//             // let content = if z.is_encoded_in_utf8() {
//             //     String::from_utf8_lossy(&z.extra_field).to_string()
//             // } else {
//             //     decode_string_complete_table(&z.extra_field, &DECODING_TABLE_CP437)
//             // };

//             // [name, String::from("\n"), content].concat()
//             name
//         }).filter(|s| s.starts_with("java.base/java/"))).finish()
//     }
// }