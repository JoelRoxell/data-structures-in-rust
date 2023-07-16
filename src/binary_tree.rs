#![allow(unused)]
use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Debug,
    rc::{self, Rc, Weak},
};

type OptionalRcNode<T> = Option<RcNode<T>>;
type RcNode<T> = Rc<RefCell<Node<T>>>;

struct BinaryTree<T> {
    root: OptionalRcNode<T>,
    n: usize,
}

#[derive(Debug)]
struct Node<T> {
    pub left: OptionalRcNode<T>,
    pub right: OptionalRcNode<T>,
    pub value: T,
    pub parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(v: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value: v,
            left: None,
            right: None,
            parent: None,
        }))
    }
}

impl<T> BinaryTree<T>
where
    T: Ord + Copy + std::fmt::Debug + std::fmt::Display,
{
    fn new() -> Self {
        Self { root: None, n: 0 }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn insert(&mut self, n: T) {
        if (self.root.is_none()) {
            self.root = Some(Node::new(n));
            self.n += 1;

            return;
        }

        let head = self.root.as_ref().unwrap();
        let (parent, branch_direction) = BinaryTree::traverse(n, Rc::clone(head));
        let new_node = Node::new(n);

        new_node.borrow_mut().parent = Some(Rc::downgrade(&parent));

        match branch_direction {
            Direction::Left => parent.borrow_mut().left = Some(new_node),
            Direction::Right => parent.borrow_mut().right = Some(new_node),
            Direction::Equal => todo!("node duplicate count is not yet supported"),
        }

        self.n += 1;
    }

    fn remove(&mut self, n: T) -> Option<T> {
        let target = match self.search(n) {
            Some(n) => n,
            None => return None,
        };

        let target = target.borrow();

        // Remove parent link if it exist
        target.parent.as_ref().map(|parent| {
            parent.upgrade().map(|p| {
                let mut parent_container = p.borrow_mut();

                if let Some(left_node) = parent_container.left.as_ref() {
                    if left_node.borrow().value == n {
                        parent_container.left = None;
                    }
                } else if let Some(right_node) = parent_container.right.as_ref() {
                    if right_node.borrow().value == n {
                        parent_container.right = None;
                    }
                }
            })
        });

        // TODO: restructure leafs

        Some(target.value)
    }

    /// Finds given element from the tree if found
    fn search(&self, n: T) -> Option<RcNode<T>> {
        let root = match self.root.as_ref() {
            Some(o) => o,
            None => return None,
        };
        let (container, direction) = BinaryTree::traverse(n, Rc::clone(root));

        match direction {
            Direction::Left => None,
            Direction::Right => None,
            Direction::Equal => Some(container),
        }
    }

    fn rebalance() {
        todo!()
    }

    /// Returns the current node that should be the parent of `target` & which direction to where
    /// it should be placed.
    fn traverse(target: T, current: RcNode<T>) -> (RcNode<T>, Direction) {
        let node = current.borrow();
        let current_value = node.value;

        match target.cmp(&current_value) {
            Ordering::Less => match &node.left {
                Some(next) => BinaryTree::traverse(target, Rc::clone(next)),
                None => (Rc::clone(&current), Direction::Left),
            },
            Ordering::Greater => match &node.right {
                Some(next) => BinaryTree::traverse(target, Rc::clone(next)),
                None => (Rc::clone(&current), Direction::Right),
            },
            Ordering::Equal => (Rc::clone(&current), Direction::Equal),
        }
    }

    /// Find longest branch by searching through the entire tree
    fn depth(current: &RcNode<T>, depth: usize, mut max_depth: Rc<RefCell<usize>>) {
        {
            let mut d = max_depth.borrow_mut();

            if depth > *d {
                *d = depth;
            }
        }

        let current = current.borrow();

        if let Some(left) = &current.left {
            BinaryTree::depth(left, depth + 1, Rc::clone(&max_depth));
        }

        if let Some(right) = &current.right {
            BinaryTree::depth(right, depth + 1, Rc::clone(&max_depth));
        }
    }

    /// Determine node print sequence
    fn print_lvls(current_level: Vec<PrintableNode<T>>) {
        let mut output = String::new();
        let mut next_level = vec![];
        let spacing = 5;

        for (i, item) in current_level.iter().enumerate() {
            match item {
                PrintableNode::Node(node, padding) => {
                    let current = node.borrow();
                    let d = format!(
                        "{:padding$}",
                        current.value,
                        padding = padding - i * spacing
                    );

                    output.push_str(&d);

                    if let Some(left) = &current.left {
                        next_level.push(PrintableNode::Node(Rc::clone(left), padding - spacing));
                    }

                    if let Some(right) = &current.right {
                        next_level.push(PrintableNode::Node(Rc::clone(right), padding + spacing));
                    }
                }

                PrintableNode::Empty => {}
            }
        }

        println!("{}", output);

        if !next_level.is_empty() {
            BinaryTree::print_lvls(next_level)
        }
    }
}

enum PrintableNode<T> {
    Node(RcNode<T>, usize),
    Empty,
}

impl<T> Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: build the str structure to represent the binary tree.
        f.write_str("This should be the binary tree presentation")
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Equal,
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::binary_tree::{BinaryTree, Node};

    use super::PrintableNode;

    #[test]
    fn test_tree_insert() {
        let mut bt = BinaryTree::new();

        bt.insert(5);
        bt.insert(8);
        bt.insert(3);
        bt.insert(2);
        bt.insert(4);

        println!("{:?}", bt);
    }

    #[test]
    fn test_tree_find() {
        let mut bt = BinaryTree::new();

        bt.insert(5);
        bt.insert(8);
        bt.insert(3);

        let d = bt.search(3);
        let d = d.unwrap();

        assert_eq!(d.borrow().value, 3);
    }

    #[test]
    fn test_tree_rm() {
        let mut a = BinaryTree::new();

        a.insert(1);
        a.insert(2);
        a.insert(3);

        assert_eq!(a.len(), 3);

        a.remove(2);
        println!("{:?}", a);
    }

    #[test]
    fn test_tree_depth() {
        let mut a = BinaryTree::new();

        a.insert(2);
        a.insert(1);
        a.insert(3);
        a.insert(4);
        a.insert(5);

        let res = Rc::new(RefCell::new(0));

        BinaryTree::depth(&a.root.unwrap(), 0, Rc::clone(&res));

        assert_eq!(*res.borrow(), 3);
    }

    #[test]
    fn test_binary_print_lvl() {
        let mut a = BinaryTree::new();

        a.insert(2);
        a.insert(1);
        a.insert(3);

        let start = vec![PrintableNode::Node(a.root.unwrap(), 10)];

        BinaryTree::print_lvls(start)
    }
}
