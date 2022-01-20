/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::atom::mp4_atom::Mp4Atom;
use crate::error::Error;

pub struct Other {
    atoms: Option<Vec<Box<dyn Mp4Atom>>>,
    start: usize,
    size: usize,
    name: String
}

impl Other {
    pub fn new(start: usize, size: usize, name: String) -> Self {
        Other {
            atoms: None,
            start,
            size,
            name
        }
    }
}