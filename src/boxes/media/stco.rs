/*
 * © 2022 Samir Dharar
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::error::Error;
use crate::Header;
use std::io::{Read, Seek, SeekFrom};
extern crate byteorder;
use byteorder::ReadBytesExt;

// Chunk offset box 'stco' or 'co64'

#[derive(Debug, Default)]
pub struct ChunkOffsetBox {
    pub offsets: Vec<u64>,
    header: Header,
    level: u8,
}

// Parse a stco box.
impl Mp4Box for ChunkOffsetBox {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let mut stco = ChunkOffsetBox {
            header,
            level,
            ..Default::default()
        };
        // Skipping 4 bytes of:-
        // VERSION: A 1-byte specification of the version of this chunk offset atom
        // FLAGS: A 3-byte space for chunk offset flags
        reader.seek(SeekFrom::Current(4))?;

        let number_of_entries = reader.read_u32::<byteorder::BigEndian>()?; // 4 bytes
                                                                            // let mut offsets: Vec<u64> = Vec::new();
        for _ in 0..number_of_entries {
            stco.offsets
                .push(reader.read_u32::<byteorder::BigEndian>()? as u64);
        }
        Ok(stco)
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

impl std::fmt::Display for ChunkOffsetBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Offsets: {:?}", &self.offsets)?;
        Ok(())
    }
}
