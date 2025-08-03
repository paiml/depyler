def test_nested_access():
    """Test reading nested dictionary values"""
    d = {"outer": {"inner": "value"}}
    # Read nested value
    val = d["outer"]["inner"]
    return val