use crate::task::{Model, ModelType, Task, TaskInput, TaskInputType, TaskOutput, TaskOutputType};
use argparse::ArgumentParser;
use std::cmp::Ordering;
use std::io::Read;
use wasmedge_tensorflow_interface::{Session, TensorType};

pub mod task {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Copy, Clone)]
    pub enum TaskInputType {
        #[serde(rename = "rgb8")]
        RGB8,
        #[serde(rename = "rgb32f")]
        RGB32f,
        #[serde(rename = "bgr8")]
        BGR8,
        #[serde(rename = "bgr32f")]
        BGR32f,
        #[serde(rename = "luma8")]
        Luma8,
        #[serde(rename = "luma32f")]
        Luma32f,
    }

    #[derive(Deserialize, Debug)]
    pub struct TaskInput {
        pub name: String,
        pub size: (u32, u32),
        #[serde(rename = "type")]
        pub input_type: TaskInputType,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum TaskOutputLabel {
        Path {
            #[serde(rename = "path")]
            label_path: String,
        },
    }

    impl TaskOutputLabel {
        pub fn to_vec(&self) -> Vec<String> {
            match self {
                TaskOutputLabel::Path { label_path } => {
                    let mut r = vec![];
                    let label_str = std::fs::read_to_string(label_path).unwrap();
                    for s in label_str.lines() {
                        r.push(s.to_string())
                    }
                    r
                }
            }
        }
    }

    #[derive(Deserialize, Debug, Clone, Copy)]
    pub enum TaskOutputType {
        #[serde(rename = "u8")]
        U8,
        #[serde(rename = "f32")]
        F32,
    }

    #[derive(Deserialize, Debug)]
    pub struct TaskOutput {
        pub name: String,
        #[serde(rename = "type")]
        pub output_type: TaskOutputType,
        pub label: TaskOutputLabel,
        pub top: u32,
    }

    #[derive(Deserialize, Debug)]
    pub enum ModelType {
        TensorFlow,
        TensorFlowLite,
    }

    #[derive(Deserialize, Debug)]
    pub struct Model {
        #[serde(rename = "path")]
        pub model_path: String,
        #[serde(rename = "type")]
        pub model_type: ModelType,
    }

    #[derive(Deserialize, Debug)]
    pub struct Task {
        pub model: Model,
        pub input: TaskInput,
        pub output: TaskOutput,
    }
}

fn check_png(img: &[u8]) -> bool {
    if img.len() < 8 {
        return false;
    }
    img.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
}

fn check_jpg(img: &[u8]) -> bool {
    img.starts_with(&[0xFF, 0xD8]) && img.ends_with(&[0xFF, 0xD9])
}

fn args_parse() -> (String, String) {
    let mut task_file_path = String::new();
    let mut img_path = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut task_file_path)
            .add_argument("task_file_path", argparse::Store, "task file path")
            .required();
        ap.refer(&mut img_path)
            .add_argument("image_path", argparse::Store, "image path");
        ap.parse_args_or_exit();
    }
    (task_file_path, img_path)
}

fn main() {
    let (task_file_path, img_path) = args_parse();
    let img = if img_path.is_empty() {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).unwrap();
        buf
    } else {
        std::fs::read(img_path).unwrap()
    };
    let model_task = {
        let model_str = std::fs::read_to_string(task_file_path).unwrap();
        serde_yaml::from_str::<task::Task>(&model_str).unwrap()
    };
    run(model_task, img.as_slice());
    test_session();
}

pub fn run(model_task: Task, image_data: &[u8]) {
    let Task {
        model,
        input,
        output,
    } = model_task;

    let Model {
        model_path,
        model_type,
    } = model;

    let model_data = std::fs::read(model_path).unwrap();
    let mod_type = match model_type {
        ModelType::TensorFlow => wasmedge_tensorflow_interface::ModelType::TensorFlow,
        ModelType::TensorFlowLite => wasmedge_tensorflow_interface::ModelType::TensorFlowLite,
    };
    let mut session = wasmedge_tensorflow_interface::Session::new(&model_data, mod_type);

    let img = image::load_from_memory(image_data).unwrap();
    match input.input_type {
        TaskInputType::RGB8 => {
            let img = img.to_rgb8();
            let resized = image::imageops::thumbnail(&img, input.size.0, input.size.1);
            let mut flat_img = Vec::new();
            for rgb in resized.pixels() {
                flat_img.push(rgb[0]);
                flat_img.push(rgb[1]);
                flat_img.push(rgb[2]);
            }

            session
                .add_input(
                    input.name.as_str(),
                    &flat_img,
                    &[1, input.size.0 as i64, input.size.1 as i64, 3],
                )
                .add_output(output.name.as_str())
                .run();
        }
        TaskInputType::RGB32f => {
            let img = img.to_rgb8();
            let resized = image::imageops::thumbnail(&img, input.size.0, input.size.1);
            let mut flat_img = Vec::new();
            for rgb in resized.pixels() {
                flat_img.push(rgb[0] as f32 / 255.);
                flat_img.push(rgb[1] as f32 / 255.);
                flat_img.push(rgb[2] as f32 / 255.);
            }
            session
                .add_input(
                    input.name.as_str(),
                    &flat_img,
                    &[1, input.size.0 as i64, input.size.1 as i64, 3],
                )
                .add_output(output.name.as_str())
                .run();
        }
        TaskInputType::BGR8 => {
            let img = img.to_bgr8();
            let resized = image::imageops::thumbnail(&img, input.size.0, input.size.1);
            let mut flat_img = Vec::new();
            for bgr in resized.pixels() {
                flat_img.push(bgr[0]);
                flat_img.push(bgr[1]);
                flat_img.push(bgr[2]);
            }
            session
                .add_input(
                    input.name.as_str(),
                    &flat_img,
                    &[1, input.size.0 as i64, input.size.1 as i64, 3],
                )
                .add_output(output.name.as_str())
                .run();
        }
        TaskInputType::BGR32f => {
            let img = img.to_bgr8();
            let resized = image::imageops::thumbnail(&img, input.size.0, input.size.1);
            let mut flat_img = Vec::new();
            for bgr in resized.pixels() {
                flat_img.push(bgr[0] as f32 / 255.);
                flat_img.push(bgr[1] as f32 / 255.);
                flat_img.push(bgr[2] as f32 / 255.);
            }
            session
                .add_input(
                    input.name.as_str(),
                    &flat_img,
                    &[1, input.size.0 as i64, input.size.1 as i64, 3],
                )
                .add_output(output.name.as_str())
                .run();
        }
        TaskInputType::Luma8 => {
            let img = img.to_luma8();
            let resized = image::imageops::thumbnail(&img, input.size.0, input.size.1);
            let mut flat_img = Vec::new();
            for luma in resized.pixels() {
                flat_img.push(luma[0]);
            }
            session
                .add_input(
                    input.name.as_str(),
                    &flat_img,
                    &[1, input.size.0 as i64, input.size.1 as i64, 1],
                )
                .add_output(output.name.as_str())
                .run();
        }
        TaskInputType::Luma32f => {
            let img = img.to_luma8();
            let resized = image::imageops::thumbnail(&img, input.size.0, input.size.1);
            let mut flat_img = Vec::new();
            for luma in resized.pixels() {
                flat_img.push(luma[0] as f32 / 255.);
            }
            session
                .add_input(
                    input.name.as_str(),
                    &flat_img,
                    &[1, input.size.0 as i64, input.size.1 as i64, 1],
                )
                .add_output(output.name.as_str())
                .run();
        }
    };

    let labels: Vec<String> = output.label.to_vec();

    match output.output_type {
        TaskOutputType::U8 => {
            let res_vec: Vec<u8> = session.get_output(output.name.as_str());
            let mut res: Vec<(usize, u8)> = res_vec.into_iter().enumerate().collect();
            res.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less));
            for i in 0..output.top {
                if let Some((idx, p)) = res.pop() {
                    let label_name = labels.get(idx).cloned().unwrap_or("undefine".to_string());
                    println!("{},{}", label_name, p);
                };
            }
        }
        TaskOutputType::F32 => {
            let res_vec: Vec<f32> = session.get_output(output.name.as_str());
            let mut res: Vec<(usize, f32)> = res_vec.into_iter().enumerate().collect();
            res.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less));
            for i in 0..output.top {
                if let Some((idx, p)) = res.pop() {
                    let label_name = labels.get(idx).cloned().unwrap_or("undefine".to_string());
                    println!("{},{}", label_name, p);
                };
            }
        }
    };
}