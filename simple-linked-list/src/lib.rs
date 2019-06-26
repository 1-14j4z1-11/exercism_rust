pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::<T>{head:None}
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(next_node) => len_node(&next_node) + 1, 
        }
    }

    pub fn push(&mut self, _element: T) {
        // match &self.head {
        //     None => self.head = Some(Box::new(Node{item: _element, next: None})),
        //     Some(mut next_node) => push_node(&mut next_node, _element),
        // };
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(next_node) => Some(peek_node(&next_node)),
        }
    }
}

fn len_node<T>(node: &Box<Node<T>>) -> usize {
    match &node.next {
        None => 0,
        Some(next_node) => len_node(&next_node) + 1,
    }
}

fn push_node<T>(node: &mut Box<Node<T>>, element: T) {
    match &node.next {
        None => node.next = Some(Box::new(Node{item: element, next: None})),
        Some(next_node) => push_node(&mut next_node, element),
    }
}

fn peek_node<T>(node: &Box<Node<T>>) -> &T {
    match &node.next {
        None => &node.item,
        Some(next_node) => peek_node(next_node),
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
