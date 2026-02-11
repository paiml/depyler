"""Wave interference pattern computations using integer arithmetic.

Tests: constructive/destructive interference, path difference, fringe spacing.
Scale factor 1000 for fixed-point.
"""


def path_difference(d1: int, d2: int) -> int:
    """Compute path difference between two sources. Fixed-point scale 1000."""
    diff: int = d1 - d2
    if diff < 0:
        diff = 0 - diff
    return diff


def is_constructive(path_diff: int, wavelength: int) -> int:
    """Check if interference is constructive (path_diff = n*lambda).
    Returns 1 if constructive, 0 otherwise. Scale 1000."""
    if wavelength == 0:
        return 0
    remainder: int = path_diff % wavelength
    if remainder == 0:
        return 1
    return 0


def is_destructive(path_diff: int, wavelength: int) -> int:
    """Check if interference is destructive (path_diff = (n+0.5)*lambda).
    Uses integer check: 2*path_diff = odd multiple of lambda. Scale 1000."""
    if wavelength == 0:
        return 0
    doubled: int = 2 * path_diff
    remainder: int = doubled % wavelength
    if remainder != 0:
        return 0
    quotient: int = doubled // wavelength
    if quotient % 2 == 1:
        return 1
    return 0


def resultant_intensity(i1: int, i2: int, phase_diff: int) -> int:
    """Resultant intensity I = I1 + I2 + 2*sqrt(I1*I2)*cos(phase).
    cos approx: cos(x) ~ 1000 - x*x/2000. Fixed-point scale 1000."""
    cos_val: int = 1000 - (phase_diff * phase_diff) // 2000
    if cos_val > 1000:
        cos_val = 1000
    if cos_val < 0 - 1000:
        cos_val = 0 - 1000
    product: int = (i1 * i2) // 1000
    root: int = product
    if root > 0:
        guess: int = root
        iterations: int = 0
        while iterations < 40:
            if guess == 0:
                break
            next_g: int = (guess + (product * 1000) // guess) // 2
            d: int = next_g - guess
            if d < 0:
                d = 0 - d
            if d < 2:
                root = next_g
                break
            guess = next_g
            iterations = iterations + 1
        root = guess
    else:
        root = 0
    cross_term: int = (2 * root * cos_val) // 1000
    result: int = i1 + i2 + cross_term
    if result < 0:
        result = 0
    return result


def fringe_spacing(wavelength: int, distance: int, slit_sep: int) -> int:
    """Young's double slit fringe spacing = lambda * D / d.
    Fixed-point scale 1000."""
    if slit_sep == 0:
        return 0
    result: int = (wavelength * distance) // slit_sep
    return result


def max_order(slit_sep: int, wavelength: int) -> int:
    """Maximum diffraction order: n_max = d / lambda."""
    if wavelength == 0:
        return 0
    result: int = slit_sep // wavelength
    return result


def intensity_single_slit(central_intensity: int, angle_param: int) -> int:
    """Single slit intensity approx: I = I0 * (sin(x)/x)^2.
    sinc approx: sinc(x) ~ 1 - x^2/6 for small x. Fixed-point scale 1000."""
    if angle_param == 0:
        return central_intensity
    sinc_val: int = 1000 - (angle_param * angle_param) // 6000
    if sinc_val < 0:
        sinc_val = 0
    result: int = (central_intensity * sinc_val * sinc_val) // (1000 * 1000)
    return result


def test_module() -> int:
    """Test interference computations."""
    ok: int = 0
    pd: int = path_difference(5000, 3000)
    if pd == 2000:
        ok = ok + 1
    constr: int = is_constructive(2000, 1000)
    if constr == 1:
        ok = ok + 1
    destr: int = is_destructive(500, 1000)
    if destr == 1:
        ok = ok + 1
    no_destr: int = is_destructive(1000, 1000)
    if no_destr == 0:
        ok = ok + 1
    fs: int = fringe_spacing(500, 2000, 1000)
    if fs == 1000:
        ok = ok + 1
    mo: int = max_order(5000, 1000)
    if mo == 5:
        ok = ok + 1
    iss: int = intensity_single_slit(1000, 0)
    if iss == 1000:
        ok = ok + 1
    return ok
