/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::Mp4Atom;
use crate::error::Error;

#[derive(Debug)]
pub struct Mdat {
    start: usize,
    size: usize,
    data: Vec<u8>
}

impl Mp4Atom for Mdat {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(Mdat {
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
        "mdat"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>> {
        None
    }
}
