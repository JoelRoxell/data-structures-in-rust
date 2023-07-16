#![allow(unused)]
use std::rc::Rc;

use crate::linked_list::LinkedList;

struct Node<T> {
    value: T,
    next: Rc<Node<T>>,
}

/// Fifo
#[derive(Debug)]
struct Queue<T: Copy + PartialEq> {
    list: LinkedList<T>,
}

impl<T> Queue<T>
where
    T: Copy + PartialEq,
{
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Adds given T to the queue
    pub fn add(&mut self, v: T) -> &mut Self {
        self.list.append(v);

        self
    }

    /// Takes first element from the queue
    pub fn pop(&mut self) -> Option<T> {
        self.list.remove(0)
    }

    /// Returns the length of underlying list
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn queue_test() {
        let mut d = Queue::new();

        d.add(1).add(2).add(3);

        assert_eq!(d.len(), 3);

        let ele = d.pop().unwrap();

        assert_eq!(ele, 1);
        assert_eq!(d.len(), 2);
    }
}
