use sled;
use log::debug;
use pyo3::{self, prelude::*, wrap_pyfunction};

#[pyclass]
#[derive(Clone)]
pub struct Db {
    inner: sled::Tree,
}

#[pyfunction]
pub fn from_disk(path: &str) -> PyResult<Db> {
    debug!("opening db: {}", path);
    let db = sled::open(path).expect("failed to open db");

    debug!("opening tree: {}", "some_tree");
    let some_tree = db.open_tree("some_tree").expect("failed to open tree");

    debug!("opened.");

    Ok(Db {
        inner: some_tree
    })
}

#[pymodule]
fn sp3e(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(from_disk, m)?)?;
    Ok(())
}
