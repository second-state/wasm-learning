Training models takes a lot of time and energy. There are comprehensive trained models which already exist. We just need to convert them to tflite so that we can use them. Here is a method to do just that. The following section is based on [this TensorFlow document](https://colab.research.google.com/github/tensorflow/models/blob/master/research/object_detection/colab_tutorials/convert_odt_model_to_TFLite.ipynb)

```bash
pip3 install object_detection
sudo apt-get remove python3-pygments
sudo apt-get install python3-pip python3-dev build-essential
pip3 install Pygments --upgrade
pip3 install object_detection
sudo apt-get install protobuf-compiler python-lxml python-pil
pip3 install Cython pandas tf-slim lvis


mkdir cd /media/nvme/20210803
cd /media/nvme/20210803

wget http://download.tensorflow.org/models/object_detection/tf2/20200711/ssd_mobilenet_v2_fpnlite_640x640_coco17_tpu-8.tar.gz
tar -zxvf ssd_mobilenet_v2_fpnlite_640x640_coco17_tpu-8.tar.gz

git clone https://github.com/tensorflow/models.git

export PYTHONPATH=$PYTHONPATH:/media/nvme/20210803/models/research

cd /media/nvme/20210803/models/research

protoc object_detection/protos/*.proto --python_out=.

cd /media/nvme/20210803/models/research/object_detection

python3 export_tflite_graph_tf2.py --trained_checkpoint_dir '/media/nvme/20210803/ssd_mobilenet_v2_fpnlite_640x640_coco17_tpu-8/checkpoint/' --output_directory '/media/nvme/20210803/ssd_mobilenet_v2_fpnlite_640x640_coco17_tpu-8/tflite' --pipeline_config_path '/media/nvme/20210803/ssd_mobilenet_v2_fpnlite_640x640_coco17_tpu-8/pipeline.config'
```
