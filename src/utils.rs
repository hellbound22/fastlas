use std::io::prelude::*;

use memmap2::Mmap;

#[derive(Debug)]
pub struct Array120(pub [u8; 120]);

impl Default for Array120 {
    fn default() -> Self {
        Array120([0; 120]) // Initialize all elements to zero
    }
}

pub fn read_bytes<T: Read + Seek>(buf: &mut [u8], reader: &mut T, acc: &mut u64) {
    reader.seek(std::io::SeekFrom::Start(*acc)).unwrap();
    reader.read(buf).unwrap();
    *acc += buf.len() as u64;
}

pub fn read_mmap_bytes(buf: &mut [u8], reader: &Mmap, acc: &mut u64) {
    let range = *acc as usize..*acc as usize + buf.len();
    let r_copy = range.clone();
    if let Some(x) = reader.get(range) {
        buf.copy_from_slice(x);
        *acc += buf.len() as u64;
    } else {
        panic!("range {:?} not found", r_copy);
    };
}
