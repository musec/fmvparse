/*
 * © 2022 Arastoo Bozorgi
 * All rights reserved.
 */

use crate::boxes::{InnerAtom, Mp4Box};
use crate::error::Error;
use crate::Header;
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Default)]
pub struct SampleTable {
    stsd: Option<Box<dyn Mp4Box>>, // sample description
    stts: Option<Box<dyn Mp4Box>>, // (decoding) time-to-sample
    ctts: Option<Box<dyn Mp4Box>>, // (composition) time to sample
    stss: Option<Box<dyn Mp4Box>>, // sync sample table
    sdtp: Option<Box<dyn Mp4Box>>, // independent and disposable samples
    stsc: Option<Box<dyn Mp4Box>>, //
    stsz: Option<Box<dyn Mp4Box>>, // sample sizes (framing)
    stco: Option<Box<dyn Mp4Box>>, // chunk offset, partial data-offset information
    sgpd: Option<Box<dyn Mp4Box>>, // sample group description
    sbgp: Option<Box<dyn Mp4Box>>, // sample-to-group

    header: Header,
    level: u8,
}

impl SampleTable {
    pub fn sample_desc_header_box(&self) -> Result<&InnerAtom, Error> {
        match self.stsd.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("stsd".to_string())),
        }
    }

    pub fn decoding_time_to_sample_box(&self) -> Result<&InnerAtom, Error> {
        match self.stts.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("stts".to_string())),
        }
    }

    pub fn composition_time_to_sample_box(&self) -> Result<&InnerAtom, Error> {
        match self.ctts.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("ctts".to_string())),
        }
    }

    pub fn sync_sample_box(&self) -> Result<&InnerAtom, Error> {
        match self.stss.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("stss".to_string())),
        }
    }

    pub fn ind_disp_samples_box(&self) -> Result<&InnerAtom, Error> {
        match self.sdtp.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("sdtp".to_string())),
        }
    }

    pub fn stsc_box(&self) -> Result<&InnerAtom, Error> {
        match self.stsc.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("stsc".to_string())),
        }
    }

    pub fn sample_sizes_box(&self) -> Result<&InnerAtom, Error> {
        match self.stsz.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("stsz".to_string())),
        }
    }

    pub fn offset_info_box(&self) -> Result<&InnerAtom, Error> {
        match self.stco.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("stco".to_string())),
        }
    }

    pub fn sample_group_desc_box(&self) -> Result<&InnerAtom, Error> {
        match self.sgpd.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("sgpd".to_string())),
        }
    }

    pub fn sample_to_group_box(&self) -> Result<&InnerAtom, Error> {
        match self.sbgp.as_ref() {
            Some(b) => Ok(b.downcast_ref::<InnerAtom>().unwrap()),
            None => Err(Error::BoxNotFound("sbgp".to_string())),
        }
    }
}

impl Mp4Box for SampleTable {
    fn parse(data: &[u8], start: usize, level: u8) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = Header::new(data, start)?;
        let mut sample_table = SampleTable {
            header,
            level,
            ..Default::default()
        };

        let mut index = 8; // skip the first 8 bytes that are Movie headers

        while index < data.len() {
            // the first 8 bytes includes the atom size and its name
            // The size is the entire size of the box, including the size and type header, fields, and all contained boxes.
            let size = BigEndian::read_u32(&data[index..index + 4]) as usize;
            let name = std::str::from_utf8(&data[index + 4..index + 8])?;
            // let name = AtomName::from(name);

            match name {
                "stsd" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.stsd = Some(b);
                }
                "stts" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.stts = Some(b);
                }
                "ctts" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.ctts = Some(b);
                }
                "stss" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.stss = Some(b);
                }
                "sdtp" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.sdtp = Some(b);
                }
                "stsc" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.stsc = Some(b);
                }
                "stsz" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.stsz = Some(b);
                }
                "stco" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.stco = Some(b);
                }
                "sgpd" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.sgpd = Some(b);
                }
                "sbgp" => {
                    let b = Box::new(InnerAtom::parse(
                        &data[index..index + size],
                        index + start,
                        level + 1,
                    )?) as Box<dyn Mp4Box>;
                    sample_table.sbgp = Some(b);
                }
                _ => {}
            }

            // atoms.push(atom);
            index += size;
        }

        Ok(sample_table)
    }

    fn start(&self) -> usize {
        self.header.start
    }

    fn end(&self) -> usize {
        self.header.start + self.header.size
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn name(&self) -> &str {
        self.header.name.as_ref()
    }

    fn read(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }

    fn fields(&self) -> Option<Vec<&Box<dyn Mp4Box>>> {
        let mut fields = vec![];
        if let Some(stsd) = self.stsd.as_ref() {
            fields.push(stsd);
        }
        if let Some(stts) = self.stts.as_ref() {
            fields.push(stts);
        }
        if let Some(ctts) = self.ctts.as_ref() {
            fields.push(ctts);
        }
        if let Some(stss) = self.stss.as_ref() {
            fields.push(stss);
        }
        if let Some(sdtp) = self.sdtp.as_ref() {
            fields.push(sdtp);
        }
        if let Some(stsc) = self.stsc.as_ref() {
            fields.push(stsc);
        }
        if let Some(stsz) = self.stsz.as_ref() {
            fields.push(stsz);
        }
        if let Some(stco) = self.stco.as_ref() {
            fields.push(stco);
        }
        if let Some(sgpd) = self.sgpd.as_ref() {
            fields.push(sgpd);
        }
        if let Some(sbgp) = self.sbgp.as_ref() {
            fields.push(sbgp);
        }

        Some(fields)
    }

    fn level(&self) -> u8 {
        self.level
    }
}
