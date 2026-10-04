#!/usr/bin/env python3

import argparse
from PIL import Image

WIDTH = 24
HEIGHT = 24
BYTES_PER_PLANE_ROW = WIDTH // 8   # 3
BYTES_PER_ROW = BYTES_PER_PLANE_ROW * 2  # 6
IMAGE_SIZE = BYTES_PER_ROW * HEIGHT      # 144


def extract_image(filename, offset):
    with open(filename, "rb") as f:
        f.seek(offset)
        data = f.read(IMAGE_SIZE)

    if len(data) != IMAGE_SIZE:
        raise ValueError(
            f"Expected {IMAGE_SIZE} bytes at offset {offset:#x}, "
            f"but only got {len(data)}"
        )

    return data


def decode(data):
    img = Image.new("L", (24, 24))
    pixels = img.load()

    plane0 = data[:72]
    plane1 = data[72:144]

    for y in range(24):
        for x in range(24):
            i = y * 3 + x // 8
            bit = x & 7  # LSB-first

            p0 = (plane0[i] >> bit) & 1
            p1 = (plane1[i] >> bit) & 1

            value = p0 | (p1 << 1)
            pixels[x, y] = value * 85

    return img

def main():
    parser = argparse.ArgumentParser(
        description="Extract a 24x24 2-bit image from a binary file"
    )
    parser.add_argument("input", help="Input binary file")
    parser.add_argument("offset", help="Byte offset, e.g. 0x21ac or 8620")
    parser.add_argument("output", help="Output PNG file")
    parser.add_argument(
        "--scale",
        type=int,
        default=10,
        help="Nearest-neighbor output scale (default: 10)"
    )

    args = parser.parse_args()
    offset = int(args.offset, 0)
    data = extract_image(args.input, offset)
    img = decode(data)

    if args.scale != 1:
        img = img.resize(
            (WIDTH * args.scale, HEIGHT * args.scale),
            Image.Resampling.NEAREST
        )

    img.save(args.output)

    print(
        f"Extracted {IMAGE_SIZE} bytes from {args.input} "
        f"at {offset:#x} -> {args.output}"
    )


if __name__ == "__main__":
    main()