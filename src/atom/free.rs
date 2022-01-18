/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::Mp4Atom;
use crate::error::Error;

#[derive(Debug)]
pub struct Free {
    start: usize,
    size: usize,
}

impl Mp4Atom for Free {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(Free {
            start,
            size: data.len(),
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
        "free"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }
}