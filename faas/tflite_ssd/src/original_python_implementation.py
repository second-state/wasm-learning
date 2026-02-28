# pip3 install wget

import cv2
from PIL import Image
import wget
import numpy as np
import os
from tflite_model_maker.config import ExportFormat
from tflite_model_maker import model_spec
from tflite_model_maker import object_detector
import tensorflow as tf
assert tf.__version__.startswith('2')
tf.get_logger().setLevel('ERROR')
from absl import logging
logging.set_verbosity(logging.ERROR)
spec = model_spec.get('efficientdet_lite0')
train_data, validation_data, test_data = object_detector.DataLoader.from_csv('gs://cloud-ml-data/img/openimage/csv/salads_ml_use.csv')
model = object_detector.create(train_data, model_spec=spec, batch_size=8, train_whole_model=True, validation_data=validation_data)
model.evaluate(test_data)
model.export(export_dir='/media/nvme')
model.evaluate_tflite('/media/nvme/model.tflite', test_data)
model_path = "/media/nvme/model.tflite"

classes = ["???"] * model.model_spec.config.num_classes
label_map = model.model_spec.config.label_map
for label_id, label_name in label_map.as_dict().items():
    classes[label_id - 1] = label_name

# Define a list of colors for visualization
COLORS = np.random.randint(0, 255, size=(len(classes), 3), dtype=np.uint8)


def preprocess_image(image_path, input_size):
    """Preprocess the input image to feed to the TFLite model"""
    img = tf.io.read_file(image_path)
    img = tf.io.decode_image(img, channels=3)
    img = tf.image.convert_image_dtype(img, tf.uint8)
    original_image = img
    resized_img = tf.image.resize(img, input_size)
    resized_img = resized_img[tf.newaxis, :]
    return resized_img, original_image


def set_input_tensor(interpreter, image):
    """Set the input tensor."""
    tensor_index = interpreter.get_input_details()[0]["index"]
    input_tensor = interpreter.tensor(tensor_index)()[0]
    input_tensor[:, :] = image


def get_output_tensor(interpreter, index):
    """Retur the output tensor at the given index."""
    output_details = interpreter.get_output_details()[index]
    tensor = np.squeeze(interpreter.get_tensor(output_details["index"]))
    return tensor


def detect_objects(interpreter, image, threshold):
    set_input_tensor(interpreter, image)
    interpreter.invoke()
    boxes = get_output_tensor(interpreter, 0)
    classes = get_output_tensor(interpreter, 1)
    scores = get_output_tensor(interpreter, 2)
    count = int(get_output_tensor(interpreter, 3))
    results = []
    for i in range(count):
        if scores[i] >= threshold:
            result = {
                "bounding_box": boxes[i],
                "class_id": classes[i],
                "score": scores[i],
            }
            results.append(result)
    return results

def run_odt_and_draw_results(image_path, interpreter, threshold=0.5):
    _, input_height, input_width, _ = interpreter.get_input_details()[0]["shape"]
    preprocessed_image, original_image = preprocess_image(
        image_path, (input_height, input_width)
    )
    results = detect_objects(interpreter, preprocessed_image, threshold=threshold)
    original_image_np = original_image.numpy().astype(np.uint8)
    for obj in results:
        ymin, xmin, ymax, xmax = obj["bounding_box"]
        xmin = int(xmin * original_image_np.shape[1])
        print("xmin plus: ")
        print(original_image_np.shape[1])
        xmax = int(xmax * original_image_np.shape[1])
        ymin = int(ymin * original_image_np.shape[0])
        print("ymin plus: ")
        print(original_image_np.shape[0])
        ymax = int(ymax * original_image_np.shape[0])
        class_id = int(obj["class_id"])
        color = [int(c) for c in COLORS[class_id]]
        cv2.rectangle(original_image_np, (xmin, ymin), (xmax, ymax), color, 2)
        y = ymin - 15 if ymin - 15 > 15 else ymin + 15
        label = "{}: {:.0f}%".format(classes[class_id], obj["score"] * 100)
        cv2.putText(
            original_image_np, label, (xmin, y), cv2.FONT_HERSHEY_SIMPLEX, 0.5, color, 2
        )
    original_uint8 = original_image_np.astype(np.uint8)
    return original_uint8

INPUT_IMAGE_URL = "https://storage.googleapis.com/cloud-ml-data/img/openimage/3/2520/3916261642_0a504acd60_o.jpg"

DETECTION_THRESHOLD = 0.3

TEMP_FILE = '/media/nvme/image.png'

# Download image and save to above location as a 512 x 512 input

interpreter = tf.lite.Interpreter(model_path=model_path)
interpreter.allocate_tensors()

detection_result_image = run_odt_and_draw_results(
	TEMP_FILE,
    interpreter,
    threshold=DETECTION_THRESHOLD
)

