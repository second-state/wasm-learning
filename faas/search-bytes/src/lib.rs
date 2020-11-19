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
	// Parse the byte array to fetch the first 10 bytes as a literal number and remove the leading zero
	// The cost of having 10 bytes to store the size (and the leading zero to ensure that one byte never exceeds 255) is negligable
	// when compared with the dissadvantage of only being able to store up to 255 bytes per byte array and/or having to manage multidimensional arrays etc.
    let mut number_of_bytes_str = "".to_string();
    for i in 0..10 {
        number_of_bytes_str.push_str(&byte_array[i].to_string());
    }
    let number_of_bytes_usize: usize = number_of_bytes_str.parse().unwrap();
    let needle: &[u8] = &byte_array.get(10..10 + number_of_bytes_usize).unwrap();
    let haystack: &[u8] = &byte_array.get(10 + number_of_bytes_usize..).unwrap();
    // Maximum allowable value is 9999999999 bytes (i.e. 9999999.999 Kb or 9.999999999 Gb)
    match haystack.windows(needle.len()).position(|window| window == needle){
        Some(_) => "Present".to_string(),
        None => "Absent".to_string()
    }
}

/*
The equivalent Javascript which creates the single byte_array is as follows
//
const buffer_1 = new ArrayBuffer(2);
const buffer_2 = new ArrayBuffer(4);
const needle = new Uint8Array(buffer_1);
const haystack = new Uint8Array(buffer_2);
needle.fill(111)
haystack.fill(222)
needle_length = needle.length;
haystack_length = haystack.length;
needle_length_string = needle_length.toString();
for (i = needle_length_string.length; i < 10; i++) {
    needle_length_string = "0" + needle_length_string;
}
if (needle_length < 1 || needle_length > 99999999 || needle_length > haystack_length) {
    console.log("The needle's length is not correct, must be between 1 and 9999999999");
} else {
    var base_array = needle_length_string.split('');
    for (i = 0; i < base_array.length; i++) base_array[i] = +base_array[i] | 0;
    console.log(base_array);
}
const buffer_to_go = new ArrayBuffer(10 + needle_length + haystack_length);
const array_to_go = new Uint8Array(buffer_to_go);
array_to_go.set(base_array);
array_to_go.set(needle, 10);
array_to_go.set(haystack, 10 + needle_length);

var settings = {
    "url": "https://rpc.ssvm.secondstate.io:8081/api/run/226/search_bytes_single_input",
    "method": "POST",
    "timeout": 0,
    "headers": {
        "Content-Type": "application/octet-stream"
    },
    "data": buffer_to_go,
};

$.ajax(settings).done(function(response) {
    console.log(response);
});
*/