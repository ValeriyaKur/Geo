use pyo3::prelude::*;
use geopolars::geodataframe::GeoDataFrame;
use rstar::{RTree, AABB};

#[pyfunction]
fn spatial_join(_py: Python, data1: &GeoDataFrame, data2: &GeoDataFrame) -> GeoDataFrame {
    // Создаем пространственный индекс для data2
    let tree = RTree::bulk_load(data2.geometry().iter().cloned().zip(data2.iter()));

    // Производим пространственное соединение
    let result: Vec<_> = data1
        .geometry()
        .iter()
        .enumerate()
        .flat_map(|(idx, geom)| {
            tree.locate_in_envelope(&geom.envelope())
                .map(move |(_, obj)| (idx, obj))
        })
        .collect();

    // Создаем новый GeoDataFrame с результатом
    let mut result_df = GeoDataFrame::new(data1.get_columns());
    result_df.add_column("result", result);

    result_df
}

#[pymodule]
fn spatial_index(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spatial_join, m)?)?;
    Ok(())
}
