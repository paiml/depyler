"""
TDD Book - Phase 3: Concurrency
Module: weakref - Weak references to objects
Coverage: ref, proxy, WeakKeyDict, WeakValueDict, WeakSet, finalize

Test Categories:
- Basic weak references (ref)
- Weak reference callbacks
- Weak proxies
- WeakKeyDictionary and WeakValueDictionary
- WeakSet
- finalize() cleanup callbacks
- WeakMethod for bound methods
- Edge cases and garbage collection
"""

import pytest
import weakref
import gc


# Helper class that supports weak references
class WeakRefable:
    """Simple class that supports weak references."""
    def __init__(self, value=None):
        self.value = value

    def __hash__(self):
        return hash(id(self))


class TestWeakRef:
    """Test basic weak reference functionality."""

    def test_weakref_creation(self):
        """Property: weakref.ref() creates weak reference."""
        obj = WeakRefable("value")
        ref = weakref.ref(obj)

        assert ref() is obj

    def test_weakref_becomes_none(self):
        """Property: Weak reference becomes None when referent deleted."""
        obj = WeakRefable("value")
        ref = weakref.ref(obj)

        del obj
        gc.collect()

        assert ref() is None

    def test_weakref_with_callback(self):
        """Property: Callback is called when referent is deleted."""
        called = []

        def callback(ref):
            called.append(True)

        obj = WeakRefable("value")
        ref = weakref.ref(obj, callback)

        del obj
        gc.collect()

        assert len(called) == 1

    def test_weakref_equality(self):
        """Property: Weak references to same object are equal."""
        obj = WeakRefable("value")
        ref1 = weakref.ref(obj)
        ref2 = weakref.ref(obj)

        assert ref1 == ref2


class TestWeakProxy:
    """Test weak proxy functionality."""

    def test_proxy_creation(self):
        """Property: proxy() creates transparent weak proxy."""
        obj = WeakRefable("value")
        proxy = weakref.proxy(obj)

        assert proxy.value == "value"

    def test_proxy_attribute_access(self):
        """Property: Proxy allows attribute access."""
        class MyClass:
            def __init__(self):
                self.value = 42

        obj = MyClass()
        proxy = weakref.proxy(obj)

        assert proxy.value == 42

    def test_proxy_raises_after_deletion(self):
        """Property: Proxy raises ReferenceError after deletion."""
        class MyClass:
            def __init__(self):
                self.value = 42

        obj = MyClass()
        proxy = weakref.proxy(obj)

        del obj
        gc.collect()

        with pytest.raises(ReferenceError):
            _ = proxy.value


class TestWeakKeyDictionary:
    """Test WeakKeyDictionary."""

    def test_weakkeydict_basic(self):
        """Property: WeakKeyDictionary stores weak key references."""
        key = WeakRefable("key")
        d = weakref.WeakKeyDictionary()
        d[key] = "value"

        assert d[key] == "value"

    def test_weakkeydict_key_deletion(self):
        """Property: Entry removed when key is deleted."""
        key = WeakRefable("key")
        d = weakref.WeakKeyDictionary()
        d[key] = "value"

        del key
        gc.collect()

        assert len(d) == 0

    def test_weakkeydict_multiple_keys(self):
        """Property: WeakKeyDictionary handles multiple keys."""
        k1 = WeakRefable("k1")
        k2 = WeakRefable("k2")
        d = weakref.WeakKeyDictionary()

        d[k1] = "v1"
        d[k2] = "v2"

        assert d[k1] == "v1"
        assert d[k2] == "v2"

        del k1
        gc.collect()

        assert len(d) == 1
        assert d[k2] == "v2"


class TestWeakValueDictionary:
    """Test WeakValueDictionary."""

    def test_weakvaluedict_basic(self):
        """Property: WeakValueDictionary stores weak value references."""
        value = WeakRefable("value")
        d = weakref.WeakValueDictionary()
        d["key"] = value

        assert d["key"] is value

    def test_weakvaluedict_value_deletion(self):
        """Property: Entry removed when value is deleted."""
        value = WeakRefable("value")
        d = weakref.WeakValueDictionary()
        d["key"] = value

        del value
        gc.collect()

        assert len(d) == 0

    def test_weakvaluedict_multiple_values(self):
        """Property: WeakValueDictionary handles multiple values."""
        v1 = WeakRefable("v1")
        v2 = WeakRefable("v2")
        d = weakref.WeakValueDictionary()

        d["k1"] = v1
        d["k2"] = v2

        assert d["k1"] is v1
        assert d["k2"] is v2

        del v1
        gc.collect()

        assert len(d) == 1
        assert d["k2"] is v2


class TestWeakSet:
    """Test WeakSet."""

    def test_weakset_creation(self):
        """Property: WeakSet stores weak references to set members."""
        obj = WeakRefable(1)
        s = weakref.WeakSet()
        s.add(obj)

        assert obj in s

    def test_weakset_member_deletion(self):
        """Property: Member removed when object is deleted."""
        obj = WeakRefable(1)
        s = weakref.WeakSet()
        s.add(obj)

        del obj
        gc.collect()

        assert len(s) == 0

    def test_weakset_multiple_members(self):
        """Property: WeakSet handles multiple members."""
        o1 = WeakRefable(1)
        o2 = WeakRefable(2)
        s = weakref.WeakSet()

        s.add(o1)
        s.add(o2)

        assert o1 in s
        assert o2 in s

        del o1
        gc.collect()

        assert len(s) == 1
        assert o2 in s


class TestFinalize:
    """Test finalize() cleanup callbacks."""

    def test_finalize_basic(self):
        """Property: finalize() registers cleanup callback."""
        called = []

        def cleanup():
            called.append(True)

        obj = WeakRefable("value")
        finalizer = weakref.finalize(obj, cleanup)

        del obj
        gc.collect()

        assert len(called) == 1

    def test_finalize_with_args(self):
        """Property: finalize() passes arguments to callback."""
        results = []

        def cleanup(a, b):
            results.append((a, b))

        obj = WeakRefable("value")
        finalizer = weakref.finalize(obj, cleanup, "arg1", "arg2")

        del obj
        gc.collect()

        assert results == [("arg1", "arg2")]

    def test_finalize_alive(self):
        """Property: finalizer.alive indicates if object exists."""
        obj = WeakRefable("value")
        finalizer = weakref.finalize(obj, lambda: None)

        assert finalizer.alive

        del obj
        gc.collect()

        assert not finalizer.alive

    def test_finalize_detach(self):
        """Property: detach() prevents callback execution."""
        called = []

        def cleanup():
            called.append(True)

        obj = WeakRefable("value")
        finalizer = weakref.finalize(obj, cleanup)
        finalizer.detach()

        del obj
        gc.collect()

        assert len(called) == 0


class TestWeakMethod:
    """Test WeakMethod for bound methods."""

    def test_weakmethod_basic(self):
        """Property: WeakMethod creates weak reference to bound method."""
        class MyClass:
            def method(self):
                return 42

        obj = MyClass()
        ref = weakref.WeakMethod(obj.method)
        method = ref()

        assert method is not None
        assert method() == 42

    def test_weakmethod_becomes_none(self):
        """Property: WeakMethod becomes None when object deleted."""
        class MyClass:
            def method(self):
                return 42

        obj = MyClass()
        ref = weakref.WeakMethod(obj.method)

        del obj
        gc.collect()

        assert ref() is None

    def test_weakmethod_with_callback(self):
        """Property: WeakMethod supports callback."""
        called = []

        def callback(ref):
            called.append(True)

        class MyClass:
            def method(self):
                return 42

        obj = MyClass()
        ref = weakref.WeakMethod(obj.method, callback)

        del obj
        gc.collect()

        assert len(called) == 1


class TestGetWeakRefs:
    """Test getweakrefs() utility."""

    def test_getweakrefs_empty(self):
        """Property: getweakrefs() returns empty list for no refs."""
        obj = WeakRefable("value")
        refs = weakref.getweakrefs(obj)

        assert refs == []

    def test_getweakrefs_with_refs(self):
        """Property: getweakrefs() returns list of weak references."""
        obj = WeakRefable("value")
        ref1 = weakref.ref(obj)
        ref2 = weakref.ref(obj)

        refs = weakref.getweakrefs(obj)

        # Python may coalesce multiple refs to same object
        assert len(refs) >= 1
        assert ref1 in refs


class TestRefCount:
    """Test weak references don't affect reference count."""

    def test_weakref_no_refcount_increase(self):
        """Property: Weak reference doesn't prevent garbage collection."""
        import sys

        obj = WeakRefable("value")
        initial_count = sys.getrefcount(obj)

        ref = weakref.ref(obj)

        # Weak ref shouldn't increase refcount (much)
        # (may increase by 1 due to internal tracking)
        assert sys.getrefcount(obj) <= initial_count + 1


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_weakref_to_nonweakrefable(self):
        """Property: Cannot create weakref to int/str."""
        with pytest.raises(TypeError):
            weakref.ref(42)

        with pytest.raises(TypeError):
            weakref.ref("string")

    def test_weakref_to_none(self):
        """Property: Cannot create weakref to None."""
        with pytest.raises(TypeError):
            weakref.ref(None)

    def test_weakref_callback_exception(self):
        """Property: Callback exceptions are handled gracefully."""
        def bad_callback(ref):
            raise RuntimeError("callback error")

        obj = WeakRefable("value")
        ref = weakref.ref(obj, bad_callback)

        # Exception in callback shouldn't crash
        del obj
        gc.collect()

        # If we reach here, exception was handled
        assert True

    def test_proxy_callable(self):
        """Property: Proxy to callable works."""
        def func():
            return 42

        proxy = weakref.proxy(func)
        assert proxy() == 42

    def test_weakkeydict_unhashable_error(self):
        """Property: WeakKeyDictionary requires hashable keys."""
        d = weakref.WeakKeyDictionary()

        with pytest.raises(TypeError):
            d[["unhashable"]] = "value"
