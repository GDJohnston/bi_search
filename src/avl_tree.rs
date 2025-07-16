use avl_node::Node;

mod avl_node;

pub struct AvlTree {
    root: Option<Node>,
}

impl AvlTree {
    pub fn new() -> AvlTree {
        AvlTree {
            root: None,
        }
    }

    pub fn load_data(mut self, data: &[u32]) {
        if self.root.is_none() && !data.is_empty() {
            self.root = Some(Node::new(data[0]));
        }

        for element in data {
            self.root = Some(self.root.unwrap().insert(*element));
        }
        // println!("{:#?}", self.root);
        println!("{}", self.root.unwrap());
    }

    pub fn search(_search_value: u32) -> bool {
        todo!("AVLTree search not implemented yet")
    }
}
