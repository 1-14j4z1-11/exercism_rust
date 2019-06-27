use std::iter::Iterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::<T> { head: None }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(next_node) => &next_node.len() + 1,
        }
    }

    pub fn push(&mut self, element: T) {
        match self.head.as_mut() {
            None => {
                self.head = Some(Box::new(Node {
                    value: element,
                    next: None,
                }))
            }
            Some(next_node) => next_node.push(element),
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(next_node) => match next_node.next.as_mut() {
                None => {
                    let popped_node = std::mem::replace(&mut self.head, None);
                    Some(popped_node.unwrap().value)
                }
                Some(_) => Some(next_node.pop()),
            },
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(next_node) => Some(next_node.peek()),
        }
    }
}

impl<T> Node<T> {
    fn len(&self) -> usize {
        match &self.next {
            None => 0,
            Some(next_node) => &next_node.len() + 1,
        }
    }

    fn push(&mut self, element: T) {
        match self.next.as_mut() {
            None => {
                self.next = Some(Box::new(Node {
                    value: element,
                    next: None,
                }))
            }
            Some(next_node) => next_node.push(element),
        };
    }

    fn pop(&mut self) -> T {
        let next_node = self.next.as_mut().unwrap();

        match next_node.next.as_mut() {
            None => {
                let popped_node = std::mem::replace(&mut self.next, None);
                popped_node.unwrap().value
            }
            Some(_) => next_node.pop(),
        }
    }

    fn peek(&self) -> &T {
        match &self.next {
            None => &self.value,
            Some(next_node) => next_node.peek(),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();

        let mut vector = vec![];
        let mut ptr = &self.head;

        while let Some(node) = ptr {
            vector.push(node.value.clone());
            ptr = &node.next;
        }

        for value in vector.into_iter().rev() {
            rev_list.push(value);
        }

        rev_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();

        for value in item {
            list.push(value.clone());
        }

        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vector = vec![];
        let mut ptr = self.head;

        while let Some(node) = ptr {
            vector.push(node.value);
            ptr = node.next;
        }

        vector
    }
}
