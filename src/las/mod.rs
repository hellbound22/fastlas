use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

use memmap2::Mmap;

use self::header::{PublicHeaderBlockRaw, PublicHeaderBlock};
use self::points::PointCloud;

pub mod header;
pub mod points;


#[derive(Debug)]
pub struct LasFile {
    file_desc: File,
    pub header: header::PublicHeaderBlock,
    pub cloud: Option<points::PointCloud>,
}

impl LasFile {
    pub fn new_from_file(file_desc: File) -> Self {
        let mut reader = BufReader::new(&file_desc);
        let raw_head = PublicHeaderBlockRaw::new_from_reader(&mut reader);
        let header = PublicHeaderBlock::new_from_raw(&raw_head);
        
        Self { header, file_desc, cloud: None }
    }

    pub fn read_point_cloud(&mut self) {
        let mmap = unsafe { Mmap::map(&self.file_desc).unwrap()  };
        let cloud = PointCloud::parse_all(&mmap, &self.header);

        self.cloud = Some(cloud);
    }

    pub fn write_points_to_file(&self, file: &mut File) {
        if let Some(pc) = &self.cloud {
            let mut buf = BufWriter::new(file);
            for point in &pc.v {
                buf.write(point.format_to_txt().as_bytes()).unwrap();
            }
        }
    }
}


