/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Default)]
pub struct Header {
    pub name: String,
    pub size: usize,
    pub start: usize,
}

impl Header {
    pub fn header(data: &[u8], start: usize) -> Result<Self, Error> {
        // the first 8 bytes includes the atom size and its name
        // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
        let size = BigEndian::read_u32(&data[0..4]) as usize;
        let name = std::str::from_utf8(&data[4..8])?.to_string();

        Ok(Self { name, size, start })
    }
}
