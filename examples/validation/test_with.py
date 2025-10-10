# Test with statement support
def read_file(filename: str) -> str:
    """Read file using with statement."""
    with open(filename) as f:
        content = f.read()
    return content

def write_file(filename: str, content: str) -> None:
    """Write file using with statement."""
    with open(filename, "w") as f:
        f.write(content)

def process_file(input_file: str, output_file: str) -> None:
    """Process file with multiple with statements."""
    with open(input_file) as fin:
        data = fin.read()

    with open(output_file, "w") as fout:
        fout.write(data.upper())
