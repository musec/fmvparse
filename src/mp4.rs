/*
 * © 2022 Arastoo Bozorgi
 * © 2022 Samir Dharar
 * All rights reserved.
 */

use crate::boxes::{AtomName, InnerAtom, Movie, Mp4Box};
use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Default)]
pub struct Mp4 {
    ftyp: Option<Box<dyn Mp4Box>>, //file type and compatibility
    wide: Option<Box<dyn Mp4Box>>, // reserved space
    mdat: Option<Box<dyn Mp4Box>>, //  media data container
    moov: Option<Box<dyn Mp4Box>>, // container for all the metadata
    free: Option<Box<dyn Mp4Box>>, // free space
    skip: Option<Box<dyn Mp4Box>>, // Unused space available in file
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

        let mut index = 0;
        let mut mp4 = Mp4 {
            size: file_size as usize,
            file: path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        while index < file_size {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.

            let mut size = vec![0u8; 4];
            let mut name = vec![0u8; 4];
            file.seek(SeekFrom::Start(index as u64))?;
            file.read_exact(&mut size)?;
            file.read_exact(&mut name)?;
            let size = BigEndian::read_u32(&size) as u64;
            let name = std::str::from_utf8(&name)?;

            let name = AtomName::from(name);

            match name {
                AtomName::FileType => {
                    let b = Box::new(InnerAtom::parse(&mut file, index, 1)?) as Box<dyn Mp4Box>;
                    mp4.ftyp = Some(b);
                }
                AtomName::Wide => {
                    let b = Box::new(InnerAtom::parse(&mut file, index, 1)?) as Box<dyn Mp4Box>;
                    mp4.wide = Some(b);
                }
                AtomName::MediaData => {
                    let b = Box::new(InnerAtom::parse(&mut file, index, 1)?) as Box<dyn Mp4Box>;
                    mp4.mdat = Some(b);
                }
                AtomName::Movie => {
                    let b = Box::new(Movie::parse(&mut file, index, 1)?) as Box<dyn Mp4Box>;
                    mp4.moov = Some(b);
                }
                AtomName::Free => {
                    let b = Box::new(InnerAtom::parse(&mut file, index, 1)?) as Box<dyn Mp4Box>;
                    mp4.free = Some(b);
                }
                AtomName::Skip => {
                    let b = Box::new(InnerAtom::parse(&mut file, index, 1)?) as Box<dyn Mp4Box>;
                    mp4.skip = Some(b);
                }
                _ => {}
            }
            index += size;
        }

        Ok(mp4)
    }

    pub fn ftype_box(&self) -> Result<&InnerAtom, Error> {
        match self.ftyp.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("ftyp".to_string())),
        }
    }

    pub fn wide_box(&self) -> Result<&InnerAtom, Error> {
        match self.wide.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("wide".to_string())),
        }
    }

    pub fn mdat_box(&self) -> Result<&InnerAtom, Error> {
        match self.mdat.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("mdat".to_string())),
        }
    }

    pub fn movie_box(&self) -> Result<&Movie, Error> {
        match self.moov.as_ref() {
            Some(b) => Ok(b.downcast_ref::<Movie>().unwrap()),
            None => Err(Error::BoxNotFound("moov".to_string())),
        }
    }

    pub fn free_box(&self) -> Result<&InnerAtom, Error> {
        match self.free.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
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

        if let Some(ftyp_box) = self.ftyp.as_ref() {
            write!(f, "{:?}", ftyp_box)?;
        }

        if let Some(wide_box) = self.wide.as_ref() {
            write!(f, "{:?}", wide_box)?;
        }

        if let Some(mdat_box) = self.mdat.as_ref() {
            write!(f, "{:?}", mdat_box)?;
        }

        if let Some(moov_box) = self.moov.as_ref() {
            write!(f, "{:?}", moov_box)?;
        }

        if let Some(free_box) = self.free.as_ref() {
            write!(f, "{:?}", free_box)?;
        }

        if let Some(skip_box) = self.skip.as_ref() {
            write!(f, "{:?}", skip_box)?;
        }
        Ok(())
    }
}
