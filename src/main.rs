use std::{fs::File, path::{Path, PathBuf}};
use clap::Parser;

mod las;
mod utils;

use las::{header::{PublicHeaderBlock, PublicHeaderBlockRaw}, LasFile};


#[derive(Parser)]
struct Cli {
    input_file: Option<String>,
    output_file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    
    let input = if let Some(input) = cli.input_file {
        PathBuf::from(&input)
    } else {
        panic!("input file must be specified");
    };

    let in_file = File::open(&input)?;

    let output = if let Some(out) = cli.output_file{
        PathBuf::from(&out)
    } else {
        let p = PathBuf::from(&format!("{}.txt", input.file_stem().unwrap().to_str().unwrap()));
        println!("Output file not specified. Using {}", p.display());
        p
    };

    let mut out_file = File::create(&output)?;

    let las = LasFile::new_from_file(in_file);

    las.write_points_to_file(&mut out_file);
    Ok(())
}

