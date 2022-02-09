/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::media::SampleTable;
use crate::boxes::{InnerAtom, Mp4Box};
use crate::error::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};
use std::io::{Read, Seek, SeekFrom};

#[derive(Default)]
pub struct MediaInfo {
    vmhd: Option<Box<dyn Mp4Box>>, // video media header, overall information (video track only)
    dinf: Option<Box<dyn Mp4Box>>, // data information box, container
    stbl: Option<Box<dyn Mp4Box>>, // sample table
    header: Header,
    level: u8,
}

impl MediaInfo {
    pub fn video_media_header_box(&self) -> Result<&InnerAtom, Error> {
        match self.vmhd.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("vmhd".to_string())),
        }
    }

    pub fn data_info_box(&self) -> Result<&InnerAtom, Error> {
        match self.dinf.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("dinf".to_string())),
        }
    }

    pub fn sample_table_box(&self) -> Result<&SampleTable, Error> {
        match self.stbl.as_ref() {
            Some(b) => Ok(b.downcast_ref::<SampleTable>().unwrap()),
            None => Err(Error::BoxNotFound("stbl".to_string())),
        }
    }
}

impl Mp4Box for MediaInfo {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let len = header.size as u64;
        let mut media_info = MediaInfo {
            header,
            level,
            ..Default::default()
        };

        let mut index = start + 8; // skip the first 8 bytes that are headers

        while index < len {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let mut size = vec![0u8; 4];
            let mut name = vec![0u8; 4];
            reader.seek(SeekFrom::Start(index as u64))?;
            reader.read_exact(&mut size)?;
            reader.read_exact(&mut name)?;
            let size = BigEndian::read_u32(&size) as u64;
            let name = std::str::from_utf8(&name)?;

            match name {
                "vmhd" => {
                    let b = Box::new(InnerAtom::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    media_info.vmhd = Some(b);
                }
                "dinf" => {
                    let b = Box::new(InnerAtom::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    media_info.dinf = Some(b);
                }
                "stbl" => {
                    let b = Box::new(SampleTable::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    media_info.stbl = Some(b);
                }
                _ => {}
            }

            index += size;
        }

        Ok(media_info)
    }

    fn start(&self) -> u64 {
        self.header.start
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        let mut fields = vec![];
        if let Some(vmhd) = self.vmhd.as_ref() {
            fields.push(vmhd.as_ref());
        }
        if let Some(dinf) = self.dinf.as_ref() {
            fields.push(dinf.as_ref());
        }
        if let Some(stbl) = self.stbl.as_ref() {
            fields.push(stbl.as_ref());
        }

        Some(fields)
    }

<<<<<<< HEAD
=======
    fn offsets(&self) -> Option<Vec<u64>> {
        None
    }

>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
    fn level(&self) -> u8 {
        self.level
    }
}
