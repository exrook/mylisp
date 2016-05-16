use parse::ListElement;
use parse::Element;
use std::collections::LinkedList;
use eval::builtin;

pub fn eval(list: LinkedList<ListElement>) -> ListElement {
    let mut list2 = LinkedList::new();
    for e in list {
        match e {
            ListElement::List(le) => {
                list2.push_back(eval(le));
            }
            // ListElement::Element(Element::Token(t)) => {
            //     list2.push_back(ListElement::Element(Element::String(t)));
            // }
            e => {
                list2.push_back(e);
            }
        }
    };
    match list2.pop_front().unwrap() {
        ListElement::Element(e) => {
            match e {
                Element::Token(f) => {
                    match builtin::lookup(&f) {
                        Some(f) => {
                            return builtin::Builtin::get_func(f)(list2);
                        }
                        None => {
                            panic!("\nUNKNOWN FUNCTION \"{}\" \n", f);
                        }
                    }
                }
                _ => {
                    panic!("ERROR: MUST START LIST WITH FUNCTION (TRY \",\"): {:#?}", e)
                }
            }
        }
        ListElement::List(l) => {
            println!("+++++++++++++++++++++++++++\n{:?}",l);
            list2.push_front(ListElement::List(l));
        }
    }
    return ListElement::List(list2);
    // list
}
