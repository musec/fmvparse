/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

mod media_info;
mod sample_table;

pub use sample_table::SampleTable;
pub use media_info::MediaInfo;


use crate::boxes::{Mp4Box, AtomName, InnerAtom};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use crate::Header;

pub struct Media {
    atoms: Vec<Box<dyn Mp4Box>>,
    header: Header,
    level: u8
}

impl Mp4Box for Media {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error> where Self: Sized {
        let mut atoms = vec![];
        let header = Header::header(data, start)?;
        let mut index = 8; // skip the first 8 bytes that are Movie headers

        while index < data.len() {

            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&data[index..index + 4]) as usize;
            let name = std::str::from_utf8(&data[index + 4..index + 8])?;
            let name = AtomName::from(name);

            let atom = match name {
                AtomName::MediaInfo => {
                    Box::new(
                        MediaInfo::parse(&data[index..index + size], index + start, level + 1)?
                    )
                        as Box<dyn Mp4Box>
                },
                _ => {
                    Box::new(
                        InnerAtom::parse(&data[index..index + size], index + start, level + 1)?
                    )
                        as Box<dyn Mp4Box>
                }
            };

            atoms.push(atom);
            index += size;
        }

        Ok(Self {
            atoms,
            header,
            level
        })
    }

    fn start(&self) -> usize {
        self.header.start
    }

    fn end(&self) -> usize {
        self.header.start + self.header.size
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Box>>> {
        Some(&self.atoms)
    }

    fn level(&self) -> u8 {
        self.level
    }
}


