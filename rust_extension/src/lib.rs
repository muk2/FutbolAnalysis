use numpy::{PyArray2, PyArray1, IntoPyArray, PyArrayMethods, ndarray::ArrayView2};
use pyo3::prelude::*;
use pyo3::Bound;

#[pyfunction]
// Corrected signature: The return type is PyResult<&'py PyArray1<f64>>
fn compute_areas<'py>(py: Python<'py>, boxes: Bound<'py, PyArray2<f64>>) -> PyResult<Bound<'py, PyArray1<f64>>> {
    // Convert to ndarray view
    let binding = boxes.readonly();
    let arr: ArrayView2<f64> = binding.as_array();

    let n = arr.shape()[0];
    let mut out = Vec::with_capacity(n);

    for i in 0..n {
        let x1 = arr[[i, 0]];
        let y1 = arr[[i, 1]];
        let x2 = arr[[i, 2]];
        let y2 = arr[[i, 3]];
        out.push((x2 - x1) * (y2 - y1));
    }

    // Convert Vec<f64> into a Python numpy array
    // Use into_pyarray_bound which is the standard way to convert a Vec owned by Rust
    // into a PyArray owned by Python.
    Ok(out.into_pyarray(py))
}

#[pymodule]
fn rust_extension(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_areas, m)?)?;
    Ok(())
}
