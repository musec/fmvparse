/*
 * © 2021 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::error::Error;

pub trait Mp4Atom {
    /// Read the atom from the data and parse it
    fn parse(data: &[u8], start: usize) -> Result<Self, Error> where Self: Sized;

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

    fn internals(&self) -> Option<&Vec<Box<dyn Mp4Atom>>>;
}

impl std::fmt::Debug for dyn Mp4Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "atom name: {}, start address: {}, size: {}",
            self.name(), self.start(), self.size()
        );

        let internals = self.internals();
        if internals.is_some() {
            write!(f, "\n");
            for internal in internals.unwrap() {
                write!(f, "    {:?} \n", internal);
            }
        }

        Ok(())
    }
}

pub enum AtomName {
    FType,
    Movie,
    Mdat,
    Free,
    Mvhd,
    Trak,
    Udata,
    Other
}

impl From<&str> for AtomName {
    fn from(name: &str) -> Self {
        match name {
            "ftyp" => AtomName::FType,
            "moov" => AtomName::Movie,
            "mdat" => AtomName::Mdat,
            "free" => AtomName::Free,
            "mvhd" => AtomName::Mvhd,
            "trak" => AtomName::Trak,
            "udta" => AtomName::Udata,
            _ => AtomName::Other
        }
    }
}

impl std::convert::From<AtomName> for &str {
    fn from(an: AtomName) -> Self {
        match an {
            AtomName::FType => "ftyp",
            AtomName::Movie => "moov",
            AtomName::Mdat => "mdat",
            AtomName::Free => "free",
            AtomName::Mvhd => "mvhd",
            AtomName::Trak => "trak",
            AtomName::Udata => "udata",
            AtomName::Other => "other"
        }
    }
}

impl std::fmt::Display for AtomName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomName::FType => write!(f, "ftyp"),
            AtomName::Movie => write!(f, "moov"),
            AtomName::Mdat => write!(f, "mdat"),
            AtomName::Free => write!(f, "free"),
            AtomName::Mvhd => write!(f, "mvhd"),
            AtomName::Trak => write!(f, "trak"),
            AtomName::Udata => write!(f, "udata"),
            AtomName::Other => write!(f, "other")
        }
    }
}

