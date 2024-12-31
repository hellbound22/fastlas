use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use memmap2::Mmap;

use file_guard::Lock;
use std::fs::OpenOptions;

use self::header::{PublicHeaderBlockRaw, PublicHeaderBlock};
use self::points::PointCloud;

pub mod header;
pub mod points;


#[derive(Debug)]
pub struct LasFile {
    file_path: PathBuf,
    pub header: header::PublicHeaderBlock,
    pub cloud: Option<points::PointCloud>,
}

impl LasFile {
    pub fn new_from_path(file_path: &PathBuf) -> Self {
        let file_handle = File::open(file_path).unwrap();
        let mut reader = BufReader::new(&file_handle);
        let raw_head = PublicHeaderBlockRaw::new_from_reader(&mut reader);
        let header = PublicHeaderBlock::new_from_raw(&raw_head);
        
        let file_path = file_path.to_owned();
        Self { header, file_path, cloud: None }
    }

    pub fn read_point_cloud(&mut self) {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(&self.file_path)
            .unwrap();
        
        // WARN: this is done to remedy unsafe block by mmap
        let lock = file_guard::lock(&mut file, Lock::Exclusive, 0, 1).unwrap();
        let mmap = unsafe { Mmap::map(&**lock).unwrap()  };
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


