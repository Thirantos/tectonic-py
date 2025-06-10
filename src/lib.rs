use std::ops::Deref;
use tectonic;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn as_detailed_message(err: &dyn Error) -> String {
    let mut msg = err.to_string();
    let mut current = err.source();
    while let Some(cause) = current {
        msg.push_str("\nCaused by: ");
        msg.push_str(&cause.to_string());
        current = cause.source();
    }
    msg
}


#[pyfunction]
fn latex_to_pdf(py: Python, source: &str) -> PyResult<Py<PyBytes>> {
    match tectonic::latex_to_pdf(source) {
        Ok(pdf_bytes) => Ok(PyBytes::new(py, &pdf_bytes).into()),
        Err(err) => Err(pyo3::exceptions::PyRuntimeError::new_err(as_detailed_message(&err)),
    }
}


/// Module definition
#[pymodule]
fn tectonic_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(latex_to_pdf, m)?)?;
    Ok(())
}
