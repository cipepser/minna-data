use common::traits::{Stack, Queue};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Default)]
pub struct Node<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    x: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Node<T> {
    pub fn new(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            x,
            next: None,
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SinglyLinkedList<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    n: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Stack<T> for SinglyLinkedList<T> {
    fn push(&mut self, x: T) {
        let u = Node::new(x);
        if self.size() == 0 {
            self.tail = Some(u.clone());
        }

        if let Some(existing) = self.head.take() {
            u.borrow_mut().next = Some(existing)
        }

        self.n += 1;
        self.head = Some(u);
    }

    fn pop(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }

        let x = self.head
            .take()
            .map(|existing| {
                match existing.borrow_mut().next.take() {
                    None => { self.tail.take(); }
                    Some(next) => { self.head = Some(next); }
                };
                Rc::try_unwrap(existing).unwrap().into_inner().x
            });

        self.n -= 1;
        x
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Queue<T> for SinglyLinkedList<T> {
    fn add(&mut self, x: T) {
        let u = Node::new(x);
        if self.size() == 0 {
            self.head = Some(u.clone());
        }

        if let Some(existing) = self.tail.take() {
            existing.borrow_mut().next = Some(u.clone());
        }

        self.n += 1;
        self.tail = Some(u);
    }

    fn remove(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            n: 0,
            head: None,
            tail: None,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
#[test]
fn singly_linked_list_stack() {
    let mut l = SinglyLinkedList::new();

    assert_eq!(l.size(), 0);
    l.push('a');
    l.push('b');
    l.push('c');
    assert_eq!(l.size(), 3);

    assert_eq!(l.pop(), Some('c'));
    assert_eq!(l.pop(), Some('b'));
    assert_eq!(l.pop(), Some('a'));
    assert!(l.pop().is_none());
}

#[cfg(test)]
#[test]
fn singly_linked_list_queue() {
    let mut l = SinglyLinkedList::new();

    assert_eq!(l.size(), 0);
    l.add('a');
    l.add('b');
    l.add('c');
    assert_eq!(l.size(), 3);

    assert_eq!(l.remove(), Some('a'));
    assert_eq!(l.remove(), Some('b'));
    assert_eq!(l.remove(), Some('c'));
    assert!(l.remove().is_none());
}