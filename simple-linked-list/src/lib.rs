use std::iter::FromIterator;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            data: element,
            next: None,
        }
    }
}
impl <T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        // unimplemented!()
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut ptr = &self.head;
        let mut link_len = 0;
        if ptr.is_none() {
            link_len
        } else {
            loop {
                if let Some(ref node) = ptr {
                    link_len += 1;
                    if node.next.is_none() {
                        break;
                    } else {
                        ptr = &node.next
                    }
                }
            }
            link_len
        }
    }

    pub fn push(&mut self, element: T) {
        // Creating a mutable reference of the Box
        if let Some(ref mut node) = self.head {
            let new_data = Box::new(Node::new(element));
            node.next = Some(new_data)
        } else {
            let new_data = Box::new(Node::new(element));
            self.head = Some(new_data);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
