use figure_second::inkscape;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./static/simple-inkscape-drawing.svg");
    let output_svg = PathBuf::from("./static/simple-inkscape-drawing-output.svg");

    let figure = PathBuf::from("./static/graph_output_example.png");

    let reader = std::fs::File::open(&path).unwrap();
    let buf = BufReader::new(reader);

    //let out: inkscape::Svg  = quick_xml::de::from_reader(&mut buf).unwrap();

    let mut buffer = Vec::new();
    let out = inkscape::parse_svg(buf, &mut buffer).unwrap();

    dbg!(&out);

    let writer = std::fs::File::create(&output_svg).unwrap();
    let write_buf = BufWriter::new(writer);

    inkscape::write_svg(write_buf, out).unwrap();
}
