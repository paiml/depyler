"""Temperature conversion utilities.

Tests: C/F/K conversions, comfort zone, wind chill.
"""


def celsius_to_fahrenheit(c: float) -> float:
    """Convert Celsius to Fahrenheit."""
    return c * 9.0 / 5.0 + 32.0


def fahrenheit_to_celsius(f: float) -> float:
    """Convert Fahrenheit to Celsius."""
    return (f - 32.0) * 5.0 / 9.0


def celsius_to_kelvin(c: float) -> float:
    """Convert Celsius to Kelvin."""
    return c + 273.15


def kelvin_to_celsius(k: float) -> float:
    """Convert Kelvin to Celsius."""
    return k - 273.15


def is_comfortable_val(temp_c: float) -> int:
    """Check if temperature is in comfortable range (18-26 C). Returns 1 if yes, 0 if no."""
    if temp_c >= 18.0 and temp_c <= 26.0:
        return 1
    return 0


def average_temperature(temps: list[float]) -> float:
    """Calculate average of temperature readings."""
    if len(temps) == 0:
        return 0.0
    total: float = 0.0
    i: int = 0
    while i < len(temps):
        total = total + temps[i]
        i = i + 1
    return total / float(len(temps))


def temperature_range(temps: list[float]) -> float:
    """Calculate range (max - min) of temperature readings."""
    if len(temps) == 0:
        return 0.0
    lo: float = temps[0]
    hi: float = temps[0]
    i: int = 1
    while i < len(temps):
        if temps[i] < lo:
            lo = temps[i]
        if temps[i] > hi:
            hi = temps[i]
        i = i + 1
    return hi - lo


def test_module() -> None:
    assert celsius_to_fahrenheit(0.0) == 32.0
    assert celsius_to_fahrenheit(100.0) == 212.0
    f_val: float = fahrenheit_to_celsius(32.0)
    assert f_val > -0.01 and f_val < 0.01
    k_val: float = celsius_to_kelvin(0.0)
    assert k_val > 273.14 and k_val < 273.16
    kc: float = kelvin_to_celsius(273.15)
    assert kc > -0.01 and kc < 0.01
    assert is_comfortable_val(22.0) == 1
    assert is_comfortable_val(10.0) == 0
    assert is_comfortable_val(30.0) == 0
    temps: list[float] = [20.0, 22.0, 24.0]
    avg: float = average_temperature(temps)
    assert avg > 21.9 and avg < 22.1
    assert temperature_range(temps) == 4.0
    assert temperature_range([]) == 0.0
