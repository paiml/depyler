"""Semantic parity test: reverse string."""


def reverse(s: str) -> str:
    result = ""
    i = len(s) - 1
    while i >= 0:
        result = result + s[i]
        i = i - 1
    return result


def main() -> None:
    print(reverse("hello"))


if __name__ == "__main__":
    main()
