use std::{fs::File, io::{Read, Seek}};

use bit_set::BitSet;

use crate::utils::read_bytes;

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
    v: Vec<Point>,
}

impl PointCloud {
    pub fn parse_number<T: Read + Seek>(file: &mut T, header: &PublicHeaderBlock, number: u64) -> Self {
        let mut acc = header.offset_point;
        let format = header.point_format;
        let lenght = header.point_lenght;


        let mut v = Vec::new();
        for x in 0..number {
            let p = Point::new_from_offset(file, acc);
            acc += lenght as u32;

            v.push(p);
            
            if x % 100_000 == 0 {
                dbg!(x);
            }
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
    x: i32,
    y: i32,
    z: i32,
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
    pub fn new_from_offset<T: Read + Seek>(file: &mut T, offset: u32) -> Self {
        let mut def = PointRaw::default();

        let mut acc = offset as u64;

        read_bytes(&mut def.x, file, &mut acc);
        read_bytes(&mut def.y, file, &mut acc);
        read_bytes(&mut def.z, file, &mut acc);
        read_bytes(&mut def.intensity, file, &mut acc);
        read_bytes(&mut def.returns_vars, file, &mut acc);
        read_bytes(&mut def.classification, file, &mut acc);
        read_bytes(&mut def.scan_angle_rank, file, &mut acc);
        read_bytes(&mut def.user_data, file, &mut acc);
        read_bytes(&mut def.point_source_id, file, &mut acc);

        let x = i32::from_le_bytes(def.x);
        let y = i32::from_le_bytes(def.y);
        let z = i32::from_le_bytes(def.z);

        let returns_vars = u16::from_le_bytes(def.returns_vars);

        let return_number = BitSet::from_bytes(&[((returns_vars & 0b0000_0111) >> 3) as u8]);
        let number_of_returns = BitSet::from_bytes(&[((returns_vars & 0b0011_1000) >> 3) as u8]);
        let scan_direction_flag = (returns_vars & 0b0100_0000) != 0;
        let edge_of_flight_line = (returns_vars & 0b0100_0000) != 0;
        // Return vars

        let intensity = i16::from_le_bytes(def.intensity);
        let classification = u8::from_le_bytes(def.classification);
        let scan_angle_rank = i8::from_le_bytes(def.scan_angle_rank);
        let user_data = u8::from_le_bytes(def.user_data);
        let point_source_id = u8::from_le_bytes(def.point_source_id);

        let addons = Addons::default();

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
