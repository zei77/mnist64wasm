#!/usr/bin/env python3
from __future__ import annotations

import numpy as np
from pathlib import Path
from .render import render_rust
from .parser import parse_bytes

def render_conv1(asm: str):
    # convert conv1 weights from asm layout [6 rows][32 filters]
    # into 32 packed 36-bit kernels, one u64 per filter.
    weights_bytes = parse_bytes(asm, "CONV1_WEIGHTS")
    weights_raw = np.frombuffer(weights_bytes, dtype=np.uint8)
    filters = weights_raw.reshape(6, 32).T.copy()
    filters &= 0x3f
    packed = np.zeros(32, dtype=np.uint64)
    for row_idx in range(6):
        packed |= filters[:, row_idx].astype(np.uint64) << np.uint64(row_idx * 6)
    print(render_rust("CONV1_WEIGHTS", packed))

    # convert weighted-match thresholds to weighted-Hamming-distance thresholds,
    # allowing the runtime to use XOR + popcount directly.
    thresholds_bytes = parse_bytes(asm, "CONV1_MATCH_THRESHOLDS")
    thresholds_data = np.frombuffer(thresholds_bytes, dtype=np.uint8)
    distance_thresholds = 108 - thresholds_data
    print(render_rust("CONV1_DISTANCE_THRESHOLDS", distance_thresholds))

def render_early_head(asm: str):
    weights_bytes = parse_bytes(asm, "EARLY_WEIGHTS")
    weights_raw = np.frombuffer(weights_bytes, dtype=np.uint8)
    weights_raw = weights_raw.reshape(400, 10)
    per_class = weights_raw.T.copy()
    early_weights = per_class.view("<u8").reshape(10, 50)
    print(render_rust("EARLY_WEIGHTS", early_weights))

    thresholds_bytes = parse_bytes(asm, "EARLY_THRESHOLDS")
    early_thresholds_data = np.frombuffer(thresholds_bytes, dtype=np.uint8)
    print(render_rust("EARLY_THRESHOLDS", early_thresholds_data))

if __name__ == "__main__":

    generated_model = Path("./mnist64/src/generated_model.asm").read_text(encoding="utf-8")
    generated_early_head = Path("./mnist64/src/generated_early_head.asm").read_text(encoding="utf-8")

    render_conv1(generated_model)
    render_early_head(generated_early_head)
