/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::Error;
use crate::Header;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct FileType {
    header: Header,
    level: u8,
}

impl Mp4Box for FileType {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        Ok(FileType {
            header,
            level,
        })
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
        None
    }

    fn level(&self) -> u8 {
        self.level
    }
}
