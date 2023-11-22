use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello(name: String) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[pymodule]
fn my_pyo3_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
