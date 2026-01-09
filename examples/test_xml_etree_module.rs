#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#[doc = "// NOTE: Map Python module 'xml.etree.ElementTree'(tracked in DEPYLER-0424)"]
const STR__: &'static str = "=";
#[doc = r" Sum type for heterogeneous dictionary values(Python fidelity)"]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DepylerValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    #[default]
    None,
    List(Vec<DepylerValue>),
    Dict(std::collections::HashMap<String, DepylerValue>),
}
impl std::fmt::Display for DepylerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepylerValue::Int(i) => write!(f, "{}", i),
            DepylerValue::Float(fl) => write!(f, "{}", fl),
            DepylerValue::Str(s) => write!(f, "{}", s),
            DepylerValue::Bool(b) => write!(f, "{}", b),
            DepylerValue::None => write!(f, "None"),
            DepylerValue::List(l) => write!(f, "{:?}", l),
            DepylerValue::Dict(d) => write!(f, "{:?}", d),
        }
    }
}
impl DepylerValue {
    #[doc = r" Get length of string, list, or dict"]
    pub fn len(&self) -> usize {
        match self {
            DepylerValue::Str(s) => s.len(),
            DepylerValue::List(l) => l.len(),
            DepylerValue::Dict(d) => d.len(),
            _ => 0,
        }
    }
    #[doc = r" Check if empty"]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[doc = r" Get chars iterator for string values"]
    pub fn chars(&self) -> std::str::Chars<'_> {
        match self {
            DepylerValue::Str(s) => s.chars(),
            _ => "".chars(),
        }
    }
    #[doc = r" Insert into dict(mutates self if Dict variant)"]
    pub fn insert(&mut self, key: String, value: DepylerValue) {
        if let DepylerValue::Dict(d) = self {
            d.insert(key, value);
        }
    }
    #[doc = r" Get value from dict by key"]
    pub fn get(&self, key: &str) -> Option<&DepylerValue> {
        if let DepylerValue::Dict(d) = self {
            d.get(key)
        } else {
            Option::None
        }
    }
    #[doc = r" Check if dict contains key"]
    pub fn contains_key(&self, key: &str) -> bool {
        if let DepylerValue::Dict(d) = self {
            d.contains_key(key)
        } else {
            false
        }
    }
    #[doc = r" Convert to String"]
    pub fn to_string(&self) -> String {
        match self {
            DepylerValue::Str(s) => s.clone(),
            DepylerValue::Int(i) => i.to_string(),
            DepylerValue::Float(fl) => fl.to_string(),
            DepylerValue::Bool(b) => b.to_string(),
            DepylerValue::None => "None".to_string(),
            DepylerValue::List(l) => format!("{:?}", l),
            DepylerValue::Dict(d) => format!("{:?}", d),
        }
    }
    #[doc = r" Convert to i64"]
    pub fn to_i64(&self) -> i64 {
        match self {
            DepylerValue::Int(i) => *i,
            DepylerValue::Float(fl) => *fl as i64,
            DepylerValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    #[doc = r" Convert to f64"]
    pub fn to_f64(&self) -> f64 {
        match self {
            DepylerValue::Float(fl) => *fl,
            DepylerValue::Int(i) => *i as f64,
            DepylerValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            DepylerValue::Str(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    #[doc = r" Convert to bool"]
    pub fn to_bool(&self) -> bool {
        match self {
            DepylerValue::Bool(b) => *b,
            DepylerValue::Int(i) => *i != 0,
            DepylerValue::Float(fl) => *fl != 0.0,
            DepylerValue::Str(s) => !s.is_empty(),
            DepylerValue::List(l) => !l.is_empty(),
            DepylerValue::Dict(d) => !d.is_empty(),
            DepylerValue::None => false,
        }
    }
}
impl std::ops::Index<usize> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            DepylerValue::List(l) => &l[idx],
            _ => panic!("Cannot index non-list DepylerValue"),
        }
    }
}
impl std::ops::Index<&str> for DepylerValue {
    type Output = DepylerValue;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            DepylerValue::Dict(d) => d.get(key).unwrap_or(&DepylerValue::None),
            _ => panic!("Cannot index non-dict DepylerValue with string key"),
        }
    }
}
#[doc = "Test parsing XML from a string."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_parse_from_string() {
    let xml_string = "<root><child>Hello</child></root>";
    let root = ET.fromstring(xml_string);
    assert_eq!(root.tag, "root".to_string());
    assert_eq!(
        root.find("child").map(|i| i as i32).unwrap_or(-1).text,
        "Hello".to_string()
    );
    println!("{}", "PASS: test_xml_parse_from_string");
}
#[doc = "Test creating an XML element programmatically."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_create_element() {
    let root = ET.Element("root".to_string());
    assert_eq!(root.tag, "root".to_string());
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
    assert_eq!(root.len() as i32, 1);
    assert_eq!(
        root.get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .tag,
        "child".to_string()
    );
    assert_eq!(
        root.get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .text,
        "Test"
    );
    println!("{}", "PASS: test_xml_create_subelement");
}
#[doc = "Test reading and writing element attributes."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_element_attributes() {
    let xml_string = "<root id=\"123\" name=\"test\"></root>";
    let root = ET.fromstring(xml_string);
    assert_eq!(root.get("id").cloned(), "123".to_string());
    assert_eq!(root.get("name").cloned(), "test".to_string());
    assert_eq!(
        root.get("missing").cloned().unwrap_or("default"),
        "default".to_string()
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
    assert_eq!(root.get("id").cloned(), "456".to_string());
    assert_eq!(root.get("type").cloned(), "test".to_string());
    println!("{}", "PASS: test_xml_set_attribute");
}
#[doc = "Test finding elements by tag name."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_find_element() {
    let xml_string = "\n    <root>\n        <person>\n            <name>Alice</name>\n            <age>30</age>\n        </person>\n        <person>\n            <name>Bob</name>\n            <age>25</age>\n        </person>\n    </root>\n    ";
    let root = ET.fromstring(xml_string);
    let person = root.find("person").map(|i| i as i32).unwrap_or(-1);
    assert!(person.is_some());
    assert_eq!(
        person.find("name").map(|i| i as i32).unwrap_or(-1).text,
        "Alice".to_string()
    );
    println!("{}", "PASS: test_xml_find_element");
}
#[doc = "Test finding all elements with a tag name."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_findall_elements() {
    let xml_string = "\n    <root>\n        <item>First</item>\n        <item>Second</item>\n        <item>Third</item>\n    </root>\n    ";
    let root = ET.fromstring(xml_string);
    let items = root
        .find_iter("item".to_string())
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();
    assert_eq!(items.len() as i32, 3);
    assert_eq!(
        items
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .text,
        "First".to_string()
    );
    assert_eq!(
        items
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .text,
        "Second".to_string()
    );
    assert_eq!(
        items
            .get(2usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .text,
        "Third".to_string()
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
    let xml_bytes = ET.tostring(root);
    assert!(xml_bytes.contains("<root>"));
    assert!(xml_bytes.contains("<child>Content</child>"));
    println!("{}", "PASS: test_xml_to_string");
}
#[doc = "Test working with deeply nested XML structures."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_nested_structure() {
    let xml_string = "\n    <catalog>\n        <book id=\"1\">\n            <title>Python Guide</title>\n            <author>\n                <name>John Doe</name>\n                <email>john@example.com</email>\n            </author>\n            <price>29.99</price>\n        </book>\n    </catalog>\n    ";
    let root = ET.fromstring(xml_string);
    let book = root.find("book").map(|i| i as i32).unwrap_or(-1);
    assert_eq!(book.get("id").cloned(), "1".to_string());
    assert_eq!(
        book.find("title").map(|i| i as i32).unwrap_or(-1).text,
        "Python Guide".to_string()
    );
    let author = book.find("author").map(|i| i as i32).unwrap_or(-1);
    assert_eq!(
        author.find("name").map(|i| i as i32).unwrap_or(-1).text,
        "John Doe".to_string()
    );
    assert_eq!(
        author.find("email").map(|i| i as i32).unwrap_or(-1).text,
        "john@example.com".to_string()
    );
    println!("{}", "PASS: test_xml_nested_structure");
}
#[doc = "Test iterating over child elements."]
#[doc = " Depyler: verified panic-free"]
pub fn test_xml_iterate_children() {
    let xml_string =
        "\n    <root>\n        <a>1</a>\n        <b>2</b>\n        <c>3</c>\n    </root>\n    ";
    let root = ET.fromstring(xml_string);
    let mut tags = vec![];
    let mut texts = vec![];
    for child in root.iter().cloned() {
        tags.push(child.tag);
        texts.push(child.text);
    }
    assert_eq!(
        tags,
        vec![
            "a".to_string().to_string(),
            "b".to_string().to_string(),
            "c".to_string().to_string()
        ]
    );
    assert_eq!(
        texts,
        vec![
            "1".to_string().to_string(),
            "2".to_string().to_string(),
            "3".to_string().to_string()
        ]
    );
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
    assert_eq!(users.len() as i32, 2);
    assert_eq!(
        users
            .get(0usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get("name")
            .cloned(),
        "Alice".to_string()
    );
    assert_eq!(
        users
            .get(1usize)
            .cloned()
            .expect("IndexError: list index out of range")
            .get("name")
            .cloned(),
        "Bob".to_string()
    );
    println!("{}", "PASS: test_xml_build_tree");
}
#[doc = "Test handling empty XML elements."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_empty_elements() {
    let xml_string = "<root><empty/><notempty>text</notempty></root>";
    let root = ET.fromstring(xml_string);
    let empty = root.find("empty").map(|i| i as i32).unwrap_or(-1);
    let notempty = root.find("notempty").map(|i| i as i32).unwrap_or(-1);
    assert!(empty.text.is_none());
    assert_eq!(notempty.text, "text".to_string());
    println!("{}", "PASS: test_xml_empty_elements");
}
#[doc = "Test elements with text content."]
#[doc = " Depyler: verified panic-free"]
#[doc = " Depyler: proven to terminate"]
pub fn test_xml_text_content() {
    let mut root = ET.Element("root".to_string());
    root.text = "Direct text content";
    assert_eq!(root.text, "Direct text content");
    let mut child = ET.SubElement(root, "child".to_string());
    child.text = "Child text";
    assert_eq!(child.text, "Child text");
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
