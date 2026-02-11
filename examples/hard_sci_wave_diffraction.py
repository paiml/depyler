"""Diffraction pattern computations using integer arithmetic.

Tests: single slit, double slit, grating, resolution limit.
Scale factor 1000 for fixed-point.
"""


def single_slit_minima(slit_width: int, order: int, wavelength: int) -> int:
    """Angle to nth minimum: sin(theta) = n*lambda/a.
    Returns sin(theta) * 1000. Scale 1000."""
    if slit_width == 0:
        return 0
    result: int = (order * wavelength) // slit_width
    return result


def double_slit_maxima(slit_sep: int, order: int, wavelength: int) -> int:
    """Angle to nth maximum: sin(theta) = n*lambda/d.
    Returns sin(theta) * 1000. Scale 1000."""
    if slit_sep == 0:
        return 0
    result: int = (order * wavelength) // slit_sep
    return result


def grating_maxima(grating_spacing: int, order: int, wavelength: int) -> int:
    """Diffraction grating maxima: sin(theta) = n*lambda/d.
    Returns sin(theta) * 1000."""
    if grating_spacing == 0:
        return 0
    result: int = (order * wavelength) // grating_spacing
    return result


def grating_resolving_power(order: int, num_slits: int) -> int:
    """Resolving power R = n * N."""
    return order * num_slits


def rayleigh_criterion(wavelength: int, aperture: int) -> int:
    """Rayleigh criterion: theta_min = 1.22 * lambda / D.
    Returns angle * 1000. Scale 1000. 1.22 ~ 1220/1000."""
    if aperture == 0:
        return 0
    result: int = (1220 * wavelength) // (aperture * 1000)
    return result


def airy_disk_radius(wavelength: int, focal_length: int, aperture: int) -> int:
    """Airy disk radius = 1.22 * lambda * f / D.
    Fixed-point scale 1000."""
    if aperture == 0:
        return 0
    result: int = (1220 * wavelength * focal_length) // (aperture * 1000 * 1000)
    return result


def diffraction_intensity_approx(central: int, phase_param: int) -> int:
    """Approximate diffraction intensity using sinc^2.
    sinc(x) ~ 1 - x^2/6 for small x. Fixed-point scale 1000."""
    if phase_param == 0:
        return central
    sinc_approx: int = 1000 - (phase_param * phase_param) // 6000
    if sinc_approx < 0:
        sinc_approx = 0
    result: int = (central * sinc_approx * sinc_approx) // (1000 * 1000)
    return result


def count_visible_orders(grating_spacing: int, wavelength: int) -> int:
    """Count number of visible diffraction orders.
    Maximum order when sin(theta) <= 1, so n <= d/lambda."""
    if wavelength == 0:
        return 0
    max_n: int = grating_spacing // wavelength
    total: int = 2 * max_n + 1
    return total


def test_module() -> int:
    """Test diffraction computations."""
    ok: int = 0
    sm: int = single_slit_minima(500, 1, 2000)
    if sm == 4:
        ok = ok + 1
    dm: int = double_slit_maxima(500, 2, 1000)
    if dm == 4:
        ok = ok + 1
    gm: int = grating_maxima(500, 1, 2000)
    if gm == 4:
        ok = ok + 1
    rp: int = grating_resolving_power(2, 1000)
    if rp == 2000:
        ok = ok + 1
    vo: int = count_visible_orders(5000, 500)
    if vo == 21:
        ok = ok + 1
    di: int = diffraction_intensity_approx(1000, 0)
    if di == 1000:
        ok = ok + 1
    zero_check: int = single_slit_minima(0, 1, 500)
    if zero_check == 0:
        ok = ok + 1
    return ok
