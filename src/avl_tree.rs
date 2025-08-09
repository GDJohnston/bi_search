use std::fmt::{Debug, Display};

use avl_node::Node;

mod avl_node;

#[derive(Debug, Default)]
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
    pub fn search(&self, search_value: u32) -> bool {
        if let Some(root_node) = &self.root {
            root_node.search(search_value)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [u32; 6] = [10, 20, 30, 40, 50, 60];

    #[test]
    /// Make sure we can construct an AVLTree without panicking
    fn avltree_constructs() {
        AvlTree::new();
    }

    #[test]
    /// Load data into the AvlTree
    fn avltree_load_data() {
        let binary_tree = AvlTree::new();
        binary_tree.load_data(&TEST_DATA);
    }
    
    #[test]
    /// Load an empty array into the AvlTree
    fn avltree_load_data_empty() {
        let binary_tree = AvlTree::new();
        let null_array: [u32; 0] = [];
        binary_tree.load_data(&null_array);
    }

    #[test]
    /// Find an expected value in the tree
    fn avltree_finds_expected_value() {
        let mut binary_tree = AvlTree::new();
        binary_tree = binary_tree.load_data(&TEST_DATA);
        binary_tree.search(30);
    }
}
