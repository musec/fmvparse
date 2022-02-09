use crate::boxes::Mp4Box;
use crate::error::Error;
use crate::Header;
use std::io::{Read, Seek};
extern crate byteorder;
use byteorder::ReadBytesExt;

// Chunk offset box 'stco' or 'co64'

#[derive(Debug, Default)]
pub struct ChunkOffsetBox {
    offsets: Vec<u64>,
    header: Header,
    level: u8,
}

/// Parse a stco box.
impl Mp4Box for ChunkOffsetBox {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let (_, _) = read_fullbox_extra(reader)?;
        let offset_count = reader.read_u32::<byteorder::BigEndian>()?;
        let mut offsets: Vec<u64> = Vec::new();
        for _ in 0..offset_count {
            offsets.push(reader.read_u32::<byteorder::BigEndian>()? as u64);
        }

        Ok(ChunkOffsetBox {
            offsets,
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

    fn offsets(&self) -> Option<Vec<u64>> {
        Some(self.offsets.clone())
    }

    fn level(&self) -> u8 {
        self.level
    }
}

fn read_fullbox_extra<R: Read + Seek>(reader: &mut R) -> Result<(u8, u32), Error> {
    let version = reader.read_u8()?;
    let flags_a = reader.read_u8()?;
    let flags_b = reader.read_u8()?;
    let flags_c = reader.read_u8()?;
    Ok((
        version,
        (flags_a as u32) << 16 | (flags_b as u32) << 8 | (flags_c as u32),
    ))
}
