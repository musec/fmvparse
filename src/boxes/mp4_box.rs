/*
 * © 2022 Arastoo Bozorgi
 * © 2022 Samir Dharar
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

    /// Get the internal boxes of this box
    fn fields(&self) -> Option<Vec<&dyn Mp4Box>>;

    /// The box level
    fn level(&self) -> u8;
}

impl_downcast!(Mp4Box);

impl std::fmt::Debug for dyn Mp4Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {{size: {}, start address: {}, end address: {}}}",
            self.name(),
            self.size(),
            self.start(),
            self.start() + self.size() as u64,
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

        let indent = self.level();
        if let Some(foo) = self.downcast_ref::<super::media::ChunkOffsetBox>() {
            for _ in 0..indent + 1 {
                write!(f, "\t")?;
            }
            let offset = foo.offsets.to_vec();
            if offset.len() < 10 {
                write!(f, "offsets: {:?}", offset)?;
            } else {
                writeln!(
                    f,
                    "offsets:{{{:?}, {:?}, {dots}}}",
                    offset[0],
                    offset[1],
                    dots = "..."
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AtomName {
    FileType,
    Wide,
    Movie,
    MediaData,
    Free,
    Skip,
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
            "wide" => AtomName::Wide,
            "mdat" => AtomName::MediaData,
            "moov" => AtomName::Movie,
            "free" => AtomName::Free,
            "skip" => AtomName::Skip,
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
            AtomName::Wide => "wide",
            AtomName::MediaData => "mdat",
            AtomName::Movie => "moov",
            AtomName::Free => "free",
            AtomName::Skip => "skip",
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
