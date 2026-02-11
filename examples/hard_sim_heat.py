def heat_transfer(k_cond: float, area: float, temp_diff: float, thickness: float) -> float:
    return k_cond * area * temp_diff / thickness

def newton_cooling(h_coeff: float, area: float, t_surface: float, t_ambient: float) -> float:
    return h_coeff * area * (t_surface - t_ambient)

def temp_after_time(t_init: float, t_env: float, decay_rate: float, time: float) -> float:
    factor: float = 1.0 / (1.0 + decay_rate * time)
    return t_env + (t_init - t_env) * factor

def thermal_resistance(thickness: float, k_cond: float, area: float) -> float:
    return thickness / (k_cond * area)

def heat_capacity_energy(mass: float, specific_heat: float, delta_t: float) -> float:
    return mass * specific_heat * delta_t

def diffuse_1d(temps: list[float], alpha: float, dt: float, dx: float) -> list[float]:
    n: int = len(temps)
    result: list[float] = []
    i: int = 0
    while i < n:
        if i == 0 or i == n - 1:
            result.append(temps[i])
        else:
            left: float = temps[i - 1]
            mid: float = temps[i]
            right: float = temps[i + 1]
            new_t: float = mid + alpha * dt / (dx * dx) * (left - 2.0 * mid + right)
            result.append(new_t)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    q: float = heat_transfer(1.0, 2.0, 50.0, 0.1)
    if q == 1000.0:
        passed = passed + 1
    nc: float = newton_cooling(10.0, 1.0, 100.0, 25.0)
    if nc == 750.0:
        passed = passed + 1
    t: float = temp_after_time(100.0, 20.0, 0.0, 5.0)
    if t == 100.0:
        passed = passed + 1
    r: float = thermal_resistance(0.1, 1.0, 2.0)
    if r == 0.05:
        passed = passed + 1
    e: float = heat_capacity_energy(1.0, 4186.0, 10.0)
    if e == 41860.0:
        passed = passed + 1
    temps: list[float] = [100.0, 50.0, 50.0, 50.0, 0.0]
    res: list[float] = diffuse_1d(temps, 1.0, 0.01, 1.0)
    r0: float = res[0]
    if r0 == 100.0:
        passed = passed + 1
    return passed
