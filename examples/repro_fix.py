# Hunt Mode repro for DEPYLER-0760: Option type annotation for None-initialized variables
# Pattern: typed variable initialized to None gets Option<T> type

def main() -> None:
    # Test: Variable with type annotation assigned None
    # Expected: let maybe_name: Option<String> = None;
    maybe_name: str = None

    # Test: is_none() check (validates var_types tracking)
    if maybe_name.is_none():
        print("No name provided")


if __name__ == "__main__":
    main()
