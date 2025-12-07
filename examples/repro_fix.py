# Hunt Mode repro for E0425: cannot find value `x` in this scope
# DEPYLER-0761: Parameter used in method call kwargs was not detected as used
# Pattern: def f(x=None): obj.method(kwarg=x) → fn f(_x: Option<T>) { obj.method(kwarg: x) }

import subprocess


def run_command(cmd: list[str], capture: bool = False, cwd: str = None) -> tuple[int, str, str]:
    """Execute a system command.

    DEPYLER-0761: When cwd is used in subprocess.run kwargs:
    - is_param_used_in_body should detect cwd usage
    - But HirExpr::MethodCall handler ignored kwargs (used ..)
    - Fixed: Now checks kwargs.iter().any() for param usage
    """
    if capture:
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=cwd)
        return (result.returncode, result.stdout, result.stderr)
    else:
        result = subprocess.run(cmd, cwd=cwd)
        return (result.returncode, "", "")


def main() -> None:
    code, out, err = run_command(["echo", "hello"], capture=True, cwd="/tmp")
    print(f"Exit code: {code}")
    print(f"Output: {out}")


if __name__ == "__main__":
    main()
