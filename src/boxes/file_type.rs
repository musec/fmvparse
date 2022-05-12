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
pub struct FileType {
    header: Header,
    level: u8,
}

impl Mp4Box for FileType {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
<<<<<<< HEAD
        Ok(FileType {
            header,
            level,
        })
=======
        Ok(FileType { header, level })
>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
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

<<<<<<< HEAD

=======
>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
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
