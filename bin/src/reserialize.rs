use std::io::Read;
use std::io::Write;
use runty8_core::{Map, SpriteSheet, Flags};

fn main() -> Result<(), ()> {
    let mut args = std::env::args();
    args.next().expect("No executable name");
    if args.len() != 3 {
        eprintln!("Usage: reserialize <map|sheet|flags> input.txt output.bin");
        std::process::exit(2);
    }
    let kind = args.next().expect("No kind argument");
    let input = args.next().expect("No input argument");
    let output = args.next().expect("No output argument");

    let mut file = std::fs::File::open(input).expect("Cannot open file");
    let mut contents = String::new();
    let (map, sheet, flags);
    file.read_to_string(&mut contents).unwrap();
    let bytes = if kind == "map" {
        map = Map::deserialize(&contents).expect("Cannot deserialize");
        map.serialize_bytes()
    } else if kind == "sheet" {
        sheet = SpriteSheet::deserialize(&contents).expect("Cannot deserialize");
        sheet.serialize_bytes()
    } else if kind == "flags" {
        flags = Flags::deserialize(&contents).expect("Cannot deserialize");
        flags.serialize_bytes()
    } else {
        panic!("Wrong kind");
    };
    std::fs::write(output, bytes).expect("Unable to write file");
    Ok(())
}
