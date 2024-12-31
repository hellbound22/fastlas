use clap::{Parser, Subcommand};

use std::{fs::File, path::PathBuf};

mod las;
mod utils;

use las::LasFile;


#[derive(Subcommand, Clone)]
enum Command {
    Header,
}

#[derive(Parser)]
struct Cli {
    input_file: Option<String>,
    output_file: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    
    let input = if let Some(input) = cli.input_file {
        PathBuf::from(&input)
    } else {
        panic!("input file must be specified");
    };

    let mut las = LasFile::new_from_path(&input);

    match cli.command {
        Some(Command::Header) => {
            dbg!(las.header);
        },
        None => {
            let output = if let Some(out) = cli.output_file{
                PathBuf::from(&out)
            } else {
                let p = PathBuf::from(&format!("{}.txt", input.file_stem().unwrap().to_str().unwrap()));
                println!("Output file not specified. Using {}", p.display());
                p
            };

            las.read_point_cloud();

            let mut out_file = File::create(&output)?;
            las.write_points_to_file(&mut out_file);
        }
    }

    Ok(())
}

