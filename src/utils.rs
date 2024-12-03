use std::io::prelude::*;

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
