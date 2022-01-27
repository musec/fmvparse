/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{AtomName, InnerAtom, Mp4Box, Track};
use crate::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};
use std::io::{Read, Seek, SeekFrom};

#[derive(Default)]
pub struct Movie {
    mvhd: Option<Box<dyn Mp4Box>>, // movie header
    tracks: Vec<Box<dyn Mp4Box>>,  // movie tracks
    udta: Option<Box<dyn Mp4Box>>, // user data
    header: Header,
    level: u8,
}

impl Movie {
    pub fn movie_header_box(&self) -> Result<&InnerAtom, Error> {
        match self.mvhd.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("mvhd".to_string())),
        }
    }

    pub fn user_data_box(&self) -> Result<&InnerAtom, Error> {
        match self.udta.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("udta".to_string())),
        }
    }

    pub fn tracks_num(&self) -> usize {
        self.tracks.len()
    }

    pub fn track_box(&self, id: usize) -> Result<&Track, Error> {
        if self.tracks_num() == 0 {
            return Err(Error::BoxNotFound("trak".to_string()));
        }

        match self.tracks.get(id) {
            Some(b) => Ok(b.downcast_ref::<Track>().unwrap()),
            None => Err(Error::BoxNotFound("trak".to_string())),
        }
    }
}

impl Mp4Box for Movie {
    fn parse<R: Read + Seek>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error> {
        let header = Header::new(reader, start)?;
        let len = header.size as u64;
        let mut movie = Movie {
            header,
            level,
            tracks: vec![],
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
            let name = AtomName::from(name);

            match name {
                AtomName::MovieHeader => {
                    let b = Box::new(InnerAtom::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    movie.mvhd = Some(b);
                }
                AtomName::Track => {
                    let b = Box::new(Track::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    movie.tracks.push(b);
                }
                AtomName::Userdata => {
                    let b = Box::new(InnerAtom::parse(
                        reader,
                        index,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    movie.udta = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(movie)
    }

    fn start(&self) -> u64 {
        self.header.start
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn fields(&self) -> Option<Vec<&dyn Mp4Box>> {
        let mut fields = vec![];
        if let Some(mvhd) = self.mvhd.as_ref() {
            fields.push(mvhd.as_ref());
        }

        for track in self.tracks.iter() {
            fields.push(track.as_ref());
        }

        if let Some(udta) = self.udta.as_ref() {
            fields.push(udta.as_ref());
        }

        Some(fields)
    }

    fn level(&self) -> u8 {
        self.level
    }
}
