def test_dict_assignment():
    """Test dictionary subscript assignment"""
    # Simple assignment
    d = {}
    d["key"] = "value"
    d[42] = "number key"
    
    # Nested assignment (not yet supported in full)
    nested = {}
    nested["outer"] = {}
    # nested["outer"]["inner"] = "value"  # TODO: This requires chained assignment
    
    return d