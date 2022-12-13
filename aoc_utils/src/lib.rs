mod core;

use crate::core::_submit;
use chrono::{Datelike, Local};
use pyo3::prelude::*;

/// Submits an answer for today's Advent of Code
#[pyfunction]
fn submit(py: Python<'_>, answer: String, part2: Option<bool>) -> PyResult<&PyAny> {
    let now = Local::now();
    let part2 = part2.unwrap_or(false);

    pyo3_asyncio::tokio::future_into_py(py, async move {
        _submit(now.year(), now.day(), part2, answer).await
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn aoc_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(submit, m)?)?;
    Ok(())
}
