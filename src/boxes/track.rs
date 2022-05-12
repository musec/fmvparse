/*
 * © 2022 Arastoo Bozorgi
 * © 2022 Samir Dharar
 * All rights reserved.
 */

use crate::boxes::{AtomName, InnerAtom, Media, Mp4Box};
use crate::error::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};
use std::io::{Read, Seek, SeekFrom};

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
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let len = header.size as u64;
        let mut track = Track {
            header,
            level,
            ..Default::default()
        };
        let mut index = start + 8; // skip the first 8 bytes that are headers

        while index < start + len {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let mut size = vec![0u8; 4];
            let mut name = vec![0u8; 4];
            reader.seek(SeekFrom::Start(index as u64))?;
            reader.read_exact(&mut size)?;
            reader.read_exact(&mut name)?;
            let size = BigEndian::read_u32(&size) as u64;
            let name = std::str::from_utf8(&name)?;
            let name = AtomName::from(name);

            match name {
                AtomName::EditLists => {
<<<<<<< HEAD
                    let b = Box::new(EditLists::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    track.edts = Some(b);
                }
                AtomName::Media => {
                    let b = Box::new(Media::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    track.mdia = Some(b);
                }
                AtomName::TrackHeader => {
                    let b = Box::new(InnerAtom::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
=======
                    let b =
                        Box::new(EditLists::parse(reader, index, level + 1)?) as Box<dyn Mp4Box>;
                    track.edts = Some(b);
                }
                AtomName::Media => {
                    let b = Box::new(Media::parse(reader, index, level + 1)?) as Box<dyn Mp4Box>;
                    track.mdia = Some(b);
                }
                AtomName::TrackHeader => {
                    let b =
                        Box::new(InnerAtom::parse(reader, index, level + 1)?) as Box<dyn Mp4Box>;
>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
                    track.tkhd = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(track)
    }

    fn start(&self) -> u64 {
        self.header.start
    }

    fn end(&self) -> u64 {
        self.header.end
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        let mut fields = vec![];
        if let Some(tkhd) = self.tkhd.as_ref() {
            fields.push(tkhd.as_ref());
        }
        if let Some(edts) = self.edts.as_ref() {
            fields.push(edts.as_ref());
        }
        if let Some(mdia) = self.mdia.as_ref() {
            fields.push(mdia.as_ref());
        }

        Some(fields)
    }

<<<<<<< HEAD
=======
    fn offsets(&self) -> Option<Vec<u64>> {
        None
    }

>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
    fn level(&self) -> u8 {
        self.level
    }
}

impl Mp4Box for EditLists {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let len = header.size as u64;
        let mut edit_list = EditLists {
            header,
            level,
            ..Default::default()
        };
        let mut index = start + 8; // skip the first 8 bytes that are headers

        while index < len {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let mut size = vec![0u8; 4];
            let mut name = vec![0u8; 4];
            reader.seek(SeekFrom::Start(index as u64))?;
            reader.read_exact(&mut size)?;
            reader.read_exact(&mut name)?;
            let size = BigEndian::read_u32(&size) as u64;
            let name = std::str::from_utf8(&name)?;

            if name == "elst" {
<<<<<<< HEAD
                let b = Box::new(InnerAtom::parse(
                    reader,
                    index + start,
                    level + 1,
                )?) as Box<dyn Mp4Box>;
=======
                let b = Box::new(InnerAtom::parse(reader, index + start, level + 1)?)
                    as Box<dyn Mp4Box>;
>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
                edit_list.elst = Some(b);
            }
            index += size;
        }

        Ok(edit_list)
    }

    fn start(&self) -> u64 {
        self.header.start
    }

    fn end(&self) -> u64 {
        self.header.end
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        let mut fields = vec![];
        if let Some(elst) = self.elst.as_ref() {
            fields.push(elst.as_ref());
        }

        Some(fields)
    }

<<<<<<< HEAD
=======
    fn offsets(&self) -> Option<Vec<u64>> {
        None
    }

>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
    fn level(&self) -> u8 {
        self.level
    }
}
