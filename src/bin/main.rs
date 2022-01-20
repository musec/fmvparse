
extern crate structopt;

// use fmvparse::reader;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use fmvparse::mp4::Mp4;

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

    let mp4 = Mp4::parse(&opt.file)?;
    println!("{:?}", mp4);
    Ok(())
}
