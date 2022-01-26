/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{AtomName, FileType, Free, MediaData, Movie, Mp4Box};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Default)]
pub struct Mp4 {
    ftyp: Option<Box<dyn Mp4Box>>, //file type and compatibility
    moov: Option<Box<dyn Mp4Box>>, // container for all the metadata
    free: Option<Box<dyn Mp4Box>>, // free space
    mdat: Option<Box<dyn Mp4Box>>, //  media data container
    size: usize,
    file: String,
}

impl Mp4 {
    pub fn parse(path: &Path) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        if file_size < 4 {
            return Err(Error::UserError("Invalid file provided".to_string()));
        }

        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer)?;

        let mut index = 0;
        let mut mp4 = Mp4 {
            size: file_size as usize,
            file: path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        while index < buffer.len() {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&buffer[index..index + 4]) as usize;
            let name = std::str::from_utf8(&buffer[index + 4..index + 8])?;
            let name = AtomName::from(name);

            match name {
                AtomName::FileType => {
                    let b = Box::new(FileType::parse(&buffer[index..index + size], index, 1)?)
                        as Box<dyn Mp4Box>;
                    mp4.ftyp = Some(b);
                }
                AtomName::Movie => {
                    let b = Box::new(Movie::parse(&buffer[index..index + size], index, 1)?)
                        as Box<dyn Mp4Box>;
                    mp4.moov = Some(b);
                }
                AtomName::MediaData => {
                    let b = Box::new(MediaData::parse(&buffer[index..size], index, 1)?)
                        as Box<dyn Mp4Box>;
                    mp4.mdat = Some(b);
                }
                AtomName::Free => {
                    let b = Box::new(Free::parse(&buffer[index..index + size], index, 1)?)
                        as Box<dyn Mp4Box>;
                    mp4.free = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(mp4)
    }

    pub fn movie_box(&self) -> Result<&Movie, Error> {
        match self.moov.as_ref() {
            Some(b) => Ok(b.downcast_ref::<Movie>().unwrap()),
            None => Err(Error::BoxNotFound("moov".to_string())),
        }
    }

    pub fn ftype_box(&self) -> Result<&FileType, Error> {
        match self.ftyp.as_ref() {
            Some(b) => Ok(b.downcast_ref::<FileType>().unwrap()),
            None => Err(Error::BoxNotFound("ftyp".to_string())),
        }
    }

    pub fn mdat_box(&self) -> Result<&MediaData, Error> {
        match self.mdat.as_ref() {
            Some(b) => Ok(b.downcast_ref::<MediaData>().unwrap()),
            None => Err(Error::BoxNotFound("mdat".to_string())),
        }
    }

    pub fn free_box(&self) -> Result<&Free, Error> {
        match self.free.as_ref() {
            Some(b) => Ok(b.downcast_ref::<Free>().unwrap()),
            None => Err(Error::BoxNotFound("free".to_string())),
        }
    }
}

impl std::fmt::Debug for Mp4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "file {}, with size {} has the following structure:",
            self.file, self.size
        )?;

        write!(f, "{:?}", self.ftyp.as_ref().unwrap())?;
        write!(f, "{:?}", self.moov.as_ref().unwrap())?;
        write!(f, "{:?}", self.free.as_ref().unwrap())?;
        write!(f, "{:?}", self.mdat.as_ref().unwrap())?;
        Ok(())
    }
}
