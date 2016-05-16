use parse::ListElement;
use std::collections::LinkedList;

pub fn comma(l: LinkedList<ListElement>) -> ListElement {
    return ListElement::List(l);
}
