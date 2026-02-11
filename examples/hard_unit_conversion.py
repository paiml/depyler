def inches_to_cm(inches: int) -> int:
    return inches * 254 // 100


def cm_to_inches(cm: int) -> int:
    return cm * 100 // 254


def miles_to_km_x10(miles: int) -> int:
    return miles * 161 // 10


def kg_to_pounds_x10(kg: int) -> int:
    return kg * 22 // 1


def pounds_to_kg_x10(pounds: int) -> int:
    return pounds * 10 // 22


def liters_to_gallons_x100(liters: int) -> int:
    return liters * 100 // 379


def feet_to_meters_x100(feet: int) -> int:
    return feet * 3048 // 100


def test_module() -> int:
    passed: int = 0
    if inches_to_cm(10) == 25:
        passed = passed + 1
    if cm_to_inches(254) == 100:
        passed = passed + 1
    if miles_to_km_x10(1) == 16:
        passed = passed + 1
    if kg_to_pounds_x10(1) == 22:
        passed = passed + 1
    if pounds_to_kg_x10(22) == 10:
        passed = passed + 1
    if liters_to_gallons_x100(379) == 100:
        passed = passed + 1
    if feet_to_meters_x100(1) == 30:
        passed = passed + 1
    return passed
