# Hunt Mode tracking file - used to verify repro fixes compile
# Current status: GREEN (last verified: E0392 - PhantomData for unused type params)
# DEPYLER-0765: Empty Generic[T] class now correctly adds PhantomData<T>

from typing import Generic, TypeVar

T = TypeVar("T")


class Box(Generic[T]):
    """An empty generic container."""

    pass


def main() -> None:
    """Main entry point."""
    b: Box[int] = Box()
    print("Created empty box")


if __name__ == "__main__":
    main()
