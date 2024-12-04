use std::{fs::File, io::{Read, Seek}};

use crate::utils::{read_bytes, Array120};

#[derive(Debug)]
pub struct PublicHeaderBlock {
    file_signature: String,
    source_id: u16,
    global_encoding: u16,
    project_id: String,
    version: String,
    system_id: String,
    generating_software: String,
    file_creation: String,
    header_size: u16,
    pub offset_point: u32,
    pub point_format: u8,
    pub point_lenght: u16,
    pub point_records: u64,
    // TODO Number of Points by Return
    pub x_scale_factor: f64,
    pub y_scale_factor: f64,
    pub z_scale_factor: f64,
}

impl PublicHeaderBlock {
    pub fn new_from_raw(raw: &PublicHeaderBlockRaw) -> Self {
        let file_signature = std::str::from_utf8(&raw.file_signature).unwrap().to_owned();
        let source_id = u16::from_le_bytes(raw.source_id);
        let global_encoding = u16::from_le_bytes(raw.global_encoding);
        let project_id = {
            format!("{:.4}-{}-{}-{}",
                u32::from_le_bytes(raw.pid_guid_data_1),
                u16::from_le_bytes(raw.pid_guid_data_2),
                u16::from_le_bytes(raw.pid_guid_data_3),
                std::str::from_utf8(&raw.pid_guid_data_4).unwrap(),
            )
        };

        let version = {
            format!("{}.{}",
                char::from(raw.version_major[0]),
                char::from(raw.version_minor[0]),
            )
        };

        let system_id = std::str::from_utf8(&raw.system_id).unwrap().to_owned().trim_end_matches('\0').to_owned();
        let generating_software = std::str::from_utf8(&raw.generating_software).unwrap().to_owned().trim_end_matches('\0').to_owned();

        let file_creation = {
            format!("{}/{}",
                u16::from_le_bytes(raw.creation_day_of_year),
                u16::from_le_bytes(raw.creation_year),
            )
        };

        let header_size = u16::from_le_bytes(raw.header_size);
        let offset_point = u32::from_le_bytes(raw.offset_point);
        let point_format = u8::from_le_bytes(raw.point_data_record_format);
        let point_lenght = u16::from_le_bytes(raw.point_data_record_lenght);

        let point_records = if version == "1.4" {
            u64::from_le_bytes(raw.nmr_point_records)
        } else {
            u32::from_le_bytes(raw.legacy_nmr_point_records) as u64
        };

        let x_scale_factor = f64::from_le_bytes(raw.x_scale_factor);
        let y_scale_factor = f64::from_le_bytes(raw.y_scale_factor);
        let z_scale_factor = f64::from_le_bytes(raw.z_scale_factor);

        Self {
            file_signature,
            source_id,
            global_encoding,
            project_id,
            version,
            system_id,
            generating_software,
            file_creation,
            header_size,
            offset_point,
            point_format,
            point_lenght,
            point_records,
            x_scale_factor,
            y_scale_factor,
            z_scale_factor,
        }
    }
}

#[derive(Debug, Default)]
pub struct PublicHeaderBlockRaw {
    file_signature: [u8; 4],
    source_id: [u8; 2],
    global_encoding: [u8; 2],
    pid_guid_data_1: [u8; 4],
    pid_guid_data_2: [u8; 2],
    pid_guid_data_3: [u8; 2],
    pid_guid_data_4: [u8; 8],
    version_major: [u8; 1],
    version_minor: [u8; 1],
    system_id: [u8; 32],
    generating_software: [u8; 32],
    creation_day_of_year: [u8; 2],
    creation_year: [u8; 2],
    header_size: [u8; 2],
    pub offset_point: [u8; 4],
    nmr_var_len_records: [u8; 4],
    pub point_data_record_format: [u8; 1],
    pub point_data_record_lenght: [u8; 2],
    legacy_nmr_point_records: [u8; 4],
    legacy_nmr_point_return: [u8; 20],

    x_scale_factor: [u8; 8],
    y_scale_factor: [u8; 8],
    z_scale_factor: [u8; 8],

    x_offset: [u8; 8],
    y_offset: [u8; 8],
    z_offset: [u8; 8],

    x_max: [u8; 8],
    y_max: [u8; 8],
    z_max: [u8; 8],

    x_min: [u8; 8],
    y_min: [u8; 8],
    z_min: [u8; 8],

    start_waveform_data_record: [u8; 8],
    start_first_ext_var_record: [u8; 8],
    nmr_ext_var_record: [u8; 4],
    nmr_point_records: [u8; 8],
    nmr_points_return: Array120,
}

impl PublicHeaderBlockRaw {
    
    pub fn new_from_reader<T: Read + Seek>(file: &mut T) -> Self {
        let mut header_buf = Self::default();

        let mut acc = 0;

        read_bytes(&mut header_buf.file_signature, file, &mut acc);
        read_bytes(&mut header_buf.source_id, file, &mut acc);
        read_bytes(&mut header_buf.global_encoding, file, &mut acc);
        read_bytes(&mut header_buf.pid_guid_data_1, file, &mut acc);
        read_bytes(&mut header_buf.pid_guid_data_2, file, &mut acc);
        read_bytes(&mut header_buf.pid_guid_data_3, file, &mut acc);
        read_bytes(&mut header_buf.pid_guid_data_4, file, &mut acc);
        read_bytes(&mut header_buf.version_major, file, &mut acc);
        read_bytes(&mut header_buf.version_minor, file, &mut acc);
        read_bytes(&mut header_buf.system_id, file, &mut acc);
        read_bytes(&mut header_buf.generating_software, file, &mut acc);
        read_bytes(&mut header_buf.creation_day_of_year, file, &mut acc);
        read_bytes(&mut header_buf.creation_year, file, &mut acc);
        read_bytes(&mut header_buf.header_size, file, &mut acc);
        read_bytes(&mut header_buf.offset_point, file, &mut acc);
        read_bytes(&mut header_buf.nmr_var_len_records, file, &mut acc);
        read_bytes(&mut header_buf.point_data_record_format, file, &mut acc);
        read_bytes(&mut header_buf.point_data_record_lenght, file, &mut acc);
        read_bytes(&mut header_buf.legacy_nmr_point_records, file, &mut acc);
        read_bytes(&mut header_buf.legacy_nmr_point_return, file, &mut acc);
        
        read_bytes(&mut header_buf.x_scale_factor, file, &mut acc);
        read_bytes(&mut header_buf.y_scale_factor, file, &mut acc);
        read_bytes(&mut header_buf.z_scale_factor, file, &mut acc);

        read_bytes(&mut header_buf.x_offset, file, &mut acc);
        read_bytes(&mut header_buf.y_offset, file, &mut acc);
        read_bytes(&mut header_buf.z_offset, file, &mut acc);

        read_bytes(&mut header_buf.x_max, file, &mut acc);
        read_bytes(&mut header_buf.y_max, file, &mut acc);
        read_bytes(&mut header_buf.z_max, file, &mut acc);

        read_bytes(&mut header_buf.x_min, file, &mut acc);
        read_bytes(&mut header_buf.y_min, file, &mut acc);
        read_bytes(&mut header_buf.z_min, file, &mut acc);

        read_bytes(&mut header_buf.start_waveform_data_record, file, &mut acc);
        read_bytes(&mut header_buf.start_first_ext_var_record, file, &mut acc);
        read_bytes(&mut header_buf.nmr_ext_var_record, file, &mut acc);

        read_bytes(&mut header_buf.nmr_point_records, file, &mut acc);

        read_bytes(&mut header_buf.nmr_points_return.0, file, &mut acc);

        header_buf
    }
}


