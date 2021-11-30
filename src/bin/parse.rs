use fmvparse::reader;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
    }

    let path_to_read = &args[1];
    println!("File: {}", path_to_read);
    if let Ok(filename) = std::fs::File::open(path_to_read) {
        let mut reader = std::io::BufReader::new(filename);
        let mut writer: Vec<u8> = vec![];
        std::io::copy(&mut reader, &mut writer).unwrap();
        let boxes = reader::gen_boxes(&writer[..]);
        // print out boxes
        for b in boxes {
            b.show_boxes();
        }
    }
    Ok(())
}
