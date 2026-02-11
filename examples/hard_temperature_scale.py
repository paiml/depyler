def celsius_to_fahrenheit(c: int) -> int:
    return c * 9 // 5 + 32


def fahrenheit_to_celsius(f: int) -> int:
    return (f - 32) * 5 // 9


def celsius_to_kelvin(c: int) -> int:
    return c + 273


def kelvin_to_celsius(k: int) -> int:
    return k - 273


def fahrenheit_to_kelvin(f: int) -> int:
    c: int = fahrenheit_to_celsius(f)
    return celsius_to_kelvin(c)


def wind_chill_simple(temp_f: int, wind_mph: int) -> int:
    if wind_mph <= 3:
        return temp_f
    result: int = 35 + (temp_f - 35) * wind_mph // (wind_mph + 10)
    return result


def heat_index_simple(temp_f: int, humidity_pct: int) -> int:
    if temp_f < 80:
        return temp_f
    adj: int = humidity_pct * (temp_f - 80) // 100
    return temp_f + adj


def test_module() -> int:
    passed: int = 0
    if celsius_to_fahrenheit(0) == 32:
        passed = passed + 1
    if celsius_to_fahrenheit(100) == 212:
        passed = passed + 1
    if fahrenheit_to_celsius(32) == 0:
        passed = passed + 1
    if celsius_to_kelvin(0) == 273:
        passed = passed + 1
    if kelvin_to_celsius(373) == 100:
        passed = passed + 1
    if fahrenheit_to_kelvin(32) == 273:
        passed = passed + 1
    if wind_chill_simple(30, 2) == 30:
        passed = passed + 1
    if heat_index_simple(70, 50) == 70:
        passed = passed + 1
    return passed
