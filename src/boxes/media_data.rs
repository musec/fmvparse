/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::Error;
use crate::Header;

#[derive(Debug)]
pub struct MediaData {
    header: Header,
    data: Vec<u8>,
    level: u8,
}

impl Mp4Box for MediaData {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error> {
        let header = Header::new(data, start)?;
        Ok(MediaData {
            header,
            data: data.to_vec(),
            level,
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
        Ok(self.data.to_vec())
    }

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        None
    }

    fn level(&self) -> u8 {
        self.level
    }
}
