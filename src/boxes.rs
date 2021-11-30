use super::reader::*;
use std::{string::String, vec::Vec};

pub struct BoxGen {
    bx_name: String,
    bx_size: u32,
}

pub trait Mp4Box {
    fn calc_size(&self) -> usize;
    fn show_boxes(&self);
}

impl Mp4Box for BoxGen {
    fn calc_size(&self) -> usize {
        self.bx_size as usize
    }

    fn show_boxes(&self) {
        println!("{} <size: {:#018X}>", self.bx_name, self.bx_size);
    }
}

impl BoxGen {
    pub fn parse_box(bx_name: String, bx_size: u32) -> BoxGen {
        BoxGen {
            bx_name: bx_name,
            bx_size: bx_size,
        }
    }
}

pub struct FileTypeBox {
    parent: BoxGen,
    mj_brand: String,
    mn_brand: String,
    cmp_brands: Vec<String>,
}

impl FileTypeBox {
    pub fn ftype_box(box_name: String, box_size: u32, mut buffer: &[u8]) -> FileTypeBox {
        buffer = &buffer[4..]; //skipping 4 bits for name
        buffer = &buffer[4..]; //skipping 4 bits for size

        let mj_brand = read_box(&buffer);
        buffer = &buffer[4..];

        let mn_brand = read_box(&buffer);
        buffer = &buffer[4..];

        let mut cmp_brands: Vec<String> = vec![];
        while buffer.len() > 0 {
            cmp_brands.push(read_box(&buffer));
            buffer = &buffer[4..];
        }

        FileTypeBox {
            parent: BoxGen::parse_box(box_name, box_size),
            mj_brand: mj_brand,
            mn_brand: mn_brand,
            cmp_brands: cmp_brands,
        }
    }
}

impl Mp4Box for FileTypeBox {
    fn calc_size(&self) -> usize {
        self.parent.calc_size()
    }

    fn show_boxes(&self) {
        self.parent.show_boxes();
        println!("  major_brand: {}", self.mj_brand);
        println!("  minor_version: {}", self.mn_brand);
        println!("  compatiable_brands:");
        for brand in &self.cmp_brands {
            println!("    Compatiable_brands:'{}'", brand);
        }
    }
}

pub struct MovieBox {
    parent: BoxGen,
    boxes: Vec<Box<dyn Mp4Box>>,
}

impl MovieBox {
    pub fn moov_box(box_size: u32, box_name: String) -> MovieBox {
        assert!(box_name == "moov");
        MovieBox {
            parent: BoxGen::parse_box(box_name, box_size),
            boxes: vec![],
        }
    }

    pub fn set_box(&mut self, boxes: Vec<Box<dyn Mp4Box>>) {
        self.boxes = boxes;
    }
}

impl Mp4Box for MovieBox {
    fn calc_size(&self) -> usize {
        self.parent.calc_size()
    }

    fn show_boxes(&self) {
        self.parent.show_boxes();
        for b in &self.boxes {
            b.show_boxes()
        }
    }
}
