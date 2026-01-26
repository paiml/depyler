import json
import subprocess
import sys
import os
from pathlib import Path
import difflib
import time
import concurrent.futures

# Configuration
CORPUS_DIR = "examples"
MANIFEST_FILE = "corpus_manifest_v1.json"
TIMEOUT_SECONDS = 10

class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'

def print_status(message, status="INFO"):
    if status == "INFO":
        print(f"[INFO] {message}")
    elif status == "PASS":
        print(f"{Colors.OKGREEN}[PASS]{Colors.ENDC} {message}")
    elif status == "FAIL":
        print(f"{Colors.FAIL}[FAIL]{Colors.ENDC} {message}")
    elif status == "WARN":
        print(f"{Colors.WARNING}[WARN]{Colors.ENDC} {message}")

def run_command(cmd, capture=True, timeout=None):
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=capture,
            text=True,
            timeout=timeout
        )
        return result
    except subprocess.TimeoutExpired:
        return None

def get_corpus_files():
    if os.path.exists(MANIFEST_FILE):
        try:
            with open(MANIFEST_FILE, 'r') as f:
                data = json.load(f)
                # Assuming manifest is a list of filenames or objects with 'filename'
                if isinstance(data, list):
                    return [f if isinstance(f, str) else f.get('file', f.get('filename')) for f in data]
                elif isinstance(data, dict):
                    # Handle different manifest formats
                    files = data.get('files', [])
                    # Extract 'path' from each file entry if it's a dict
                    return [f['path'] if isinstance(f, dict) else f for f in files]
        except Exception as e:
            print_status(f"Could not read manifest: {e}", "WARN")
    
    # Fallback to glob
    print_status("Falling back to globbing examples/*.py", "WARN")
    return [str(p) for p in Path(CORPUS_DIR).glob("*.py")]

def verify_file(py_file_path):
    file_name = os.path.basename(py_file_path)
    base_name = os.path.splitext(file_name)[0]
    rs_file_path = py_file_path.replace(".py", ".rs")
    bin_path = f"./{base_name}_bin"

    # 1. Transpile
    # Note: Using --force or similar if needed, assuming 'depyler transpile' is the command
    transpile_cmd = f"cargo run --quiet --release --bin depyler -- transpile {py_file_path}"
    t_res = run_command(transpile_cmd, timeout=30)
    
    if not t_res or t_res.returncode != 0:
        return {
            "file": file_name,
            "status": "TRANSPILE_FAIL",
            "details": t_res.stderr if t_res else "Timeout"
        }

    # 2. Compile Rust
    # We use rustc directly to verify it's a valid standalone binary
    compile_cmd = f"rustc {rs_file_path} -o {bin_path}"
    c_res = run_command(compile_cmd, timeout=30)
    
    if not c_res or c_res.returncode != 0:
        return {
            "file": file_name,
            "status": "COMPILE_FAIL",
            "details": c_res.stderr if c_res else "Timeout"
        }

    # 3. Run Python (Reference)
    py_cmd = f"python3 {py_file_path}"
    py_res = run_command(py_cmd, timeout=TIMEOUT_SECONDS)
    
    if not py_res:
         return {"file": file_name, "status": "PYTHON_TIMEOUT", "details": "Python execution timed out"}

    # 4. Run Rust (Candidate)
    rs_res = run_command(bin_path, timeout=TIMEOUT_SECONDS)
    
    # Cleanup binary
    if os.path.exists(bin_path):
        os.remove(bin_path)

    if not rs_res:
        return {"file": file_name, "status": "RUST_TIMEOUT", "details": "Rust execution timed out"}

    # 5. Compare
    py_out = py_res.stdout.strip()
    rs_out = rs_res.stdout.strip()
    
    if py_out != rs_out:
        # Generate diff
        diff = list(difflib.unified_diff(
            py_out.splitlines(), 
            rs_out.splitlines(), 
            fromfile='Python', 
            tofile='Rust', 
            lineterm=''
        ))
        return {
            "file": file_name,
            "status": "SEMANTIC_MISMATCH",
            "details": "\n".join(diff[:10])  # Show first 10 lines of diff
        }

    return {"file": file_name, "status": "SUCCESS", "details": ""}

def main():
    print(f"{Colors.HEADER}=== DR. POPPER'S ABSOLUTE FALSIFIER ==={Colors.ENDC}")
    print("Verifying the claim of 100% Semantic Parity...")
    
    files = get_corpus_files()
    files = [f for f in files if f] # Filter None
    
    # Resolve paths
    files = [f if os.path.exists(f) else os.path.join(CORPUS_DIR, os.path.basename(f)) for f in files]
    files = [f for f in files if os.path.exists(f)]

    print(f"Target Corpus: {len(files)} files")
    
    results = []
    failures = 0
    
    # Run in parallel for speed, but verify strictly
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        future_to_file = {executor.submit(verify_file, f): f for f in files}
        
        for i, future in enumerate(concurrent.futures.as_completed(future_to_file)):
            f = future_to_file[future]
            try:
                res = future.result()
                results.append(res)
                
                if res["status"] == "SUCCESS":
                    print(f"[{i+1}/{len(files)}] {Colors.OKGREEN}✔ {os.path.basename(f)}{Colors.ENDC}")
                else:
                    failures += 1
                    print(f"[{i+1}/{len(files)}] {Colors.FAIL}✘ {os.path.basename(f)}{Colors.ENDC} - {res['status']}")
                    if res['details']:
                        print(f"{Colors.WARNING}  > {res['details'].replace(chr(10), chr(10)+'  > ')}{Colors.ENDC}")
            except Exception as exc:
                failures += 1
                print(f"[{i+1}/{len(files)}] {Colors.FAIL}✘ {os.path.basename(f)}{Colors.ENDC} - EXCEPTION: {exc}")

    print(f"\n{Colors.HEADER}=== FINAL VERDICT ==={Colors.ENDC}")
    
    success_count = len(files) - failures
    success_rate = (success_count / len(files)) * 100 if files else 0
    
    print(f"Files Checked: {len(files)}")
    print(f"Verified Successes: {success_count}")
    print(f"Failures: {failures}")
    print(f"True Success Rate: {success_rate:.2f}%")
    
    if failures == 0 and len(files) > 0:
        print(f"\n{Colors.OKGREEN}{Colors.BOLD}VERDICT: CORROBORATED.{Colors.ENDC}")
        print("The claim of 100% parity stands... for now.")
    else:
        print(f"\n{Colors.FAIL}{Colors.BOLD}VERDICT: FALSIFIED.{Colors.ENDC}")
        print("The claim of 100% parity is FALSE.")
        print("The theory must be discarded or revised.")
        sys.exit(1)

if __name__ == "__main__":
    main()
