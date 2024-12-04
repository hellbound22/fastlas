use std::fs::File;
use std::io::BufReader;
use std::io::Read;


use memmap2::Mmap;

use self::header::{PublicHeaderBlockRaw, PublicHeaderBlock};
use self::points::PointCloud;

pub mod header;
pub mod points;


#[derive(Debug)]
pub struct LasFile {
    header: header::PublicHeaderBlock,
    pub cloud: points::PointCloud,
}

impl LasFile {
    pub fn new_from_file(file: File) -> Self {
        let mmap = unsafe { Mmap::map(&file).unwrap()  };

        let mut reader = BufReader::new(file);
        let raw_head = PublicHeaderBlockRaw::new_from_reader(&mut reader);
        let header = PublicHeaderBlock::new_from_raw(&raw_head);
        
        dbg!(&header);

        //let mut buf = Vec::new();
        //reader.read_to_end(&mut buf).unwrap();
        //

        //let cloud = PointCloud::parse_number(&mmap, &header, 10);
        let cloud = PointCloud::parse_all(&mmap, &header);


        Self { header, cloud }
    }
}


