/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::Error;

pub trait Mp4Atom {
    /// Read the atom from the data and parse it
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error> where Self: Sized;

    /// The start address of the box
    fn start(&self) -> usize;

    /// The end address of the box
    fn end(&self) -> usize;

    /// The box size in bytes
    fn size(&self) -> usize;

    /// The box name
    fn name(&self) -> &str;

    /// Read the box content
    fn read(&self) -> Result<Vec<u8>, Error>;

    /// Get the internal boxes of this box
    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>>;

    /// The box level
    fn level(&self) -> u8;
}

impl std::fmt::Debug for dyn Mp4Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {{start address: {}, size: {}}} \n",
            self.name(), self.start(), self.size()
        )?;

        let internals = self.internals();
        if internals.is_some() {
            for internal in internals.unwrap() {
                // add indent based on the level
                for _ in 0..internal.level() - 1 {
                    write!(f, "\t")?;
                }
                write!(f, "{:?}", internal)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AtomName {
    FType,
    Movie,
    MediaData,
    Free,
    MovieHeader,
    Track,
    Udata,
    TrackHeader,
    EditLists,
    Media,
    MediaInfo,
    SampleTable,
    SampleDesc,
    Other
}

impl From<&str> for AtomName {
    fn from(name: &str) -> Self {
        match name {
            "ftyp" => AtomName::FType,
            "moov" => AtomName::Movie,
            "mdat" => AtomName::MediaData,
            "free" => AtomName::Free,
            "mvhd" => AtomName::MovieHeader,
            "trak" => AtomName::Track,
            "udta" => AtomName::Udata,
            "tkhd" => AtomName::TrackHeader,
            "edts" => AtomName::EditLists,
            "mdia" => AtomName::Media,
            "minf" => AtomName::MediaInfo,
            "stbl" => AtomName::SampleTable,
            "stsd" => AtomName::SampleDesc,
            _ => AtomName::Other
        }
    }
}

impl std::convert::From<AtomName> for &str {
    fn from(an: AtomName) -> Self {
        match an {
            AtomName::FType => "ftyp",
            AtomName::Movie => "moov",
            AtomName::MediaData => "mdat",
            AtomName::Free => "free",
            AtomName::MovieHeader => "mvhd",
            AtomName::Track => "trak",
            AtomName::Udata => "udata",
            AtomName::TrackHeader => "tkhd",
            AtomName::EditLists => "rdts",
            AtomName::Media => "mdia",
            AtomName::MediaInfo => "minf",
            AtomName::SampleTable => "stbl",
            AtomName::SampleDesc => "stsd",
            AtomName::Other => "other"
        }
    }
}

