use wasm_bindgen::prelude::*;

pub fn read_file_sync(path: &str) -> Vec<u8> {
    let fs = node_require("fs");
    fs.readFileSync(path)
}

pub fn write_file_sync(path: &str, data: &[u8]) {
    let fs = node_require("fs");
    fs.writeFileSync(path, data);
}

pub fn append_file_sync(path: &str, data: &[u8]) {
    let fs = node_require("fs");
    fs.appendFileSync(path, data);
}

pub fn copy_file_sync(path_src: &str, path_dest: &str) {
    let fs = node_require("fs");
    fs.copyFileSync(path_src, path_dest);
}

pub fn rename_sync(path_src: &str, path_dest: &str) {
    let fs = node_require("fs");
    fs.renameSync(path_src, path_dest);
}

pub fn unlink_sync(path: &str) {
    let fs = node_require("fs");
    fs.unlinkSync(path);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = require)]
    fn node_require(s: &str) -> NodeFs;

    #[derive(Clone, Debug)]
    type NodeFs;

    #[wasm_bindgen(method, js_name = readFileSync, structural)]
    fn readFileSync(me: &NodeFs, path: &str) -> Vec<u8>;

    #[wasm_bindgen(method, js_name = writeFileSync, structural)]
    fn writeFileSync(me: &NodeFs, path: &str, data: &[u8]);

    #[wasm_bindgen(method, js_name = appendFileSync, structural)]
    fn appendFileSync(me: &NodeFs, path: &str, data: &[u8]);

    #[wasm_bindgen(method, js_name = copyFileSync, structural)]
    fn copyFileSync(me: &NodeFs, path_src: &str, path_dest: &str);

    #[wasm_bindgen(method, js_name = renameSync, structural)]
    fn renameSync(me: &NodeFs, path_src: &str, path_dest: &str);

    #[wasm_bindgen(method, js_name = unlinkSync, structural)]
    fn unlinkSync(me: &NodeFs, path: &str);
}
