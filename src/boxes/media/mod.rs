/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

mod media_info;
mod sample_table;

pub use media_info::MediaInfo;
pub use sample_table::SampleTable;

use crate::boxes::{AtomName, InnerAtom, Mp4Box};
use crate::error::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};

#[derive(Default)]
pub struct Media {
    mdhd: Option<Box<dyn Mp4Box>>, // media header
    hdlr: Option<Box<dyn Mp4Box>>, // media handler
    minf: Option<Box<dyn Mp4Box>>, // media info
    header: Header,
    level: u8,
}

impl Media {
    pub fn media_header_box(&self) -> Result<&InnerAtom, Error> {
        match self.mdhd.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("mdhd".to_string())),
        }
    }

    pub fn media_handler_box(&self) -> Result<&InnerAtom, Error> {
        match self.hdlr.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("hdlr".to_string())),
        }
    }

    pub fn media_info_box(&self) -> Result<&MediaInfo, Error> {
        match self.minf.as_ref() {
            Some(b) => Ok(b.downcast_ref::<MediaInfo>().unwrap()),
            None => Err(Error::BoxNotFound("minf".to_string())),
        }
    }
}

impl Mp4Box for Media {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = Header::new(data, start)?;
        let mut media = Media {
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
            let name = AtomName::from(name);

            match name {
                AtomName::MediaInfo => {
                    let b = Box::new(MediaInfo::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    media.minf = Some(b);
                }
                AtomName::MediaHeader => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    media.mdhd = Some(b);
                }
                AtomName::MediaHandler => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    media.hdlr = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(media)
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

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        let mut fields = vec![];
        if let Some(mdhd) = self.mdhd.as_ref() {
            fields.push(mdhd.as_ref());
        }
        if let Some(hdlr) = self.hdlr.as_ref() {
            fields.push(hdlr.as_ref());
        }
        if let Some(minf) = self.minf.as_ref() {
            fields.push(minf.as_ref());
        }

        Some(fields)
    }

    fn level(&self) -> u8 {
        self.level
    }
}
