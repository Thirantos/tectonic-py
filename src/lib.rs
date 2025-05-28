use std::ops::Deref;
use tectonic;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
fn latex_to_pdf(py: Python, source: &str) -> PyResult<Py<PyBytes>> {
    match tectonic::latex_to_pdf(source) {
        Ok(pdf_bytes) => {
            Ok(PyBytes::new(py, &pdf_bytes).into())
        },
        Err(err) => Err(pyo3::exceptions::PyRuntimeError::new_err(err.to_string())),
    }
}


/// Module definition
#[pymodule]
fn tectonic_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(latex_to_pdf, m)?)?;
    Ok(())
}