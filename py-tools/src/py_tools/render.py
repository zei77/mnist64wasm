import numpy as np
import textwrap

def _render_array(arr: np.ndarray) -> str:
    def _to_hex(x, pattern, depth):
        if not isinstance(x, np.ndarray):
            return f"0x{x:{pattern}}"
        text = [_to_hex(y, pattern, depth + 1) for y in x]
        if depth > 0:
            return "[" + ", ".join(text) + "]"

        return ", ".join(text)

    if arr.dtype == np.uint8:
        pattern = "02x"
    elif arr.dtype == np.uint16 or arr.dtype == np.int16:
        pattern = "04x"
    elif arr.dtype == np.uint32 or arr.dtype == np.int32:
        pattern = "08x"
    elif arr.dtype == np.uint64 or arr.dtype == np.int64:
        pattern = "016x"
    else:
        raise ValueError(f"Unsupported dtype: {arr.dtype}")

    return _to_hex(arr, pattern, 0)

def render_rust(name: str, data: np.ndarray) -> str:
    wrapped = textwrap.fill(
        _render_array(data),
        width=120,
        initial_indent="    ",
        subsequent_indent="    "
    )

    if data.dtype == np.uint8:
        rust_type = "u8"
    elif data.dtype == np.uint64:
        rust_type = "u64"
    elif data.dtype == np.int64:
        rust_type = "i64"
    elif data.dtype == np.uint16:
        rust_type = "u16"
    elif data.dtype == np.uint32:
        rust_type = "u32"
    elif data.dtype == np.int16:
        rust_type = "i16"
    else:
        raise ValueError(f"Unsupported dtype: {data.dtype}")

    for size in reversed(data.shape):
        rust_type = f"[{rust_type}; {size}]"

    lines = f"pub static {name}: &{rust_type} = &[\n{wrapped}\n];"
    return lines
