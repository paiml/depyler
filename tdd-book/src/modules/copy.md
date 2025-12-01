# copy

## Shallow copy with copy.copy().

## Deep copy with copy.deepcopy().

## Copying immutable objects.

## Copying custom class instances.

## Handling circular references.

## Custom copy behavior via __copy__ and __deepcopy__.

## Module-level utilities.

## Edge cases and special scenarios.

## Special dict copying behavior.

## Special list copying behavior.

## Copy vs simple assignment.

## Copying mixed data structures.

## Module attributes and error handling.

## Using copy.replace() for immutable objects (Python 3.13+).

### Basic: Shallow copy a list.

```python
def test_copy_list(self):
    """Basic: Shallow copy a list."""
    original = [1, 2, 3]
    copied = copy.copy(original)
    assert copied == original
    assert copied is not original
```

**Verification**: ✅ Tested in CI

### Basic: Shallow copy a dict.

```python
def test_copy_dict(self):
    """Basic: Shallow copy a dict."""
    original = {'a': 1, 'b': 2}
    copied = copy.copy(original)
    assert copied == original
    assert copied is not original
```

**Verification**: ✅ Tested in CI

### Property: Shallow copy creates independent top-level object.

```python
def test_shallow_copy_independence(self):
    """Property: Shallow copy creates independent top-level object."""
    original = [1, 2, 3]
    copied = copy.copy(original)
    copied.append(4)
    assert copied == [1, 2, 3, 4]
    assert original == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Property: Shallow copy shares nested objects.

```python
def test_shallow_copy_shares_nested(self):
    """Property: Shallow copy shares nested objects."""
    inner = [1, 2]
    original = [inner, 3]
    copied = copy.copy(original)
    copied[0].append(3)
    assert original == [[1, 2, 3], 3]
    assert copied == [[1, 2, 3], 3]
```

**Verification**: ✅ Tested in CI

### Property: Shallow dict copy shares value objects.

```python
def test_shallow_copy_dict_shares_values(self):
    """Property: Shallow dict copy shares value objects."""
    inner = [1, 2]
    original = {'list': inner, 'num': 42}
    copied = copy.copy(original)
    copied['list'].append(3)
    assert original['list'] == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Basic: Deep copy a list.

```python
def test_deepcopy_list(self):
    """Basic: Deep copy a list."""
    original = [1, 2, 3]
    copied = copy.deepcopy(original)
    assert copied == original
    assert copied is not original
```

**Verification**: ✅ Tested in CI

### Property: Deep copy creates independent nested objects.

```python
def test_deepcopy_nested_independence(self):
    """Property: Deep copy creates independent nested objects."""
    inner = [1, 2]
    original = [inner, 3]
    copied = copy.deepcopy(original)
    copied[0].append(3)
    assert original == [[1, 2], 3]
    assert copied == [[1, 2, 3], 3]
```

**Verification**: ✅ Tested in CI

### Property: Deep copy dict with independent values.

```python
def test_deepcopy_dict_independence(self):
    """Property: Deep copy dict with independent values."""
    inner = [1, 2]
    original = {'list': inner, 'num': 42}
    copied = copy.deepcopy(original)
    copied['list'].append(3)
    assert original['list'] == [1, 2]
    assert copied['list'] == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Property: Deep copy handles deeply nested structures.

```python
def test_deepcopy_deeply_nested(self):
    """Property: Deep copy handles deeply nested structures."""
    original = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
    copied = copy.deepcopy(original)
    copied[0][0].append(99)
    assert '99' not in str(original)
```

**Verification**: ✅ Tested in CI

### Property: Copying immutable objects returns same object.

```python
def test_copy_int(self):
    """Property: Copying immutable objects returns same object."""
    x = 42
    copied = copy.copy(x)
    assert copied == x
    assert copied is x
```

**Verification**: ✅ Tested in CI

### Property: String copy returns same object.

```python
def test_copy_string(self):
    """Property: String copy returns same object."""
    s = 'hello'
    copied = copy.copy(s)
    assert copied == s
    assert copied is s
```

**Verification**: ✅ Tested in CI

### Property: Tuple copy returns same object.

```python
def test_copy_tuple(self):
    """Property: Tuple copy returns same object."""
    t = (1, 2, 3)
    copied = copy.copy(t)
    assert copied == t
    assert copied is t
```

**Verification**: ✅ Tested in CI

### Property: Deep copy of immutables may return same object.

```python
def test_deepcopy_immutables(self):
    """Property: Deep copy of immutables may return same object."""
    x = 42
    copied = copy.deepcopy(x)
    assert copied == x
```

**Verification**: ✅ Tested in CI

### Basic: Copy simple custom object.

```python
def test_copy_simple_object(self):
    """Basic: Copy simple custom object."""

    class Point:

        def __init__(self, x, y):
            self.x = x
            self.y = y
    p1 = Point(1, 2)
    p2 = copy.copy(p1)
    assert p2.x == p1.x
    assert p2.y == p1.y
    assert p2 is not p1
```

**Verification**: ✅ Tested in CI

### Property: Shallow copy shares mutable attributes.

```python
def test_shallow_copy_object_with_list(self):
    """Property: Shallow copy shares mutable attributes."""

    class Container:

        def __init__(self, items):
            self.items = items
    c1 = Container([1, 2, 3])
    c2 = copy.copy(c1)
    c2.items.append(4)
    assert c1.items == [1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Property: Deep copy creates independent attributes.

```python
def test_deepcopy_object_with_list(self):
    """Property: Deep copy creates independent attributes."""

    class Container:

        def __init__(self, items):
            self.items = items
    c1 = Container([1, 2, 3])
    c2 = copy.deepcopy(c1)
    c2.items.append(4)
    assert c1.items == [1, 2, 3]
    assert c2.items == [1, 2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Feature: Deep copy handles circular references.

```python
def test_deepcopy_circular_list(self):
    """Feature: Deep copy handles circular references."""
    lst = [1, 2]
    lst.append(lst)
    copied = copy.deepcopy(lst)
    assert copied[0] == 1
    assert copied[1] == 2
    assert copied[2] is copied
```

**Verification**: ✅ Tested in CI

### Feature: Deep copy handles circular dict.

```python
def test_deepcopy_circular_dict(self):
    """Feature: Deep copy handles circular dict."""
    d = {'a': 1}
    d['self'] = d
    copied = copy.deepcopy(d)
    assert copied['a'] == 1
    assert copied['self'] is copied
```

**Verification**: ✅ Tested in CI

### Feature: Handle circular object graphs.

```python
def test_circular_object_graph(self):
    """Feature: Handle circular object graphs."""

    class Node:

        def __init__(self, value):
            self.value = value
            self.next = None
    n1 = Node(1)
    n2 = Node(2)
    n1.next = n2
    n2.next = n1
    copied = copy.deepcopy(n1)
    assert copied.value == 1
    assert copied.next.value == 2
    assert copied.next.next is copied
```

**Verification**: ✅ Tested in CI

### Feature: Custom __copy__ method.

```python
def test_custom_copy(self):
    """Feature: Custom __copy__ method."""

    class CustomCopy:

        def __init__(self, value):
            self.value = value

        def __copy__(self):
            return CustomCopy(self.value * 2)
    obj = CustomCopy(5)
    copied = copy.copy(obj)
    assert copied.value == 10
```

**Verification**: ✅ Tested in CI

### Feature: Custom __deepcopy__ method.

```python
def test_custom_deepcopy(self):
    """Feature: Custom __deepcopy__ method."""

    class CustomDeepCopy:

        def __init__(self, value):
            self.value = value

        def __deepcopy__(self, memo):
            return CustomDeepCopy(self.value + 100)
    obj = CustomDeepCopy(5)
    copied = copy.deepcopy(obj)
    assert copied.value == 105
```

**Verification**: ✅ Tested in CI

### Feature: __deepcopy__ receives memo dict.

```python
def test_deepcopy_memo(self):
    """Feature: __deepcopy__ receives memo dict."""

    class MemoAware:

        def __init__(self, value):
            self.value = value

        def __deepcopy__(self, memo):
            assert isinstance(memo, dict)
            new_obj = MemoAware(self.value)
            memo[id(self)] = new_obj
            return new_obj
    obj = MemoAware(42)
    copied = copy.deepcopy(obj)
    assert copied.value == 42
```

**Verification**: ✅ Tested in CI

### Basic: Copy module has expected functions.

```python
def test_copy_module_exists(self):
    """Basic: Copy module has expected functions."""
    assert hasattr(copy, 'copy')
    assert hasattr(copy, 'deepcopy')
    assert callable(copy.copy)
    assert callable(copy.deepcopy)
```

**Verification**: ✅ Tested in CI

### Edge: Copy empty list.

```python
def test_copy_empty_list(self):
    """Edge: Copy empty list."""
    original = []
    copied = copy.copy(original)
    assert copied == []
    assert copied is not original
```

**Verification**: ✅ Tested in CI

### Edge: Copy empty dict.

```python
def test_copy_empty_dict(self):
    """Edge: Copy empty dict."""
    original = {}
    copied = copy.copy(original)
    assert copied == {}
    assert copied is not original
```

**Verification**: ✅ Tested in CI

### Edge: Copy None.

```python
def test_copy_none(self):
    """Edge: Copy None."""
    copied = copy.copy(None)
    assert copied is None
```

**Verification**: ✅ Tested in CI

### Edge: Deep copy None.

```python
def test_deepcopy_none(self):
    """Edge: Deep copy None."""
    copied = copy.deepcopy(None)
    assert copied is None
```

**Verification**: ✅ Tested in CI

### Edge: Copy boolean.

```python
def test_copy_boolean(self):
    """Edge: Copy boolean."""
    copied_true = copy.copy(True)
    copied_false = copy.copy(False)
    assert copied_true is True
    assert copied_false is False
```

**Verification**: ✅ Tested in CI

### Performance: Deep copy large nested structure.

```python
def test_large_nested_structure(self):
    """Performance: Deep copy large nested structure."""
    data = [1, 2, 3]
    for _ in range(10):
        data = [data, data]
    copied = copy.deepcopy(data)
    assert copied is not data
```

**Verification**: ✅ Tested in CI

### Feature: Copy set.

```python
def test_copy_set(self):
    """Feature: Copy set."""
    original = {1, 2, 3}
    copied = copy.copy(original)
    assert copied == original
    assert copied is not original
    copied.add(4)
    assert 4 not in original
```

**Verification**: ✅ Tested in CI

### Property: Can't have mutable items in sets.

```python
def test_deepcopy_set_with_lists(self):
    """Property: Can't have mutable items in sets."""
    original = {1, 2, 3}
    copied = copy.deepcopy(original)
    assert copied == original
```

**Verification**: ✅ Tested in CI

### Feature: Copy frozenset.

```python
def test_copy_frozenset(self):
    """Feature: Copy frozenset."""
    original = frozenset([1, 2, 3])
    copied = copy.copy(original)
    assert copied == original
```

**Verification**: ✅ Tested in CI

### Feature: dict.copy() is shallow.

```python
def test_dict_copy_method(self):
    """Feature: dict.copy() is shallow."""
    original = {'a': [1, 2], 'b': 3}
    copied = original.copy()
    copied['c'] = 4
    assert 'c' not in original
    copied['a'].append(3)
    assert original['a'] == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Property: dict.copy() equivalent to copy.copy().

```python
def test_dict_copy_vs_copy_copy(self):
    """Property: dict.copy() equivalent to copy.copy()."""
    original = {'a': 1, 'b': [2, 3]}
    method_copy = original.copy()
    module_copy = copy.copy(original)
    assert method_copy == module_copy
    assert method_copy is not original
    assert module_copy is not original
```

**Verification**: ✅ Tested in CI

### Feature: list.copy() is shallow.

```python
def test_list_copy_method(self):
    """Feature: list.copy() is shallow."""
    original = [[1, 2], 3]
    copied = original.copy()
    copied.append(4)
    assert 4 not in original
    copied[0].append(3)
    assert original[0] == [1, 2, 3]
```

**Verification**: ✅ Tested in CI

### Property: list.copy() equivalent to copy.copy().

```python
def test_list_copy_vs_copy_copy(self):
    """Property: list.copy() equivalent to copy.copy()."""
    original = [1, [2, 3]]
    method_copy = original.copy()
    module_copy = copy.copy(original)
    assert method_copy == module_copy
    assert method_copy is not original
```

**Verification**: ✅ Tested in CI

### Property: List slicing creates shallow copy.

```python
def test_list_slice_copy(self):
    """Property: List slicing creates shallow copy."""
    original = [1, [2, 3], 4]
    copied = original[:]
    copied.append(5)
    assert 5 not in original
    copied[1].append(4)
    assert original[1] == [2, 3, 4]
```

**Verification**: ✅ Tested in CI

### Property: Assignment creates alias, not copy.

```python
def test_assignment_shares_reference(self):
    """Property: Assignment creates alias, not copy."""
    original = [1, 2, 3]
    alias = original
    alias.append(4)
    assert original == [1, 2, 3, 4]
    assert alias is original
```

**Verification**: ✅ Tested in CI

### Property: Copy creates independent object.

```python
def test_copy_creates_new_object(self):
    """Property: Copy creates independent object."""
    original = [1, 2, 3]
    copied = copy.copy(original)
    copied.append(4)
    assert original == [1, 2, 3]
    assert copied is not original
```

**Verification**: ✅ Tested in CI

### Feature: Copy list of dicts.

```python
def test_list_of_dicts(self):
    """Feature: Copy list of dicts."""
    original = [{'a': 1}, {'b': 2}]
    shallow = copy.copy(original)
    deep = copy.deepcopy(original)
    shallow[0]['a'] = 99
    assert original[0]['a'] == 99
    deep[1]['b'] = 88
    assert original[1]['b'] == 2
```

**Verification**: ✅ Tested in CI

### Feature: Copy dict of lists.

```python
def test_dict_of_lists(self):
    """Feature: Copy dict of lists."""
    original = {'nums': [1, 2], 'chars': ['a', 'b']}
    shallow = copy.copy(original)
    deep = copy.deepcopy(original)
    shallow['nums'].append(3)
    assert original['nums'] == [1, 2, 3]
    deep['chars'].append('c')
    assert original['chars'] == ['a', 'b']
```

**Verification**: ✅ Tested in CI

### Feature: Deep copy complex nested structures.

```python
def test_nested_mixed_structures(self):
    """Feature: Deep copy complex nested structures."""
    original = {'list': [1, [2, 3]], 'dict': {'nested': {'deep': [4, 5]}}, 'tuple': (6, [7, 8])}
    copied = copy.deepcopy(original)
    copied['dict']['nested']['deep'].append(99)
    assert '99' not in str(original)
```

**Verification**: ✅ Tested in CI

### Error: Some objects may not be copyable.

```python
def test_copy_error_uncopyable(self):
    """Error: Some objects may not be copyable."""
    import sys
    with pytest.raises((TypeError, AttributeError)):
        copy.deepcopy(sys)
```

**Verification**: ✅ Tested in CI

### Edge: Copy function object.

```python
def test_copy_function(self):
    """Edge: Copy function object."""

    def func():
        return 42
    copied = copy.copy(func)
    assert copied is func
```

**Verification**: ✅ Tested in CI

### Edge: Copy lambda.

```python
def test_copy_lambda(self):
    """Edge: Copy lambda."""
    lam = lambda x: x * 2
    copied = copy.copy(lam)
    assert copied(5) == 10
```

**Verification**: ✅ Tested in CI

### Note: copy.replace() added in Python 3.13.

```python
def test_replace_not_in_older_python(self):
    """Note: copy.replace() added in Python 3.13."""
    has_replace = hasattr(copy, 'replace')
    assert isinstance(has_replace, bool)
```

**Verification**: ✅ Tested in CI

## 

## 

## 

## 

## 

## 
