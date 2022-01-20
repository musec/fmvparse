/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{Mp4Atom, AtomName, Track, InnerAtom, Free};
use crate::Error;
use byteorder::{BigEndian, ByteOrder};
use crate::Header;

pub struct Movie {
    atoms: Vec<Box<dyn Mp4Atom>>,
    header: Header
}

// impl Movie {
//
// }

struct MovieHeader {
    header: Header,
    data: Vec<u8>
}

struct Udata {
    header: Header,
    data: Vec<u8>
}



impl Mp4Atom for Movie {

    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
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
                AtomName::MovieHeader => {
                    Box::new(
                        MovieHeader::parse(&data[index..index + size], index + start)?
                    )
                        as Box<dyn Mp4Atom>
                }
                AtomName::Track => {
                    Box::new(
                        Track::parse(&data[index..index + size], index + start)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::Udata => {
                    Box::new(
                        Udata::parse(&data[index..index + size], index + start)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::Free =>  {
                    Box::new(
                        Free::parse(&data[index..index + size], index + start)?
                    )
                        as Box<dyn Mp4Atom>
                }
                _ => {
                    Box::new(
                        InnerAtom::parse(&data[index..index + size], index + start)?
                    )
                        as Box<dyn Mp4Atom>
                }
            };

            atoms.push(atom);
            index += size;
        }

        Ok(Self {
            atoms,
            header
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

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        Some(&self.atoms)
    }
}

impl Mp4Atom for MovieHeader {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        let header = Header::header(&data, start)?;
        Ok(MovieHeader {
            header,
            data: data.to_vec()
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
        Ok(self.data.clone())
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}

impl Mp4Atom for Udata {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        let header = Header::header(&data, start)?;
        Ok(Udata {
            header,
            data: data.to_vec()
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
        Ok(self.data.clone())
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}