#![allow(unused)]

use common::traits::USet;

#[derive(Debug, Clone)]
pub struct MyUSet<T: Clone + PartialEq + Eq> {
    inner: Vec<T>,
}

impl<T: Clone + PartialEq + Eq> MyUSet<T> {
    fn new() -> MyUSet<T> {
        MyUSet { inner: Vec::new() }
    }
}

impl<T: Clone + PartialEq + Eq> USet<T> for MyUSet<T> {
    fn size(&self) -> usize {
        self.inner.len()
    }

    fn add(&mut self, x: T) -> bool {
        if self.find(&x.clone()).is_some() {
            return false;
        }
        self.inner.insert(self.size(), x);
        true
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        let mut index = self.size();
        for (i, y) in self.inner.iter().enumerate() {
            if y == x {
                index = i;
                break;
            }
        }

        if index == self.size() {
            return None;
        }
        self.inner.remove(index);
        Some(x.clone())
    }

    fn find(&self, x: &T) -> Option<T> {
        for y in self.inner.iter() {
            if y == x {
                return Some(y.clone());
            }
        }
        None
    }
}

#[test]
fn uset() {
    let mut u = MyUSet::new();
    assert_eq!(u.size(), 0);

    assert!(u.add(0));
    assert_eq!(u.size(), 1);
    // [0]
    assert!(!u.add(0));
    assert_eq!(u.size(), 1);

    assert!(u.add(1));
    assert!(u.add(2));
    assert!(u.add(3));
    // [0, 1, 2, 3]
    assert_eq!(u.size(), 4);
    assert_eq!(u.find(&1), Some(1));
    assert!(u.find(&5).is_none());

    assert!(u.remove(&5).is_none());
    assert_eq!(u.remove(&1), Some(1));
    // [0, 2, 3]
    assert_eq!(u.size(), 3);
}
