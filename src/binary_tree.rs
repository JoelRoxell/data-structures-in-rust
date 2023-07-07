#![allow(unused)]
use std::{
    cell::RefCell,
    rc::{self, Rc},
};

type OptionalRcNode<T> = Option<RcNode<T>>;
type RcNode<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct BinaryTree<T> {
    root: OptionalRcNode<T>,
}

#[derive(Debug)]
struct Node<T> {
    pub left: OptionalRcNode<T>,
    pub right: OptionalRcNode<T>,
    pub value: T,
}

impl<T> Node<T> {
    pub fn new(v: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value: v,
            left: None,
            right: None,
        }))
    }
}

impl<T> BinaryTree<T>
where
    T: PartialOrd + Copy + std::fmt::Debug,
{
    fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, n: T) {
        if (self.root.is_none()) {
            self.root = Some(Node::new(n));

            return;
        }

        let head = self.root.as_ref().unwrap();
        let (parent, pos) = BinaryTree::traverse(Rc::clone(head), n);
        let new_node = Some(Node::new(n));

        match pos {
            Direction::Left => parent.borrow_mut().left = new_node,
            Direction::Right => parent.borrow_mut().right = new_node,
            Direction::Equal => todo!(),
        }
    }

    fn search(&self, n: T) -> Option<RcNode<T>> {
        let root = match self.root.as_ref() {
            Some(o) => o,
            None => return None,
        };
        let (container, direction) = BinaryTree::traverse(Rc::clone(root), n);

        match direction {
            Direction::Left => None,
            Direction::Right => None,
            Direction::Equal => Some(container),
        }
    }

    fn rebalance() {
        todo!()
    }

    fn traverse(current: RcNode<T>, n: T) -> (RcNode<T>, Direction) {
        let node = current.borrow();
        let current_value = node.value;

        if (n < current_value) {
            match &node.left {
                Some(next) => BinaryTree::traverse(Rc::clone(next), n),
                None => (Rc::clone(&current), Direction::Left),
            }
        } else if (n > current_value) {
            match &node.right {
                Some(next) => BinaryTree::traverse(Rc::clone(next), n),
                None => (Rc::clone(&current), Direction::Right),
            }
        } else {
            (Rc::clone(&current), Direction::Equal)
        }
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

    #[test]
    fn test_tree_insert() {
        let mut bt = BinaryTree::new();

        bt.insert(5);
        bt.insert(8);
        bt.insert(3);
        bt.insert(2);
        bt.insert(4);
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
}
