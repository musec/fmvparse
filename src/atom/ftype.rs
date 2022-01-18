/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::Mp4Atom;
use crate::error::Error;

#[derive(Debug)]
pub struct FType {
    start: usize,
    size: usize,
    data: Vec<u8>
}

impl Mp4Atom for FType {
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> {
        Ok(FType {
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
        "ftyp"
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }
}