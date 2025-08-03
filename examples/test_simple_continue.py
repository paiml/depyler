def test_simple_continue():
    """Test basic continue statement"""
    for i in range(5):
        if i == 2:
            continue
        print(i)