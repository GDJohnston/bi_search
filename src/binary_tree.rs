use std::{
    cmp::{self, Ordering},
    rc::Weak,
};

#[derive(Debug, Default)]
pub struct Node {
    value: u32,
    height: u32,
    _parent: Weak<Node>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32) -> Node {
        Node {
            value,
            ..Default::default()
        }
    }
    pub fn insert(mut self, value: u32) -> Node {
        let new_node = || Box::new(Node::new(value));

        match value.cmp(&self.value) {
            Ordering::Less => self.left = Some(Box::new(self.left.take().unwrap_or(new_node()).insert(value))),
            Ordering::Greater => self.right = Some(Box::new(self.right.take().unwrap_or(new_node()).insert(value))),
            Ordering::Equal => return self,
        };

        self.update_height();

        let balance = self.get_balance();
        if balance > 1 {
            let left = self.left.take();
            return match self.value.cmp(&left.as_ref().unwrap().value) {
                Ordering::Less => self.rotate_right(),
                Ordering::Greater => {
                    self.left = Some(Box::new(left.unwrap().rotate_left()));
                    self.rotate_right()
                }
                Ordering::Equal => panic!(),
            };
        }
        if balance < -1 {
            return match self.value.cmp(&self.right.as_ref().unwrap().value) {
                Ordering::Less => self.rotate_left(),
                Ordering::Greater => {
                    self.right = Some(Box::new(self.right.unwrap().rotate_right()));
                    self.rotate_left()
                }
                Ordering::Equal => panic!(),
            };
        }
        self
    }

    fn update_height(&mut self) {
        let left_height = Self::get_height(&self.left);
        let right_height = Self::get_height(&self.right);

        self.height = 1 + cmp::max(left_height, right_height);
    }

    fn get_height(child: &Option<Box<Node>>) -> u32 {
        match child {
            Some(x) => x.height,
            None => 0,
        }
    }

    fn get_balance(&self) -> i32 {
        // let left_height = match &self.left {
        //     Some(boxed_height) => boxed_height.height,
        //     None => 0,
        // };
        let left_height  = Self::get_height(&self.left);
        let right_height = Self::get_height(&self.right);
        (left_height as i32 - right_height as i32) as i32
    }

    fn rotate_right(mut self) -> Node {
        let mut x = self.left.take().unwrap();
        let t2 = x.right;
        self.left = t2;
        self.update_height();
        x.right = Some(Box::new(self));
        x.update_height();
        *x
    }

    fn rotate_left(mut self) -> Node {
        let mut y = self.right.take().unwrap();
        let t2 = y.left;
        self.right = t2;
        self.update_height();
        y.left = Some(Box::new(self));
        y.update_height();
        *y
    }
}

pub fn binary_tree_sort(data: &[u32], _x: &u32) -> bool {
    if data.len() == 0 {
        return false;
    }

    let mut root = Node::new(data[0]);
    for element in data {
        root = root.insert(*element);
    }
    println!("{:#?}", root);
    true
}
