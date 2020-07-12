#![allow(unused)]

use common::traits::List;

#[derive(Debug, Clone)]
pub struct MyList<T: Clone + PartialEq + Eq> {
    inner: Vec<T>,
}

impl<T: Clone + PartialEq + Eq> MyList<T> {
    fn new() -> MyList<T> {
        MyList { inner: Vec::new() }
    }
}

impl<T: Clone + PartialEq + Eq> List<T> for MyList<T> {
    fn size(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, i: usize) -> Option<T> {
        self.inner.get(i).cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if let Some(y) = self.inner.get_mut(i) {
            *y = x.clone();
        }
        Some(x)
    }

    fn add(&mut self, i: usize, x: T) {
        self.inner.insert(i, x);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.size() {
            return None;
        }
        let x = self.inner.remove(i);
        Some(x)
    }
}

#[test]
fn list() {
    let mut l = MyList::new();
    assert_eq!(l.size(), 0);

    l.add(0, 0); // [0]
    assert_eq!(l.size(), 1);
    l.add(0, 0); // [0, 0]
    l.add(0, 0); // [0, 0, 0]
    l.add(0, 0); // [0, 0, 0, 0]
    assert_eq!(l.size(), 4);

    assert!(l.remove(5).is_none());
    assert_eq!(l.remove(3), Some(0)); // [0, 0, 0, '0']
    assert_eq!(l.remove(1), Some(0)); // [0, '0', 0]
    assert_eq!(l.remove(1), Some(0)); // [0, '0']
    assert_eq!(l.size(), 1);

    l.add(1, 1);
    l.add(2, 2);
    l.add(3, 3);
    // [0, 1, 2, 3]
    assert_eq!(l.get(0), Some(0));
    assert_eq!(l.get(1), Some(1));
    assert_eq!(l.get(2), Some(2));
    assert_eq!(l.get(3), Some(3));
    assert!(l.get(4).is_none());

    assert!(l.set(1, 10).is_some()); // [0, 10, 2, 3]
    assert_eq!(l.get(1), Some(10));
    assert_eq!(l.size(), 4);
}
