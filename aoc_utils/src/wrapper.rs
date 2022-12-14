use pyo3::prelude::*;

/// Submits an answer for today's Advent of Code
#[pyfunction]
fn submit(answer: String, part2: Option<bool>) -> PyResult<()> {
    let part2 = part2.unwrap_or(false);
    crate::core::submit(&answer, part2);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn aoc_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(submit, m)?)?;
    Ok(())
}
