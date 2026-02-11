def flatten_dict(prefix: str, src_keys: list[str], src_vals: list[str], out_keys: list[str], out_vals: list[str]) -> int:
    i: int = 0
    while i < len(src_keys):
        sk: str = src_keys[i]
        sv: str = src_vals[i]
        full: str = sk
        if len(prefix) > 0:
            full = prefix + "." + sk
        out_keys.append(full)
        out_vals.append(sv)
        i = i + 1
    return len(out_keys)

def build_dot_path(parts: list[str]) -> str:
    result: str = ""
    i: int = 0
    while i < len(parts):
        if i > 0:
            result = result + "."
        result = result + parts[i]
        i = i + 1
    return result

def split_dot_path(path: str) -> list[str]:
    parts: list[str] = []
    cur: str = ""
    i: int = 0
    while i < len(path):
        ch: str = path[i]
        if ch == ".":
            parts.append(cur)
            cur = ""
        else:
            cur = cur + ch
        i = i + 1
    if len(cur) > 0:
        parts.append(cur)
    return parts

def depth_of_path(path: str) -> int:
    parts: list[str] = split_dot_path(path)
    return len(parts)

def count_at_depth(paths: list[str], target_depth: int) -> int:
    count: int = 0
    i: int = 0
    while i < len(paths):
        if depth_of_path(paths[i]) == target_depth:
            count = count + 1
        i = i + 1
    return count

def test_module() -> int:
    passed: int = 0
    ok: list[str] = []
    ov: list[str] = []
    flatten_dict("root", ["a", "b"], ["1", "2"], ok, ov)
    if len(ok) == 2 and ok[0] == "root.a":
        passed = passed + 1
    if ov[1] == "2":
        passed = passed + 1
    p: str = build_dot_path(["x", "y", "z"])
    if p == "x.y.z":
        passed = passed + 1
    parts: list[str] = split_dot_path("a.b.c")
    if len(parts) == 3 and parts[1] == "b":
        passed = passed + 1
    if depth_of_path("a.b.c") == 3:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
