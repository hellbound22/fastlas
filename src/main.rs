
mod las;
mod utils;

use las::{header::{PublicHeaderBlock, PublicHeaderBlockRaw}, LasFile};


use std::fs::File;


fn main() -> std::io::Result<()> {
    let file = File::open("../ot_Garopaba_Classified.las")?;

    let las = LasFile::new_from_file(file);
    
    //dbg!(&las);

    let mut new = File::create("../rs_parsed.txt")?;
    las.write_points_to_file(&mut new);
    Ok(())
}

