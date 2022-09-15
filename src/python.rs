use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fmt;

use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;

use inkscape::Inkscape;

#[pyclass]
pub struct Updater {
    base_file: PathBuf,
    output_file: Option<PathBuf>,
}

#[pymethods]
impl Updater {
    #[new]
    #[args(output_file = "None")]
    pub fn new(base_file: PathBuf, output_file: Option<PathBuf>) -> Self {
        Self {
            base_file,
            output_file,
        }
    }

    /// show all available ids of `Rectangle` and `Image` types in the inkscape file
    pub fn ids(&self) -> PyResult<Vec<String>> {
        let inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let layer_names = inkscape
            .object_ids()
            .map(Into::into)
            .collect::<Vec<String>>();

        Ok(layer_names)
    }

    /// show all available ids of `Rectangle` and `Image` types in the inkscape file
    pub fn layer_names(&self) -> PyResult<Vec<String>> {
        let inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(inkscape
            .get_layers()
            .iter()
            .map(|layer| layer.name().to_string())
            .collect())
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

        let output_file = self.output_file.as_ref().unwrap_or(&self.base_file);

        write_inkscape(&output_file, inkscape).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }

    pub fn dimensions(&self, id: String) -> PyResult<Dimensions> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        let (width, height) = inkscape
            .dimensions(&id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(Dimensions { width, height })
    }

    pub fn relative_dimensions(&self, id: String, height: f64) -> PyResult<(f64, f64)> {
        let dims = self.dimensions(id)?;

        let width = height * dims.width / dims.height;

        return Ok((width, height));
    }

    #[args(method = "VisibleMethod::Name")]
    pub fn hide_layers(&self, name_or_ids: Vec<String>, method: VisibleMethod) -> PyResult<()> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        for name_or_id in name_or_ids {
            let matched_layer = inkscape.get_layers_mut().iter_mut().find(|layer| {
                if method == VisibleMethod::Name {
                    layer.name() == name_or_id
                } else {
                    layer.id() == name_or_id
                }
            });

            if let Some(layer) = matched_layer {
                layer.set_hidden();
            } else {
                return Err(PyValueError::new_err(format!(
                    "layer {method} {name_or_id} was not found in the existing {method}s"
                )));
            }
        }

        let output_file = self.output_file.as_ref().unwrap_or(&self.base_file);
        write_inkscape(&output_file, inkscape).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }

    #[args(method = "VisibleMethod::Name")]
    pub fn show_layers(&self, name_or_ids: Vec<String>, method: VisibleMethod) -> PyResult<()> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        for name_or_id in name_or_ids {
            let matched_layer = inkscape.get_layers_mut().iter_mut().find(|layer| {
                if method == VisibleMethod::Name {
                    layer.name() == name_or_id
                } else {
                    layer.id() == name_or_id
                }
            });

            if let Some(layer) = matched_layer {
                layer.set_visible();
            } else {
                return Err(PyValueError::new_err(format!(
                    "layer {method} {name_or_id} was not found in the existing {method}s"
                )));
            }
        }

        let output_file = self.output_file.as_ref().unwrap_or(&self.base_file);
        write_inkscape(&output_file, inkscape).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }

    /// set all layers to visible
    pub fn show_all_layers(&self) -> PyResult<()> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        for layer in inkscape.get_layers_mut() {
            layer.set_visible();
        }

        let output_file = self.output_file.as_ref().unwrap_or(&self.base_file);
        write_inkscape(&output_file, inkscape).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }

    /// set all layers to hidden
    pub fn hide_all_layers(&self) -> PyResult<()> {
        let mut inkscape =
            read_inkscape(&self.base_file).map_err(|e| PyValueError::new_err(e.to_string()))?;

        for layer in inkscape.get_layers_mut() {
            layer.set_hidden();
        }

        let output_file = self.output_file.as_ref().unwrap_or(&self.base_file);
        write_inkscape(&output_file, inkscape).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }

    /// export the inkscape SVG to an inkscape file
    ///
    /// exports the svg used for `self.output_file`, or `self.base_file` if none
    /// were specified
    ///
    /// under the hood, this is simply a call to the `inkscape` command line utility
    #[args(dpi = 96)]
    pub fn to_png(&self, output_path: PathBuf, dpi: usize) -> PyResult<()> {
        let sh = xshell::Shell::new().map_err(|e| PyValueError::new_err(e.to_string()))?;

        let dpi = dpi.to_string();

        let inkscape_input = self.output_file.as_ref().unwrap_or(&self.base_file);
        let input = inkscape_input.display().to_string();
        let output = output_path.display().to_string();

        xshell::cmd!(sh, "inkscape {input} --export-dpi={dpi} -o {output}")
            .quiet()
            .run()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }
}

#[pyclass]
#[derive(Clone, Eq, PartialEq)]
pub enum VisibleMethod {
    Id,
    Name,
}

impl fmt::Display for VisibleMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
        }
    }
}

#[pyclass]
pub struct Dimensions {
    width: f64,
    height: f64,
}

#[pymethods]
impl Dimensions {
    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
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

fn write_inkscape(output_file: &Path, inkscape: Inkscape) -> Result<()> {
    // write the updated inkscape file to to `self.output_file`
    let writer = std::fs::File::create(&output_file)
        .with_context(|| {
            format!(
                "failed to create output inkscape file at {}",
                output_file.display()
            )
        })
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let write_buf = BufWriter::new(writer);
    inkscape.write_svg(write_buf).unwrap();

    Ok(())
}
