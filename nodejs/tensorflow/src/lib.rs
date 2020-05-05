use wasm_bindgen::prelude::*;
use tract_tensorflow::prelude::*;
use nodejs_helper;
use std::io::Cursor;

#[wasm_bindgen]
pub fn infer(params: &str) -> String {
    let ps: (String, String, usize, usize) = serde_json::from_str(params).unwrap();
    let model_filename = &ps.0;
    let image_filename = &ps.1;
    let image_height = ps.2;
    let image_width = ps.3;

    nodejs_helper::console::log(&format!("Model: {:?}", model_filename));
    nodejs_helper::console::log(&format!("Image: {:?}", image_filename));

    let model_data = &nodejs_helper::fs::read_file_sync(model_filename);
    let image_data = &nodejs_helper::fs::read_file_sync(image_filename);

    let res: (f32, u32) = infer_impl (model_data, image_data, image_height, image_width).unwrap();
    return serde_json::to_string(&res).unwrap();
    /*
    if res.1 == 0 {
        return serde_json::to_string("None detected").unwrap();
    } else {
        let s = format!("detected object id {} with probability {}", res.1, res.0);
        return serde_json::to_string(&s).unwrap();
    }
    */
    // return serde_json::to_string(&res).unwrap();

    // let s = format!("{:?}", res);
    // nodejs_helper::console::log(&s);
    // return "success".to_string();
}

fn infer_impl (model_data: &[u8], image_data: &[u8], image_height: usize, image_width: usize) -> TractResult<(f32, u32)> {

    nodejs_helper::console::time("Inference");

    // load the model
    let mut model_data_mut = Cursor::new(model_data);
    let mut model =
        tract_tensorflow::tensorflow().model_for_read(&mut model_data_mut)?;
    nodejs_helper::console::time_log("Inference", "Model loaded");

    // specify input type and shape
    model.set_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, image_height, image_width, 3)))?;

    // optimize the model and get an execution plan
    let model = model.into_optimized()?;
    let plan = SimplePlan::new(&model)?;
    nodejs_helper::console::time_log("Inference", "Plan loaded");

    // open image, resize it and make a Tensor out of it
    // let image = image::open(image_filename).unwrap().to_rgb();
    let image = image::load_from_memory(image_data).unwrap().to_rgb();
    nodejs_helper::console::time_log("Inference", "Image loaded");
    let resized =
        image::imageops::resize(&image, image_height as u32, image_width as u32, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, image_height, image_width, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();
    nodejs_helper::console::time_log("Inference", "Image resized");

    // run the plan on the input
    let result = plan.run(tvec!(image))?;
    nodejs_helper::console::time_log("Inference", "Model applied");

    // find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // let s = format!("result: {:?}", best);
    // nodejs_helper::console::log(&s);
    nodejs_helper::console::time_end("Inference");
    match best {
        Some(t) => Ok(t),
        None => Ok((0.0, 0)),
    }
}
