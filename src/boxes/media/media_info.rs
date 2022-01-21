/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{Mp4Box, InnerAtom};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use crate::Header;
use crate::boxes::media::SampleTable;

#[derive(Default)]
pub struct MediaInfo {
    vmhd: Option<Box<dyn Mp4Box>>, // video media header, overall information (video track only)
    dinf: Option<Box<dyn Mp4Box>>, // data information box, container
    stbl: Option<Box<dyn Mp4Box>>, // sample table
    header: Header,
    level: u8
}

impl MediaInfo {
    pub fn video_media_header_box(&self) -> Result<&InnerAtom, Error> {
        match self.vmhd.as_ref() {
            Some(b) => {
                Ok(b.downcast_ref::<InnerAtom>().unwrap())
            },
            None => Err(Error::BoxNotFound("vmhd".to_string()))
        }
    }

    pub fn data_info_box(&self) -> Result<&InnerAtom, Error> {
        match self.dinf.as_ref() {
            Some(b) => {
                Ok(b.downcast_ref::<InnerAtom>().unwrap())
            },
            None => Err(Error::BoxNotFound("dinf".to_string()))
        }
    }

    pub fn sample_table_box(&self) -> Result<&SampleTable, Error> {
        match self.stbl.as_ref() {
            Some(b) => {
                Ok(b.downcast_ref::<SampleTable>().unwrap())
            },
            None => Err(Error::BoxNotFound("stbl".to_string()))
        }
    }
}

impl Mp4Box for MediaInfo {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error> where Self: Sized {
        let header = Header::header(data, start)?;
        let mut media_info = MediaInfo {
            header,
            level,
            ..Default::default()
        };

        let mut index = 8; // skip the first 8 bytes that are Movie headers

        while index < data.len() {

            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&data[index..index + 4]) as usize;
            let name = std::str::from_utf8(&data[index + 4..index + 8])?;
            // let name = AtomName::from(name);

            match name {
                "vmhd" => {
                    let b = Box::new(
                        InnerAtom::parse(&data[index..index + size], index + start, level + 1)?
                    ) as Box<dyn Mp4Box>;
                    media_info.vmhd = Some(b);
                },
                "dinf" => {
                    let b = Box::new(
                        InnerAtom::parse(&data[index..index + size], index + start, level + 1)?
                    ) as Box<dyn Mp4Box>;
                    media_info.dinf = Some(b);
                },
                "stbl" => {
                    let b = Box::new(
                        SampleTable::parse(&data[index..index + size], index + start, level + 1)?
                    ) as Box<dyn Mp4Box>;
                    media_info.stbl = Some(b);
                },
                _ => {}
            }

            index += size;
        }

        Ok(media_info)
    }

    fn start(&self) -> usize {
        self.header.start
    }

    fn end(&self) -> usize {
        self.header.start + self.header.size
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn fields(&self) -> Option<Vec<&Box<dyn Mp4Box>>> {
        let mut fields = vec![];
        if self.vmhd.as_ref().is_some() {
            fields.push(self.vmhd.as_ref().unwrap());
        }
        if self.dinf.as_ref().is_some() {
            fields.push(self.dinf.as_ref().unwrap());
        }
        if self.stbl.as_ref().is_some() {
            fields.push(self.stbl.as_ref().unwrap());
        }

        Some(fields)
    }

    fn level(&self) -> u8 {
        self.level
    }
}