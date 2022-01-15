use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // Invariant: each node should have exactly two pointers to it. Each
        // node in the middle of the list is pointed at by its predecessor and
        // successor, while the nodes on the ends are pointed to by the list
        // itself.

        // The new node needs +2 links. Everything else should get +0.
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connect the old_head.
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
            }
            None => {
                // empty list, need to set the tail.
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // replace list head with None, and take ownership of old_head...
        // old_head itself being a Rc.
        self.head.take().map(|old_head_rc| {
            // start a new block so that node_mut_ref goes out of scope before
            // we use old_head_rc again - so the compiler can tell we don't still
            // have the mutable reference...
            {
                // get a mutable reference to the actual node within the RefCell
                let mut old_head_node_mut_ref = old_head_rc.borrow_mut();
                // take ownership of the Rc from the old head, pointing to the new head
                match old_head_node_mut_ref.next.take() {
                    Some(new_head) => {
                        // .take() would work here too...the point is that the
                        // new_head should have its prev set to None
                        new_head.borrow_mut().prev = None;
                        // NB here we're re-using the Rc which was originally
                        // owned by old_head - so there's no need for a clone.
                        self.head = Some(new_head)
                    }
                    None => {
                        // emptying list
                        self.tail = None;
                    }
                }
            }
            Rc::try_unwrap(old_head_rc).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| node.borrow())
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
