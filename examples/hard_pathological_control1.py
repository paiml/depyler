# Pathological control flow: Deeply nested if/elif/else chains (6 levels)
# Tests: multi-dimensional classification with many branches


def classify_6d(a: int, b: int, c: int, d: int, e: int, f: int) -> int:
    """Classify a 6-dimensional point into one of many categories."""
    if a > 0:
        if b > 0:
            if c > 0:
                if d > 0:
                    if e > 0:
                        if f > 0:
                            return 1
                        else:
                            return 2
                    else:
                        if f > 0:
                            return 3
                        else:
                            return 4
                else:
                    if e > 0:
                        if f > 0:
                            return 5
                        else:
                            return 6
                    else:
                        return 7
            else:
                if d > 0:
                    if e > 0:
                        return 8
                    else:
                        return 9
                else:
                    return 10
        else:
            if c > 0:
                if d > 0:
                    return 11
                else:
                    if e > 0:
                        return 12
                    else:
                        return 13
            else:
                return 14
    else:
        if b > 0:
            if c > 0:
                if d > 0:
                    return 15
                else:
                    return 16
            else:
                return 17
        else:
            if c > 0:
                return 18
            else:
                if d > 0:
                    return 19
                else:
                    return 20


def priority_classifier(score: int, age: int, level: int) -> int:
    """Multi-level priority classification."""
    if score >= 90:
        if age >= 30:
            if level >= 5:
                return 1
            elif level >= 3:
                return 2
            else:
                return 3
        elif age >= 20:
            if level >= 5:
                return 4
            else:
                return 5
        else:
            return 6
    elif score >= 70:
        if age >= 30:
            return 7
        elif age >= 20:
            return 8
        else:
            return 9
    elif score >= 50:
        if level >= 5:
            return 10
        else:
            return 11
    else:
        return 12


def test_module() -> int:
    passed: int = 0
    # Test 1: all positive
    if classify_6d(1, 1, 1, 1, 1, 1) == 1:
        passed = passed + 1
    # Test 2: last negative
    if classify_6d(1, 1, 1, 1, 1, 0) == 2:
        passed = passed + 1
    # Test 3: all negative-ish
    if classify_6d(0, 0, 0, 0, 0, 0) == 20:
        passed = passed + 1
    # Test 4: mixed
    if classify_6d(1, 0, 1, 1, 0, 0) == 11:
        passed = passed + 1
    # Test 5: priority classifier
    if priority_classifier(95, 35, 6) == 1:
        passed = passed + 1
    # Test 6: mid-range
    if priority_classifier(75, 25, 3) == 8:
        passed = passed + 1
    # Test 7: low score
    if priority_classifier(30, 40, 2) == 12:
        passed = passed + 1
    return passed
