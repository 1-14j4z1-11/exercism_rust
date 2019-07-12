pub struct TreeSet<T: PartialOrd> {
    root: Option<Box<ColorTreeNode<T>>>,
}

impl<T: PartialOrd> TreeSet<T> {
    pub fn new() -> TreeSet<T> {
        TreeSet { root: None }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self.root.as_ref() {
            None => false,
            Some(node) => node.contains(value),
        }
    }

    pub fn add(&mut self, value: T) {
        match self.root.as_mut() {
            None => self.root = Some(Box::new(ColorTreeNode::new(value, NodeColor::Black))),
            Some(node) => node.add(value),
        }
    }

    pub fn remove(&mut self, value: &T) {
        if self.root.is_none() {
            return;
        };

        let node = self.root.take().unwrap();
        self.root = node.remove(value);
    }
}


impl<T> TreeSet<T> where T: PartialOrd, T: std::fmt::Debug {
    pub fn to_string(&self) -> String {
        match &self.root {
            None => "".to_string(),
            Some(node) => node.to_string(),
        }
    }
}

enum NodeColor {
    Red,
    Black,
}

struct ColorTreeNode<T: PartialOrd> {
    value: T,
    left: Option<Box<ColorTreeNode<T>>>,
    right: Option<Box<ColorTreeNode<T>>>,
    color: NodeColor,
}

impl<T: PartialOrd> ColorTreeNode<T> {
    fn new(value: T, color: NodeColor) -> ColorTreeNode<T> {
        ColorTreeNode {
            value: value,
            left: None,
            right: None,
            color: color,
        }
    }

    fn contains(&self, value: &T) -> bool {
        if *value < self.value {
            match self.left.as_ref() {
                None => false,
                Some(left) => left.contains(value),
            }
        } else if *value > self.value {
            match self.right.as_ref() {
                None => false,
                Some(right) => right.contains(value),
            }
        } else {
            true
        }
    }

    fn add(&mut self, value: T) {
        if value < self.value {
            match self.left.as_mut() {
                None => {
                    self.left = Some(Box::new(ColorTreeNode::new(value, NodeColor::Red)));
                }
                Some(left) => left.add(value),
            }
        } else if value > self.value {
            match self.right.as_mut() {
                None => {
                    self.right = Some(Box::new(ColorTreeNode::new(value, NodeColor::Red)));
                }
                Some(right) => right.add(value),
            }
        } else {
            return;
        }
    }

    fn remove(mut self, value: &T) -> Option<Box<ColorTreeNode<T>>> {
        if *value < self.value {
            match self.left {
                None => Some(Box::new(self)),
                Some(left) => { self.left = left.remove(value); Some(Box::new(self)) },
            }
        } else if *value > self.value {
            match self.right {
                None => Some(Box::new(self)),
                Some(right) => { self.right = right.remove(value); Some(Box::new(self)) },
            }
        } else {
            if self.right.is_none() {
                return self.left.take();
            }

            if self.left.is_none() {
                return self.right.take();
            }

            let mut left = self.left.unwrap();

            if left.right.is_none() {
                left.right = self.right;
                return Some(left);
            }

            let mut left_max = left.take_max();
            left_max.left = Some(left);
            left_max.right = self.right;

            Some(left_max)
        }
    }

    fn take_max(&mut self) -> Box<ColorTreeNode<T>> {
        if self.right.is_none() {
            panic!();
        }

        {
            let right = self.right.as_mut().unwrap();

            match right.right {
                Some(_) => return right.take_max(),
                None => { },
            };
        }

        let mut right = self.right.take().unwrap();
        self.right = right.left.take();
        right
    }
}

impl<T> ColorTreeNode<T> where T: PartialOrd, T: std::fmt::Debug {
    fn to_string(&self) -> String {
        let left_str = match &self.left {
            None => "_".to_string(),
            Some(left) => left.to_string(),
        };
        let right_str = match &self.right {
            None => "_".to_string(),
            Some(right) => right.to_string(),
        };
        
        format!("({},{:?},{})", left_str, self.value, right_str)
    }
}

fn rotate_left<T: PartialOrd>(mut root: Box<ColorTreeNode<T>>) -> Box<ColorTreeNode<T>> {
    fn can_rotate<T: PartialOrd>(node: &ColorTreeNode<T>) -> bool {
        match node.right.as_ref() {
            None => false,
            Some(sub) => match sub.left {
                None => false,
                Some(_) => true,
            },
        }
    };

    if !can_rotate(&root) {
        return root;
    }

    fn take_left<T: PartialOrd>(node: &mut Option<Box<ColorTreeNode<T>>>) -> Box<ColorTreeNode<T>> {
        node.as_mut().unwrap().left.take().unwrap()
    };

    let right_l = take_left(&mut root.right);
    let mut right = root.right.take().unwrap();
    root.right = Some(right_l);

    right.left = Some(root);
    right
}

fn rotate_right<T: PartialOrd>(mut root: Box<ColorTreeNode<T>>) -> Box<ColorTreeNode<T>> {
    fn can_rotate<T: PartialOrd>(node: &ColorTreeNode<T>) -> bool {
        match node.left.as_ref() {
            None => false,
            Some(sub) => match sub.right {
                None => false,
                Some(_) => true,
            },
        }
    };

    if !can_rotate(&root) {
        return root;
    }

    fn take_right<T: PartialOrd>(
        node: &mut Option<Box<ColorTreeNode<T>>>,
    ) -> Box<ColorTreeNode<T>> {
        node.as_mut().unwrap().right.take().unwrap()
    };

    let left_r = take_right(&mut root.left);
    let mut left = root.left.take().unwrap();
    root.left = Some(left_r);

    left.right = Some(root);
    left
}
