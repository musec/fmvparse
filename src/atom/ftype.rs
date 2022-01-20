/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::Mp4Atom;
use crate::error::Error;
use crate::header::Header;

#[derive(Debug)]
pub struct FType {
    header: Header,
    data: Vec<u8>
}

impl Mp4Atom for FType {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        let header = Header::header(data, start)?;
        Ok(FType {
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