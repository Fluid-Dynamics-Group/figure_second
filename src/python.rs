use crate::inkscape;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;

#[pyclass]
pub struct Updater {
    base_file: PathBuf,
    output_file: PathBuf,
}

#[pymethods]
impl Updater {
    #[new]
    pub fn new(base_file: PathBuf, output_file: PathBuf) -> Self {
        Self {
            base_file,
            output_file,
        }
    }

    /// show all available ids of `Rectangle` and `Image` types in the inkscape file
    pub fn ids(&self) -> PyResult<Vec<String>> {
        let inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(inkscape.ids().map(Into::into).collect::<Vec<String>>())
    }

    pub fn update(&self, map: HashMap<String, PathBuf>) -> PyResult<()> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        for (k, v) in map {
            let base64_encoding = inkscape::EncodedImage::from_path(&v)
                .with_context(|| format!("failed to encode to BASE64 for id `{k}`"))
                .map_err(|e| PyValueError::new_err(e.to_string()))?;

            inkscape
                .id_to_image(&k, base64_encoding)
                .with_context(|| format!("failed to update inkscape structure for id `{k}`"))
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        }

        // write the updated inkscape file to to `self.output_file`
        let writer = std::fs::File::create(&self.output_file)
            .with_context(|| {
                format!(
                    "failed to create output inkscape file at {}",
                    self.output_file.display()
                )
            })
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let write_buf = BufWriter::new(writer);
        inkscape.write_svg(write_buf).unwrap();

        Ok(())
    }

    pub fn dimensions(&self, id: String) -> PyResult<Dimensions> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let (width, height) = inkscape
            .dimensions(&id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(Dimensions {
            width: width.round() as usize,
            height: height.round() as usize,
        })
    }
}

#[pyclass]
pub struct Dimensions {
    width: usize,
    height: usize,
}

#[pymethods]
impl Dimensions {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
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
