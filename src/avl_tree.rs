use std::fmt::{Debug, Display};

use avl_node::Node;

mod avl_node;


#[derive(Debug)]
pub struct AvlTree {
    root: Option<Node>,
}

impl Display for AvlTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = self.root.as_ref() {
            std::fmt::Display::fmt(root, f)
        } else {
            f.write_str("AvlTree empty!")
        }
    }
}

impl AvlTree {
    /// Creates a new, empty [`AvlTree`].
    pub fn new() -> AvlTree {
        AvlTree { root: None }
    }

    /// Loads data into the [`AvlTree`].
    pub fn load_data(mut self, data: &[u32]) -> Self {
        if self.root.is_none() && !data.is_empty() {
            self.root = Some(Node::new(data[0]));
        }

        for element in data {
            if let Some(x) = self.root {
                self.root = Some(x.insert(*element));  
            }
        }
        self
    }

    /// Searches for a value in the tree
    pub fn search(_search_value: u32) -> bool {
        todo!("AVLTree search not implemented yet")
    }
}
