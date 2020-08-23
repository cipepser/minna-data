use common::traits::SSet;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use rand::Rng;

const MAX_HEIGHT: usize = 32;

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone, Default, Debug)]
pub struct Node<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> {
    x: T,
    next: Vec<Option<Link<T>>>,
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> Node<T> {
    pub fn new(x: T, height: usize) -> Self {
        Node {
            x,
            next: vec![None; height + 1],
        }
    }

    pub fn height(&self) -> usize {
        self.next.len() - 1
    }
}

#[derive(Debug, Clone, Default)]
pub struct SkipListSSet<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> {
    n: usize,
    h: usize,
    sentinel: Link<T>,
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> SSet<T> for SkipListSSet<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn add(&mut self, x: T) -> bool {
        let mut u = self.sentinel.clone();
        let mut r = self.h;
        let mut stack = vec![Default::default(); MAX_HEIGHT];

        loop {
            while u.borrow().next.get(r).unwrap().clone().is_some()
                && u.borrow().next.get(r).unwrap().clone().unwrap().borrow().x < x {
                let next = u.borrow().next.get(r).unwrap().clone().unwrap();
                u = next;
            }
            if u.borrow().next.get(r).unwrap().clone().is_some() &&
                u.borrow().next.get(r).unwrap().clone().unwrap().borrow().x == x {
                return false;
            }
            if r == 0 {
                break;
            }
            stack.insert(r, u.clone());
            r -= 1;
        };

        let mut w = Rc::new(RefCell::new(
            Node::new(x, pick_height())
        ));
        while self.h < w.borrow().height() {
            self.h += 1;
            stack.insert(self.h, self.sentinel.clone());
        }

        let h = w.borrow().height() + 1;
        for i in 0..h {
            println!("inner: {:?}", stack.get(i).unwrap());
            w.borrow_mut().next.insert(
                i,
                stack.get(i).unwrap().borrow()
                    .next.get(i).unwrap() // TODO: ここでNoneの場合の処理
                    .clone(),
            );
            stack.get(i).unwrap().borrow_mut().next.insert(
                i,
                Some(w.clone()),
            );
        }
        self.n += 1;
        true
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        unimplemented!()
    }

    fn find(&self, x: &T) -> Option<T> {
        match self.find_pred_link(&x) {
            None => None,
            Some(link) => {
                Some(link.borrow().x.clone())
            }
        }
    }

    fn compare(&self, x: &T, y: &T) -> Ordering {
        if x > y {
            Ordering::Greater
        } else if x < y {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> SkipListSSet<T> {
    pub fn new() -> Self {
        let sentinel = Rc::new(RefCell::new(
            Node::new(Default::default(), MAX_HEIGHT)
        ));

        SkipListSSet {
            n: 0,
            h: 0,
            sentinel,
        }
    }

    pub fn find_pred_link(&self, x: &T) -> Option<Link<T>> {
        let mut u = self.sentinel.clone();
        let mut r = self.h;

        loop {
            while u.borrow().next.get(r).unwrap().clone().is_some()
                && u.borrow().next.get(r).unwrap().clone().unwrap().borrow().x < *x {
                let next = u.borrow().next.get(r).unwrap().clone().unwrap();
                u = next;
            };
            if r == 0 {
                break;
            }
            r -= 1;
        };
        Some(u)
    }
}

pub fn pick_height() -> usize {
    let mut rng = rand::thread_rng();

    let mut k = 0;
    let mut m = 1;

    let z: usize = rng.gen();
    while (z & m) != 0 {
        k += 1;
        m <<= 1;
    }
    if k > MAX_HEIGHT {
        pick_height()
    } else {
        k
    }
}

#[cfg(test)]
#[test]
fn skip_list_sset() {
    let mut s: SkipListSSet<usize> = SkipListSSet::new();

    println!("test: {:?}", s.find_pred_link(&10));
    println!("s: {:?}", s);
    assert_eq!(s.size(), 0);
    assert!(s.add(10));
    assert_eq!(s.size(), 1);
    // println!("s: {:?}", s);
    assert!(s.add(20));
    // println!("s: {:?}", s);
    assert_eq!(s.find(&15), Some(20));
}

#[cfg(test)]
#[test]
fn sample() {
    // let none: Option<Option<usize>> = None;
    //
    // if none.is_some() && none.unwrap().is_none() {
    //     println!("hello");
    // } else {
    //     println!("bye");
    // }
    //

    // let h = pick_height();
    // println!("{:?}", h);
    // let x = Rc::new(1);
    // assert_eq!(Rc::strong_count(&x), 1);
    //
    // let y = x.clone();
    // assert_eq!(Rc::strong_count(&x), 2);
    // assert_eq!(Rc::strong_count(&y), 2);
    //
    // assert!(Rc::ptr_eq(&x, &y));
    // eprintln!("x = {0:p} (points to {0:})", x);
    // eprintln!("y = {0:p} (points to {0:})", y);
    //
    // std::mem::drop(y);
    // assert_eq!(Rc::strong_count(&x), 1);
    // let z = x.as_ref();
    // assert_eq!(Rc::strong_count(&x), 1);
}