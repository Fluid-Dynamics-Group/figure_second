use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./static/simple-inkscape-drawing.svg");
    let output_svg = PathBuf::from("./static/simple-inkscape-drawing-output.svg");

    let figure = PathBuf::from("./static/graph_output_example.png");
    let encoded_image = inkscape::EncodedImage::from_path(figure).unwrap();

    let reader = std::fs::File::open(&path).unwrap();
    let buf = BufReader::new(reader);

    let mut buffer = Vec::new();
    let mut out = inkscape::Inkscape::parse_svg(buf, &mut buffer).unwrap();

    let writer = std::fs::File::create(&output_svg).unwrap();
    let write_buf = BufWriter::new(writer);

    out.id_to_image("rect286", encoded_image).unwrap();

    out.write_svg(write_buf).unwrap();
}
