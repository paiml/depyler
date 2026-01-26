"""Semantic parity test: string endswith."""


def check_suffix(s: str, suffix: str) -> bool:
    return s.endswith(suffix)


def main() -> None:
    if check_suffix("hello.py", ".py"):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
