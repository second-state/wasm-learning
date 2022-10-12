# Flask-WasmEdge Demo

- Usage Reference: [Tests](https://github.com/SAtacker/WasmEdge/tree/pysdk/bindings/python/tests)
- Docs: [Developer Hosted](https://SAtacker.github.io/WasmEdge/)
- Note: It's advisable to use a virtual env

- Setup:

```bash
python3 -m virtualenv ~/wasm-demo
source ~/wasm-demo/bin/activate
python3 -m pip install -i https://test.pypi.org/simple/ WasmEdge==0.1.8
python3 -m pip install Flask
```

- Run:

```bash
FLASK_APP=flaskr FLASK_ENV=development python3 -m flask run
```
