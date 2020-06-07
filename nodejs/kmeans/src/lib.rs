use wasm_bindgen::prelude::*;
// use ndarray::{Array2, ArrayView1, ArrayView2};
use ndarray::{Array2};
use std::str::FromStr;

#[wasm_bindgen]
pub fn fit (csv_content: &[u8], dim: i32, num_clusters: i32) -> String {
    let data = read_data(csv_content, dim as usize);
    let (means, _clusters) = rkm::kmeans_lloyd(&data.view(), num_clusters as usize);

    // The following code groups the points into clusters around the means 
    // let data_view = data.view();
    // let groups = separate_groups(&data_view, &clusters);
    return serde_json::to_string(&means).unwrap();
}

fn read_data(csv_content: &[u8], dim: usize) -> Array2<f32> {
    let v : Vec<u8> = csv_content.to_vec();
    println!("INPUT length is {}", v.len());

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

/*
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
*/
