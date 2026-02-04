class TestStack:
    def __init__(self):
        self.items: list[int] = []

    def pop(self) -> int:
        if self.items:
            return self.items.pop()
        return 0

def main() -> None:
    s = TestStack()
    s.items.append(1)
    result = s.pop()
    print(result)

if __name__ == "__main__":
    main()
