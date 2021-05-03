import tensorflow as tf
interpreter = tf.lite.Interpreter(model_path="detect.tflite")
interpreter.allocate_tensors()

# Get the input details
input_details = interpreter.get_input_details()
print("Input details:")
print(input_details)

# Get the output details
output_details = interpreter.get_output_details()
print("Output details:")
print(output_details)

print("de_boxes")
#print(output_details[0]['index'][0])
print(output_details[0]['index'])
#det_classes = interpreter.get_tensor(output_details[1]['index'])[0]
#det_scores = interpreter.get_tensor(output_details[2]['index'])[0]
#num_det = interpreter.get_tensor(output_details[3]['index'])[0]