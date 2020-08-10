use common::traits::List;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

type Link<T> = Rc<RefCell<Node<T>>>;
type Wink<T> = Weak<RefCell<Node<T>>>;

#[derive(Clone, Default)]
pub struct Node<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    x: T,
    next: Option<Link<T>>,
    prev: Option<Wink<T>>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{ ")?;

        write!(f, "x: {:?}", self.x)?;
        match self.next.as_ref() {
            None => write!(f, ", next: None")?,
            Some(link) => {
                write!(f, ", next: {{ Some(x: {:?})", link.as_ref().borrow().x)?;
                match link.as_ref().borrow().next {
                    None => write!(f, ", next: None }}")?,
                    Some(_) => write!(f, ", next: Some }}")?,
                };
            }
        }
        match self.prev.as_ref() {
            None => write!(f, ", prev: None")?,
            Some(link) => {
                write!(f, ", prev: {{ Some(x: {:?})", link.upgrade().as_ref().unwrap().borrow().x)?;
                match link.upgrade().as_ref().unwrap().borrow().prev {
                    None => write!(f, ", prev: None }}")?,
                    Some(_) => write!(f, ", prev: Some }}")?,
                };
            }
        }

        write!(f, " }}")
    }
}


impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Node<T> {
    pub fn new(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            x,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DoublyLinkedList<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    n: usize,
    dummy: Link<T>,
}

// impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Drop for DoublyLinkedList<T> {
//     fn drop(&mut self) {
//         // while self.remove(0).is_some() {}
//         unimplemented!()
//     }
// }

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for DoublyLinkedList<T> {
    fn size(&self) -> usize { self.n }

    fn get(&self, i: usize) -> Option<T> {
        match self.get_node(i) {
            None => None,
            Some(l) => Some(l.borrow().x.clone()),
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let u = self.get_node(i);
        let y = match u.clone() {
            None => return None,
            Some(link) => link.borrow().x.clone(),
        };
        u.unwrap().borrow_mut().x = x;
        Some(y)
    }

    fn add(&mut self, i: usize, x: T) {
        self.add_before(self.get_node(i).unwrap(), x);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        match self.get_node(i) {
            None => None,
            Some(w) => {
                let x = w.borrow().x.clone();
                self.remove_node(w);
                Some(x)
            }
        }
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        let dummy: Link<T> = Default::default();
        dummy.borrow_mut().next = Some(dummy.clone());
        dummy.borrow_mut().prev = Some(Rc::downgrade(&dummy));

        DoublyLinkedList {
            n: 0,
            dummy,
        }
    }

    pub fn get_node(&self, i: usize) -> Option<Link<T>> {
        let mut p: Option<Link<T>> = None;
        if i < self.size() / 2 {
            p = self.dummy.borrow().next.clone();
            for _ in 0..i {
                p = p.take().unwrap().borrow_mut()
                    .next.as_ref()
                    .map(|link| link.clone());
            }
        } else {
            p = Some(self.dummy.clone());
            for _ in (i + 1..self.size() + 1).rev() {
                p = p.take().unwrap().borrow_mut()
                    .prev.as_ref()
                    .map(|link| link.upgrade().unwrap().clone());
            }
        }
        p
    }

    pub fn add_before(&mut self, w: Link<T>, x: T) {
        let u = Node::new(x);

        u.borrow_mut().prev = w.borrow().prev.clone();
        u.borrow_mut().next = Some(w.clone());
        u.borrow().next.as_ref().unwrap()
            .borrow_mut().prev = Some(Rc::downgrade(&u));
        u.borrow().prev.as_ref().unwrap()
            .upgrade().unwrap().borrow_mut().next = Some(u.clone());

        self.n += 1;
    }

    pub fn remove_node(&mut self, w: Link<T>) {
        w.borrow().prev.as_ref().unwrap()
            .upgrade().unwrap().borrow_mut().next = w.borrow().next.clone();
        w.borrow().next.as_ref().unwrap()
            .borrow_mut().prev = w.borrow().prev.clone();
        self.n -= 1;
    }
}

#[cfg(test)]
#[test]
fn doubly_linked_list() {
    let mut l: DoublyLinkedList<char> = DoublyLinkedList::new();

    assert_eq!(l.size(), 0);
    l.add(0, 'a');
    assert_eq!(l.size(), 1);
    assert_eq!(l.get(0), Some('a'));
    l.set(0, 'b');
    assert_eq!(l.get(0), Some('b'));

    l.add(0, 'a');
    assert_eq!(l.size(), 2);
    assert_eq!(l.get(0), Some('a'));
    assert_eq!(l.get(1), Some('b'));

    l.add(0, 'x');
    assert_eq!(l.size(), 3);
    assert_eq!(l.get(0), Some('x'));
    assert_eq!(l.get(1), Some('a'));
    assert_eq!(l.get(2), Some('b'));

    assert_eq!(l.remove(0), Some('x'));
    assert_eq!(l.size(), 2);
    assert_eq!(l.get(0), Some('a'));
    assert_eq!(l.get(1), Some('b'));
}
