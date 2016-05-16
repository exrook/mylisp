use parse::ListElement;
use std::collections::LinkedList;

pub fn print(l: LinkedList<ListElement>) -> ListElement {
    println!("PRINTING: {:#?}", l);
    return ListElement::List(l);
}
