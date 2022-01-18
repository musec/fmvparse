
extern crate structopt;

use fmvparse::reader;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "fmv-parser",
about = "Parse fmv files and extract the internal structures."
)]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short, long, env = "FMV_FILE")]
    /// The input file to be parsed
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    println!("In file: {:?}", opt.file);
    let file = File::open(&opt.file).expect("Something went wrong reading the file");

    let mut reader = BufReader::new(file);
    let mut writer: Vec<u8> = vec![];
    io::copy(&mut reader, &mut writer)?;
    let atoms = reader::gen_boxes(&writer[..]);
    // print out boxes
    for atom in atoms {
        atom.show_boxes();
    }
    Ok(())
}
