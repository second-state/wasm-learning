/// example.rs - a basic example that loads the simple iris data set.
///
/// The data set is fed into the k-means algorithm and then the means
/// and clusters are printed and plotted to a file.
///
/// This example must be run from the crate root for the relative paths
/// for the example data to be correct; this program will panic if the
/// input file cannot be found.
///
/// You can run this program with the cargo run command:
/// `cargo run --example example`
use ndarray::{Array2, ArrayView1, ArrayView2};
use plotters::prelude::*;
use std::str::FromStr;

fn read_test_data() -> Array2<f32> {
    let mut data_reader = csv::Reader::from_path("data/iris.data.csv").unwrap();
    let mut data: Vec<f32> = Vec::new();
    for record in data_reader.records() {
        for field in record.unwrap().iter() {
            let value = f32::from_str(field);
            data.push(value.unwrap());
        }
    }
    Array2::from_shape_vec((data.len() / 2, 2), data).unwrap()
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

fn plot_means_clusters(data: &ArrayView2<f32>, means: &ArrayView2<f32>, clusters: &[usize]) {
    let root_area = BitMapBackend::new("test.png", (800, 600)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let groups = separate_groups(data, clusters);

    let mut context = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("K Means Demo", ("Arial", 40))
        .build_ranged(1.8f32..5.0f32, 0.0f32..3.0f32)
        .unwrap();

    context.configure_mesh().draw().unwrap();
    context
        .draw_series(
            groups
                .0
                .iter()
                .map(|coords| Circle::new(((*coords)[0], (*coords)[1]), 3, RGBColor(100, 140, 220).filled())),
        )
        .unwrap();
    context
        .draw_series(
            groups
                .1
                .iter()
                .map(|coords| Circle::new(((*coords)[0], (*coords)[1]), 3, RGBColor(120, 80, 30).filled())),
        )
        .unwrap();
    context
        .draw_series(
            groups
                .2
                .iter()
                .map(|coords| Circle::new(((*coords)[0], (*coords)[1]), 3, RGBColor(120, 240, 70).filled())),
        )
        .unwrap();
    context
        .draw_series(
            means
                .outer_iter()
                .map(|coords| TriangleMarker::new((coords[0], coords[1]), 8, RED.filled())),
        )
        .unwrap();
}

pub fn main() {
    let data = read_test_data();
    let (means, clusters) = rkm::kmeans_lloyd(&data.view(), 3);
    println!(
        "data:\n{:?}\nmeans:\n{:?}\nclusters:\n{:?}",
        data, means, clusters
    );
    plot_means_clusters(&data.view(), &means.view(), &clusters);
}
