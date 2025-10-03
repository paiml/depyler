# functools

## functools.reduce() - Apply function cumulatively to sequence.

## functools.partial() - Partially apply function arguments.

## functools.lru_cache() - Memoization decorator.

## functools.wraps() - Preserve metadata when creating decorators.

## functools.total_ordering() - Fill in missing comparison methods.

## functools.cmp_to_key() - Convert comparison function to key function.

## functools.singledispatch() - Single-dispatch generic functions.

## functools.cache() - Unbounded cache (Python 3.9+).

## functools.cached_property() - Cached property decorator (Python 3.8+).

### Basic: Sum all elements using reduce.

```python
def test_reduce_sum(self):
    """Basic: Sum all elements using reduce."""
    result = functools.reduce(lambda x, y: x + y, [1, 2, 3, 4])
    assert result == 10
```

**Verification**: ✅ Tested in CI

### Feature: Multiply all elements.

```python
def test_reduce_multiply(self):
    """Feature: Multiply all elements."""
    result = functools.reduce(lambda x, y: x * y, [1, 2, 3, 4])
    assert result == 24
```

**Verification**: ✅ Tested in CI

### Feature: Provide initial value.

```python
def test_reduce_with_initializer(self):
    """Feature: Provide initial value."""
    result = functools.reduce(lambda x, y: x + y, [1, 2, 3], 10)
    assert result == 16
```

**Verification**: ✅ Tested in CI

### Edge: Single element returns that element.

```python
def test_reduce_single_element(self):
    """Edge: Single element returns that element."""
    result = functools.reduce(lambda x, y: x + y, [42])
    assert result == 42
```

**Verification**: ✅ Tested in CI

### Edge: Empty list with initializer returns initializer.

```python
def test_reduce_empty_with_initializer(self):
    """Edge: Empty list with initializer returns initializer."""
    result = functools.reduce(lambda x, y: x + y, [], 0)
    assert result == 0
```

**Verification**: ✅ Tested in CI

### Error: Empty list without initializer raises TypeError.

```python
def test_reduce_empty_without_initializer_raises(self):
    """Error: Empty list without initializer raises TypeError."""
    with pytest.raises(TypeError):
        functools.reduce(lambda x, y: x + y, [])
```

**Verification**: ✅ Tested in CI

### Basic: Create partially applied function.

```python
def test_partial_basic(self):
    """Basic: Create partially applied function."""

    def power(base, exponent):
        return base ** exponent
    square = functools.partial(power, exponent=2)
    assert square(5) == 25
    assert square(3) == 9
```

**Verification**: ✅ Tested in CI

### Feature: Partial with positional arguments.

```python
def test_partial_with_positional(self):
    """Feature: Partial with positional arguments."""

    def multiply(x, y, z):
        return x * y * z
    double = functools.partial(multiply, 2)
    assert double(3, 4) == 24
```

**Verification**: ✅ Tested in CI

### Edge: Can override partial keywords.

```python
def test_partial_override_keywords(self):
    """Edge: Can override partial keywords."""

    def greet(name, greeting='Hello'):
        return f'{greeting}, {name}!'
    casual_greet = functools.partial(greet, greeting='Hey')
    assert casual_greet('Alice') == 'Hey, Alice!'
    formal = casual_greet('Bob', greeting='Good morning')
    assert formal == 'Good morning, Bob!'
```

**Verification**: ✅ Tested in CI

### Property: Partial returns callable object.

```python
def test_partial_callable_object(self):
    """Property: Partial returns callable object."""

    def add(x, y):
        return x + y
    add_five = functools.partial(add, 5)
    assert callable(add_five)
    assert add_five(3) == 8
```

**Verification**: ✅ Tested in CI

### Basic: Cache function results.

```python
def test_lru_cache_basic(self):
    """Basic: Cache function results."""
    call_count = 0

    @functools.lru_cache(maxsize=128)
    def expensive_func(n):
        nonlocal call_count
        call_count += 1
        return n * 2
    result1 = expensive_func(5)
    assert result1 == 10
    assert call_count == 1
    result2 = expensive_func(5)
    assert result2 == 10
    assert call_count == 1
```

**Verification**: ✅ Tested in CI

### Feature: Different arguments create different cache entries.

```python
def test_lru_cache_different_args(self):
    """Feature: Different arguments create different cache entries."""

    @functools.lru_cache(maxsize=128)
    def square(n):
        return n ** 2
    assert square(3) == 9
    assert square(4) == 16
    assert square(3) == 9
```

**Verification**: ✅ Tested in CI

### Feature: Cache statistics via cache_info().

```python
def test_lru_cache_info(self):
    """Feature: Cache statistics via cache_info()."""

    @functools.lru_cache(maxsize=128)
    def add(x, y):
        return x + y
    add(1, 2)
    add(1, 2)
    add(2, 3)
    info = add.cache_info()
    assert info.hits == 1
    assert info.misses == 2
    assert info.currsize == 2
```

**Verification**: ✅ Tested in CI

### Feature: Clear cache with cache_clear().

```python
def test_lru_cache_clear(self):
    """Feature: Clear cache with cache_clear()."""

    @functools.lru_cache(maxsize=128)
    def multiply(x, y):
        return x * y
    multiply(2, 3)
    assert multiply.cache_info().currsize == 1
    multiply.cache_clear()
    assert multiply.cache_info().currsize == 0
```

**Verification**: ✅ Tested in CI

### Edge: maxsize=None creates unbounded cache.

```python
def test_lru_cache_maxsize_none(self):
    """Edge: maxsize=None creates unbounded cache."""

    @functools.lru_cache(maxsize=None)
    def identity(x):
        return x
    for i in range(1000):
        identity(i)
    info = identity.cache_info()
    assert info.currsize == 1000
    assert info.maxsize is None
```

**Verification**: ✅ Tested in CI

### Basic: Preserve function metadata in decorators.

```python
def test_wraps_preserves_metadata(self):
    """Basic: Preserve function metadata in decorators."""

    def my_decorator(func):

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            return func(*args, **kwargs)
        return wrapper

    @my_decorator
    def greet(name):
        """Say hello to someone."""
        return f'Hello, {name}!'
    assert greet.__name__ == 'greet'
    assert greet.__doc__ == 'Say hello to someone.'
```

**Verification**: ✅ Tested in CI

### Edge: Without @wraps, metadata is lost.

```python
def test_wraps_without_decorator(self):
    """Edge: Without @wraps, metadata is lost."""

    def bad_decorator(func):

        def wrapper(*args, **kwargs):
            return func(*args, **kwargs)
        return wrapper

    @bad_decorator
    def original():
        """Original docstring."""
        pass
    assert original.__name__ == 'wrapper'
    assert original.__doc__ is None
```

**Verification**: ✅ Tested in CI

### Basic: Define only __eq__ and __lt__, get all comparisons.

```python
def test_total_ordering_basic(self):
    """Basic: Define only __eq__ and __lt__, get all comparisons."""

    @functools.total_ordering
    class Number:

        def __init__(self, value):
            self.value = value

        def __eq__(self, other):
            return self.value == other.value

        def __lt__(self, other):
            return self.value < other.value
    a = Number(5)
    b = Number(10)
    assert a < b
    assert a <= b
    assert b > a
    assert b >= a
    assert a != b
    assert a == Number(5)
```

**Verification**: ✅ Tested in CI

### Basic: Sort with comparison function.

```python
def test_cmp_to_key_basic(self):
    """Basic: Sort with comparison function."""

    def compare(x, y):
        if x > y:
            return -1
        elif x < y:
            return 1
        else:
            return 0
    data = [3, 1, 4, 1, 5, 9, 2, 6]
    sorted_data = sorted(data, key=functools.cmp_to_key(compare))
    assert sorted_data == [9, 6, 5, 4, 3, 2, 1, 1]
```

**Verification**: ✅ Tested in CI

### Basic: Dispatch based on first argument type.

```python
def test_singledispatch_basic(self):
    """Basic: Dispatch based on first argument type."""

    @functools.singledispatch
    def process(arg):
        return f'generic: {arg}'

    @process.register(int)
    def _(arg):
        return f'int: {arg * 2}'

    @process.register(str)
    def _(arg):
        return f'str: {arg.upper()}'
    assert process(5) == 'int: 10'
    assert process('hello') == 'str: HELLO'
    assert process([1, 2]) == 'generic: [1, 2]'
```

**Verification**: ✅ Tested in CI

### Feature: Register multiple types at once.

```python
def test_singledispatch_with_types(self):
    """Feature: Register multiple types at once."""

    @functools.singledispatch
    def show(obj):
        return 'unknown'

    @show.register(list)
    @show.register(tuple)
    def _(obj):
        return f'sequence: {len(obj)}'
    assert show([1, 2, 3]) == 'sequence: 3'
    assert show((1, 2)) == 'sequence: 2'
```

**Verification**: ✅ Tested in CI

### Basic: Simple unbounded cache decorator.

```python
def test_cache_basic(self):
    """Basic: Simple unbounded cache decorator."""
    if not hasattr(functools, 'cache'):
        pytest.skip('cache() requires Python 3.9+')
    call_count = 0

    @functools.cache
    def fibonacci(n):
        nonlocal call_count
        call_count += 1
        if n < 2:
            return n
        return fibonacci(n - 1) + fibonacci(n - 2)
    result = fibonacci(10)
    assert result == 55
    assert call_count == 11
```

**Verification**: ✅ Tested in CI

### Basic: Property computed only once.

```python
def test_cached_property_basic(self):
    """Basic: Property computed only once."""
    call_count = 0

    class Circle:

        def __init__(self, radius):
            self.radius = radius

        @functools.cached_property
        def area(self):
            nonlocal call_count
            call_count += 1
            return 3.14159 * self.radius ** 2
    c = Circle(5)
    area1 = c.area
    assert call_count == 1
    area2 = c.area
    assert call_count == 1
    assert area1 == area2
```

**Verification**: ✅ Tested in CI

## 

## 
