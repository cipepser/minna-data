#![allow(unused)]

use common::traits::SSet;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct MySSet<T: Clone + PartialEq + Eq> {
    inner: Vec<T>,
}

impl<T: Clone + PartialEq + Eq> MySSet<T> {
    fn new() -> MySSet<T> {
        MySSet { inner: Vec::new() }
    }
}

impl<T: Clone + PartialEq + Eq + PartialOrd> SSet<T> for MySSet<T> {
    fn size(&self) -> usize {
        self.inner.len()
    }

    fn add(&mut self, x: T) -> bool {
        for y in self.inner.iter() {
            if *y == x {
                return false;
            }
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
        let mut v = vec![];
        for y in self.inner.iter() {
            if y >= x {
                match v.pop() {
                    None => v.push(y),
                    Some(z) => {
                        if y < z {
                            v.push(y)
                        } else {
                            v.push(z)
                        }
                    }
                }
            }
        }

        if let Some(z) = v.pop() {
            return Some(z.clone());
        } else {
            None
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

#[test]
fn sset() {
    let mut s = MySSet::new();
    assert_eq!(s.size(), 0);
    assert!(s.add(0));
    assert_eq!(s.size(), 1);

    assert_eq!(s.compare(&1, &2), Ordering::Less);
    assert_eq!(s.compare(&2, &2), Ordering::Equal);
    assert_eq!(s.compare(&3, &2), Ordering::Greater);

    assert!(s.add(2));
    assert!(s.add(4)); // [0, 2, 4]
    assert_eq!(s.find(&2), Some(2));
    assert_eq!(s.find(&1), Some(2));
    assert!(s.find(&5).is_none());
}