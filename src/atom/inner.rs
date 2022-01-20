/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::Mp4Atom;
use crate::error::Error;
use crate::header::Header;

/// This is for the boxes that have no inner structures
pub struct InnerAtom {
    header: Header,
    data: Vec<u8>
}

impl Mp4Atom for InnerAtom {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> where Self: Sized {

        // let size = BigEndian::read_u32(&data[0..4]) as usize;
        // let name = std::str::from_utf8(&data[4..8])?.to_string();
        let header = Header::header(data, start)?;

        Ok(InnerAtom {
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
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}

