use fmvparse::reader;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
    }
    let path = &args[1];
    // --snip--
    println!("In file: {}", path);
    let file = File::open(path).expect("Something went wrong reading the file");
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
