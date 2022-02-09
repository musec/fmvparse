/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::Error;
use crate::Header;
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Free {
    header: Header,
    level: u8,
}

impl Mp4Box for Free {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        Ok(Free { header, level })
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
