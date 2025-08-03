def test_simple_break():
    """Test basic break statement"""
    for i in range(10):
        if i == 5:
            break
        print(i)

def test_simple_continue():
    """Test basic continue statement"""
    for i in range(10):
        if i % 2 == 0:
            continue
        print(i)

def test_nested_break():
    """Test break in nested loops"""
    for i in range(3):
        for j in range(3):
            if i == 1 and j == 1:
                break
            print(i, j)

def test_while_break():
    """Test break in while loop"""
    i = 0
    while True:
        if i >= 5:
            break
        print(i)
        i += 1

def test_while_continue():
    """Test continue in while loop"""
    i = 0
    while i < 10:
        i += 1
        if i % 2 == 0:
            continue
        print(i)