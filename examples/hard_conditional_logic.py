# Complex if/elif/else chains with type annotations for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def classify_number(n: int) -> int:
    """Classify number: 0=zero, 1=positive even, 2=positive odd, 3=negative even, 4=negative odd."""
    if n == 0:
        return 0
    elif n > 0 and n % 2 == 0:
        return 1
    elif n > 0:
        return 2
    elif n % 2 == 0:
        return 3
    else:
        return 4


def grade_score(score: int) -> int:
    """Convert score to grade: 5=A, 4=B, 3=C, 2=D, 1=F."""
    if score >= 90:
        return 5
    elif score >= 80:
        return 4
    elif score >= 70:
        return 3
    elif score >= 60:
        return 2
    else:
        return 1


def clamp(value: int, lo: int, hi: int) -> int:
    """Clamp value between lo and hi."""
    if value < lo:
        return lo
    elif value > hi:
        return hi
    else:
        return value


def sign(x: int) -> int:
    """Return sign of x: -1, 0, or 1."""
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0


def bmi_category(weight_kg_x10: int, height_cm: int) -> int:
    """Return BMI category from scaled weight and height.
    1=underweight, 2=normal, 3=overweight, 4=obese."""
    if height_cm <= 0:
        return 0
    bmi_scaled: int = (weight_kg_x10 * 100 * 100) // (height_cm * height_cm)
    if bmi_scaled < 185:
        return 1
    elif bmi_scaled < 250:
        return 2
    elif bmi_scaled < 300:
        return 3
    else:
        return 4


def test_module() -> int:
    """Test all conditional logic functions."""
    assert classify_number(0) == 0
    assert classify_number(4) == 1
    assert classify_number(3) == 2
    assert classify_number(-6) == 3
    assert classify_number(-7) == 4
    assert grade_score(95) == 5
    assert grade_score(85) == 4
    assert grade_score(75) == 3
    assert grade_score(65) == 2
    assert grade_score(55) == 1
    assert clamp(5, 1, 10) == 5
    assert clamp(-5, 0, 100) == 0
    assert clamp(200, 0, 100) == 100
    assert sign(42) == 1
    assert sign(-42) == -1
    assert sign(0) == 0
    assert bmi_category(700, 170) == 2
    assert bmi_category(400, 170) == 1
    assert bmi_category(1200, 170) == 4
    return 0


if __name__ == "__main__":
    test_module()
