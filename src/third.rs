use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}
