#[doc = "// TODO: Map Python module 'xml.etree.ElementTree'"]
const STR__: &'static str = "=";
#[doc = "Test parsing XML from a string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_parse_from_string() {
    assert!(root.tag == "root".to_string());
    assert!(root.find("child").map(|i| i as i32).unwrap_or(-1).text == "Hello".to_string());
    println!("{}", "PASS: test_xml_parse_from_string");
}
#[doc = "Test creating an XML element programmatically."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_create_element() {
    assert!(root.tag == "root".to_string());
    assert!(root.text.is_none());
    println!("{}", "PASS: test_xml_create_element");
}
#[doc = "Test creating child elements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_create_subelement() {
    let root = ET.Element("root".to_string());
    let mut child = ET.SubElement(root, "child".to_string());
    child.text = "Test";
    assert!(root.len() as i32 == 1);
    assert!(
        {
            let base = &root;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .tag == "child".to_string()
    );
    assert!(
        {
            let base = &root;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .text
            == "Test"
    );
    println!("{}", "PASS: test_xml_create_subelement");
}
#[doc = "Test reading and writing element attributes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_element_attributes() {
    assert!(root.get(&"id".to_string()).cloned() == "123".to_string());
    assert!(root.get(&"name".to_string()).cloned() == "test".to_string());
    assert!(
        root.get(&"missing".to_string())
            .cloned()
            .unwrap_or("default".to_string())
            == "default".to_string()
    );
    println!("{}", "PASS: test_xml_element_attributes");
}
#[doc = "Test setting attributes programmatically."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_set_attribute() {
    let root = ET.Element("root".to_string());
    root.set("id".to_string(), "456".to_string());
    root.set("type".to_string(), "test".to_string());
    assert!(root.get(&"id".to_string()).cloned() == "456".to_string());
    assert!(root.get(&"type".to_string()).cloned() == "test".to_string());
    println!("{}", "PASS: test_xml_set_attribute");
}
#[doc = "Test finding elements by tag name."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_find_element() {
    let root = ET.fromstring("<root><empty/><notempty>text</notempty></root>".to_string());
    assert!(person.is_some());
    assert!(person.find("name").map(|i| i as i32).unwrap_or(-1).text == "Alice".to_string());
    println!("{}", "PASS: test_xml_find_element");
}
#[doc = "Test finding all elements with a tag name."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_findall_elements() {
    let root = ET.fromstring("<root><empty/><notempty>text</notempty></root>".to_string());
    assert!(items.len() as i32 == 3);
    assert!(
        {
            let base = &items;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .text
            == "First".to_string()
    );
    assert!(
        {
            let base = &items;
            let idx: i32 = 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .text
            == "Second".to_string()
    );
    assert!(
        {
            let base = &items;
            let idx: i32 = 2;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .text
            == "Third".to_string()
    );
    println!("{}", "PASS: test_xml_findall_elements");
}
#[doc = "Test converting XML tree to string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_to_string() {
    let root = ET.Element("root".to_string());
    let mut child = ET.SubElement(root, "child".to_string());
    child.text = "Content";
    assert!(xml_bytes.contains_key(&"<root>".to_string()));
    assert!(xml_bytes.contains_key(&"<child>Content</child>".to_string()));
    println!("{}", "PASS: test_xml_to_string");
}
#[doc = "Test working with deeply nested XML structures."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_nested_structure() {
    let root = ET.fromstring("<root><empty/><notempty>text</notempty></root>".to_string());
    let book = root.find("book").map(|i| i as i32).unwrap_or(-1);
    assert!(book.get(&"id".to_string()).cloned() == "1".to_string());
    assert!(book.find("title").map(|i| i as i32).unwrap_or(-1).text == "Python Guide".to_string());
    assert!(author.find("name").map(|i| i as i32).unwrap_or(-1).text == "John Doe".to_string());
    assert!(
        author.find("email").map(|i| i as i32).unwrap_or(-1).text == "john@example.com".to_string()
    );
    println!("{}", "PASS: test_xml_nested_structure");
}
#[doc = "Test iterating over child elements."]
#[doc = " Depyler: verified panic-free"]
pub fn test_xml_iterate_children() {
    let root = ET.fromstring("<root><empty/><notempty>text</notempty></root>".to_string());
    let mut tags = vec![];
    let mut texts = vec![];
    for child in root.iter().cloned() {
        tags.push(child.tag);
        texts.push(child.text);
    }
    assert!(tags == vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert!(texts == vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    println!("{}", "PASS: test_xml_iterate_children");
}
#[doc = "Test building a complete XML tree from scratch."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_build_tree() {
    let users = ET.Element("users".to_string());
    let user1 = ET.SubElement(users, "user".to_string());
    user1.set("name".to_string(), "Alice".to_string());
    user1.set("id".to_string(), "1".to_string());
    let user2 = ET.SubElement(users, "user".to_string());
    user2.set("name".to_string(), "Bob".to_string());
    user2.set("id".to_string(), "2".to_string());
    assert!(users.len() as i32 == 2);
    assert!(
        {
            let base = &users;
            let idx: i32 = 0;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .get(&"name".to_string())
        .cloned()
            == "Alice".to_string()
    );
    assert!(
        {
            let base = &users;
            let idx: i32 = 1;
            let actual_idx = if idx < 0 {
                base.len().saturating_sub(idx.abs() as usize)
            } else {
                idx as usize
            };
            base.get(actual_idx).cloned().unwrap_or_default()
        }
        .get(&"name".to_string())
        .cloned()
            == "Bob".to_string()
    );
    println!("{}", "PASS: test_xml_build_tree");
}
#[doc = "Test handling empty XML elements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_empty_elements() {
    let root = ET.fromstring("<root><empty/><notempty>text</notempty></root>".to_string());
    assert!(empty.text.is_none());
    assert!(notempty.text == "text".to_string());
    println!("{}", "PASS: test_xml_empty_elements");
}
#[doc = "Test elements with text content."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_text_content() {
    let mut root = ET.Element("root".to_string());
    root.text = "Direct text content";
    assert!(root.text == "Direct text content");
    let mut child = ET.SubElement(root, "child".to_string());
    child.text = "Child text";
    assert!(child.text == "Child text");
    println!("{}", "PASS: test_xml_text_content");
}
#[doc = "Run all xml.etree.ElementTree tests."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn main() {
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "XML.ETREE.ELEMENTTREE MODULE TESTS");
    println!("{}", STR__.repeat(60 as usize));
    test_xml_parse_from_string();
    test_xml_create_element();
    test_xml_create_subelement();
    test_xml_element_attributes();
    test_xml_set_attribute();
    test_xml_find_element();
    test_xml_findall_elements();
    test_xml_to_string();
    test_xml_nested_structure();
    test_xml_iterate_children();
    test_xml_build_tree();
    test_xml_empty_elements();
    test_xml_text_content();
    println!("{}", STR__.repeat(60 as usize));
    println!("{}", "ALL XML.ETREE.ELEMENTTREE TESTS PASSED!");
    println!("{}", "Total tests: 13");
    println!("{}", STR__.repeat(60 as usize));
}
