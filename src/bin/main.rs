extern crate structopt;

// use fmvparse::reader;
use fmvparse::mp4::Mp4;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "fmv-parser",
    about = "Parse Mp4 files and extract the internal structures."
)]
struct Opt {
    #[structopt(short, long, env = "FMV_FILE")]
    /// The input file to be parsed
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let mp4 = Mp4::parse(&opt.file)?;
    println!("{:?}", mp4);
    Ok(())
}
