use common::traits::List;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

type Link<T> = Rc<RefCell<Node<T>>>;
type Wink<T> = Weak<RefCell<Node<T>>>;

#[derive(Clone, Default)]
// #[derive(Clone, Default, Debug)]
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
    dummy: Node<T>,
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
        unimplemented!()
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        let mut dummy: Node<T> = Default::default();
        let inner = Rc::new(RefCell::new(dummy.clone()));
        dummy.next = Some(inner.clone());
        dummy.prev = Some(Rc::downgrade(&inner));

        println!("dummy: {:?}", dummy);
        DoublyLinkedList {
            n: 0,
            dummy,
        }
    }

    pub fn get_node(&self, i: usize) -> Option<Link<T>> {
        let mut p: Option<Link<T>> = None;
        if i < self.size() / 2 {
            p = self.dummy.next.clone();
            for _ in 0..i + 1 {
                // println!("aa");
                p = p.take().unwrap().borrow_mut()
                    .next.as_ref()
                    .map(|link| link.clone());
            }
        } else {
            // println!("bbb");
            p = Some(Rc::new(RefCell::new(self.dummy.clone())));
            // println!("p(dummy): {:?}", p);
            for _ in (i + 1 ..self.size() + 1).rev() {
                // println!("ccc");
                p = p.take().unwrap().borrow_mut()
                    .prev.as_ref()
                    .map(|link| link.upgrade().unwrap().clone());
            }
        }
        println!("p: {:?}", p);
        p
    }

    pub fn add_before(&mut self, w: Link<T>, x: T) {
        let u = Node::new(x);
        println!("u(before): {:?}", u);
        println!("w(before): {:?}", w);

        u.borrow_mut().prev = w.borrow().prev.clone();
        println!("u(a): {:?}", u);
        println!("w(a): {:?}", w);
        u.borrow_mut().next = Some(w.clone());
        println!("u(b): {:?}", u);
        println!("w(b): {:?}", w);

        u.borrow_mut().next.as_ref().unwrap()
            .borrow_mut().prev = Some(Rc::downgrade(&u));
        println!("u(c): {:?}", u);
        println!("w(c): {:?}", w);
        u.borrow_mut().prev.as_ref().unwrap()
            .upgrade().unwrap().borrow_mut().next = Some(u.clone());
        println!("u(d): {:?}", u);
        println!("w(d): {:?}", w);

        self.n += 1;
    }
}

#[cfg(test)]
#[test]
fn doubly_linked_list() {
    let mut l: DoublyLinkedList<char> = DoublyLinkedList::new();

    assert_eq!(l.size(), 0);
    println!("l: {:?}", l);
    l.add(0, 'a');
    println!("---");
    println!("l: {:?}", l);
    assert_eq!(l.get(0), Some('a'));

    // l.push('b');
    // l.push('c');
    // assert_eq!(l.size(), 3);
    //
    // assert_eq!(l.pop(), Some('c'));
    // assert_eq!(l.pop(), Some('b'));
    // assert_eq!(l.pop(), Some('a'));
    // assert!(l.pop().is_none());
}
