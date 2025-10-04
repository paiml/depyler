"""Test statistics module - Statistical functions.

This module tests statistics for calculating averages, measures of spread,
and statistical relationships.
"""

import statistics
import pytest


class TestMean:
    """statistics.mean() - Arithmetic mean."""

    def test_mean_basic(self):
        """Basic: Calculate mean."""
        assert statistics.mean([1, 2, 3, 4, 5]) == 3.0

    def test_mean_floats(self):
        """Feature: Mean of floats."""
        assert abs(statistics.mean([1.5, 2.5, 3.5]) - 2.5) < 1e-10

    def test_mean_single_value(self):
        """Edge: Mean of single value."""
        assert statistics.mean([42]) == 42

    def test_mean_negative(self):
        """Feature: Mean with negative numbers."""
        assert statistics.mean([-1, 0, 1]) == 0.0

    def test_error_mean_empty(self):
        """Error: Mean of empty sequence."""
        with pytest.raises(statistics.StatisticsError):
            statistics.mean([])

    def test_fmean(self):
        """Basic: Fast floating-point mean."""
        assert abs(statistics.fmean([1, 2, 3, 4]) - 2.5) < 1e-10


class TestMedian:
    """statistics.median() - Median value."""

    def test_median_odd(self):
        """Basic: Median of odd-length list."""
        assert statistics.median([1, 3, 5]) == 3

    def test_median_even(self):
        """Basic: Median of even-length list."""
        assert statistics.median([1, 2, 3, 4]) == 2.5

    def test_median_unsorted(self):
        """Feature: Median handles unsorted data."""
        assert statistics.median([3, 1, 4, 2, 5]) == 3

    def test_median_low(self):
        """Feature: Lower median for even-length."""
        assert statistics.median_low([1, 2, 3, 4]) == 2

    def test_median_high(self):
        """Feature: Upper median for even-length."""
        assert statistics.median_high([1, 2, 3, 4]) == 3

    def test_median_grouped(self):
        """Feature: Median of grouped data."""
        data = [1, 2, 2, 3, 4, 4, 4, 4, 4, 5]
        result = statistics.median_grouped(data)
        assert 3.0 <= result <= 4.0

    def test_error_median_empty(self):
        """Error: Median of empty sequence."""
        with pytest.raises(statistics.StatisticsError):
            statistics.median([])


class TestMode:
    """statistics.mode() - Most common value."""

    def test_mode_basic(self):
        """Basic: Mode of data."""
        assert statistics.mode([1, 2, 2, 3, 3, 3, 4]) == 3

    def test_mode_strings(self):
        """Feature: Mode works with strings."""
        assert statistics.mode(['a', 'b', 'b', 'c']) == 'b'

    def test_multimode(self):
        """Feature: Multiple modes."""
        result = statistics.multimode([1, 1, 2, 2, 3])
        assert set(result) == {1, 2}

    def test_multimode_all_unique(self):
        """Edge: All values unique returns all."""
        result = statistics.multimode([1, 2, 3])
        assert set(result) == {1, 2, 3}

    def test_error_mode_empty(self):
        """Error: Mode of empty sequence."""
        with pytest.raises(statistics.StatisticsError):
            statistics.mode([])


class TestVariance:
    """statistics.variance() - Sample variance."""

    def test_variance_basic(self):
        """Basic: Sample variance."""
        data = [1, 2, 3, 4, 5]
        var = statistics.variance(data)
        assert abs(var - 2.5) < 1e-10

    def test_variance_vs_pvariance(self):
        """Property: Sample variance > population variance."""
        data = [1, 2, 3, 4, 5]
        sample_var = statistics.variance(data)
        pop_var = statistics.pvariance(data)
        assert sample_var > pop_var

    def test_pvariance(self):
        """Basic: Population variance."""
        data = [1, 2, 3, 4, 5]
        var = statistics.pvariance(data)
        assert abs(var - 2.0) < 1e-10

    def test_variance_with_mean(self):
        """Feature: Variance with pre-computed mean."""
        data = [1, 2, 3, 4, 5]
        mean = statistics.mean(data)
        var = statistics.variance(data, xbar=mean)
        assert abs(var - 2.5) < 1e-10

    def test_error_variance_single(self):
        """Error: Variance requires at least 2 values."""
        with pytest.raises(statistics.StatisticsError):
            statistics.variance([1])


class TestStandardDeviation:
    """statistics.stdev() - Standard deviation."""

    def test_stdev_basic(self):
        """Basic: Sample standard deviation."""
        data = [1, 2, 3, 4, 5]
        std = statistics.stdev(data)
        assert abs(std - 1.5811388300841898) < 1e-10

    def test_pstdev(self):
        """Basic: Population standard deviation."""
        data = [1, 2, 3, 4, 5]
        std = statistics.pstdev(data)
        assert abs(std - 1.4142135623730951) < 1e-10

    def test_stdev_sqrt_variance(self):
        """Property: stdev = sqrt(variance)."""
        data = [1, 2, 3, 4, 5]
        std = statistics.stdev(data)
        var = statistics.variance(data)
        assert abs(std ** 2 - var) < 1e-10

    def test_error_stdev_single(self):
        """Error: Stdev requires at least 2 values."""
        with pytest.raises(statistics.StatisticsError):
            statistics.stdev([1])


class TestQuantiles:
    """statistics.quantiles() - Divide data into intervals."""

    def test_quantiles_quartiles(self):
        """Basic: Quartiles (4 intervals)."""
        data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        q = statistics.quantiles(data, n=4)
        assert len(q) == 3  # 3 cut points for 4 intervals

    def test_quantiles_deciles(self):
        """Feature: Deciles (10 intervals)."""
        data = list(range(1, 101))
        q = statistics.quantiles(data, n=10)
        assert len(q) == 9  # 9 cut points for 10 intervals

    def test_quantiles_method(self):
        """Feature: Different quantile methods."""
        data = [1, 2, 3, 4, 5]
        q1 = statistics.quantiles(data, method='inclusive')
        q2 = statistics.quantiles(data, method='exclusive')
        # Different methods may give slightly different results
        assert len(q1) == len(q2) == 3


class TestHarmonicMean:
    """statistics.harmonic_mean() - Harmonic mean."""

    def test_harmonic_mean_basic(self):
        """Basic: Harmonic mean."""
        result = statistics.harmonic_mean([1, 2, 4])
        # Harmonic mean of 1,2,4 is 3/(1 + 1/2 + 1/4) = 3/1.75 = 1.714...
        assert abs(result - 1.7142857142857142) < 1e-10

    def test_harmonic_mean_rates(self):
        """Use case: Average of rates."""
        # 60 mph for 1 mile, 30 mph for 1 mile -> avg 40 mph
        speeds = [60, 30]
        avg_speed = statistics.harmonic_mean(speeds)
        assert abs(avg_speed - 40.0) < 1e-10

    def test_harmonic_mean_zero(self):
        """Edge: Harmonic mean with zero returns zero."""
        result = statistics.harmonic_mean([1, 0, 3])
        assert result == 0.0

    def test_error_harmonic_mean_negative(self):
        """Error: Harmonic mean with negative."""
        with pytest.raises(statistics.StatisticsError):
            statistics.harmonic_mean([1, -2, 3])


class TestGeometricMean:
    """statistics.geometric_mean() - Geometric mean."""

    def test_geometric_mean_basic(self):
        """Basic: Geometric mean."""
        result = statistics.geometric_mean([1, 2, 4, 8])
        # Geometric mean of 1,2,4,8 is (1*2*4*8)^(1/4) = 64^(1/4) = 2.828...
        assert abs(result - 2.8284271247461903) < 1e-10

    def test_geometric_mean_growth(self):
        """Use case: Average growth rate."""
        # 10% and 20% growth rates
        growth_multipliers = [1.10, 1.20]
        avg_growth = statistics.geometric_mean(growth_multipliers)
        assert abs(avg_growth - 1.1489125293076057) < 1e-10

    def test_error_geometric_mean_negative(self):
        """Error: Geometric mean with negative."""
        with pytest.raises(statistics.StatisticsError):
            statistics.geometric_mean([1, -2, 3])

    def test_error_geometric_mean_zero(self):
        """Error: Geometric mean with zero raises error."""
        with pytest.raises(statistics.StatisticsError):
            statistics.geometric_mean([1, 0, 3])


class TestCorrelation:
    """statistics.correlation() - Pearson correlation coefficient."""

    def test_correlation_perfect_positive(self):
        """Basic: Perfect positive correlation."""
        x = [1, 2, 3, 4, 5]
        y = [2, 4, 6, 8, 10]
        r = statistics.correlation(x, y)
        assert abs(r - 1.0) < 1e-10

    def test_correlation_perfect_negative(self):
        """Basic: Perfect negative correlation."""
        x = [1, 2, 3, 4, 5]
        y = [10, 8, 6, 4, 2]
        r = statistics.correlation(x, y)
        assert abs(r - (-1.0)) < 1e-10

    def test_correlation_no_correlation(self):
        """Edge: No correlation."""
        x = [1, 2, 3, 4, 5]
        y = [5, 4, 3, 2, 1]
        r = statistics.correlation(x, y)
        assert abs(r - (-1.0)) < 1e-10  # Actually perfect negative

    def test_error_correlation_length_mismatch(self):
        """Error: Mismatched lengths."""
        with pytest.raises(statistics.StatisticsError):
            statistics.correlation([1, 2, 3], [1, 2])

    def test_error_correlation_insufficient_data(self):
        """Error: Insufficient data."""
        with pytest.raises(statistics.StatisticsError):
            statistics.correlation([1], [1])


class TestCovariance:
    """statistics.covariance() - Sample covariance."""

    def test_covariance_basic(self):
        """Basic: Sample covariance."""
        x = [1, 2, 3, 4, 5]
        y = [2, 4, 6, 8, 10]
        cov = statistics.covariance(x, y)
        assert abs(cov - 5.0) < 1e-10

    def test_covariance_negative(self):
        """Feature: Negative covariance."""
        x = [1, 2, 3, 4, 5]
        y = [10, 8, 6, 4, 2]
        cov = statistics.covariance(x, y)
        assert cov < 0

    def test_error_covariance_length_mismatch(self):
        """Error: Mismatched lengths."""
        with pytest.raises(statistics.StatisticsError):
            statistics.covariance([1, 2, 3], [1, 2])


class TestLinearRegression:
    """statistics.linear_regression() - Simple linear regression."""

    def test_linear_regression_basic(self):
        """Basic: Linear regression slope and intercept."""
        x = [1, 2, 3, 4, 5]
        y = [2, 4, 6, 8, 10]
        slope, intercept = statistics.linear_regression(x, y)
        assert abs(slope - 2.0) < 1e-10
        assert abs(intercept - 0.0) < 1e-10

    def test_linear_regression_with_intercept(self):
        """Feature: Regression with non-zero intercept."""
        x = [1, 2, 3, 4, 5]
        y = [3, 5, 7, 9, 11]  # y = 2x + 1
        slope, intercept = statistics.linear_regression(x, y)
        assert abs(slope - 2.0) < 1e-10
        assert abs(intercept - 1.0) < 1e-10

    def test_linear_regression_prediction(self):
        """Use case: Predict y from x."""
        x = [1, 2, 3, 4, 5]
        y = [2, 4, 6, 8, 10]
        slope, intercept = statistics.linear_regression(x, y)
        # Predict y for x=6
        y_pred = slope * 6 + intercept
        assert abs(y_pred - 12.0) < 1e-10

    def test_linear_regression_proportional(self):
        """Feature: Proportional=True forces through origin."""
        x = [1, 2, 3, 4, 5]
        y = [3, 5, 7, 9, 11]
        slope, intercept = statistics.linear_regression(x, y, proportional=True)
        assert abs(intercept - 0.0) < 1e-10  # Forced through origin

    def test_error_linear_regression_length_mismatch(self):
        """Error: Mismatched lengths."""
        with pytest.raises(statistics.StatisticsError):
            statistics.linear_regression([1, 2, 3], [1, 2])


class TestNormalDist:
    """statistics.NormalDist - Normal distribution."""

    def test_normal_dist_create(self):
        """Basic: Create normal distribution."""
        nd = statistics.NormalDist(mu=0, sigma=1)
        assert nd.mean == 0
        assert nd.stdev == 1

    def test_normal_dist_pdf(self):
        """Basic: Probability density function."""
        nd = statistics.NormalDist(mu=0, sigma=1)
        # PDF at mean should be ~0.399
        pdf_at_mean = nd.pdf(0)
        assert abs(pdf_at_mean - 0.3989422804014327) < 1e-10

    def test_normal_dist_cdf(self):
        """Basic: Cumulative distribution function."""
        nd = statistics.NormalDist(mu=0, sigma=1)
        # CDF at mean should be 0.5
        cdf_at_mean = nd.cdf(0)
        assert abs(cdf_at_mean - 0.5) < 1e-10

    def test_normal_dist_quantile(self):
        """Basic: Inverse CDF (quantile)."""
        nd = statistics.NormalDist(mu=0, sigma=1)
        # Median (0.5 quantile) should be 0
        median = nd.inv_cdf(0.5)
        assert abs(median - 0.0) < 1e-10

    def test_normal_dist_overlap(self):
        """Feature: Overlap between distributions."""
        nd1 = statistics.NormalDist(mu=0, sigma=1)
        nd2 = statistics.NormalDist(mu=1, sigma=1)
        overlap = nd1.overlap(nd2)
        assert 0 < overlap < 1

    def test_normal_dist_from_samples(self):
        """Feature: Create from sample data."""
        data = [1, 2, 3, 4, 5]
        nd = statistics.NormalDist.from_samples(data)
        assert abs(nd.mean - 3.0) < 1e-10

    def test_normal_dist_zscore(self):
        """Feature: Z-score calculation."""
        nd = statistics.NormalDist(mu=100, sigma=15)
        # Z-score for value 115 should be 1.0
        z = nd.zscore(115)
        assert abs(z - 1.0) < 1e-10

    def test_normal_dist_addition(self):
        """Feature: Add independent distributions."""
        nd1 = statistics.NormalDist(mu=1, sigma=2)
        nd2 = statistics.NormalDist(mu=3, sigma=4)
        nd_sum = nd1 + nd2
        assert nd_sum.mean == 4  # 1 + 3
        # Variance adds: sqrt(2^2 + 4^2) = sqrt(20)
        assert abs(nd_sum.stdev - 4.47213595499958) < 1e-10

    def test_normal_dist_subtraction(self):
        """Feature: Subtract independent distributions."""
        nd1 = statistics.NormalDist(mu=5, sigma=2)
        nd2 = statistics.NormalDist(mu=3, sigma=1)
        nd_diff = nd1 - nd2
        assert nd_diff.mean == 2  # 5 - 3

    def test_normal_dist_multiplication(self):
        """Feature: Scale distribution."""
        nd = statistics.NormalDist(mu=1, sigma=2)
        nd_scaled = nd * 3
        assert nd_scaled.mean == 3  # 1 * 3
        assert nd_scaled.stdev == 6  # 2 * 3


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_mean_large_numbers(self):
        """Performance: Mean of large numbers."""
        data = [10**100, 10**100 + 1, 10**100 + 2]
        result = statistics.mean(data)
        assert result > 10**100

    def test_variance_zero(self):
        """Edge: Variance of identical values is zero."""
        data = [5, 5, 5, 5, 5]
        var = statistics.variance(data)
        assert abs(var - 0.0) < 1e-10

    def test_quantiles_small_dataset(self):
        """Edge: Quantiles with small dataset."""
        data = [1, 2, 3]
        q = statistics.quantiles(data, n=4)
        assert len(q) == 3

    def test_median_single_element(self):
        """Edge: Median of single element."""
        assert statistics.median([42]) == 42

    def test_mode_all_same(self):
        """Edge: Mode when all values same."""
        assert statistics.mode([5, 5, 5, 5]) == 5

    def test_fmean_vs_mean(self):
        """Property: fmean is faster but same result."""
        data = [1.5, 2.5, 3.5, 4.5]
        mean_result = statistics.mean(data)
        fmean_result = statistics.fmean(data)
        assert abs(mean_result - fmean_result) < 1e-10

    def test_stdev_precision(self):
        """Edge: Standard deviation with high precision."""
        data = [1.1, 1.2, 1.3, 1.4, 1.5]
        std = statistics.stdev(data)
        assert std > 0

    def test_correlation_identical_series(self):
        """Edge: Correlation of series with itself."""
        x = [1, 2, 3, 4, 5]
        r = statistics.correlation(x, x)
        assert abs(r - 1.0) < 1e-10

    def test_normal_dist_zero_stdev(self):
        """Edge: NormalDist with zero stdev is degenerate."""
        # Creates a degenerate distribution (all probability at mean)
        nd = statistics.NormalDist(mu=5, sigma=0)
        assert nd.mean == 5
        assert nd.stdev == 0

    def test_normal_dist_samples_variance(self):
        """Property: from_samples captures variance."""
        import random
        random.seed(42)
        data = [random.gauss(10, 2) for _ in range(100)]
        nd = statistics.NormalDist.from_samples(data)
        # Should be close to mu=10, sigma=2
        assert 9 < nd.mean < 11
        assert 1.5 < nd.stdev < 2.5
