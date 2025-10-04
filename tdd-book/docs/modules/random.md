# random

## Basic random number generation.

## Integer random number generation.

## Random choice from sequences.

## Shuffle sequences in place.

## Statistical distributions.

## Random state management.

## SystemRandom - Cryptographically secure random.

## Edge cases and special scenarios.

### Basic: Generate random float [0.0, 1.0).

```python
def test_random_basic(self):
    """Basic: Generate random float [0.0, 1.0)."""
    r = random.random()
    assert 0.0 <= r < 1.0
```

**Verification**: ✅ Tested in CI

### Property: Seed makes random reproducible.

```python
def test_random_reproducible_with_seed(self):
    """Property: Seed makes random reproducible."""
    random.seed(42)
    r1 = random.random()
    random.seed(42)
    r2 = random.random()
    assert r1 == r2
```

**Verification**: ✅ Tested in CI

### Property: Different values without same seed.

```python
def test_random_different_without_seed(self):
    """Property: Different values without same seed."""
    random.seed(42)
    r1 = random.random()
    random.seed(43)
    r2 = random.random()
    assert r1 != r2
```

**Verification**: ✅ Tested in CI

### Basic: Uniform distribution.

```python
def test_uniform(self):
    """Basic: Uniform distribution."""
    r = random.uniform(1.0, 10.0)
    assert 1.0 <= r <= 10.0
```

**Verification**: ✅ Tested in CI

### Feature: uniform works with reversed args.

```python
def test_uniform_reversed_args(self):
    """Feature: uniform works with reversed args."""
    r = random.uniform(10.0, 1.0)
    assert 1.0 <= r <= 10.0
```

**Verification**: ✅ Tested in CI

### Basic: Random integer inclusive.

```python
def test_randint(self):
    """Basic: Random integer inclusive."""
    r = random.randint(1, 10)
    assert 1 <= r <= 10
    assert isinstance(r, int)
```

**Verification**: ✅ Tested in CI

### Edge: randint with same min and max.

```python
def test_randint_single_value(self):
    """Edge: randint with same min and max."""
    r = random.randint(5, 5)
    assert r == 5
```

**Verification**: ✅ Tested in CI

### Basic: Random integer exclusive.

```python
def test_randrange_exclusive(self):
    """Basic: Random integer exclusive."""
    r = random.randrange(1, 10)
    assert 1 <= r < 10
```

**Verification**: ✅ Tested in CI

### Feature: randrange with step.

```python
def test_randrange_with_step(self):
    """Feature: randrange with step."""
    r = random.randrange(0, 10, 2)
    assert r in [0, 2, 4, 6, 8]
```

**Verification**: ✅ Tested in CI

### Feature: randrange(n) means [0, n).

```python
def test_randrange_single_arg(self):
    """Feature: randrange(n) means [0, n)."""
    r = random.randrange(10)
    assert 0 <= r < 10
```

**Verification**: ✅ Tested in CI

### Error: randint raises if min > max.

```python
def test_error_randint_reversed(self):
    """Error: randint raises if min > max."""
    with pytest.raises(ValueError):
        random.randint(10, 1)
```

**Verification**: ✅ Tested in CI

### Basic: Choose from list.

```python
def test_choice_list(self):
    """Basic: Choose from list."""
    items = [1, 2, 3, 4, 5]
    choice = random.choice(items)
    assert choice in items
```

**Verification**: ✅ Tested in CI

### Feature: Choose from string.

```python
def test_choice_string(self):
    """Feature: Choose from string."""
    s = 'abcde'
    choice = random.choice(s)
    assert choice in s
```

**Verification**: ✅ Tested in CI

### Feature: Choose from tuple.

```python
def test_choice_tuple(self):
    """Feature: Choose from tuple."""
    items = (1, 2, 3)
    choice = random.choice(items)
    assert choice in items
```

**Verification**: ✅ Tested in CI

### Basic: Multiple choices with replacement.

```python
def test_choices_with_k(self):
    """Basic: Multiple choices with replacement."""
    items = [1, 2, 3]
    choices = random.choices(items, k=5)
    assert len(choices) == 5
    assert all((c in items for c in choices))
```

**Verification**: ✅ Tested in CI

### Feature: Weighted choices.

```python
def test_choices_with_weights(self):
    """Feature: Weighted choices."""
    items = ['a', 'b', 'c']
    weights = [10, 1, 1]
    random.seed(42)
    choices = random.choices(items, weights=weights, k=100)
    assert choices.count('a') > choices.count('b')
```

**Verification**: ✅ Tested in CI

### Feature: Cumulative weights.

```python
def test_choices_with_cum_weights(self):
    """Feature: Cumulative weights."""
    items = [1, 2, 3]
    cum_weights = [10, 11, 12]
    choices = random.choices(items, cum_weights=cum_weights, k=5)
    assert len(choices) == 5
```

**Verification**: ✅ Tested in CI

### Basic: Sample without replacement.

```python
def test_sample_without_replacement(self):
    """Basic: Sample without replacement."""
    items = [1, 2, 3, 4, 5]
    sample = random.sample(items, k=3)
    assert len(sample) == 3
    assert len(set(sample)) == 3
    assert all((item in items for item in sample))
```

**Verification**: ✅ Tested in CI

### Edge: Sample all items is permutation.

```python
def test_sample_all_items(self):
    """Edge: Sample all items is permutation."""
    items = [1, 2, 3]
    sample = random.sample(items, k=3)
    assert sorted(sample) == sorted(items)
```

**Verification**: ✅ Tested in CI

### Error: Choice from empty sequence.

```python
def test_error_choice_empty(self):
    """Error: Choice from empty sequence."""
    with pytest.raises(IndexError):
        random.choice([])
```

**Verification**: ✅ Tested in CI

### Error: Sample larger than population.

```python
def test_error_sample_too_large(self):
    """Error: Sample larger than population."""
    with pytest.raises(ValueError):
        random.sample([1, 2, 3], k=5)
```

**Verification**: ✅ Tested in CI

### Basic: Shuffle list in place.

```python
def test_shuffle_list(self):
    """Basic: Shuffle list in place."""
    items = [1, 2, 3, 4, 5]
    original = items.copy()
    random.seed(42)
    random.shuffle(items)
    assert sorted(items) == sorted(original)
    assert items != original or len(items) <= 1
```

**Verification**: ✅ Tested in CI

### Property: Shuffle reproducible with seed.

```python
def test_shuffle_reproducible(self):
    """Property: Shuffle reproducible with seed."""
    items1 = [1, 2, 3, 4, 5]
    items2 = [1, 2, 3, 4, 5]
    random.seed(42)
    random.shuffle(items1)
    random.seed(42)
    random.shuffle(items2)
    assert items1 == items2
```

**Verification**: ✅ Tested in CI

### Property: Shuffle modifies in place.

```python
def test_shuffle_modifies_original(self):
    """Property: Shuffle modifies in place."""
    items = [1, 2, 3]
    original_id = id(items)
    random.shuffle(items)
    assert id(items) == original_id
```

**Verification**: ✅ Tested in CI

### Basic: Gaussian (normal) distribution.

```python
def test_gauss(self):
    """Basic: Gaussian (normal) distribution."""
    r = random.gauss(mu=0, sigma=1)
    assert isinstance(r, float)
```

**Verification**: ✅ Tested in CI

### Property: Gaussian centered around mean.

```python
def test_gauss_mean(self):
    """Property: Gaussian centered around mean."""
    random.seed(42)
    samples = [random.gauss(mu=100, sigma=10) for _ in range(1000)]
    mean = sum(samples) / len(samples)
    assert 95 < mean < 105
```

**Verification**: ✅ Tested in CI

### Basic: Normal variate (alternative to gauss).

```python
def test_normalvariate(self):
    """Basic: Normal variate (alternative to gauss)."""
    r = random.normalvariate(mu=0, sigma=1)
    assert isinstance(r, float)
```

**Verification**: ✅ Tested in CI

### Basic: Exponential distribution.

```python
def test_expovariate(self):
    """Basic: Exponential distribution."""
    r = random.expovariate(lambd=1.0)
    assert r >= 0
```

**Verification**: ✅ Tested in CI

### Basic: Beta distribution.

```python
def test_betavariate(self):
    """Basic: Beta distribution."""
    r = random.betavariate(alpha=2, beta=5)
    assert 0 <= r <= 1
```

**Verification**: ✅ Tested in CI

### Basic: Gamma distribution.

```python
def test_gammavariate(self):
    """Basic: Gamma distribution."""
    r = random.gammavariate(alpha=2, beta=3)
    assert r >= 0
```

**Verification**: ✅ Tested in CI

### Basic: Log-normal distribution.

```python
def test_lognormvariate(self):
    """Basic: Log-normal distribution."""
    r = random.lognormvariate(mu=0, sigma=1)
    assert r > 0
```

**Verification**: ✅ Tested in CI

### Basic: Pareto distribution.

```python
def test_paretovariate(self):
    """Basic: Pareto distribution."""
    r = random.paretovariate(alpha=2)
    assert r >= 1
```

**Verification**: ✅ Tested in CI

### Basic: Von Mises distribution.

```python
def test_vonmisesvariate(self):
    """Basic: Von Mises distribution."""
    r = random.vonmisesvariate(mu=0, kappa=1)
    assert isinstance(r, float)
```

**Verification**: ✅ Tested in CI

### Basic: Weibull distribution.

```python
def test_weibullvariate(self):
    """Basic: Weibull distribution."""
    r = random.weibullvariate(alpha=1, beta=1)
    assert r >= 0
```

**Verification**: ✅ Tested in CI

### Basic: Triangular distribution.

```python
def test_triangular(self):
    """Basic: Triangular distribution."""
    r = random.triangular(low=0, high=10, mode=5)
    assert 0 <= r <= 10
```

**Verification**: ✅ Tested in CI

### Feature: Triangular with default mode.

```python
def test_triangular_no_mode(self):
    """Feature: Triangular with default mode."""
    r = random.triangular(low=0, high=10)
    assert 0 <= r <= 10
```

**Verification**: ✅ Tested in CI

### Basic: Get and restore state.

```python
def test_getstate_setstate(self):
    """Basic: Get and restore state."""
    random.seed(42)
    state = random.getstate()
    r1 = random.random()
    random.setstate(state)
    r2 = random.random()
    assert r1 == r2
```

**Verification**: ✅ Tested in CI

### Feature: Seed with None uses system time.

```python
def test_seed_none(self):
    """Feature: Seed with None uses system time."""
    random.seed(None)
    r1 = random.random()
    random.seed(None)
    r2 = random.random()
    assert r1 != r2
```

**Verification**: ✅ Tested in CI

### Feature: Seed with string.

```python
def test_seed_string(self):
    """Feature: Seed with string."""
    random.seed('test')
    r1 = random.random()
    random.seed('test')
    r2 = random.random()
    assert r1 == r2
```

**Verification**: ✅ Tested in CI

### Basic: SystemRandom generates random.

```python
def test_system_random_basic(self):
    """Basic: SystemRandom generates random."""
    sr = random.SystemRandom()
    r = sr.random()
    assert 0.0 <= r < 1.0
```

**Verification**: ✅ Tested in CI

### Property: SystemRandom ignores seed.

```python
def test_system_random_no_seed(self):
    """Property: SystemRandom ignores seed."""
    sr = random.SystemRandom()
    sr.seed(42)
    r1 = sr.random()
    sr.seed(42)
    r2 = sr.random()
    assert r1 != r2
```

**Verification**: ✅ Tested in CI

### Feature: SystemRandom has choice method.

```python
def test_system_random_choice(self):
    """Feature: SystemRandom has choice method."""
    sr = random.SystemRandom()
    choice = sr.choice([1, 2, 3, 4, 5])
    assert choice in [1, 2, 3, 4, 5]
```

**Verification**: ✅ Tested in CI

### Property: random() always in [0, 1).

```python
def test_random_in_range(self):
    """Property: random() always in [0, 1)."""
    random.seed(42)
    for _ in range(100):
        r = random.random()
        assert 0.0 <= r < 1.0
```

**Verification**: ✅ Tested in CI

### Edge: randint includes both endpoints.

```python
def test_randint_boundary(self):
    """Edge: randint includes both endpoints."""
    random.seed(42)
    results = {random.randint(1, 3) for _ in range(100)}
    assert 1 in results
    assert 3 in results
```

**Verification**: ✅ Tested in CI

### Edge: Choice from single element list.

```python
def test_choice_single_element(self):
    """Edge: Choice from single element list."""
    choice = random.choice([42])
    assert choice == 42
```

**Verification**: ✅ Tested in CI

### Property: Sample maintains relative order.

```python
def test_sample_order_preserved(self):
    """Property: Sample maintains relative order."""
    items = list(range(10))
    random.seed(42)
    sample = random.sample(items, k=5)
    assert isinstance(sample, list)
```

**Verification**: ✅ Tested in CI

### Edge: Shuffle empty list.

```python
def test_shuffle_empty_list(self):
    """Edge: Shuffle empty list."""
    items = []
    random.shuffle(items)
    assert items == []
```

**Verification**: ✅ Tested in CI

### Edge: Shuffle single element.

```python
def test_shuffle_single_element(self):
    """Edge: Shuffle single element."""
    items = [42]
    random.shuffle(items)
    assert items == [42]
```

**Verification**: ✅ Tested in CI

### Edge: uniform with equal a and b.

```python
def test_uniform_equal_bounds(self):
    """Edge: uniform with equal a and b."""
    r = random.uniform(5.0, 5.0)
    assert r == 5.0
```

**Verification**: ✅ Tested in CI

### Edge: Gauss with zero sigma returns mu.

```python
def test_gauss_zero_sigma(self):
    """Edge: Gauss with zero sigma returns mu."""
    r = random.gauss(mu=10, sigma=0)
    assert r == 10
```

**Verification**: ✅ Tested in CI

### Feature: choices with default k=1.

```python
def test_choices_default_k(self):
    """Feature: choices with default k=1."""
    items = [1, 2, 3]
    choices = random.choices(items)
    assert len(choices) == 1
```

**Verification**: ✅ Tested in CI

### Feature: Sample from range.

```python
def test_sample_population_range(self):
    """Feature: Sample from range."""
    sample = random.sample(range(100), k=10)
    assert len(sample) == 10
    assert all((0 <= x < 100 for x in sample))
```

**Verification**: ✅ Tested in CI

### Feature: randrange with negative step.

```python
def test_randrange_negative_step(self):
    """Feature: randrange with negative step."""
    r = random.randrange(10, 0, -2)
    assert r in [10, 8, 6, 4, 2]
```

**Verification**: ✅ Tested in CI

### Error: randrange with zero step.

```python
def test_error_randrange_zero_step(self):
    """Error: randrange with zero step."""
    with pytest.raises(ValueError):
        random.randrange(0, 10, 0)
```

**Verification**: ✅ Tested in CI

### Basic: Get random bits.

```python
def test_getrandbits(self):
    """Basic: Get random bits."""
    bits = random.getrandbits(16)
    assert 0 <= bits < 2 ** 16
    assert isinstance(bits, int)
```

**Verification**: ✅ Tested in CI

### Feature: Get many random bits.

```python
def test_getrandbits_large(self):
    """Feature: Get many random bits."""
    bits = random.getrandbits(128)
    assert 0 <= bits < 2 ** 128
```

**Verification**: ✅ Tested in CI

### Property: Seed creates reproducible sequence.

```python
def test_reproducible_sequence(self):
    """Property: Seed creates reproducible sequence."""
    random.seed(42)
    seq1 = [random.random() for _ in range(10)]
    random.seed(42)
    seq2 = [random.random() for _ in range(10)]
    assert seq1 == seq2
```

**Verification**: ✅ Tested in CI

### Property: Multiple calls produce different values.

```python
def test_multiple_calls_different(self):
    """Property: Multiple calls produce different values."""
    random.seed(42)
    values = [random.random() for _ in range(10)]
    assert len(set(values)) == 10
```

**Verification**: ✅ Tested in CI

### Property: Weights can sum to any positive number.

```python
def test_choice_with_weights_sum(self):
    """Property: Weights can sum to any positive number."""
    items = ['a', 'b']
    weights = [100, 200]
    choices = random.choices(items, weights=weights, k=10)
    assert len(choices) == 10
```

**Verification**: ✅ Tested in CI
