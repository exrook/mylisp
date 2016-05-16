use std::collections::LinkedList;

use parse::ListElement;

mod defn;
mod comma;
mod print;
mod math;

// pub fn builtins() -> HashMap<String,> {
//
// }

pub struct Builtin {
    func: Box<Fn(LinkedList<ListElement>) -> ListElement>
}

impl Builtin {
    fn new(f: Box<Fn(LinkedList<ListElement>) -> ListElement>) -> Builtin {
        Builtin{func: f}
    }
    pub fn get_func(builtin: Builtin) -> Box<Fn(LinkedList<ListElement>) -> ListElement> {
        return builtin.func;
    }
}

pub fn lookup(name: &str) -> Option<Builtin> {
    Some(Builtin::new(match name {
        "print" => {Box::new(print::print)}
        "," => {Box::new(comma::comma)}
        "+" => {Box::new(math::plus)}
        "i+"=> {Box::new(math::iplus)}
        "*" => {Box::new(math::mul)}
        "i*"=> {Box::new(math::imul)}
        // "*" => {Some()}
        // "defn" => {Some(Builtin::new(Box::new(defn::defn)))}
        _ => return None
    }))
}
