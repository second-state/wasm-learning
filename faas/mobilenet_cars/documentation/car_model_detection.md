# Creating TensorFlow Car Model Detection (using Stanford Car Model)

```
pip3.9 install requests
pip3.9 install six
pip3.9 install wheel
pip3.9 install onnx
pip3.9 install tensorflow-addons
```

Then install conda from official web site

Then more system installations

```
pip3.9 install tensorflow
pip3.9 install tensorflow-addons
git clone https://github.com/onnx/onnx-tensorflow.git && cd onnx-tensorflow && pip3.9 install -e .
pip3.9 install torch
pip3.9 install torchvision
pip3.9 install logging
pip3.9 install pandas
```

```
cd /Users/tpmccallum
git clone https://github.com/onnx/onnx-tensorflow.git && cd onnx-tensorflow && pip3.9 install -e .
```

We append this location to the sys path below inside the Python code `sys.path.append("/Users/tpmccallum/onnx-tensorflow")`



Below is the Python syntax to run. But first we need to be in a location where the TensorFlow model files are located.

```bash
cd /Users/tpmccallum/SecondState/wasm-learning/faas/mobilenet_cars/src
```

```bash
python3.9
```

```
# This gets the above onnx tensorflow into scope
import sys
sys.path.append("/Users/tpmccallum/onnx-tensorflow")

import torch
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
from torchvision import datasets, transforms
from torch.autograd import Variable
import onnx
from onnx_tf.backend import prepare

class Net(nn.Module):
    def __init__(self):
        super(Net, self).__init__()
        self.conv1 = nn.Conv2d(1, 10, kernel_size=5)
        self.conv2 = nn.Conv2d(10, 20, kernel_size=5)
        self.conv2_drop = nn.Dropout2d()
        self.fc1 = nn.Linear(320, 50)
        self.fc2 = nn.Linear(50, 10)
    def forward(self, x):
        x = F.relu(F.max_pool2d(self.conv1(x), 2))
        x = F.relu(F.max_pool2d(self.conv2_drop(self.conv2(x)), 2))
        x = x.view(-1, 320)
        x = F.relu(self.fc1(x))
        x = F.dropout(x, training=self.training)
        x = self.fc2(x)
        return F.log_softmax(x, dim=1)

def train(model, device, train_loader, optimizer, epoch):
 model.train()
 for batch_idx, (data, target) in enumerate(train_loader):
     data, target = data.to(device), target.to(device)
     optimizer.zero_grad()
     output = model(data)
     loss = F.nll_loss(output, target)
     loss.backward()
     optimizer.step()
     if batch_idx % 1000 == 0:
         print('Train Epoch: {} \tLoss: {:.6f}'.format(
                 epoch,  loss.item()))

def test(model, device, test_loader):
    model.eval()
    test_loss = 0
    correct = 0
    with torch.no_grad():
        for data, target in test_loader:
            data, target = data.to(device), target.to(device)
            output = model(data)
            test_loss += F.nll_loss(output, target, reduction='sum').item() # sum up batch loss
            pred = output.max(1, keepdim=True)[1] # get the index of the maxlog-probability
            correct += pred.eq(target.view_as(pred)).sum().item()
    test_loss /= len(test_loader.dataset)
    print('\nTest set: Average loss: {:.4f}, Accuracy: {}/{} ({:.0f}%)\n'.format(test_loss, correct, len(test_loader.dataset),100. * correct / len(test_loader.dataset)))
```
Get the dataset

Dataset preprepared via https://github.com/phongdinhv/stanford-cars-model) and we already have data available in this directory i.e. `mnist.pth`


Instantiate torch like this

First change the line in `/Library/Frameworks/Python.framework/Versions/3.9/lib/python3.9/site-packages/torch/serialization.py` by getting rid of `map_location=None` and changing that to `map_location='cpu'` as shown below.

```
def load(f, map_location='cpu', pickle_module=pickle, **pickle_load_args):
```

Put the following file in the same directory where you are working (along side the mnist.pth file)

https://raw.githubusercontent.com/victoresque/pytorch-template/master/parse_config.py


Open the file and remove everything that has to do with logging and logs

Then also put the contents of this file https://raw.githubusercontent.com/victoresque/pytorch-template/master/utils/util.py (saved as a file called utils.py) in the same directory where we are working.

Then execute the following commands

```
import parse_config

import pathlib

pathlib.WindowsPath = pathlib.PosixPath
```

The following section will take the pth file and create a new mnist.pb directory which contains a new pb file amongst other things
```
trained_model = Net()
trained_model.load_state_dict(torch.load('mnist.pth')['state_dict'],strict=False)
dummy_input = Variable(torch.randn(1, 1, 28, 28))
torch.onnx.export(trained_model, dummy_input, "mnist.onnx")
model = onnx.load('mnist.onnx')
import os
os.environ['TF_ENABLE_ONEDNN_OPTS'] = '1'
tf_rep = prepare(model)
tf_rep.export_graph('mnist.pb')
```


# References

Converting .pth to .pb as per these instructions https://analyticsindiamag.com/converting-a-model-from-pytorch-to-tensorflow-guide-to-onnx/

Dataset provided via https://github.com/phongdinhv/stanford-cars-model



