//#[cfg(feature="python")]
pub mod python;

//#[cfg(feature="python")]
#[pyo3::pymodule]
fn figure_second(_py: pyo3::Python<'_>, m: &pyo3::prelude::PyModule) -> pyo3::PyResult<()> {
    m.add_class::<python::Updater>()?;
    m.add_class::<python::Dimensions>()?;
    m.add_class::<python::VisibleMethod>()?;
    Ok(())
}
