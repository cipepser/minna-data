use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Bag<T: Clone + PartialEq + Eq + Hash> {
    inner: HashSet<(T, u32)>,
}

impl<T: Clone + PartialEq + Eq + Hash> Bag<T> {
    fn new() -> Bag<T> {
        Bag {
            inner: HashSet::new(),
        }
    }

    fn add(&mut self, x: T) -> bool {
        let mut i = 0;
        loop {
            if self.inner.get(&(x.clone(), i)).is_none() {
                self.inner.insert((x.clone(), i));
                break;
            }
            i += 1;
        }
        true
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        let mut i = 0;
        loop {
            if self.inner.get(&(x.clone(), i)).is_none() {
                if i == 0 {
                    return None;
                } else {
                    self.inner.remove(&(x.clone(), i - 1));
                    break;
                }
            }
            i += 1;
        }
        Some(x.clone())
    }

    fn find(&self, x: &T) -> Option<T> {
        match self.inner.get(&(x.clone(), 0)) {
            None => None,
            Some(tuple) => Some(tuple.clone().0),
        }
    }

    fn find_all(&self, x: &T) -> Option<Vec<T>> {
        let mut v = vec![];
        let mut i = 0;
        loop {
            if self.inner.get(&(x.clone(), i)).is_none() {
                if i == 0 {
                    return None;
                } else {
                    break;
                }
            }
            v.push(x.clone());
            i += 1;
        }
        Some(v)
    }

    fn size(&self) -> usize {
        self.inner.len()
    }
}

#[test]
fn bag() {
    let mut bag = Bag::new();

    assert!(bag.add(1));
    assert_eq!(bag.size(), 1);
    assert!(bag.add(1));
    assert_eq!(bag.size(), 2);
    assert!(bag.add(2));
    assert_eq!(bag.size(), 3);

    assert_eq!(bag.find(&1), Some(1));
    assert_eq!(bag.find(&2), Some(2));
    assert!(bag.find(&3).is_none());

    assert_eq!(bag.find_all(&1), Some(vec![1, 1]));
    assert_eq!(bag.find_all(&2), Some(vec![2]));
    assert!(bag.find_all(&3).is_none());

    assert!(bag.remove(&3).is_none());
    assert!(bag.remove(&1).is_some());
    assert_eq!(bag.size(), 2);
    assert!(bag.remove(&2).is_some());
    assert_eq!(bag.size(), 1);
}