use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn hello_rust() -> PyResult<String> {
    Ok("Hello from rust".to_string())
}


#[pymodule]
fn rustchess(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_rust,m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
