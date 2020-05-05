use wasm_bindgen::prelude::*;
use ndarray::{Array2, ArrayView1, ArrayView2};
use std::str::FromStr;

use nodejs_helper;

#[wasm_bindgen]
pub fn fit (params: &str) -> String {
    let ps: (String, usize, usize) = serde_json::from_str(params).unwrap();
    let data_path = &ps.0;
    let dim = ps.1;
    let num_clusters = ps.2;

    let data = read_data(data_path, dim);
    let (means, clusters) = rkm::kmeans_lloyd(&data.view(), num_clusters);

    let data_view = data.view();
    let groups = separate_groups(&data_view, &clusters);
    nodejs_helper::console::log(&format!("Cluster #1 has {} points", groups.0.len()));
    nodejs_helper::console::log(&format!("Cluster #2 has {} points", groups.1.len()));
    nodejs_helper::console::log(&format!("Cluster #3 has {} points", groups.2.len()));
    // nodejs_helper::console::log(&format!("means {:?} clusters {:?}", means, clusters));
    
    return serde_json::to_string(&means).unwrap();
}

fn read_data(data_path: &str, dim: usize) -> Array2<f32> {
    let csv_content: &[u8] = &nodejs_helper::fs::read_file_sync(data_path);
    let mut data_reader = csv::Reader::from_reader(csv_content);
    let mut data: Vec<f32> = Vec::new();
    for record in data_reader.records() {
        for field in record.unwrap().iter() {
            let value = f32::from_str(field);
            data.push(value.unwrap());
        }
    }
    Array2::from_shape_vec((data.len() / dim, dim), data).unwrap()
}

fn separate_groups<'a>(
    data: &'a ArrayView2<f32>,
    clusters: &[usize],
) -> (
    Vec<ArrayView1<'a, f32>>,
    Vec<ArrayView1<'a, f32>>,
    Vec<ArrayView1<'a, f32>>,
) {
    data.outer_iter().zip(clusters.iter()).fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |mut state, (point, &cluster)| {
            match cluster {
                0 => state.0.push(point),
                1 => state.1.push(point),
                2 => state.2.push(point),
                _ => (),
            }
            state
        },
    )
}
