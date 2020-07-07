use wasm_bindgen::prelude::*;
use tract_tensorflow::prelude::*;
use std::io::Cursor;

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> String {
    let model_data = include_bytes!("mobilenet_v2_1.4_224_frozen.pb");
    let res: (f32, u32) = infer_impl (model_data, image_data, 224, 224).unwrap();

    let labels = include_str!("imagenet_slim_labels.txt");
    let mut label_lines = labels.lines();
    for _i in 1..res.1 {
      label_lines.next();
    }
    let ret: (String, f32) = (label_lines.next().unwrap().to_string(), res.0);
    return serde_json::to_string(&ret).unwrap();
}

fn infer_impl (model_data: &[u8], image_data: &[u8], image_height: usize, image_width: usize) -> TractResult<(f32, u32)> {
    // load the model
    let mut model_data_mut = Cursor::new(model_data);
    let mut model = tract_tensorflow::tensorflow().model_for_read(&mut model_data_mut)?;
    
    // specify input type and shape
    model.set_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, image_height, image_width, 3)))?;
    // optimize the model and get an execution plan
    let model = model.into_optimized()?;
    let plan = SimplePlan::new(&model)?;
    
    // open image, resize it and make a Tensor out of it
    let image = image::load_from_memory(image_data).unwrap().to_rgb();
    let resized = image::imageops::resize(&image, image_height as u32, image_width as u32, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, image_height, image_width, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();
    
    // run the plan on the input
    let result = plan.run(tvec!(image))?;
    
    // find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    match best {
        Some(t) => Ok(t),
        None => Ok((0.0, 0)),
    }
}
