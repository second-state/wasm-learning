import os
import time

from flask import g, jsonify
from flask import Flask


def create_app(test_config=None):
    # create and configure the app
    app = Flask(__name__, instance_relative_config=True)
    app.config.from_mapping(SECRET_KEY="dev")

    if test_config is None:
        # load the instance config, if it exists, when not testing
        app.config.from_pyfile("config.py", silent=True)
    else:
        # load the test config if passed in
        app.config.from_mapping(test_config)

    # ensure the instance folder exists
    try:
        os.makedirs(app.instance_path)
    except OSError:
        pass

    import WasmEdge

    log = WasmEdge.Logging()
    log.debug()
    app.wasm_vm = WasmEdge.VM()
    return app


app = create_app()


@app.route("/")
def hello_world():
    return "<p>Hello, World!</p>"


# a simple page that says hello
@app.route("/hello-flask")
def hello():
    return "Flask: Hello, World!"


@app.route("/fibo_wasm/<n>")
def fibo_wasm(n):
    n = int(n)
    fib_wasm = os.path.join(
        os.path.abspath(os.path.join(os.path.abspath(__file__), os.pardir, os.pardir)),
        "wasm_bins/fibonacci.wasm",
    )
    if n < 25:
        res, result = app.wasm_vm.run(fib_wasm, "fib", [n])
    else:
        return "Avoid hogging... n should be less than 25"

    diff = time.time() - g.start
    return jsonify({"result": result, "time": diff}), 200


def fibonacci(n):
    a = 0
    b = 1

    # Check is n is less
    # than 0
    if n < 0:
        print("Incorrect input")

    # Check is n is equal
    # to 0
    elif n == 0:
        return 0

    # Check if n is equal to 1
    elif n == 1:
        return b
    else:
        for i in range(1, n):
            c = a + b
            a = b
            b = c
        return b


@app.route("/fibo_py/<n>")
def fibo_py(n):
    n = int(n)+1

    if n < 25:
        try:
            result = fibonacci(n)
        except:
            return f"Cannot compute fibo of {n}", 400
    else:
        return "Avoid hogging... n should be less than 25"

    diff = time.time() - g.start
    return jsonify({"result": result, "time": diff}), 200


@app.before_request
def before_request():
    g.start = time.time()
