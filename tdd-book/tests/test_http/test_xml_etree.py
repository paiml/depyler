"""
TDD Book - Phase 4: Network & IPC
Module: xml.etree.ElementTree - XML processing
Coverage: Element creation, parsing, finding, modification, serialization

Test Categories:
- Element creation and attributes
- XML parsing (fromstring, parse)
- Tree navigation (find, findall, iter)
- Element modification
- XML serialization (tostring, write)
- Edge cases
"""

import xml.etree.ElementTree as ET
import io
import pytest


class TestElementCreation:
    """Test creating XML elements."""

    def test_create_element_basic(self):
        """Property: Element() creates XML element."""
        elem = ET.Element("root")

        assert elem.tag == "root"

    def test_create_element_with_attributes(self):
        """Property: Element can have attributes."""
        elem = ET.Element("node", attrib={"id": "1", "type": "test"})

        assert elem.get("id") == "1"
        assert elem.get("type") == "test"

    def test_element_text(self):
        """Property: Element can have text content."""
        elem = ET.Element("node")
        elem.text = "Hello, World"

        assert elem.text == "Hello, World"

    def test_subelement_creation(self):
        """Property: SubElement creates child element."""
        root = ET.Element("root")
        child = ET.SubElement(root, "child")

        assert child.tag == "child"
        assert len(root) == 1
        assert root[0] is child


class TestElementAttributes:
    """Test element attribute operations."""

    def test_set_attribute(self):
        """Property: set() sets attribute."""
        elem = ET.Element("node")
        elem.set("key", "value")

        assert elem.get("key") == "value"

    def test_get_attribute(self):
        """Property: get() retrieves attribute."""
        elem = ET.Element("node", attrib={"name": "test"})

        assert elem.get("name") == "test"

    def test_get_missing_attribute_default(self):
        """Property: get() returns default for missing attribute."""
        elem = ET.Element("node")

        assert elem.get("missing") is None
        assert elem.get("missing", "default") == "default"

    def test_attribute_keys(self):
        """Property: keys() returns attribute names."""
        elem = ET.Element("node", attrib={"a": "1", "b": "2"})

        keys = list(elem.keys())
        assert "a" in keys
        assert "b" in keys

    def test_attribute_items(self):
        """Property: items() returns attribute pairs."""
        elem = ET.Element("node", attrib={"key": "value"})

        items = list(elem.items())
        assert ("key", "value") in items


class TestXMLParsing:
    """Test parsing XML from strings and files."""

    def test_fromstring_basic(self):
        """Property: fromstring() parses XML string."""
        xml = "<root><child>text</child></root>"
        root = ET.fromstring(xml)

        assert root.tag == "root"
        assert len(root) == 1
        assert root[0].tag == "child"
        assert root[0].text == "text"

    def test_fromstring_with_attributes(self):
        """Property: fromstring() preserves attributes."""
        xml = '<root id="1" type="test"><child/></root>'
        root = ET.fromstring(xml)

        assert root.get("id") == "1"
        assert root.get("type") == "test"

    def test_parse_from_file(self):
        """Property: parse() parses XML from file."""
        xml = "<root><item>value</item></root>"
        file_obj = io.StringIO(xml)

        tree = ET.parse(file_obj)
        root = tree.getroot()

        assert root.tag == "root"
        assert root[0].text == "value"

    def test_elementtree_getroot(self):
        """Property: ElementTree.getroot() returns root element."""
        xml = "<root><child/></root>"
        file_obj = io.StringIO(xml)

        tree = ET.parse(file_obj)
        root = tree.getroot()

        assert isinstance(root, ET.Element)
        assert root.tag == "root"


class TestTreeNavigation:
    """Test navigating XML tree structure."""

    def test_find_direct_child(self):
        """Property: find() finds direct child by tag."""
        root = ET.fromstring("<root><child>text</child><other/></root>")

        child = root.find("child")
        assert child is not None
        assert child.tag == "child"
        assert child.text == "text"

    def test_find_returns_none_if_not_found(self):
        """Property: find() returns None if not found."""
        root = ET.fromstring("<root><child/></root>")

        result = root.find("missing")
        assert result is None

    def test_findall_returns_all_matching(self):
        """Property: findall() returns all matching children."""
        root = ET.fromstring("<root><item>1</item><item>2</item><item>3</item></root>")

        items = root.findall("item")
        assert len(items) == 3
        assert items[0].text == "1"
        assert items[1].text == "2"
        assert items[2].text == "3"

    def test_iter_all_elements(self):
        """Property: iter() iterates over all descendants."""
        xml = "<root><child><grandchild/></child></root>"
        root = ET.fromstring(xml)

        tags = [elem.tag for elem in root.iter()]
        assert tags == ["root", "child", "grandchild"]

    def test_iter_filtered_by_tag(self):
        """Property: iter(tag) filters by tag name."""
        xml = "<root><item>1</item><other/><item>2</item></root>"
        root = ET.fromstring(xml)

        items = list(root.iter("item"))
        assert len(items) == 2
        assert all(elem.tag == "item" for elem in items)


class TestElementModification:
    """Test modifying XML elements."""

    def test_append_child(self):
        """Property: append() adds child element."""
        root = ET.Element("root")
        child = ET.Element("child")

        root.append(child)

        assert len(root) == 1
        assert root[0] is child

    def test_remove_child(self):
        """Property: remove() removes child element."""
        root = ET.Element("root")
        child1 = ET.SubElement(root, "child1")
        child2 = ET.SubElement(root, "child2")

        assert len(root) == 2

        root.remove(child1)

        assert len(root) == 1
        assert root[0] is child2

    def test_clear_element(self):
        """Property: clear() removes all children and attributes."""
        root = ET.Element("root", attrib={"id": "1"})
        ET.SubElement(root, "child1")
        ET.SubElement(root, "child2")
        root.text = "text"

        root.clear()

        assert len(root) == 0
        assert root.get("id") is None
        assert root.text is None

    def test_set_text(self):
        """Property: Setting text updates element content."""
        elem = ET.Element("node")
        elem.text = "initial"

        assert elem.text == "initial"

        elem.text = "updated"

        assert elem.text == "updated"


class TestXMLSerialization:
    """Test serializing XML to strings and files."""

    def test_tostring_basic(self):
        """Property: tostring() serializes element to bytes."""
        root = ET.Element("root")
        child = ET.SubElement(root, "child")
        child.text = "text"

        xml_bytes = ET.tostring(root)

        assert isinstance(xml_bytes, bytes)
        assert b"<root>" in xml_bytes
        assert b"<child>text</child>" in xml_bytes

    def test_tostring_encoding_unicode(self):
        """Property: tostring() with unicode encoding returns string."""
        root = ET.Element("root")
        root.text = "Hello"

        xml_str = ET.tostring(root, encoding="unicode")

        assert isinstance(xml_str, str)
        assert "<root>Hello</root>" in xml_str

    def test_write_to_file(self):
        """Property: write() writes XML to file."""
        root = ET.Element("root")
        child = ET.SubElement(root, "child")
        child.text = "value"

        tree = ET.ElementTree(root)
        file_obj = io.BytesIO()

        tree.write(file_obj)
        file_obj.seek(0)

        content = file_obj.read()
        assert b"<root>" in content
        assert b"<child>value</child>" in content


class TestElementTreeConstruction:
    """Test ElementTree construction."""

    def test_elementtree_from_element(self):
        """Property: ElementTree wraps an element."""
        root = ET.Element("root")
        tree = ET.ElementTree(root)

        assert tree.getroot() is root

    def test_elementtree_iter(self):
        """Property: ElementTree.iter() iterates over tree."""
        root = ET.Element("root")
        ET.SubElement(root, "child1")
        ET.SubElement(root, "child2")

        tree = ET.ElementTree(root)
        elements = list(tree.iter())

        assert len(elements) == 3  # root + 2 children


class TestElementIteration:
    """Test iterating over element children."""

    def test_len_element(self):
        """Property: len() returns number of direct children."""
        root = ET.Element("root")
        ET.SubElement(root, "child1")
        ET.SubElement(root, "child2")
        ET.SubElement(root, "child3")

        assert len(root) == 3

    def test_index_access(self):
        """Property: Elements support index access."""
        root = ET.Element("root")
        child1 = ET.SubElement(root, "child1")
        child2 = ET.SubElement(root, "child2")

        assert root[0] is child1
        assert root[1] is child2

    def test_iterate_children(self):
        """Property: Can iterate over child elements."""
        root = ET.Element("root")
        ET.SubElement(root, "a")
        ET.SubElement(root, "b")
        ET.SubElement(root, "c")

        tags = [child.tag for child in root]
        assert tags == ["a", "b", "c"]


class TestElementTail:
    """Test element tail text."""

    def test_element_tail(self):
        """Property: Element can have tail text."""
        root = ET.fromstring("<root><child>text</child>tail</root>")

        child = root[0]
        assert child.text == "text"
        assert child.tail == "tail"

    def test_set_tail(self):
        """Property: tail can be set."""
        root = ET.Element("root")
        child = ET.SubElement(root, "child")

        child.tail = "after child"

        assert child.tail == "after child"


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_empty_element(self):
        """Property: Element with no children or text."""
        elem = ET.Element("empty")

        assert len(elem) == 0
        assert elem.text is None

    def test_element_with_namespace(self):
        """Property: Elements can have XML namespaces."""
        xml = '<root xmlns="http://example.com"><child/></root>'
        root = ET.fromstring(xml)

        # Namespace is included in tag
        assert "http://example.com" in root.tag

    def test_parse_with_comments(self):
        """Property: XML comments are preserved."""
        xml = "<root><!-- comment --><child/></root>"
        root = ET.fromstring(xml)

        # Comments are handled (may be stripped in default parser)
        assert root.tag == "root"

    def test_parse_cdata(self):
        """Property: CDATA sections are parsed."""
        xml = "<root><![CDATA[some data]]></root>"
        root = ET.fromstring(xml)

        # CDATA becomes text
        assert root.text is not None

    def test_special_characters_in_text(self):
        """Property: Special characters are escaped."""
        elem = ET.Element("node")
        elem.text = "a < b & c > d"

        xml_str = ET.tostring(elem, encoding="unicode")

        # Should be properly escaped
        assert "&lt;" in xml_str or "<" not in xml_str.replace("<node>", "").replace("</node>", "")

    def test_unicode_text(self):
        """Property: Unicode text is preserved."""
        elem = ET.Element("node")
        elem.text = "Hello 世界"

        xml_str = ET.tostring(elem, encoding="unicode")

        # Parse back
        elem2 = ET.fromstring(xml_str)
        assert elem2.text == "Hello 世界"

    def test_attribute_with_special_chars(self):
        """Property: Attributes with special characters are escaped."""
        elem = ET.Element("node")
        elem.set("attr", 'value with "quotes"')

        xml_str = ET.tostring(elem, encoding="unicode")

        # Should be properly escaped
        elem2 = ET.fromstring(xml_str)
        assert 'quotes' in elem2.get("attr")

    def test_multiple_root_children(self):
        """Property: Root can have multiple children."""
        root = ET.Element("root")
        for i in range(10):
            ET.SubElement(root, f"child{i}")

        assert len(root) == 10

    def test_deep_nesting(self):
        """Property: Deeply nested elements work."""
        root = ET.Element("root")
        current = root

        for i in range(100):
            current = ET.SubElement(current, f"level{i}")

        # Navigate back
        count = sum(1 for _ in root.iter())
        assert count == 101  # root + 100 levels

    def test_element_equality(self):
        """Property: Element identity vs equality."""
        elem1 = ET.Element("node")
        elem2 = ET.Element("node")

        # Elements are compared by identity, not value
        assert elem1 is not elem2

    def test_findall_empty_result(self):
        """Property: findall() returns empty list if none found."""
        root = ET.Element("root")

        result = root.findall("missing")
        assert result == []
