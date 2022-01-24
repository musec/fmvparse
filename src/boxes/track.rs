/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{AtomName, InnerAtom, Media, Mp4Box};
use crate::error::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};

#[derive(Default)]
pub struct Track {
    tkhd: Option<Box<dyn Mp4Box>>, // track header
    edts: Option<Box<dyn Mp4Box>>, // edit lists
    mdia: Option<Box<dyn Mp4Box>>, // media metadata
    header: Header,
    level: u8,
}

#[derive(Default)]
pub struct EditLists {
    elst: Option<Box<dyn Mp4Box>>, // an edit list
    header: Header,
    level: u8,
}

impl Track {
    pub fn track_header_box(&self) -> Result<&InnerAtom, Error> {
        match self.tkhd.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("tkhd".to_string())),
        }
    }

    pub fn edit_lists_box(&self) -> Result<&EditLists, Error> {
        match self.edts.as_ref() {
            Some(b) => Ok(b.downcast_ref::<EditLists>().unwrap()),
            None => Err(Error::BoxNotFound("edts".to_string())),
        }
    }

    pub fn media_box(&self) -> Result<&Media, Error> {
        match self.mdia.as_ref() {
            Some(b) => Ok(b.downcast_ref::<Media>().unwrap()),
            None => Err(Error::BoxNotFound("mdia".to_string())),
        }
    }
}

impl EditLists {
    pub fn list_box(&self) -> Result<&InnerAtom, Error> {
        match self.elst.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("elst".to_string())),
        }
    }
}

impl Mp4Box for Track {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = Header::new(data, start)?;
        let mut track = Track {
            header,
            level,
            ..Default::default()
        };
        let mut index = 8; // skip the first 8 bytes that are Movie headers

        while index < data.len() {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&data[index..index + 4]) as usize;
            let name = std::str::from_utf8(&data[index + 4..index + 8])?;
            let name = AtomName::from(name);

            match name {
                AtomName::EditLists => {
                    let b = Box::new(EditLists::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    track.edts = Some(b);
                }
                AtomName::Media => {
                    let b = Box::new(Media::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    track.mdia = Some(b);
                }
                AtomName::TrackHeader => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    track.tkhd = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(track)
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

    fn fields(&self) -> Option<Vec<&Box<dyn Mp4Box>>> {
        let mut fields = vec![];
        if self.tkhd.as_ref().is_some() {
            fields.push(self.tkhd.as_ref().unwrap());
        }
        if self.edts.as_ref().is_some() {
            fields.push(self.edts.as_ref().unwrap());
        }
        if self.mdia.as_ref().is_some() {
            fields.push(self.mdia.as_ref().unwrap());
        }

        Some(fields)
    }

    fn level(&self) -> u8 {
        self.level
    }
}

impl Mp4Box for EditLists {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error> {
        let header = Header::new(data, start)?;
        let mut edit_list = EditLists {
            header,
            level,
            ..Default::default()
        };
        let mut index = 8; // skip the first 8 bytes that are Movie headers

        while index < data.len() {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&data[index..index + 4]) as usize;
            let name = std::str::from_utf8(&data[index + 4..index + 8])?;

            if name == "elst" {
                let b = Box::new(InnerAtom::parse(
                    &data[index..index + size],
                    index + start,
                    level + 1,
                )?) as Box<dyn Mp4Box>;
                edit_list.elst = Some(b);
            }
            index += size;
        }

        Ok(edit_list)
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

    fn fields(&self) -> Option<Vec<&Box<dyn Mp4Box>>> {
        let mut fields = vec![];
        if self.elst.as_ref().is_some() {
            fields.push(self.elst.as_ref().unwrap());
        }

        Some(fields)
    }

    fn level(&self) -> u8 {
        self.level
    }
}
