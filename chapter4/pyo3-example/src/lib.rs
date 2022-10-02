use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn fib(a: u64) -> PyResult<u64> {
    if a <= 2 {
        return Ok(1);
    }
    Ok(fib(a - 2)? + fib(a - 1)?)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    Ok(())
}
