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
            "udata" => AtomName::Udata,
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

