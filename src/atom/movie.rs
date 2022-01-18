/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::{Mp4Atom, AtomName};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};

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

struct Trac {
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
        let mut index = 4;

        while index < data.len() {
            // the first 8 bytes includes the atom size and its name
            let size = BigEndian::read_u32(&data[0..index]) as usize;
            let name = std::str::from_utf8(&data[index..index + 4])?;
            let name = AtomName::from(name);
            index += 4;

            let atom = match name {
                AtomName::Mvhd => {
                    Box::new(
                        Mvhd::parse(&data[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::Trak => {
                    Box::new(
                        Movie::parse(&data[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::Udata => {
                    Box::new(
                        Udata::parse(&data[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                },
                _ => {
                    return Err(Error::Unknown("Atom type cannot be parsed".to_string()));
                }
            };

            atoms.push(atom);
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
}

impl Mp4Atom for Trac {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(Trac {
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
}