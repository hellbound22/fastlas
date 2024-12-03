use std::fs::File;
use std::io::BufReader;

use self::header::{PublicHeaderBlockRaw, PublicHeaderBlock};
use self::points::PointCloud;

pub mod header;
pub mod points;


#[derive(Debug)]
pub struct LasFile {
    header: header::PublicHeaderBlock,
    cloud: points::PointCloud,
}

impl LasFile {
    pub fn new_from_file(file: File) -> Self {
        let mut reader = BufReader::new(file);
        let raw_head = PublicHeaderBlockRaw::new_from_reader(&mut reader);
        let header = PublicHeaderBlock::new_from_raw(&raw_head);
        
        dbg!(header.point_records);

        let cloud = PointCloud::parse_number(&mut reader, &header, 1000);

        Self { header, cloud }
    }
}


