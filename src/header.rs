/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use std::io::{Seek, SeekFrom, Read};

#[derive(Debug, Default)]
pub struct Header {
    pub name: String,
    pub size: usize,
    pub start: u64,
}

impl Header {
    pub fn new<R: Read + Seek>(reader: &mut R, start: u64) -> Result<Self, Error> {
        // the first 8 bytes includes the atom size and its name
        // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
        let mut size = vec![0u8; 4];
        let mut name = vec![0u8; 4];
        reader.seek(SeekFrom::Start(start as u64))?;
        reader.read_exact(&mut size)?;
        reader.read_exact(&mut name)?;
        let size = BigEndian::read_u32(&size) as usize;
        let name = std::str::from_utf8(&name)?.to_string();

        Ok(Self { name, size, start })
    }
}
