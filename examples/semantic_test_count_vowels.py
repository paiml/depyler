"""Semantic parity test: count vowels."""


def count_vowels(s: str) -> int:
    count = 0
    i = 0
    while i < len(s):
        c = s[i]
        if c == "a" or c == "e" or c == "i" or c == "o" or c == "u":
            count = count + 1
        i = i + 1
    return count


def main() -> None:
    print(count_vowels("hello world"))


if __name__ == "__main__":
    main()
