use super::boxes::*;
use byteorder::{BigEndian, ByteOrder};
use std::{boxed::Box, str, vec::Vec};

pub fn read_box(buffer: &[u8]) -> String {
    let bf_name = &buffer[..4];
    str::from_utf8(&bf_name).unwrap().to_string()
}

fn gen_box(buffer: &[u8]) -> Option<Box<dyn Mp4Box>> {
    if buffer.len() < 4 {
        return None;
    }

    let bx_size = {
        let bf_size = &buffer[..4];
        BigEndian::read_u32(bf_size)
    };

    if bx_size > buffer.len() as u32 {
        return None;
    }

    let bx_name = {
        let bf_name = &buffer[4..];
        read_box(&bf_name)
    };

    let bx_buf = &buffer[..bx_size as usize];

    let new_box: Box<dyn Mp4Box> = match bx_name.as_ref() {
        "ftyp" => Box::new(FileTypeBox::ftype_box(bx_name, bx_size, &bx_buf)),
        "moov" => {
            let mut res = MovieBox::moov_box(bx_size, bx_name);
            let boxes = gen_boxes(&buffer[8..res.calc_size()]);
            res.set_box(boxes);
            Box::new(res)
        }
        _ => Box::new(BoxGen::parse_box(bx_name, bx_size)),
    };

    Some(new_box)
}

pub fn gen_boxes(mut buffer: &[u8]) -> Vec<Box<dyn Mp4Box>> {
    let mut output = Vec::new();
    loop {
        match gen_box(buffer) {
            Some(bx) => {
                buffer = &buffer[bx.calc_size()..];
                output.push(bx);
            }
            _ => break,
        }
    }
    output
}
