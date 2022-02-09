/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::Error;
use downcast_rs::Downcast;
use std::io::{Read, Seek};

pub trait Mp4Box: Downcast {
    /// Read the atom from the data and parse it
    fn parse<R>(reader: &mut R, start: u64, level: u8) -> Result<Self, Error>
    where
        Self: Sized,
        R: Read + Seek;

    /// The start address of the box
    fn start(&self) -> u64;

    /// The box size in bytes
    fn size(&self) -> usize;

    /// The box name
    fn name(&self) -> &str;

    // /// Read the box content
    // fn read(&self, reader: &mut File) -> Result<Vec<u8>, Error>;

    /// Get the internal boxes of this box
    fn fields(&self) -> Option<Vec<&dyn Mp4Box>>;

    /// The box level
    fn level(&self) -> u8;
<<<<<<< HEAD
=======

    /// Print offsets of stco boxes
    fn offsets(&self) -> Option<Vec<u64>>;
>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
}

impl_downcast!(Mp4Box);

impl std::fmt::Debug for dyn Mp4Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {{start address: {}, size: {}}}",
            self.name(),
            self.start(),
            self.size()
        )?;

        let internals = self.fields();
        if let Some(internals) = internals {
            for internal in internals {
                // add indent based on the level
                for _ in 0..internal.level() - 1 {
                    write!(f, "\t")?;
                }
                write!(f, "{:?}", internal)?;
            }
        }
<<<<<<< HEAD
=======

        let off_print = self.offsets();
        let indent = self.level();

        if let Some(off_print) = off_print {
            // add indent based on the level
            for _ in 0..indent - 1 {
                write!(f, "\t")?;
            }
            write!(f, "{:?}", off_print)?;
        }
>>>>>>> cce9eb5 (Added STCO atom parsing. Parsing works fine but the indentation problem has to be fixed.)
        Ok(())
    }
}

#[derive(Debug)]
pub enum AtomName {
    FileType,
    Movie,
    MediaData,
    Free,
    MovieHeader,
    Track,
    Userdata,
    TrackHeader,
    EditLists,
    Media,
    MediaHeader,
    MediaHandler,
    MediaInfo,
    SampleTable,
    SampleDesc,
    Other,
}

impl From<&str> for AtomName {
    fn from(name: &str) -> Self {
        match name {
            "ftyp" => AtomName::FileType,
            "moov" => AtomName::Movie,
            "mdat" => AtomName::MediaData,
            "free" => AtomName::Free,
            "mvhd" => AtomName::MovieHeader,
            "trak" => AtomName::Track,
            "udta" => AtomName::Userdata,
            "tkhd" => AtomName::TrackHeader,
            "edts" => AtomName::EditLists,
            "mdia" => AtomName::Media,
            "mdhd" => AtomName::MediaHeader,
            "hdlr" => AtomName::MediaHandler,
            "minf" => AtomName::MediaInfo,
            "stbl" => AtomName::SampleTable,
            "stsd" => AtomName::SampleDesc,
            _ => AtomName::Other,
        }
    }
}

impl std::convert::From<AtomName> for &str {
    fn from(an: AtomName) -> Self {
        match an {
            AtomName::FileType => "ftyp",
            AtomName::Movie => "moov",
            AtomName::MediaData => "mdat",
            AtomName::Free => "free",
            AtomName::MovieHeader => "mvhd",
            AtomName::Track => "trak",
            AtomName::Userdata => "udata",
            AtomName::TrackHeader => "tkhd",
            AtomName::EditLists => "rdts",
            AtomName::Media => "mdia",
            AtomName::MediaHeader => "mdhd",
            AtomName::MediaHandler => "hdlr",
            AtomName::MediaInfo => "minf",
            AtomName::SampleTable => "stbl",
            AtomName::SampleDesc => "stsd",
            AtomName::Other => "other",
        }
    }
}
