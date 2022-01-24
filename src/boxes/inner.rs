/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::Mp4Box;
use crate::Error;
use crate::Header;

/// This is for the boxes that have no inner structures
pub struct InnerAtom {
    header: Header,
    data: Vec<u8>,
    level: u8,
}

impl Mp4Box for InnerAtom {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = Header::new(data, start)?;

        Ok(InnerAtom {
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
        Ok(self.data.clone())
    }

    fn fields(&self) -> Option<Vec<&Box<dyn Mp4Box>>> {
        None
    }

    fn level(&self) -> u8 {
        self.level
    }
}
