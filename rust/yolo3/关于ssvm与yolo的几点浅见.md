这篇文章主要讲解当前ssvm-tensorflow-interface存在的两种问题：代码与编译问题。首先从wasm-learning中的faas部分看起。faas中使用tensorflow的大致分为两类任务：分类与检测，使用到的模型分别为.pb && .tflite文件，对应着ssvm-tensorflow-interface的两个api（下面会说到）。

在这些任务中，使用最多的是：

```
 let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlow);
 let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
 let res_vec: Vec<f32> = session.get_output("");
```

以上代码是ssvm-tensorflow-interface的session create以及获取tensor的api，用途为创建会话，运行会话以及获取pb&&tflite模型推理得到的张量，下面再来看ssvm-tensorflow-interface的的代码，一共有一下几类，都是根据tensorflow c api的底层代码封装得到的：

```
pub fn new<S: AsRef<[u8]>>(model_buf: S, mod_type: ModelType) -> Session
pub fn add_input<T: TensorType>(
        &mut self,
        name: &str,
        tensor_buf: &[T],
        shape: &[i64],
    ) -> &mut Session
pub fn add_output(&mut self, name: &str) -> &mut Session
pub fn clear_input(&mut self) -> &mut Session
pub fn run(&mut self) -> &mut Session
pub fn get_output<T: TensorType>(&self, name: &str) -> Vec<T> 
```

上述代码的功能分别是模型会话的创建，输入的添加，输出的添加，清除输入，运行会话，以及获取张量。

以上这些可以处理一些简单的分类任务，比如通过模型推理获取概率分数，然后通过各种数值操作或排序操作得到最终结果。但是如果是复杂类型的任务，比如yolo则只能得到模型的推理结果，而不能进行接下来复杂的张量操作。先来看yolo的一段python代码

```
interpreter = tf.lite.Interpreter(model_path=FLAGS.weights)
        interpreter.allocate_tensors()
        input_details = interpreter.get_input_details()
        output_details = interpreter.get_output_details()
        print(input_details)
        print(output_details)
        interpreter.set_tensor(input_details[0]['index'], images_data)
        interpreter.invoke()
        pred = [interpreter.get_tensor(output_details[i]['index']) for i in range(len(output_details))]
```

上述代码为tensorflow yolo4的一段python脚本，可以看到这段代码可以用ssvm-tensorflow-interface的两种代码来获取结果：

```
pub fn new<S: AsRef<[u8]>>(model_buf: S, mod_type: ModelType) -> Session
pub fn get_output<T: TensorType>(&self, name: &str) -> Vec<T> 
```

但是前提是在知晓其tensor name的条件下才可以。tfhub官网已有的pb和tflite文件，已给出了tensorname，因此可以使用get output来获取张量结果。但是yolo是由yolo.weights权重文件转换为yolo.tflite，无法得到其tensorname（进行了多次实验，均失败），这是第一点。

下面再看一段代码：

```
def filter_boxes(box_xywh, scores, score_threshold=0.4, input_shape = tf.constant([416,416])):
    scores_max = tf.math.reduce_max(scores, axis=-1)

    mask = scores_max >= score_threshold
    class_boxes = tf.boolean_mask(box_xywh, mask)
    pred_conf = tf.boolean_mask(scores, mask)
    class_boxes = tf.reshape(class_boxes, [tf.shape(scores)[0], -1, tf.shape(class_boxes)[-1]])
    pred_conf = tf.reshape(pred_conf, [tf.shape(scores)[0], -1, tf.shape(pred_conf)[-1]])

    box_xy, box_wh = tf.split(class_boxes, (2, 2), axis=-1)

    input_shape = tf.cast(input_shape, dtype=tf.float32)

    box_yx = box_xy[..., ::-1]
    box_hw = box_wh[..., ::-1]

    box_mins = (box_yx - (box_hw / 2.)) / input_shape
    box_maxes = (box_yx + (box_hw / 2.)) / input_shape
    boxes = tf.concat([
        box_mins[..., 0:1],  # y_min
        box_mins[..., 1:2],  # x_min
        box_maxes[..., 0:1],  # y_max
        box_maxes[..., 1:2]  # x_max
    ], axis=-1)
    # return tf.concat([boxes, pred_conf], axis=-1)
    return (boxes, pred_conf)
```

上述代码是在获取yolo模型结果之后对张量进行的各种tensor operation。可以看到代码中有

```
tf.math.reduce_max()
tf.boolean_mask()
tf.reshape()
tf.split()
tf.cast()
tf.shape() 
tf.concat()
```

获取张量最大值，求布尔掩码，重组，分割，转换，求维度数组，链接等各种tensor operation。这些复杂的tensor operation在ssvm-tensorflow-interface并不存在。经过研究ndarry等rust编码的相关lib后，虽然其中存在reshape等操作。但是这些lib对于tensor数据结构的定义各不相同，有的转换后会造成tensor shape mismatch的问题，有些参数，转换的结果并不一致，且没有获取维度数组的操作。

另外就是已封装好的算法，比如非最大值抑制算法（nms）：

```
    boxes, scores, classes, valid_detections = tf.image.combined_non_max_suppression(
        boxes=tf.reshape(boxes, (tf.shape(boxes)[0], -1, 1, 4)),
        scores=tf.reshape(
            pred_conf, (tf.shape(pred_conf)[0], -1, tf.shape(pred_conf)[-1])),
        max_output_size_per_class=50,
        max_total_size=50,
        iou_threshold=FLAGS.iou,
        score_threshold=FLAGS.score
    )
```

以上是在tensorflow中由c++编码的底层算法，再由python调用。作用是去除图片中多余的box，而rust里没有已实现的nms算法。代码问题至此结束，下面阐述编译问题。

在使用ssvmup与cargo wasm编译时，出现了同样的错误：

```
error: failed to run custom build command for `bzip2-sys v0.1.10+1.0.8`

Caused by:
  process didn't exit successfully: `/home/azureuser/yolo/target/release/build/bzip2-sys-6b94e045c2574d83/build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-env-changed=BZIP2_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_wasm32-unknown-unknown
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_wasm32_unknown_unknown
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_wasm32-unknown-unknown
  cargo:rerun-if-env-changed=PKG_CONFIG_wasm32_unknown_unknown
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_wasm32-unknown-unknown
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_wasm32_unknown_unknown
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  TARGET = Some("wasm32-unknown-unknown")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-unknown-linux-gnu")
  CC_wasm32-unknown-unknown = None
  CC_wasm32_unknown_unknown = None
  TARGET_CC = None
  CC = None
  CFLAGS_wasm32-unknown-unknown = None
  CFLAGS_wasm32_unknown_unknown = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  running: "clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-I" "bzip2-1.0.8" "-D_FILE_OFFSET_BITS=64" "-DBZ_NO_STDIO" "-o" "/home/azureuser/yolo/target/wasm32-unknown-unknown/release/build/bzip2-sys-85d63e3a4766ba0d/out/lib/bzip2-1.0.8/blocksort.o" "-c" "bzip2-1.0.8/blocksort.c"

  --- stderr


  error occurred: Failed to find tool. Is `clang` installed?
```

这个错误通过google并没有找到解决方法，在一些wasm的github中提了issue，得到的回复是wasi目前并不支持cross compile。

总结

当前ssvm tensorflow interface存在的问题，个人认为当前的api只能处理简单的分类任务，不适用复杂的张量操作。必须要定义完整的tensor数据结构，以及各类的tensor operation，然后解决如何能编译成wasm的问题才可以。另外，关于pb&&tflite模型文件的问题，因为tfhub官网的pb文件不能直接用于wasm，所以pb的api存在的意义不大（个人意见）。我们的目标是将rust编码的模型在云端编译为wasm，所以必不可少的要有一套成体系的tensor数据结构以及tensor操作。