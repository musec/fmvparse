/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{Mp4Atom, AtomName, FType, Movie, MediaData, Free, InnerAtom};
use std::path::Path;
use crate::error::Error;
use std::fs::File;
use std::io::Read;
use byteorder::{BigEndian, ByteOrder};


pub struct Mp4 {
    atoms: Vec<Box<dyn Mp4Atom>>,
    size: usize,
    file: String
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

        let mut atoms = vec![];
        let mut index = 0;

        while index < buffer.len() {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&buffer[index..index + 4]) as usize;
            let name = std::str::from_utf8(&buffer[index + 4..index + 8])?;
            let name = AtomName::from(name);

            let atom = match name {
                AtomName::FType => {
                    Box::new(
                        FType::parse(&buffer[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::Movie => {
                    Box::new(
                    Movie::parse(&buffer[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                },
                AtomName::MediaData => {
                    Box::new(
                    MediaData::parse(&buffer[index..size], index)?
                    )
                        as Box<dyn Mp4Atom>
                }
                AtomName::Free => {
                    Box::new(
                    Free::parse(&buffer[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                },
                _ => {
                    Box::new(
                        InnerAtom::parse(&buffer[index..index + size], index)?
                    )
                        as Box<dyn Mp4Atom>
                }
            };

            atoms.push(atom);
            index += size;
        }


        Ok(Mp4 {
            atoms,
            size: file_size as usize,
            file: path.to_str().unwrap().to_string()
        })
    }
}

impl std::fmt::Debug for Mp4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "file {}, with size {} has the following structure:\n",
                self.file, self.size
        );

        for atom in self.atoms.iter() {
            write!(f, "{:?} \n", atom);
        }
        Ok(())
    }
}