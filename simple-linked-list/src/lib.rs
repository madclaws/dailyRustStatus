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
impl<T: Copy> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Copy> SimpleLinkedList<T> {
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
        let new_data = Box::new(Node::new(element));
        if self.head.is_none() {
            self.head = Some(new_data);
        } else {
            let mut ptr = &mut self.head;
            loop {
                if let Some(ref mut node) = ptr {
                    if node.next.is_none() {
                        node.next = Some(new_data);
                        break;
                    } else {
                        ptr = &mut node.next
                    }
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // Find the element, whose next's is None
        // node->next == None, node = None
        // node->next->next = None, node->next = None

        let mut popped_value: Option<T> = None;
        let mut prev_ptr_count = -1;
        if self.head.is_none() {
            return None
        } else {
            let mut ptr = &self.head;
            loop {
                if let Some(ref node) = ptr {
                    if node.next.is_none() {
                        popped_value = Some(node.data);
                        break;
                    } else {
                        prev_ptr_count += 1;
                        ptr = &node.next;
                    }
                } 
            }
        }
        if prev_ptr_count == -1 {
            self.head = None;
            return popped_value;
        } else {
            let mut ptr = &self.head;
            let mut counter = 0;
            loop {
                if let Some(ref node) = ptr {
                    if counter == prev_ptr_count {
                        // node.next = None; 
                    } else{
                        counter += 1;
                        ptr = &node.next;
                    }
                } 
            }
        }
        // if self.head.is_none() {
        //     return None
        // } else {
        // let mut ptr = &self.head;
        // let mut prev_ptr: &Option<Box<Node<T>>> = &None;
        // loop {
        //     if let Some(ref node) = ptr {
        //         if node.next.is_none() {
        //             if let Some(ref prev_node) = prev_ptr {
        //                 return Some(prev_node.data)
        //             }
        //         } else {
        //             prev_ptr = ptr;
        //         }
        //     }
        // }
        // }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(ref node) = self.head {
            Some(&node.data)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let rev_list: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut ptr = &self.head;
        if ptr.is_none() {
            self
        } else {
            loop {
                if let Some(ref node) = ptr {
                    if node.next.is_none() {
                        break;
                    } else {
                        ptr = &node.next
                    }
                }
            }
            self
        }
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
