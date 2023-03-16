// Resources
// - https://learnersbucket.com/examples/algorithms/find-height-and-width-of-binary-tree/

use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut lines = INPUT.lines();
    let mut root_node = TreeNode::new(lines.next().unwrap());

    for line in lines {
        root_node.insert(line);
    }

    let height = height(&Some(Box::new(root_node.clone())));
    let width = width(&Some(Box::new(root_node)));

    println!("Height: {height}");
    println!("Width: {width}");

    println!("Product: {}", width * height);
}

fn height(node: &Option<Box<TreeNode>>) -> usize {
    match node {
        None => 0,
        Some(n) => {
            let left_height = height(&n.left);
            let right_height = height(&n.right);
            1 + left_height.max(right_height)
        }
    }
}

fn width(root: &Option<Box<TreeNode>>) -> usize {
    let mut max_width = 0;
    let mut queue = VecDeque::new();
    if let Some(node) = root {
        queue.push_back(node);
        while !queue.is_empty() {
            let level_size = queue.len();
            max_width = max_width.max(level_size);
            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();
                if let Some(left) = &node.left {
                    queue.push_back(left);
                }
                if let Some(right) = &node.right {
                    queue.push_back(right);
                }
            }
        }
    }
    max_width
}

#[derive(Default, Clone)]
struct TreeNode {
    value: &'static str,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: &'static str) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    fn insert(&mut self, value: &'static str) {
        if value > self.value {
            match self.left {
                None => {
                    self.left = Some(Box::new(Self {
                        value,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(value),
            }
        } else {
            match self.right {
                None => {
                    self.right = Some(Box::new(Self {
                        value,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(value),
            }
        }
    }
}
