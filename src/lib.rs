extern crate pyo3;

use pyo3::prelude::*;

#[pyclass]
struct DictionaryFormatter {
    #[pyo3(get)]
    name: String,
}

#[pymethods]
impl DictionaryFormatter {
    #[new]
    fn new(name: String) -> Self {
        return DictionaryFormatter { name };
    }
}

#[pymodule]
fn uf2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DictionaryFormatter>()?;
    Ok(())
}