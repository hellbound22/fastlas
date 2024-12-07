use std::{fs::File, io::{Read, Seek}};

use bit_set::BitSet;
use memmap2::Mmap;

use crate::utils::{read_bytes, read_mmap_bytes};

use super::header::PublicHeaderBlock;

#[derive(Debug, Default)]
struct GpsTime(i64);

#[derive(Debug, Default)]
struct Color {
    r: u16,
    g: u16,
    b: u16,
}

#[derive(Debug, Default)]
struct WavePacket {
    desc_index: u8,
    byte_offset_wave_data: u64,
    wave_size: u32,
    point_wave_location: f32, 
    parametric_dx: f32,
    parametric_dy: f32,
    parametric_dz: f32,
}

#[derive(Debug, Default)]
struct Nir(u16);

#[derive(Debug, Default)]
struct Addons {
    gps_time: Option<GpsTime>,
    color: Option<Color>,
    wave_packet: Option<WavePacket>,
    nir: Option<Nir>
}

#[derive(Debug, Default)]
pub struct PointCloud {
    pub v: Vec<Point>,
}

impl PointCloud {
    pub fn parse_all(file: &Mmap, header: &PublicHeaderBlock) -> Self {
        Self::parse_number(file, header, header.point_records)
    }

    pub fn parse_number(file: &Mmap, header: &PublicHeaderBlock, number: u64) -> Self {
        let mut acc = header.offset_point as u64;
        let lenght = header.point_lenght;

        let mut v = Vec::new();

        for x in 0..number {
            let p = Point::new_from_buf(file, &mut acc, header);

            v.push(p);
        }
        
        Self {v}

    }
}

#[derive(Debug, Default)]
pub struct PointRaw {
    x: [u8; 4],
    y: [u8; 4],
    z: [u8; 4],
    intensity: [u8; 2],
    returns_vars: [u8; 2],
    classification: [u8; 1],
    scan_angle_rank: [u8; 1],
    user_data: [u8; 1],
    point_source_id: [u8; 1],
}


#[derive(Debug, Default)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    // If this is 0 convert to None
    intensity: i16,
    return_number: BitSet,
    number_of_returns: BitSet,
    scan_direction_flag: bool,
    edge_of_flight_line: bool,
    classification: u8,
    scan_angle_rank: i8,
    user_data: u8,
    point_source_id: u8,
    addons: Addons,
}

impl Point {
    pub fn format_to_txt(&self) -> String {
        format!("{} {} {}\n", self.x, self.y, self.z)
    }

    pub fn new_from_buf(file: &Mmap, acc: &mut u64, header: &PublicHeaderBlock) -> Self {
        let mut def = PointRaw::default();

        read_mmap_bytes(&mut def.x, file, acc);
        read_mmap_bytes(&mut def.y, file, acc);
        read_mmap_bytes(&mut def.z, file, acc);
        
        read_mmap_bytes(&mut def.intensity, file, acc);
        read_mmap_bytes(&mut def.returns_vars, file, acc);
        read_mmap_bytes(&mut def.classification, file, acc);
        read_mmap_bytes(&mut def.scan_angle_rank, file, acc);
        read_mmap_bytes(&mut def.user_data, file, acc);
        read_mmap_bytes(&mut def.point_source_id, file, acc);

        let x = i32::from_le_bytes(def.x) as f64 * header.x_scale_factor;
        let y = i32::from_le_bytes(def.y) as f64 * header.y_scale_factor;
        let z = i32::from_le_bytes(def.z) as f64 * header.x_scale_factor;

        let returns_vars = u16::from_le_bytes(def.returns_vars);

        let return_number = BitSet::from_bytes(&[((returns_vars & 0b0000_0111) >> 3) as u8]);
        let number_of_returns = BitSet::from_bytes(&[((returns_vars & 0b0011_1000) >> 3) as u8]);
        //let return_number = Default::default();
        //let number_of_returns = Default::default();

        let scan_direction_flag = (returns_vars & 0b0100_0000) != 0;
        let edge_of_flight_line = (returns_vars & 0b0100_0000) != 0;

        let intensity = i16::from_le_bytes(def.intensity);
        let classification = u8::from_le_bytes(def.classification);
        let scan_angle_rank = i8::from_le_bytes(def.scan_angle_rank);
        let user_data = u8::from_le_bytes(def.user_data);
        let point_source_id = u8::from_le_bytes(def.point_source_id);

        let mut addons = Addons::default();

        match header.point_format {
            1 => {
                let mut raw_gps_time = [0u8; 8];
                read_mmap_bytes(&mut raw_gps_time, file, acc);
                let gps_time = GpsTime(i64::from_le_bytes(raw_gps_time));
                addons.gps_time = Some(gps_time);
            }
            _ => unimplemented!()
        }

        Self {
            x, y, z,
            return_number,
            number_of_returns,
            scan_direction_flag,
            edge_of_flight_line,
            intensity,
            classification,
            scan_angle_rank,
            user_data,
            point_source_id,
            addons,
        }
    }
}
