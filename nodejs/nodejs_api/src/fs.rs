use wasm_bindgen::prelude::*;
use core::mem;

pub fn read_file_sync(path: &str) -> Vec<u8> {
    assert_eq!(mem::size_of::<usize>(), 4);

    let fs = node_require("fs");
    fs.node_fs_read_file_sync(path)
}

pub fn write_file_sync(path: &str, data: &[u8]) {
    assert_eq!(mem::size_of::<usize>(), 4);

    let fs = node_require("fs");
    fs.node_fs_write_file_sync(path, data);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = require)]
    fn node_require(s: &str) -> NodeFs;

    #[derive(Clone, Debug)]
    type NodeFs;

    #[wasm_bindgen(method, js_name = readFileSync, structural)]
    fn node_fs_read_file_sync(me: &NodeFs, path: &str) -> Vec<u8>;

    #[wasm_bindgen(method, js_name = writeFileSync, structural)]
    fn node_fs_write_file_sync(me: &NodeFs, path: &str, data: &[u8]);
}
