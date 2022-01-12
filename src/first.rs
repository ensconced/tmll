use std::mem::replace;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, num: i32) {
        let old_head = replace(&mut self.head, Link::Empty);
        self.head = Link::More(Box::new(Node {
            elem: num,
            next: old_head,
        }));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let first_link = replace(&mut self.head, Link::Empty);
        if let Link::More(result) = first_link {
            self.head = result.next;
            Some(result.elem)
        } else {
            None
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = replace(&mut boxed_node.next, Link::Empty);
            // now boxed_node goes out of scope and gets dropped
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
