use serde_json::json;
use std::env;
use std::error::Error;
use std::io::{self, Read};
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let img_width: u64 = args[1].parse::<u64>().unwrap();
    let img_height: u64 = args[2].parse::<u64>().unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    let mut flattened: Vec<f32> = Vec::new();

    // Parse stdin: vactor<flattended u8>.
    io::stdin().read_to_end(&mut buffer)?;
    for num in buffer {
        flattened.push(num.into());
    }

    // Load up the graph as a byte array and create a tensorflow graph.
    let model = include_bytes!("mtcnn.pb");
    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?;

    // The `input` tensor expects BGR pixel data.
    let input = Tensor::new(&[img_height, img_width, 3]).with_values(&flattened)?;

    // Use input params from the existing module.
    let min_size = Tensor::new(&[]).with_values(&[20f32])?;
    let thresholds = Tensor::new(&[3]).with_values(&[0.6f32, 0.7f32, 0.7f32])?;
    let factor = Tensor::new(&[]).with_values(&[0.709f32])?;
    let mut args = SessionRunArgs::new();

    // Load default parameters and input image.
    args.add_feed(&graph.operation_by_name_required("min_size")?, 0, &min_size);
    args.add_feed(
        &graph.operation_by_name_required("thresholds")?,
        0,
        &thresholds,
    );
    args.add_feed(&graph.operation_by_name_required("factor")?, 0, &factor);
    args.add_feed(&graph.operation_by_name_required("input")?, 0, &input);

    // Request the following outputs after the session runs.
    let bbox = args.request_fetch(&graph.operation_by_name_required("box")?, 0);
    let session = Session::new(&SessionOptions::new(), &graph)?;
    session.run(&mut args)?;

    // Our bounding box extents.
    let bbox_res: Tensor<f32> = args.fetch(bbox)?;

    // Print results.
    let mut iter = 0;
    let mut json_vec: Vec<[f32; 4]> = Vec::new();
    while (iter * 4) < bbox_res.len() {
        json_vec.push([
            bbox_res[4 * iter + 1], // x1
            bbox_res[4 * iter],     // y1
            bbox_res[4 * iter + 3], // x2
            bbox_res[4 * iter + 2], // y2
        ]);
        iter += 1;
    }
    let json_obj = json!(json_vec);
    println!("{}", json_obj.to_string());
    Ok(())
}
