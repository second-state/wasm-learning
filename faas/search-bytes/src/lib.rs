use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn search_bytes(haystack: &[u8], needle: &[u8]) -> String {
    match haystack.windows(needle.len()).position(|window| window == needle){
        Some(_) => "Present".to_string(),
        None => "Absent".to_string()
    }
}

#[wasm_bindgen]
pub fn search_bytes_single_input(byte_array: &[u8]) -> String {
	// Parse the byte array to fetch the first 3 bytes as a literal number
    let mut number_of_bytes_str = "".to_string();
    number_of_bytes_str.push_str(&byte_array[0].to_string());
    number_of_bytes_str.push_str(&byte_array[1].to_string());
    number_of_bytes_str.push_str(&byte_array[2].to_string());
    let number_of_bytes_usize: usize = number_of_bytes_str.parse().unwrap();
    let needle: &[u8] = &byte_array.get(2..2 + number_of_bytes_usize).unwrap();
    let haystack: &[u8] = &byte_array.get(2 + number_of_bytes_usize..).unwrap();
    // If byte array was [123, 12, 1 ...] then number_of_bytes_u64 should be 123121 i.e. 123, 121 bytes long
    // Maximum allowable value is 0.255255255 Gb i.e. 255255255 bytes
    match haystack.windows(needle.len()).position(|window| window == needle){
        Some(_) => "Present".to_string(),
        None => "Absent".to_string()
    }
}