use pyo3::prelude::*;
use geopolars::geodataframe::GeoDataFrame;
use rstar::RTree;

#[pyfunction]
fn spatial_join(_py: Python, data1: &amp;GeoDataFrame, data2: &amp;GeoDataFrame) -> GeoDataFrame {
    // Создаем пространственный индекс для data2
    let tree = RTree::bulk_load(data2.geometry().iter().cloned().zip(data2.iter()));

    // Производим пространственное соединение
    let result = data1
        .geometry()
        .iter()
        .enumerate()
        .map(|(idx, geom)| {
            let mut joined_data = vec![];
            for (_, obj) in tree.locate_in_envelope(&amp;geom.envelope()) {
                joined_data.push((idx, obj));
            }
            joined_data
        })
        .collect();

    // Создаем новый GeoDataFrame с результатом
    let mut result_df = GeoDataFrame::new(data1.get_columns());
    result_df.add_column("result", result);

    result_df
}

#[pymodule]
fn spatial_index(_py: Python, m: &amp;PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(spatial_join, m)?)?;

    Ok(())
}
