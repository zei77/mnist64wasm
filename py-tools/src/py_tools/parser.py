import re

LABEL_RE = re.compile(r"^\s*([A-Za-z_][A-Za-z0-9_]*):\s*$")

def parse_bytes(text: str, start_label: str) -> bytes:
    lines = text.splitlines()
    start_idx = None
    for i, line in enumerate(lines):
        line_no_comment = line.split("//", 1)[0].strip()

        m = LABEL_RE.match(line_no_comment)
        if m and m.group(1) == start_label:
            start_idx = i + 1
            break

    if start_idx is None:
        raise ValueError(f"Label not found: {start_label}")

    values = []
    for line in lines[start_idx:]:
        line = line.split("//", 1)[0].strip()

        if LABEL_RE.match(line):
            break

        if ".byte" not in line:
            continue

        payload = line.split(".byte", 1)[1]

        for token in payload.split(","):
            token = token.strip()

            if not token:
                continue

            if token.startswith("$"):
                values.append(int(token[1:], 16))
            else:
                values.append(int(token, 0))

    return bytes(values)