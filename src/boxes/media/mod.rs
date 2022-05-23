/*
 * © 2022 Arastoo Bozorgi
 * © 2022 Samir Dharar
 * All rights reserved.
 */

mod media_info;
mod sample_table;
mod stco;

pub use media_info::MediaInfo;
pub use sample_table::SampleTable;
pub use stco::ChunkOffsetBox;

use crate::boxes::{AtomName, InnerAtom, Mp4Box};
use crate::error::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};
use std::io::{Read, Seek, SeekFrom};

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
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let len = header.size as u64;
        let mut media = Media {
            header,
            level,
            ..Default::default()
        };

        let mut index = start + 8; // skip the first 8 bytes that are headers

        while index < start + len {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let mut size = vec![0u8; 4];
            let mut name = vec![0u8; 4];
            reader.seek(SeekFrom::Start(index as u64))?;
            reader.read_exact(&mut size)?;
            reader.read_exact(&mut name)?;
            let size = BigEndian::read_u32(&size) as u64;
            let name = std::str::from_utf8(&name)?;
            let name = AtomName::from(name);

            match name {
                AtomName::MediaInfo => {
                    let b =
                        Box::new(MediaInfo::parse(reader, index, level + 1)?) as Box<dyn Mp4Box>;
                    media.minf = Some(b);
                }
                AtomName::MediaHeader => {
                    let b =
                        Box::new(InnerAtom::parse(reader, index, level + 1)?) as Box<dyn Mp4Box>;
                    media.mdhd = Some(b);
                }
                AtomName::MediaHandler => {
                    let b =
                        Box::new(InnerAtom::parse(reader, index, level + 1)?) as Box<dyn Mp4Box>;
                    media.hdlr = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(media)
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
