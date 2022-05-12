/*
 * © 2022 Arastoo Bozorgi
 * © 2022 Samir Dharar
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::Error;
use crate::Header;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct MediaData {
    header: Header,
    // data: Vec<u8>,
    level: u8,
}

impl Mp4Box for MediaData {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        Ok(MediaData {
            header,
            // data: data.to_vec(),
            level,
        })
    }

    fn start(&self) -> u64 {
        self.header.start
    }

    fn end(&self) -> u64 {
        self.header.end
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        None
    }

    fn offsets(&self) -> Option<Vec<u64>> {
        None
    }

    fn level(&self) -> u8 {
        self.level
    }
}
