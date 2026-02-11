def dft_real(signal: list[float]) -> list[float]:
    n: int = len(signal)
    result: list[float] = []
    k: int = 0
    while k < n:
        real_sum: float = 0.0
        j: int = 0
        while j < n:
            angle: float = 2.0 * 3.14159265 * k * j / (n * 1.0)
            cos_val: float = 1.0 - (angle * angle) / 2.0 + (angle * angle * angle * angle) / 24.0
            real_sum = real_sum + signal[j] * cos_val
            j = j + 1
        result.append(real_sum)
        k = k + 1
    return result

def dft_imag(signal: list[float]) -> list[float]:
    n: int = len(signal)
    result: list[float] = []
    k: int = 0
    while k < n:
        imag_sum: float = 0.0
        j: int = 0
        while j < n:
            angle: float = 2.0 * 3.14159265 * k * j / (n * 1.0)
            sin_val: float = angle - (angle * angle * angle) / 6.0
            imag_sum = imag_sum - signal[j] * sin_val
            j = j + 1
        result.append(imag_sum)
        k = k + 1
    return result

def magnitude_spectrum(real_part: list[float], imag_part: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(real_part)
    i: int = 0
    while i < n:
        r: float = real_part[i]
        im: float = imag_part[i]
        mag: float = (r * r + im * im) ** 0.5
        result.append(mag)
        i = i + 1
    return result

def power_spectrum(real_part: list[float], imag_part: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(real_part)
    i: int = 0
    while i < n:
        r: float = real_part[i]
        im: float = imag_part[i]
        result.append(r * r + im * im)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    sig: list[float] = [1.0, 0.0, 0.0, 0.0]
    r: list[float] = dft_real(sig)
    r0: float = r[0]
    diff: float = r0 - 1.0
    if diff < 0.01 and diff > (0.0 - 0.01):
        passed = passed + 1
    n: int = len(r)
    if n == 4:
        passed = passed + 1
    im: list[float] = dft_imag(sig)
    im0: float = im[0]
    diff2: float = im0 - 0.0
    if diff2 < 0.01 and diff2 > (0.0 - 0.01):
        passed = passed + 1
    mag: list[float] = magnitude_spectrum(r, im)
    mag0: float = mag[0]
    diff3: float = mag0 - 1.0
    if diff3 < 0.01 and diff3 > (0.0 - 0.01):
        passed = passed + 1
    ps: list[float] = power_spectrum(r, im)
    ps0: float = ps[0]
    diff4: float = ps0 - 1.0
    if diff4 < 0.01 and diff4 > (0.0 - 0.01):
        passed = passed + 1
    return passed
