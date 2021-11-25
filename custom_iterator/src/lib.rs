use std::mem;

#[derive(Debug)]
pub enum Node<T> {
    Leaf(T),
    Children(Vec<Node<T>>)
}

struct NodeIter<'a, T>{
    children: &'a [Node<T>],
    parent: Option<Box<NodeIter<'a, T>>>
}

impl <T> Default for NodeIter<'_, T> {
    fn default() -> Self{
        NodeIter{children: &[], parent: None}
    }
}

impl<'a, T> Iterator for NodeIter<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.children.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                },
                None => None
            },
            Some(Node::Leaf(val)) => {
                self.children = &self.children[1..];
                Some(val)
            },
            Some(Node::Children(children)) => {
                self.children = &self.children[1..];
                *self = NodeIter{
                    children: children.as_slice(),
                    parent: Some(Box::new(mem::take(self)))
                };
                self.next()
            }
        }
    }
}


impl <T: std::fmt::Debug>Node<T>{
    pub fn traverse(&self) {
        match self{
            Node::Leaf(val) => {println!("{:?}", val)},
            Node::Children(children) => {
                for node in children {
                    node.traverse();
                }
            }
        }
    }

    fn iter(&self) -> NodeIter<T> {
        NodeIter{
            children: std::slice::from_ref(self),
            parent: None
        }
    }
}

pub fn create_tree<T>(a: T, b: T) -> Node<T> {
    let tree = Node::Children(
        vec![Node::Leaf(a), Node::Leaf(b)]
    );
    tree
}

mod tests{
    use super::Node;

    #[test]
    fn test_iterator(){
        let tree = Node::Children(vec![
            Node::Leaf('a'),
            Node::Leaf('b'),
            Node::Children(vec![
                Node::Leaf('c'),
                Node::Leaf('d'),
                Node::Children(vec![]),
            ]),
            Node::Children(vec![
                Node::Children(vec![
                    Node::Children(vec![Node::Leaf('e')]),
                    Node::Leaf('f'),
                ]),
            ]),
        ]);
        let nodes: Vec<char> = tree.iter().copied().collect();
        assert_eq!(nodes, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }
}