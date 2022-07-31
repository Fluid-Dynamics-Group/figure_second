use figure_second::inkscape;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("Hello, world!");

    let path = PathBuf::from("./static/simple-inkscape-drawing.svg");
    let reader = std::fs::File::open(&path).unwrap();
    let mut buf = BufReader::new(reader);

    let out: inkscape::Svg  = quick_xml::de::from_reader(&mut buf).unwrap();

    dbg!(&out);
}
