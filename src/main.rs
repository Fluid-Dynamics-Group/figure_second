use figure_second::inkscape;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::fs;

fn main() {
    let path = PathBuf::from("./static/simple-inkscape-drawing.svg");
    let reader = std::fs::File::open(&path).unwrap();
    let mut buf = BufReader::new(reader);

    //let out: inkscape::Svg  = quick_xml::de::from_reader(&mut buf).unwrap();

    let mut buffer = Vec::new();
    let out = inkscape::parse_svg(buf, &mut buffer).unwrap();

    dbg!(&out);
}
