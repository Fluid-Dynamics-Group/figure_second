use figure_second::inkscape;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("Hello, world!");

    //let path = PathBuf::from("./static/simple-inkscape-drawing.svg");
    let path = PathBuf::from("./static/track_inviscid_flow_from_viscous_second_partial_nice.svg");
    let out_path  = PathBuf::from("/tmp/svg_output.svg");
    let reader = std::fs::File::open(&path).unwrap();
    let mut buf = BufReader::new(reader);

    let svg: inkscape::Svg  = quick_xml::de::from_reader(&mut buf).unwrap();

    dbg!(&svg);

    println!("writing the output svg");

    let file = fs::File::create(&out_path).unwrap();
    quick_xml::se::to_writer(&file, &svg).unwrap();
}
