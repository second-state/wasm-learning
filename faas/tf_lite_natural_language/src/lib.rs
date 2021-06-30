use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;

#[wasm_bindgen]
pub fn classify(input_string: String) -> String {
    // The return type of wasmEdge can only be String or Vec<u8> (not primitive f32 which is what TF Lite returns to us) so we use Vec<u8>

    // Load in the model and label data
    let model_data: &[u8] = include_bytes!("model.tflite");

    // Create a TF Lite session
    let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);

    // Load TFLite input (not as original string) as byte array
    session.add_input("input_1", input_string.as_bytes(), &[1,256]);

    // Specify the output
    session.add_output("Identity");

    // Run 
    session.run();

    // Fetch results as f32
    let tf_lite_result_as_f32: Vec<f32> = session.get_output("Identity");
    println!("Result length is: {:?}", tf_lite_result_as_f32.len());
    let mut stringResult = String::from("The words you provided have a ");
    stringResult.push_str(&tf_lite_result_as_f32[0].to_string());
    stringResult.push_str(" negative connotation, and a ");
    stringResult.push_str(&tf_lite_result_as_f32[1].to_string());
    stringResult.push_str(" positive connotation.");
    stringResult
}
