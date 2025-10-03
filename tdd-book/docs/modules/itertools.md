# itertools

## itertools.chain() - Chain multiple iterables.

## itertools.combinations() - r-length subsequences.

## itertools.permutations() - r-length permutations.

## itertools.product() - Cartesian product.

## itertools.count() - Count from start indefinitely.

## itertools.cycle() - Cycle through iterable indefinitely.

## itertools.repeat() - Repeat element.

## itertools.zip_longest() - Zip iterables, filling missing values.

## itertools.groupby() - Group consecutive equal elements.

## itertools.dropwhile() - Drop elements while predicate is true.

## itertools.takewhile() - Take elements while predicate is true.

## itertools.islice() - Slice an iterator.

## itertools.accumulate() - Cumulative sums or operations.

### Basic: Chain two lists together.

```python
def test_chain_basic(self):
    """Basic: Chain two lists together."""
    result = list(itertools.chain([1, 2], [3, 4]))
    assert result == [1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Feature: Chain many iterables.

```python
def test_chain_multiple_iterables(self):
    """Feature: Chain many iterables."""
    result = list(itertools.chain([1], [2, 3], [4, 5, 6]))
    assert result == [1, 2, 3, 4, 5, 6]
```

**Verification**: ✅ Tested in CI

### Feature: Chain from a list of iterables.

```python
def test_chain_from_iterable(self):
    """Feature: Chain from a list of iterables."""
    result = list(itertools.chain.from_iterable([[1, 2], [3, 4], [5]]))
    assert result == [1, 2, 3, 4, 5]
```

**Verification**: ✅ Tested in CI

### Edge: Chain with empty iterables.

```python
def test_chain_empty_iterables(self):
    """Edge: Chain with empty iterables."""
    result = list(itertools.chain([1], [], [2], []))
    assert result == [1, 2]
```

**Verification**: ✅ Tested in CI

### Property: Chain different iterable types.

```python
def test_chain_different_types(self):
    """Property: Chain different iterable types."""
    result = list(itertools.chain([1, 2], (3, 4), 'ab'))
    assert result == [1, 2, 3, 4, 'a', 'b']
```

**Verification**: ✅ Tested in CI

### Basic: Generate 2-element combinations.

```python
def test_combinations_basic(self):
    """Basic: Generate 2-element combinations."""
    result = list(itertools.combinations([1, 2, 3], 2))
    assert result == [(1, 2), (1, 3), (2, 3)]
```

**Verification**: ✅ Tested in CI

### Feature: Generate 3-element combinations.

```python
def test_combinations_length_3(self):
    """Feature: Generate 3-element combinations."""
    result = list(itertools.combinations('ABCD', 3))
    assert result == [('A', 'B', 'C'), ('A', 'B', 'D'), ('A', 'C', 'D'), ('B', 'C', 'D')]
```

**Verification**: ✅ Tested in CI

### Edge: r equals iterable length (single combination).

```python
def test_combinations_r_equals_n(self):
    """Edge: r equals iterable length (single combination)."""
    result = list(itertools.combinations([1, 2, 3], 3))
    assert result == [(1, 2, 3)]
```

**Verification**: ✅ Tested in CI

### Edge: r=0 produces single empty tuple.

```python
def test_combinations_r_zero(self):
    """Edge: r=0 produces single empty tuple."""
    result = list(itertools.combinations([1, 2, 3], 0))
    assert result == [()]
```

**Verification**: ✅ Tested in CI

### Edge: r > n produces empty result.

```python
def test_combinations_r_greater_than_n(self):
    """Edge: r > n produces empty result."""
    result = list(itertools.combinations([1, 2], 5))
    assert result == []
```

**Verification**: ✅ Tested in CI

### Property: Combinations maintain input order.

```python
def test_combinations_preserves_order(self):
    """Property: Combinations maintain input order."""
    result = list(itertools.combinations([3, 1, 2], 2))
    assert result == [(3, 1), (3, 2), (1, 2)]
```

**Verification**: ✅ Tested in CI

### Basic: Generate all permutations.

```python
def test_permutations_basic(self):
    """Basic: Generate all permutations."""
    result = list(itertools.permutations([1, 2, 3]))
    assert len(result) == 6
    assert (1, 2, 3) in result
    assert (3, 2, 1) in result
```

**Verification**: ✅ Tested in CI

### Feature: Generate r-length permutations.

```python
def test_permutations_with_r(self):
    """Feature: Generate r-length permutations."""
    result = list(itertools.permutations([1, 2, 3], 2))
    assert len(result) == 6
    assert result == [(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)]
```

**Verification**: ✅ Tested in CI

### Edge: r=0 produces single empty tuple.

```python
def test_permutations_r_zero(self):
    """Edge: r=0 produces single empty tuple."""
    result = list(itertools.permutations([1, 2, 3], 0))
    assert result == [()]
```

**Verification**: ✅ Tested in CI

### Feature: Permutations of string.

```python
def test_permutations_string(self):
    """Feature: Permutations of string."""
    result = list(itertools.permutations('AB'))
    assert result == [('A', 'B'), ('B', 'A')]
```

**Verification**: ✅ Tested in CI

### Basic: Cartesian product of two iterables.

```python
def test_product_basic(self):
    """Basic: Cartesian product of two iterables."""
    result = list(itertools.product([1, 2], ['a', 'b']))
    assert result == [(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]
```

**Verification**: ✅ Tested in CI

### Feature: Product of three iterables.

```python
def test_product_three_iterables(self):
    """Feature: Product of three iterables."""
    result = list(itertools.product([1, 2], [3, 4], [5, 6]))
    assert len(result) == 8
    assert (1, 3, 5) in result
    assert (2, 4, 6) in result
```

**Verification**: ✅ Tested in CI

### Feature: Product with repeat parameter.

```python
def test_product_repeat(self):
    """Feature: Product with repeat parameter."""
    result = list(itertools.product([0, 1], repeat=3))
    assert len(result) == 8
    assert (0, 0, 0) in result
    assert (1, 1, 1) in result
```

**Verification**: ✅ Tested in CI

### Edge: Product with empty iterable is empty.

```python
def test_product_empty_iterable(self):
    """Edge: Product with empty iterable is empty."""
    result = list(itertools.product([1, 2], []))
    assert result == []
```

**Verification**: ✅ Tested in CI

### Basic: Count from 0.

```python
def test_count_basic(self):
    """Basic: Count from 0."""
    counter = itertools.count()
    result = [next(counter) for _ in range(5)]
    assert result == [0, 1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Feature: Count from custom start.

```python
def test_count_with_start(self):
    """Feature: Count from custom start."""
    counter = itertools.count(10)
    result = [next(counter) for _ in range(3)]
    assert result == [10, 11, 12]
```

**Verification**: ✅ Tested in CI

### Feature: Count with custom step.

```python
def test_count_with_step(self):
    """Feature: Count with custom step."""
    counter = itertools.count(0, 2)
    result = [next(counter) for _ in range(5)]
    assert result == [0, 2, 4, 6, 8]
```

**Verification**: ✅ Tested in CI

### Edge: Count with negative step.

```python
def test_count_negative_step(self):
    """Edge: Count with negative step."""
    counter = itertools.count(10, -1)
    result = [next(counter) for _ in range(5)]
    assert result == [10, 9, 8, 7, 6]
```

**Verification**: ✅ Tested in CI

### Basic: Cycle through a list.

```python
def test_cycle_basic(self):
    """Basic: Cycle through a list."""
    cycler = itertools.cycle([1, 2, 3])
    result = [next(cycler) for _ in range(7)]
    assert result == [1, 2, 3, 1, 2, 3, 1]
```

**Verification**: ✅ Tested in CI

### Feature: Cycle through a string.

```python
def test_cycle_string(self):
    """Feature: Cycle through a string."""
    cycler = itertools.cycle('AB')
    result = [next(cycler) for _ in range(5)]
    assert result == ['A', 'B', 'A', 'B', 'A']
```

**Verification**: ✅ Tested in CI

### Basic: Repeat element indefinitely.

```python
def test_repeat_basic(self):
    """Basic: Repeat element indefinitely."""
    repeater = itertools.repeat(5)
    result = [next(repeater) for _ in range(4)]
    assert result == [5, 5, 5, 5]
```

**Verification**: ✅ Tested in CI

### Feature: Repeat element n times.

```python
def test_repeat_with_times(self):
    """Feature: Repeat element n times."""
    result = list(itertools.repeat('A', 3))
    assert result == ['A', 'A', 'A']
```

**Verification**: ✅ Tested in CI

### Edge: Repeat 0 times produces empty.

```python
def test_repeat_zero_times(self):
    """Edge: Repeat 0 times produces empty."""
    result = list(itertools.repeat(1, 0))
    assert result == []
```

**Verification**: ✅ Tested in CI

### Basic: Zip iterables of different lengths.

```python
def test_zip_longest_basic(self):
    """Basic: Zip iterables of different lengths."""
    result = list(itertools.zip_longest([1, 2], ['a', 'b', 'c']))
    assert result == [(1, 'a'), (2, 'b'), (None, 'c')]
```

**Verification**: ✅ Tested in CI

### Feature: Custom fill value.

```python
def test_zip_longest_fillvalue(self):
    """Feature: Custom fill value."""
    result = list(itertools.zip_longest([1, 2], ['a', 'b', 'c'], fillvalue=0))
    assert result == [(1, 'a'), (2, 'b'), (0, 'c')]
```

**Verification**: ✅ Tested in CI

### Edge: All iterables same length (like zip).

```python
def test_zip_longest_all_same_length(self):
    """Edge: All iterables same length (like zip)."""
    result = list(itertools.zip_longest([1, 2], ['a', 'b']))
    assert result == [(1, 'a'), (2, 'b')]
```

**Verification**: ✅ Tested in CI

### Basic: Group consecutive identical elements.

```python
def test_groupby_basic(self):
    """Basic: Group consecutive identical elements."""
    data = [1, 1, 2, 2, 2, 3, 1]
    result = [(k, list(g)) for k, g in itertools.groupby(data)]
    assert result == [(1, [1, 1]), (2, [2, 2, 2]), (3, [3]), (1, [1])]
```

**Verification**: ✅ Tested in CI

### Feature: Group by custom key function.

```python
def test_groupby_with_key(self):
    """Feature: Group by custom key function."""
    data = ['apple', 'apricot', 'banana', 'blueberry']
    result = [(k, list(g)) for k, g in itertools.groupby(data, key=lambda x: x[0])]
    assert result == [('a', ['apple', 'apricot']), ('b', ['banana', 'blueberry'])]
```

**Verification**: ✅ Tested in CI

### Edge: groupby requires sorted data for complete grouping.

```python
def test_groupby_sorted_first(self):
    """Edge: groupby requires sorted data for complete grouping."""
    data = [1, 2, 1, 2]
    unsorted_result = [(k, list(g)) for k, g in itertools.groupby(data)]
    assert unsorted_result == [(1, [1]), (2, [2]), (1, [1]), (2, [2])]
    sorted_data = sorted(data)
    sorted_result = [(k, list(g)) for k, g in itertools.groupby(sorted_data)]
    assert sorted_result == [(1, [1, 1]), (2, [2, 2])]
```

**Verification**: ✅ Tested in CI

### Basic: Drop elements while less than 5.

```python
def test_dropwhile_basic(self):
    """Basic: Drop elements while less than 5."""
    result = list(itertools.dropwhile(lambda x: x < 5, [1, 2, 6, 7, 3]))
    assert result == [6, 7, 3]
```

**Verification**: ✅ Tested in CI

### Edge: All elements satisfy predicate.

```python
def test_dropwhile_all_dropped(self):
    """Edge: All elements satisfy predicate."""
    result = list(itertools.dropwhile(lambda x: x > 0, [1, 2, 3]))
    assert result == []
```

**Verification**: ✅ Tested in CI

### Edge: First element fails predicate.

```python
def test_dropwhile_none_dropped(self):
    """Edge: First element fails predicate."""
    result = list(itertools.dropwhile(lambda x: x > 5, [1, 2, 3]))
    assert result == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Basic: Take elements while less than 5.

```python
def test_takewhile_basic(self):
    """Basic: Take elements while less than 5."""
    result = list(itertools.takewhile(lambda x: x < 5, [1, 2, 6, 7, 3]))
    assert result == [1, 2]
```

**Verification**: ✅ Tested in CI

### Edge: All elements satisfy predicate.

```python
def test_takewhile_all_taken(self):
    """Edge: All elements satisfy predicate."""
    result = list(itertools.takewhile(lambda x: x > 0, [1, 2, 3]))
    assert result == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Edge: First element fails predicate.

```python
def test_takewhile_none_taken(self):
    """Edge: First element fails predicate."""
    result = list(itertools.takewhile(lambda x: x > 5, [1, 2, 3]))
    assert result == []
```

**Verification**: ✅ Tested in CI

### Basic: Take first n elements.

```python
def test_islice_stop_only(self):
    """Basic: Take first n elements."""
    result = list(itertools.islice(range(10), 5))
    assert result == [0, 1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Feature: Slice with start and stop.

```python
def test_islice_start_stop(self):
    """Feature: Slice with start and stop."""
    result = list(itertools.islice(range(10), 2, 7))
    assert result == [2, 3, 4, 5, 6]
```

**Verification**: ✅ Tested in CI

### Feature: Slice with step.

```python
def test_islice_with_step(self):
    """Feature: Slice with step."""
    result = list(itertools.islice(range(10), 0, 10, 2))
    assert result == [0, 2, 4, 6, 8]
```

**Verification**: ✅ Tested in CI

### Property: Works with infinite iterators.

```python
def test_islice_infinite_iterator(self):
    """Property: Works with infinite iterators."""
    result = list(itertools.islice(itertools.count(), 5))
    assert result == [0, 1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Basic: Cumulative sum (default).

```python
def test_accumulate_default_sum(self):
    """Basic: Cumulative sum (default)."""
    result = list(itertools.accumulate([1, 2, 3, 4]))
    assert result == [1, 3, 6, 10]
```

**Verification**: ✅ Tested in CI

### Feature: Cumulative with custom function.

```python
def test_accumulate_with_function(self):
    """Feature: Cumulative with custom function."""
    import operator
    result = list(itertools.accumulate([1, 2, 3, 4], operator.mul))
    assert result == [1, 2, 6, 24]
```

**Verification**: ✅ Tested in CI

### Edge: Empty iterable produces empty result.

```python
def test_accumulate_empty(self):
    """Edge: Empty iterable produces empty result."""
    result = list(itertools.accumulate([]))
    assert result == []
```

**Verification**: ✅ Tested in CI
