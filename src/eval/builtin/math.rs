use parse::ListElement;
use parse::Element;
use std::collections::LinkedList;

pub fn plus(l: LinkedList<ListElement>) -> ListElement {
    let mut sum: f64 = 0.0;
    for e in l {
        match e {
            ListElement::Element(e) => {
                match e {
                    Element::Float(f) => {
                        sum += f;
                    }
                    Element::Integer(i) => {
                        sum += i as f64;
                    }
                    _ => {
                        panic!("\nBad argument to plus: \"{:#?}\"\n", e);
                    }
                }
            }
            ListElement::List(l) => {
                sum += match plus(l) {
                    ListElement::Element(e) => {
                        match e {
                            Element::Float(f) => {
                                f
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    _ => {
                       unreachable!();
                    }
                };
            }
        }
    }
    return ListElement::Element(Element::Float(sum));
}

pub fn iplus(l: LinkedList<ListElement>) -> ListElement {
    let mut sum: i64 = 0;
    for e in l {
        match e {
            ListElement::Element(e) => {
                match e {
                    Element::Float(f) => {
                        sum += f as i64;
                    }
                    Element::Integer(i) => {
                        sum += i;
                    }
                    _ => {
                        panic!("\nBad argument to iplus: \"{:#?}\"\n", e);
                    }
                }
            }
            ListElement::List(l) => {
                sum += match iplus(l) {
                    ListElement::Element(e) => {
                        match e {
                            Element::Integer(i) => {
                                i
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    _ => {
                       unreachable!();
                    }
                };
            }
        }
    }
    return ListElement::Element(Element::Integer(sum));
}

pub fn mul(l: LinkedList<ListElement>) -> ListElement {
    let mut product: f64 = 1.0;
    for e in l {
        match e {
            ListElement::Element(e) => {
                match e {
                    Element::Float(f) => {
                        product *= f;
                    }
                    Element::Integer(i) => {
                        product *= i as f64;
                    }
                    _ => {
                        panic!("\nBad argument to mul: \"{:#?}\"\n", e);
                    }
                }
            }
            ListElement::List(l) => {
                product *= match iplus(l) {
                    ListElement::Element(e) => {
                        match e {
                            Element::Float(i) => {
                                i
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    _ => {
                       unreachable!();
                    }
                };
            }
        }
    }
    return ListElement::Element(Element::Float(product));
}

pub fn imul(l: LinkedList<ListElement>) -> ListElement {
    let mut product: i64 = 1;
    for e in l {
        match e {
            ListElement::Element(e) => {
                match e {
                    Element::Float(f) => {
                        product *= f as i64;
                    }
                    Element::Integer(i) => {
                        product *= i;
                    }
                    _ => {
                        panic!("\nBad argument to imul: \"{:#?}\"\n", e);
                    }
                }
            }
            ListElement::List(l) => {
                product *= match iplus(l) {
                    ListElement::Element(e) => {
                        match e {
                            Element::Integer(i) => {
                                i
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    _ => {
                       unreachable!();
                    }
                };
            }
        }
    }
    return ListElement::Element(Element::Integer(product));
}
