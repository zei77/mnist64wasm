MNIST64_DIR := mnist64
PYTHON := python

.PHONY: build-wasm mnist64-clone model-params

wasm:
	cargo build -p wasm --target wasm32-unknown-unknown --release 

$(MNIST64_DIR):
	git clone https://github.com/jarnoh/mnist64.git

inference/src/params.rs: $(MNIST64_DIR) py-tools/src/py_tools/export_params.py
	uv run --project py-tools $(PYTHON) -m py_tools.export_params > inference/src/params.rs

model-params: inference/src/params.rs
