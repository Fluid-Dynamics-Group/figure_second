use crate::inkscape;

use std::path::PathBuf;
use std::path::Path;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufWriter;
use anyhow::Result;
use anyhow::Context;

struct Updater {
    base_file: PathBuf,
    output_file: PathBuf,
}

impl Updater {
    fn new(base_file: PathBuf, output_file:PathBuf) -> Self {
        Self {
            base_file, output_file
        }
    }

    /// show all available ids of `Rectangle` and `Image` types in the inkscape file
    fn ids(&self) -> Result<Vec<String>> {
        let inkscape = read_inkscape(&self.base_file)?;
        Ok(inkscape.ids().map(Into::into).collect::<Vec<String>>())
    }

    fn update(&self, map: HashMap<String, PathBuf>) -> Result<()> {
        let mut inkscape = read_inkscape(&self.base_file)?;

        for (k,v) in map {
            let base64_encoding = inkscape::EncodedImage::from_path(&v)
                .with_context(|| format!("failed to encode to BASE64 for id `{k}`"))?;

            inkscape.id_to_image(&k,base64_encoding)
                .with_context(|| format!("failed to update inkscape structure for id `{k}`"))?;
        }

        // write the updated inkscape file to to `self.output_file`
        let writer = std::fs::File::create(&self.output_file)
            .with_context(|| format!("failed to create output inkscape file at {}", self.output_file.display()))?;
        let write_buf = BufWriter::new(writer);
        inkscape.write_svg(write_buf).unwrap();

        Ok(())
    }
}

fn read_inkscape(path: &Path) -> Result<inkscape::Inkscape> {
    let reader = std::fs::File::open(&path)
        .with_context(|| format!("failed to open input inkscape file {}", path.display()))?;
    let buf = BufReader::new(reader);
    let mut buffer = Vec::new();
    let out = inkscape::Inkscape::parse_svg(buf, &mut buffer)
        .with_context(|| format!("failed to parse input svg {} - this should not happen if you have a valid inkscape file", path.display()))?;
    Ok(out)
}
