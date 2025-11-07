use numpy::{PyArray2, IntoPyArray};
use ndarray::ArrayView2;
use pyo3::prelude::*;

#[pyfunction]
fn compute_areas<'py>(py: Python<'py>, boxes: Bound<'py, PyArray2<f64>>) -> PyResult<&'py PyArray2<f64>> {
    // Convert to ndarray view
    let arr: ArrayView2<f64> = boxes.readonly().as_array();

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
    Ok(out.into_pyarray(py))
}

#[pymodule]
fn rust_extension(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_areas, m)?)?;
    Ok(())
}