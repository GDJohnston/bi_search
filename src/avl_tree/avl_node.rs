use std::{cmp::{self, Ordering}, fmt::{Debug, Display}, rc::Weak};

pub struct Node {
    value: u32,
    height: u32,
    _parent: Weak<Node>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Creates a new [`Node`].
    pub fn new(value: u32) -> Node {
        Node {
            value,
            height: 1,
            _parent: Weak::new(),
            left: None,
            right: None,
        }
    }

    /// Insert value as a new [`Node`].
    ///
    /// # Panics
    ///
    /// Panics if duplicate nodes exist which should be impossible
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
                Ordering::Equal => panic!(),  // Should be impossible, would be a duplicated value
            };
        }
        if balance < -1 {
            return match self.value.cmp(&self.right.as_ref().unwrap().value) {
                Ordering::Less => self.rotate_left(),
                Ordering::Greater => {
                    self.right = Some(Box::new(self.right.unwrap().rotate_right()));
                    self.rotate_left()
                }
                Ordering::Equal => panic!(),  // Should be impossible, would be a duplicated value
            };
        }
        self
    }

    /// Updates the height of this [`Node`].
    fn update_height(&mut self) {
        let left_height = Self::get_height(&self.left);
        let right_height = Self::get_height(&self.right);

        self.height = 1 + cmp::max(left_height, right_height);
    }

    /// Returns the height of the child [`Node`]
    fn get_height(child: &Option<Box<Node>>) -> u32 {
        // could probably be replaced with `is_none then` logic
        match child {
            Some(x) => x.height,
            None => 0,
        }
    }

    /// Returns the balance of this [`Node`].
    fn get_balance(&self) -> i32 {
        // let left_height = match &self.left {
        //     Some(boxed_height) => boxed_height.height,
        //     None => 0,
        // };
        let left_height  = Self::get_height(&self.left);
        let right_height = Self::get_height(&self.right);
        left_height as i32 - right_height as i32
    }

    /// Rotate right around this [`Node`].
    ///
    /// # Panics
    ///
    /// Panics if `self.left` is [`None`].
    fn rotate_right(mut self) -> Node {
        // @todo `Option::replace` might be better
        let mut x = self.left.take().unwrap();
        let t2 = x.right;
        self.left = t2;
        self.update_height();
        x.right = Some(Box::new(self));
        x.update_height();
        *x
    }
    /// Rotate left around this [`Node`].
    ///
    /// # Panics
    ///
    /// Panics if `self.right` is [`None`].
    fn rotate_left(mut self) -> Node {
        // @todo `Option::replace` might be better
        let mut y = self.right.take().unwrap();
        let t2 = y.left;
        self.right = t2;
        self.update_height();
        y.left = Some(Box::new(self));
        y.update_height();
        *y
    }

    /// Compares the search value to the nodes value.
    /// 
    /// Returns true of the values match otherwise calls
    /// search on the left or right nodes. If there are no
    /// more nodes in the tree and we have not matched the
    /// search value we return false.
    pub fn search(&self, search_value: u32) -> bool {
        match self.value.cmp(&search_value) {
            Ordering::Equal => true,
            Ordering::Less => 
            {
                match &self.left {
                    Some(node) => node.search(search_value),
                    None => false,
                }
            },
            Ordering::Greater =>
            {
                match &self.right {
                    Some(node) => node.search(search_value),
                    None => false,
                }
            },
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Custom debug to hide some fields
        f.debug_struct("Node").field("value", &self.value).field("left", &self.left).field("right", &self.right).finish()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = (self.height * 8  + 3) as usize;
        let value = self.value;
        let left_line = if self.left.is_some() {'/'} else {' '};
        let right_line = if self.right.is_some() {'\\'} else {' '};

        let mut res = writeln!(f, "{value:^padding$}");
        if self.left.is_some() || self.right.is_some() {
            let center = format!("{left_line}  {right_line}");
            res = writeln!(f, "{center:^padding$}");
        }
        if self.left.is_some() {
            res = Display::fmt(self.left.as_ref().unwrap(), f);
        }
        if self.right.is_some() {
            res = Display::fmt(self.right.as_ref().unwrap(), f);
        }
        res
    }
}