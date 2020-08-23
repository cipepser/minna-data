use common::traits::List;
use array_based_lists::array_deque::ArrayDeque;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

type Link<T> = Rc<RefCell<Node<T>>>;
type Wink<T> = Weak<RefCell<Node<T>>>;

#[derive(Clone, Default, Debug)]
pub struct Location<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    u: Link<T>,
    j: usize,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Location<T> {
    pub fn new(u: Link<T>, j: usize) -> Self {
        Location { u, j }
    }
}

#[derive(Clone, Default, Debug)]
pub struct BDeque<T: Clone + PartialEq + Eq + Default + std::fmt::Debug>(ArrayDeque<T>);

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> BDeque<T> {
    pub fn with_capacity(len: usize) -> Self {
        BDeque(ArrayDeque::with_capacity(len + 1))
    }
}


#[derive(Clone, Default)]
pub struct Node<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    d: BDeque<T>,
    next: Option<Link<T>>,
    prev: Option<Wink<T>>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{ ")?;

        write!(f, "d: {:?}", self.d)?;
        match self.next.as_ref() {
            None => write!(f, ", next: None")?,
            Some(link) => {
                write!(f, ", next: {{ Some(x: {:?})", link.as_ref().borrow().d)?;
                match link.as_ref().borrow().next {
                    None => write!(f, ", next: None }}")?,
                    Some(_) => write!(f, ", next: Some }}")?,
                };
            }
        }
        match self.prev.as_ref() {
            None => write!(f, ", prev: None")?,
            Some(link) => {
                write!(f, ", prev: {{ Some(x: {:?})", link.upgrade().as_ref().unwrap().borrow().d)?;
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
    pub fn new(b: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            d: BDeque::with_capacity(b),
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpaceEfficientList<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    n: usize,
    dummy: Link<T>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for SpaceEfficientList<T> {
    fn size(&self) -> usize { self.n }

    fn get(&self, i: usize) -> Option<T> {
        unimplemented!();
        // match self.get_node(i) {
        //     None => None,
        //     Some(l) => Some(l.borrow().x.clone()),
        // }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        unimplemented!();

        // let u = self.get_node(i);
        // let y = match u.clone() {
        //     None => return None,
        //     Some(link) => link.borrow().x.clone(),
        // };
        // u.unwrap().borrow_mut().x = x;
        // Some(y)
    }

    fn add(&mut self, i: usize, x: T) {
        unimplemented!();
        // self.add_before(self.get_node(i).unwrap(), x);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        unimplemented!();

        // match self.get_node(i) {
        //     None => None,
        //     Some(w) => {
        //         let x = w.borrow().x.clone();
        //         self.remove_node(w);
        //         Some(x)
        //     }
        // }
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> SpaceEfficientList<T> {
    pub fn new() -> Self {
        unimplemented!();
        // let dummy: Link<T> = Default::default();
        // dummy.borrow_mut().next = Some(dummy.clone());
        // dummy.borrow_mut().prev = Some(Rc::downgrade(&dummy));
        //
        // SpaceEfficientList {
        //     n: 0,
        //     dummy,
        // }
    }

    pub fn get_node(&self, i: usize) -> Option<Link<T>> {
        unimplemented!();
        // if i < self.size() / 2 {
        //     let mut p = self.dummy.borrow().next.clone();
        //     for _ in 0..i {
        //         p = p.take().unwrap().borrow_mut()
        //             .next.as_ref()
        //             .cloned();
        //     }
        //     p
        // } else {
        //     let mut p = Some(self.dummy.clone());
        //     for _ in (i + 1..self.size() + 1).rev() {
        //         p = p.take().unwrap().borrow_mut()
        //             .prev.as_ref()
        //             .map(|link| link.upgrade().unwrap());
        //     }
        //     p
        // }
    }

    pub fn add_before(&mut self, w: Link<T>, x: T) {
        unimplemented!();
        // let u = Node::new(x);
        //
        // u.borrow_mut().prev = w.borrow().prev.clone();
        // u.borrow_mut().next = Some(w);
        // u.borrow().next.as_ref().unwrap()
        //     .borrow_mut().prev = Some(Rc::downgrade(&u));
        // u.borrow().prev.as_ref().unwrap()
        //     .upgrade().unwrap().borrow_mut().next = Some(u.clone());
        //
        // self.n += 1;
    }

    pub fn remove_node(&mut self, w: Link<T>) {
        unimplemented!();
        // w.borrow().prev.as_ref().unwrap()
        //     .upgrade().unwrap().borrow_mut().next = w.borrow().next.clone();
        // w.borrow().next.as_ref().unwrap()
        //     .borrow_mut().prev = w.borrow().prev.clone();
        // self.n -= 1;
    }

    pub fn get_location(&self, mut i: usize) -> Location<T> {
        if i < self.size()/2 {
            let mut u = self.dummy.borrow().next.clone();

            while i >= u.take().unwrap().borrow_mut().d.0.size() {
                i -= u.take().unwrap().borrow().d.0.size();
                u = u.take().unwrap().borrow().next.clone();
            }
        } else {

        }


        // Location::new()
        unimplemented!();
    }
}


#[cfg(test)]
#[test]
fn space_efficient_list() {}