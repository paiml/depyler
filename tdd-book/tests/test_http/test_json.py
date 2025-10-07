"""
TDD Book - Phase 4: Network & IPC
Module: json - JSON encoding and decoding
Coverage: dumps, loads, dump, load, JSONEncoder, JSONDecoder

Test Categories:
- JSON encoding (dumps, dump)
- JSON decoding (loads, load)
- Data types (dict, list, str, int, float, bool, None)
- Custom encoders/decoders
- Pretty printing (indent, separators)
- Error handling
- Edge cases
"""

import json
import io
import pytest
from decimal import Decimal


class TestJSONDumps:
    """Test json.dumps() - serialize to JSON string."""

    def test_dumps_dict(self):
        """Property: dumps() serializes dict to JSON string."""
        data = {"key": "value", "number": 42}
        result = json.dumps(data)

        assert isinstance(result, str)
        assert '"key"' in result
        assert '"value"' in result

    def test_dumps_list(self):
        """Property: dumps() serializes list to JSON array."""
        data = [1, 2, 3, "four"]
        result = json.dumps(data)

        assert result == '[1, 2, 3, "four"]'

    def test_dumps_nested(self):
        """Property: dumps() handles nested structures."""
        data = {"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}
        result = json.dumps(data)

        assert '"users"' in result
        assert '"Alice"' in result
        assert '"Bob"' in result

    def test_dumps_primitive_types(self):
        """Property: dumps() handles all JSON primitive types."""
        assert json.dumps("string") == '"string"'
        assert json.dumps(42) == "42"
        assert json.dumps(3.14) == "3.14"
        assert json.dumps(True) == "true"
        assert json.dumps(False) == "false"
        assert json.dumps(None) == "null"

    def test_dumps_empty_containers(self):
        """Property: dumps() handles empty containers."""
        assert json.dumps({}) == "{}"
        assert json.dumps([]) == "[]"

    def test_dumps_with_indent(self):
        """Property: dumps() with indent produces pretty output."""
        data = {"key": "value", "list": [1, 2, 3]}
        result = json.dumps(data, indent=2)

        assert "\n" in result  # Should have newlines
        assert "  " in result  # Should have indentation

    def test_dumps_sort_keys(self):
        """Property: dumps() with sort_keys sorts dictionary keys."""
        data = {"z": 1, "a": 2, "m": 3}
        result = json.dumps(data, sort_keys=True)

        # 'a' should come before 'z' in output
        pos_a = result.index('"a"')
        pos_z = result.index('"z"')
        assert pos_a < pos_z


class TestJSONLoads:
    """Test json.loads() - deserialize from JSON string."""

    def test_loads_dict(self):
        """Property: loads() deserializes JSON object to dict."""
        json_str = '{"key": "value", "number": 42}'
        result = json.loads(json_str)

        assert result == {"key": "value", "number": 42}

    def test_loads_list(self):
        """Property: loads() deserializes JSON array to list."""
        json_str = '[1, 2, 3, "four"]'
        result = json.loads(json_str)

        assert result == [1, 2, 3, "four"]

    def test_loads_nested(self):
        """Property: loads() handles nested structures."""
        json_str = '{"users": [{"name": "Alice"}, {"name": "Bob"}]}'
        result = json.loads(json_str)

        assert result["users"][0]["name"] == "Alice"
        assert result["users"][1]["name"] == "Bob"

    def test_loads_primitive_types(self):
        """Property: loads() handles all JSON primitive types."""
        assert json.loads('"string"') == "string"
        assert json.loads("42") == 42
        assert json.loads("3.14") == 3.14
        assert json.loads("true") is True
        assert json.loads("false") is False
        assert json.loads("null") is None

    def test_loads_unicode(self):
        """Property: loads() handles Unicode strings."""
        json_str = '{"message": "Hello 世界"}'
        result = json.loads(json_str)

        assert result["message"] == "Hello 世界"

    def test_loads_escaped_characters(self):
        """Property: loads() handles escaped characters."""
        json_str = '{"text": "Line 1\\nLine 2\\tTabbed"}'
        result = json.loads(json_str)

        assert "\n" in result["text"]
        assert "\t" in result["text"]


class TestJSONDumpLoad:
    """Test json.dump() and json.load() - file I/O."""

    def test_dump_to_file(self):
        """Property: dump() writes JSON to file object."""
        data = {"key": "value", "number": 42}
        file_obj = io.StringIO()

        json.dump(data, file_obj)
        file_obj.seek(0)

        content = file_obj.read()
        assert '"key"' in content
        assert '"value"' in content

    def test_load_from_file(self):
        """Property: load() reads JSON from file object."""
        json_str = '{"key": "value", "number": 42}'
        file_obj = io.StringIO(json_str)

        result = json.load(file_obj)

        assert result == {"key": "value", "number": 42}

    def test_dump_load_roundtrip(self):
        """Property: dump() and load() roundtrip data."""
        original = {"users": ["Alice", "Bob"], "count": 2, "active": True}

        # Dump to file
        file_obj = io.StringIO()
        json.dump(original, file_obj)

        # Load back
        file_obj.seek(0)
        result = json.load(file_obj)

        assert result == original


class TestJSONDumpsLoadsRoundtrip:
    """Test dumps() and loads() roundtrip behavior."""

    def test_roundtrip_dict(self):
        """Property: dumps()/loads() roundtrip preserves dict."""
        original = {"key": "value", "nested": {"inner": 123}}
        json_str = json.dumps(original)
        result = json.loads(json_str)

        assert result == original

    def test_roundtrip_list(self):
        """Property: dumps()/loads() roundtrip preserves list."""
        original = [1, "two", 3.0, True, None, {"nested": "dict"}]
        json_str = json.dumps(original)
        result = json.loads(json_str)

        assert result == original

    def test_roundtrip_unicode(self):
        """Property: dumps()/loads() preserves Unicode."""
        original = {"text": "Hello 世界 🌍"}
        json_str = json.dumps(original, ensure_ascii=False)
        result = json.loads(json_str)

        assert result == original

    def test_roundtrip_numbers(self):
        """Property: dumps()/loads() preserves numbers."""
        original = {"int": 42, "float": 3.14159, "negative": -10}
        json_str = json.dumps(original)
        result = json.loads(json_str)

        assert result == original


class TestJSONEncoder:
    """Test JSONEncoder customization."""

    def test_custom_encoder_class(self):
        """Property: Custom JSONEncoder can serialize custom types."""

        class Point:
            def __init__(self, x, y):
                self.x = x
                self.y = y

        class PointEncoder(json.JSONEncoder):
            def default(self, obj):
                if isinstance(obj, Point):
                    return {"x": obj.x, "y": obj.y, "_type": "Point"}
                return super().default(obj)

        point = Point(10, 20)
        result = json.dumps(point, cls=PointEncoder)

        assert '"x": 10' in result
        assert '"y": 20' in result

    def test_encoder_default_method(self):
        """Property: JSONEncoder.default() handles unknown types."""

        class CustomEncoder(json.JSONEncoder):
            def default(self, obj):
                if isinstance(obj, Decimal):
                    return float(obj)
                return super().default(obj)

        data = {"price": Decimal("19.99")}
        result = json.dumps(data, cls=CustomEncoder)

        loaded = json.loads(result)
        assert loaded["price"] == 19.99


class TestJSONDecoder:
    """Test JSONDecoder customization."""

    def test_custom_object_hook(self):
        """Property: object_hook allows custom deserialization."""

        def point_decoder(dct):
            if "_type" in dct and dct["_type"] == "Point":
                return (dct["x"], dct["y"])
            return dct

        json_str = '{"_type": "Point", "x": 10, "y": 20}'
        result = json.loads(json_str, object_hook=point_decoder)

        assert result == (10, 20)

    def test_parse_int_hook(self):
        """Property: parse_int allows custom int parsing."""
        json_str = '{"count": 42}'

        def custom_int(s):
            return int(s) * 2

        result = json.loads(json_str, parse_int=custom_int)

        assert result["count"] == 84  # 42 * 2


class TestJSONFormatting:
    """Test JSON output formatting options."""

    def test_separators_compact(self):
        """Property: separators control output formatting."""
        data = {"a": 1, "b": 2}
        result = json.dumps(data, separators=(",", ":"))

        # Should have no spaces after : or ,
        assert '{"a":1,"b":2}' == result

    def test_separators_spaced(self):
        """Property: separators can add spacing."""
        data = {"a": 1}
        result = json.dumps(data, separators=(", ", ": "))

        assert '{"a": 1}' == result

    def test_indent_nested(self):
        """Property: indent applies to nested structures."""
        data = {"outer": {"inner": {"deep": "value"}}}
        result = json.dumps(data, indent=2)

        lines = result.split("\n")
        # Should have multiple indent levels
        assert len(lines) > 3

    def test_ensure_ascii_true(self):
        """Property: ensure_ascii=True escapes non-ASCII."""
        data = {"text": "Hello 世界"}
        result = json.dumps(data, ensure_ascii=True)

        # Non-ASCII should be escaped as \\uXXXX
        assert "\\u" in result

    def test_ensure_ascii_false(self):
        """Property: ensure_ascii=False preserves Unicode."""
        data = {"text": "Hello 世界"}
        result = json.dumps(data, ensure_ascii=False)

        # Unicode should be preserved
        assert "世界" in result


class TestJSONErrors:
    """Test JSON error handling."""

    def test_loads_invalid_json(self):
        """Property: loads() raises JSONDecodeError on invalid JSON."""
        with pytest.raises(json.JSONDecodeError):
            json.loads("{invalid json}")

    def test_loads_trailing_comma(self):
        """Property: loads() raises on trailing comma."""
        with pytest.raises(json.JSONDecodeError):
            json.loads('{"key": "value",}')

    def test_loads_single_quotes(self):
        """Property: loads() requires double quotes."""
        with pytest.raises(json.JSONDecodeError):
            json.loads("{'key': 'value'}")

    def test_dumps_unsupported_type(self):
        """Property: dumps() raises TypeError on unsupported types."""

        class CustomObject:
            pass

        with pytest.raises(TypeError):
            json.dumps(CustomObject())

    def test_loads_unclosed_string(self):
        """Property: loads() raises on unclosed string."""
        with pytest.raises(json.JSONDecodeError):
            json.loads('{"key": "value}')

    def test_loads_unexpected_token(self):
        """Property: loads() raises on unexpected token."""
        with pytest.raises(json.JSONDecodeError):
            json.loads("{key: value}")  # Missing quotes


class TestJSONEdgeCases:
    """Test edge cases and special values."""

    def test_dumps_empty_string(self):
        """Property: dumps() handles empty string."""
        assert json.dumps("") == '""'

    def test_loads_empty_object(self):
        """Property: loads() handles empty object."""
        assert json.loads("{}") == {}

    def test_loads_empty_array(self):
        """Property: loads() handles empty array."""
        assert json.loads("[]") == []

    def test_dumps_nested_empty(self):
        """Property: dumps() handles nested empty structures."""
        data = {"outer": {"inner": []}}
        result = json.dumps(data)

        assert json.loads(result) == data

    def test_dumps_very_large_number(self):
        """Property: dumps() handles large numbers."""
        data = {"big": 10**100}
        result = json.dumps(data)

        loaded = json.loads(result)
        assert loaded["big"] == 10**100

    def test_dumps_negative_numbers(self):
        """Property: dumps() handles negative numbers."""
        data = {"neg_int": -42, "neg_float": -3.14}
        result = json.dumps(data)

        loaded = json.loads(result)
        assert loaded["neg_int"] == -42
        assert loaded["neg_float"] == -3.14

    def test_loads_whitespace(self):
        """Property: loads() ignores whitespace."""
        json_str = """
        {
            "key"  :  "value"  ,
            "number"  :  42
        }
        """
        result = json.loads(json_str)

        assert result == {"key": "value", "number": 42}

    def test_dumps_boolean_values(self):
        """Property: dumps() maps Python bool to JSON bool."""
        assert json.dumps(True) == "true"
        assert json.dumps(False) == "false"

    def test_loads_boolean_values(self):
        """Property: loads() maps JSON bool to Python bool."""
        assert json.loads("true") is True
        assert json.loads("false") is False

    def test_dumps_null(self):
        """Property: dumps() maps None to null."""
        assert json.dumps(None) == "null"

    def test_loads_null(self):
        """Property: loads() maps null to None."""
        assert json.loads("null") is None

    def test_dumps_list_with_none(self):
        """Property: dumps() handles None in lists."""
        data = [1, None, 3]
        result = json.dumps(data)

        assert result == "[1, null, 3]"

    def test_special_characters_in_strings(self):
        """Property: dumps()/loads() handle special characters."""
        data = {"text": 'Line 1\nLine 2\tTab\r\n"Quoted"'}
        json_str = json.dumps(data)
        result = json.loads(json_str)

        assert result == data

    def test_unicode_escape_sequences(self):
        """Property: loads() handles Unicode escape sequences."""
        json_str = '{"emoji": "\\ud83d\\ude00"}'
        result = json.loads(json_str)

        # Should decode to actual emoji
        assert len(result["emoji"]) >= 1

    def test_allow_nan_false(self):
        """Property: allow_nan=False rejects NaN/Infinity."""
        import math

        with pytest.raises(ValueError):
            json.dumps({"value": math.nan}, allow_nan=False)

        with pytest.raises(ValueError):
            json.dumps({"value": math.inf}, allow_nan=False)
