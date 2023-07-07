#![allow(unused)]
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type MaybeNode<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct LinkedList<T: Copy> {
    head: MaybeNode<T>,
    tail: MaybeNode<T>,
    len: usize,
}

#[derive(Debug, Clone)]
struct Node<T: Copy> {
    pub value: T,
    pub next: MaybeNode<T>,
    // TODO: doubly linked list
}

impl<T: Copy> Node<T> {
    fn new(value: T, next: MaybeNode<T>) -> Self {
        Self { next, value }
    }

    fn iter(&self) -> NodeIter<T> {
        let d = self.clone();

        NodeIter {
            cursor: Some(Rc::new(RefCell::new(d))),
        }
    }
}

pub struct NodeIter<T: Copy> {
    cursor: MaybeNode<T>,
}

impl<T: Copy> Iterator for NodeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.cursor.take() {
            let cursor = cursor.borrow();

            self.cursor = cursor.next.clone();

            Some(cursor.value)
        } else {
            None
        }
    }
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn iter(&self) -> NodeIter<T> {
        self.head
            .as_ref()
            .expect("List does not have a head")
            .borrow()
            .iter()
    }

    pub fn append(&mut self, value: T) {
        if let Some(d) = self.tail.as_mut() {
            let old_tail = self.tail.take();
            let new_tail = Some(Rc::new(RefCell::new(Node::new(value, None))));

            old_tail.unwrap().borrow_mut().next = new_tail.clone();

            self.tail = new_tail;
        } else {
            let new_node = Some(Rc::new(RefCell::new(Node::new(value, None))));

            self.tail = new_node.clone();
            self.head = new_node;
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut current = Some(Rc::clone(self.head.as_ref().unwrap()));
        let mut prev: MaybeNode<T> = None;

        for i in 0..self.len() {
            if i == index {
                let current = current.take().expect("Failed to take current node");
                let mut current = current.borrow_mut();

                if let Some(next) = current.next.take() {
                    match prev {
                        Some(prev) => {
                            let mut prev = prev.borrow_mut();
                            prev.next = Some(next);
                        }
                        None => self.head = Some(next),
                    }

                    // TODO: update tail if necessary

                    self.len -= 1;
                }

                return Some(current.value);
            }

            let container = current.unwrap();
            let node = container.borrow();

            prev = Some(Rc::clone(&container));

            match &node.next {
                Some(n) => current = Some(Rc::clone(n)),
                None => break,
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[test]
fn test_linked_list_t() {
    let mut list = LinkedList::new();

    list.append(1);
    list.append(2);
    list.append(4);
    list.append(8);

    assert_eq!(list.iter().nth(2).unwrap(), 4);
    assert_eq!(list.iter().sum::<i32>(), 15);
}

#[test]
#[ignore]
fn test_node_iter() {
    let mut list = Node::new(1, Some(Rc::new(RefCell::new(Node::new(2, None)))));

    for i in list.iter() {
        println!("{}", i);
    }
}

#[test]
fn test_linked_list_remove() {
    let mut list = LinkedList::new();

    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);

    let ele = list.remove(0).unwrap();

    assert_eq!(ele, 1);
    assert_eq!(list.len(), 4);

    let ele = list.remove(2).unwrap();

    assert_eq!(ele, 4);
    assert_eq!(list.len(), 3);
}
