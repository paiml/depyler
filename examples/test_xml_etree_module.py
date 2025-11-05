"""
Comprehensive test suite for xml.etree.ElementTree module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests xml.etree.ElementTree core features:
- Parsing XML from strings
- Creating elements programmatically
- Finding elements by tag
- Reading/writing attributes
- Building XML trees
- Converting to string
"""

import xml.etree.ElementTree as ET


def test_xml_parse_from_string():
    """Test parsing XML from a string."""
    xml_string = '<root><child>Hello</child></root>'
    root = ET.fromstring(xml_string)
    assert root.tag == 'root'
    assert root.find('child').text == 'Hello'
    print("PASS: test_xml_parse_from_string")


def test_xml_create_element():
    """Test creating an XML element programmatically."""
    root = ET.Element('root')
    assert root.tag == 'root'
    assert root.text is None
    print("PASS: test_xml_create_element")


def test_xml_create_subelement():
    """Test creating child elements."""
    root = ET.Element('root')
    child = ET.SubElement(root, 'child')
    child.text = 'Test'
    assert len(root) == 1
    assert root[0].tag == 'child'
    assert root[0].text == 'Test'
    print("PASS: test_xml_create_subelement")


def test_xml_element_attributes():
    """Test reading and writing element attributes."""
    xml_string = '<root id="123" name="test"></root>'
    root = ET.fromstring(xml_string)
    assert root.get('id') == '123'
    assert root.get('name') == 'test'
    assert root.get('missing', 'default') == 'default'
    print("PASS: test_xml_element_attributes")


def test_xml_set_attribute():
    """Test setting attributes programmatically."""
    root = ET.Element('root')
    root.set('id', '456')
    root.set('type', 'test')
    assert root.get('id') == '456'
    assert root.get('type') == 'test'
    print("PASS: test_xml_set_attribute")


def test_xml_find_element():
    """Test finding elements by tag name."""
    xml_string = '''
    <root>
        <person>
            <name>Alice</name>
            <age>30</age>
        </person>
        <person>
            <name>Bob</name>
            <age>25</age>
        </person>
    </root>
    '''
    root = ET.fromstring(xml_string)
    person = root.find('person')
    assert person is not None
    assert person.find('name').text == 'Alice'
    print("PASS: test_xml_find_element")


def test_xml_findall_elements():
    """Test finding all elements with a tag name."""
    xml_string = '''
    <root>
        <item>First</item>
        <item>Second</item>
        <item>Third</item>
    </root>
    '''
    root = ET.fromstring(xml_string)
    items = root.findall('item')
    assert len(items) == 3
    assert items[0].text == 'First'
    assert items[1].text == 'Second'
    assert items[2].text == 'Third'
    print("PASS: test_xml_findall_elements")


def test_xml_to_string():
    """Test converting XML tree to string."""
    root = ET.Element('root')
    child = ET.SubElement(root, 'child')
    child.text = 'Content'
    xml_bytes = ET.tostring(root, encoding='unicode')
    assert '<root>' in xml_bytes
    assert '<child>Content</child>' in xml_bytes
    print("PASS: test_xml_to_string")


def test_xml_nested_structure():
    """Test working with deeply nested XML structures."""
    xml_string = '''
    <catalog>
        <book id="1">
            <title>Python Guide</title>
            <author>
                <name>John Doe</name>
                <email>john@example.com</email>
            </author>
            <price>29.99</price>
        </book>
    </catalog>
    '''
    root = ET.fromstring(xml_string)
    book = root.find('book')
    assert book.get('id') == '1'
    assert book.find('title').text == 'Python Guide'
    author = book.find('author')
    assert author.find('name').text == 'John Doe'
    assert author.find('email').text == 'john@example.com'
    print("PASS: test_xml_nested_structure")


def test_xml_iterate_children():
    """Test iterating over child elements."""
    xml_string = '''
    <root>
        <a>1</a>
        <b>2</b>
        <c>3</c>
    </root>
    '''
    root = ET.fromstring(xml_string)
    tags = []
    texts = []
    for child in root:
        tags.append(child.tag)
        texts.append(child.text)
    assert tags == ['a', 'b', 'c']
    assert texts == ['1', '2', '3']
    print("PASS: test_xml_iterate_children")


def test_xml_build_tree():
    """Test building a complete XML tree from scratch."""
    # Create structure: <users><user name="Alice" /><user name="Bob" /></users>
    users = ET.Element('users')
    user1 = ET.SubElement(users, 'user')
    user1.set('name', 'Alice')
    user1.set('id', '1')
    user2 = ET.SubElement(users, 'user')
    user2.set('name', 'Bob')
    user2.set('id', '2')

    assert len(users) == 2
    assert users[0].get('name') == 'Alice'
    assert users[1].get('name') == 'Bob'
    print("PASS: test_xml_build_tree")


def test_xml_empty_elements():
    """Test handling empty XML elements."""
    xml_string = '<root><empty/><notempty>text</notempty></root>'
    root = ET.fromstring(xml_string)
    empty = root.find('empty')
    notempty = root.find('notempty')
    assert empty.text is None
    assert notempty.text == 'text'
    print("PASS: test_xml_empty_elements")


def test_xml_text_content():
    """Test elements with text content."""
    root = ET.Element('root')
    root.text = 'Direct text content'
    assert root.text == 'Direct text content'

    child = ET.SubElement(root, 'child')
    child.text = 'Child text'
    assert child.text == 'Child text'
    print("PASS: test_xml_text_content")


def main():
    """Run all xml.etree.ElementTree tests."""
    print("=" * 60)
    print("XML.ETREE.ELEMENTTREE MODULE TESTS")
    print("=" * 60)

    # Basic parsing and creation
    test_xml_parse_from_string()
    test_xml_create_element()
    test_xml_create_subelement()

    # Attributes
    test_xml_element_attributes()
    test_xml_set_attribute()

    # Finding elements
    test_xml_find_element()
    test_xml_findall_elements()

    # Converting to string
    test_xml_to_string()

    # Advanced usage
    test_xml_nested_structure()
    test_xml_iterate_children()
    test_xml_build_tree()

    # Edge cases
    test_xml_empty_elements()
    test_xml_text_content()

    print("=" * 60)
    print("ALL XML.ETREE.ELEMENTTREE TESTS PASSED!")
    print("Total tests: 13")
    print("=" * 60)


if __name__ == "__main__":
    main()
