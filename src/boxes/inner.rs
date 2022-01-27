/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::Error;
use crate::Header;
use std::io::{Read, Seek};

/// This is for the boxes that have no inner structures
pub struct InnerAtom {
    header: Header,
    // data: Vec<u8>,
    level: u8,
}

impl Mp4Box for InnerAtom {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;

        Ok(InnerAtom {
            header,
            // data: data.to_vec(),
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
