/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::{Mp4Atom, AtomName};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use crate::atom::free::Free;

pub struct Movie {
    atoms: Vec<Box<dyn Mp4Atom>>,
    start: usize,
    size: usize,
}

// impl Movie {
//
// }

struct Mvhd {
    start: usize,
    size: usize,
    data: Vec<u8>
}

struct Trak {
    // atoms: Vec<Box<dyn Mp4Atom>>,
    start: usize,
    size: usize,
    data: Vec<u8>
}

struct Udata {
    // atoms: Vec<Box<dyn Mp4Atom>>,
    start: usize,
    size: usize,
    data: Vec<u8>
}



impl Mp4Atom for Movie {

    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        let mut atoms = vec![];
        let mut index = 8; // skip the first 8 bytes that are Movie headers

        while index < data.len() {

            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&data[index..index + 4]) as usize;
            let name = std::str::from_utf8(&data[index + 4..index + 8])?;
            let name = AtomName::from(name);

            let atom = match name {
                AtomName::Mvhd => {
                    Box::new(
                        Mvhd::parse(&data[index..index + size], index + start)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::Trak => {
                    Box::new(
                        Trak::parse(&data[index..index + size], index + start)?
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
                    return Err(Error::Unknown("Atom type in Movie cannot be parsed".to_string()));
                }
            };

            atoms.push(atom);
            index += size;
        }

        Ok(Self {
            atoms,
            start,
            size: data.len()
        })
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.start + self.size
    }

    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &str {
        "moov"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        Some(&self.atoms)
    }
}

impl Mp4Atom for Mvhd {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(Mvhd {
            start,
            size: data.len(),
            data: data.to_vec()
        })
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.start + self.size
    }

    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &str {
        "mvhd"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}

impl Mp4Atom for Trak {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(Trak {
            start,
            size: data.len(),
            data: data.to_vec()
        })
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.start + self.size
    }

    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &str {
        "trac"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}

impl Mp4Atom for Udata {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(Udata {
            start,
            size: data.len(),
            data: data.to_vec()
        })
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.start + self.size
    }

    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &str {
        "udata"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}