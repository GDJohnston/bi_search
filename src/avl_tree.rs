use std::fmt::{Debug, Display};

use avl_node::Node;

mod avl_node;


#[derive(Debug)]
pub struct AvlTree {
    root: Option<Node>,
}

impl Display for AvlTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.root.as_ref().unwrap(),f)
    }
}

impl AvlTree {
    pub fn new() -> AvlTree {
        AvlTree { root: None }
    }

    pub fn load_data(mut self, data: &[u32]) -> Self {
        if self.root.is_none() && !data.is_empty() {
            self.root = Some(Node::new(data[0]));
        }

        for element in data {
            self.root = Some(self.root.unwrap().insert(*element));
        }
        self
    }

    pub fn search(_search_value: u32) -> bool {
        todo!("AVLTree search not implemented yet")
    }
}
