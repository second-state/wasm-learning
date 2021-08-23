use ssvm_tensorflow_interface;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn classify(input_string: &str, data: &str) -> String {
    // Create new hash map to store the string and i32 relationship which this model depends on
    // Blank for now
    let mut map_name: HashMap<String, i32> = HashMap::new();
    println!("Input string: {:?}", input_string);
    println!("Data: {:?}", data);

    let individualLine: Vec<&str> = data.split(|c| c == '\n').collect();

    //println!("individualLine: {:?}", individualLine);
    for text in individualLine {
        println!("Processing text: {:?}", text);
        let v: Vec<&str> = text.split(|c| c == '|').collect();
        if (v.len() > 1) {
            map_name.insert(v[0].to_string(), v[1].parse::<i32>().unwrap());
        }
    }

    // Create a vec to hold the data which we will be giving to the model
    let size = 256;
    let mut vecForModel: Vec<i32> = Vec::with_capacity(size);

    // Build the input string vec by splitting on space
    let vInputString: Vec<&str> = input_string.split(|c| c == ' ').collect();
    for word in vInputString {
        println!("Processing word: {:?}", word);
        if (map_name.contains_key(word) && vecForModel.len() < 256) {
            vecForModel.push(*map_name.get(word).unwrap());
        }
    }

    // Start processing a return value
    let mut stringResult = String::from("");

    println!("Final Vec For Model: {:?}", vecForModel);
    if (vecForModel.len() > 0) {
        // Load in the model
        let model_data: &[u8] = include_bytes!("model.tflite");

        // Create a TF Lite session
        let mut session = ssvm_tensorflow_interface::Session::new(
            model_data,
            ssvm_tensorflow_interface::ModelType::TensorFlowLite,
        );

        // Top up the input if it falls short of 256
        while (vecForModel.len() < size) {
            vecForModel.push(0);
        }

        // Load TFLite input (not as original string) as byte arra0
        session.add_input("input_1", &vecForModel[..], &[1, 256]);

        // Print the input for testing purposes
        println!("{:?}", &vecForModel[..]);

        // Specify the output
        session.add_output("Identity");

        // Run
        session.run();

        // Fetch results as f32
        let tf_lite_result_as_f32: Vec<f32> = session.get_output("Identity");
        println!("Result length is: {:?}", tf_lite_result_as_f32.len());
        stringResult.push_str("The words you provided have a ");
        stringResult.push_str(&tf_lite_result_as_f32[0].to_string());
        stringResult.push_str(" negative connotation, and a ");
        stringResult.push_str(&tf_lite_result_as_f32[1].to_string());
        stringResult.push_str(" positive connotation.");
    } else {
        stringResult.push_str("None of the words you provided were recognised :(");
    }

    // Return the result
    stringResult
}
