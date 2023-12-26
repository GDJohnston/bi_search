use std::{cell::RefCell, cmp::Ordering, rc::Weak};

fn main() {
    let data = [
        21, 30, 31, 42, 53, 63, 68, 69, 70, 79, 83, 92, 106, 111, 121, 128, 150, 152, 153, 194,
        201, 224, 236, 242, 245, 247, 248, 252, 254, 255, 262, 269, 275, 285, 310, 315, 317, 318,
        324, 332, 338, 351, 352, 367, 375, 402, 409, 413, 428, 430, 451, 454, 473, 477, 478, 486,
        488, 491, 515, 526, 552, 570, 573, 611, 620, 625, 639, 640, 663, 665, 686, 717, 723, 728,
        736, 752, 773, 788, 814, 817, 819, 820, 822, 829, 830, 839, 852, 872, 909, 917, 954, 961,
        963, 972, 975, 976, 987, 989, 996, 998,
    ];

    let x = 976;
    let found = binary_search(&data, &x);
    binary_tree_sort(&data, &x);
    if found {
        println!("found")
    } else {
        println!("failed")
    };
}

fn binary_search(data: &[u32], x: &u32) -> bool {
    let mut l = 0;
    let mut r = data.len() - 1;

    while l <= r {
        let m = (l + r) / 2; //(l.add(r) as f32).div(2.0).floor() as usize;
        match data[m].cmp(&x) {
            Ordering::Equal => break,
            Ordering::Greater => r = m - 1,
            Ordering::Less => l = m + 1,
        }
    }
    l <= r
}

#[derive(Debug)]
struct Node {
    value: u32,
    parent: RefCell<Weak<Node>>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u32) -> Node {
        Node {
            value: value,
            parent: RefCell::new(Weak::new()),
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, value: u32) {
        let new_node = || Box::new(Node::new(value));

        match value.cmp(&self.value) {
            Ordering::Less => self.left.get_or_insert_with(new_node).insert(value),
            Ordering::Greater => self.right.get_or_insert_with(new_node).insert(value),
            Ordering::Equal => (),
        };
    }
}

fn binary_tree_sort(data: &[u32], x: &u32) -> bool {
    if data.len() == 0 {
        return false;
    }

    let mut root = Box::new(Node::new(data[0]));
    data.into_iter()
        .for_each(|value: &u32| root.insert(*value));

    println!("{:#?}", root);
    true
}
