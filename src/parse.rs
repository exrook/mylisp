use lex::LexElement;
use std::collections::LinkedList;

#[derive(Debug)]
pub enum ListElement {
    Element(Element),
    List(LinkedList<ListElement>)
}

#[derive(Debug)]
pub enum Element {
    Function(String),
    Variable(String),
    Token(String),
    String(String),
    Integer(i64),
    Float(f64),
}

impl Element {
    fn from(l: LexElement) -> Element {
        match l {
            LexElement::Float(f) => Element::Float(f),
            LexElement::Token(t) => Element::Token(t),
            LexElement::String(s) => Element::String(s),
            LexElement::Integer(i) => Element::Integer(i),
            _ => {
                panic!("Can't convert parens")
            }
        }
    }
}

pub fn parse(tokens: Vec<LexElement>) -> LinkedList<ListElement> {
    let mut list = LinkedList::new();
    let mut stack = Vec::new();
    for t in tokens {
        match t {
            LexElement::OpenParen => {
                stack.push(list);
                list = LinkedList::new();
            }
            LexElement::CloseParen => {
                let mut list2 = match stack.pop() {
                    Some(l) => l,
                    None => panic!("Unmatched Paren")
                };
                list2.push_back(ListElement::List(list));
                list = list2;
            }
            e => {
                list.push_back(ListElement::Element(Element::from(e)));
            }
        }
    }
    return list;
}
