use common::traits::List;
use std::cmp::max;
use std::iter::repeat;
use crate::array_stack::ArrayStack;

#[derive(Debug, Clone, Default)]
pub struct DualArrayDeque<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for DualArrayDeque<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1).clone()
        } else {
            self.back.get(i - self.front.size()).clone()
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, x)
        } else {
            self.back.set(i - self.front.size(), x)
        }
    }

    fn add(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i, x);
        } else {
            self.back.add(i - self.front.size(), x)
        }
        self.balance();
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = if i < self.front.size() {
            self.front.remove(self.front.size() - i - 1)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        x
    }
}


impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> DualArrayDeque<T> {
    pub fn new() -> Self {
        DualArrayDeque {
            front: ArrayStack::with_capacity(1),
            back: ArrayStack::with_capacity(0),
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        DualArrayDeque {
            front: ArrayStack::with_capacity(len / 2),
            back: ArrayStack::with_capacity(len - len / 2),
        }
    }

    fn allocate(len: usize) -> Box<[T]> {
        repeat(T::default())
            .take(len)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn balance(&mut self) {
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let n = self.front.size() + self.back.size();

            let nf = n / 2;
            let mut af = Self::allocate(max(2 * nf, 1));
            for i in 0..nf {
                let new = af.get_mut(nf - i - 1).unwrap();
                let old = self.get(i).unwrap();
                *new = old.clone();
            }

            let nb = n - nf;
            let mut ab = Self::allocate(max(2 * nb, 1));
            for i in 0..nb {
                let new = ab.get_mut(i).unwrap();
                let old = self.get(nf + i).unwrap();
                *new = old.clone();
            }

            self.front = ArrayStack {
                n: nf,
                heap: af,
            };
            self.back = ArrayStack {
                n: nb,
                heap: ab,
            };
        }
    }
}


#[cfg(test)]
#[test]
fn dual_array_deque() {
    // same situation as Fig2.4 in an ods-textbook.
    let mut a: DualArrayDeque<char> = DualArrayDeque::with_capacity(2);
    assert_eq!(a.size(), 0);
    a.add(0, 'a');
    a.add(1, 'b');
    a.add(2, 'c');
    a.add(3, 'c');
    a.add(4, 'd');
    a.remove(2);
    assert_eq!(a.front.size(), 2);
    assert_eq!(a.back.size(), 2);
    assert_eq!(a.size(), 4);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));

    a.add(3, 'x');
    assert_eq!(a.front.size(), 2);
    assert_eq!(a.back.size(), 3);
    assert_eq!(a.size(), 5);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('x'));
    assert_eq!(a.get(4), Some('d'));

    a.add(4, 'y');
    assert_eq!(a.front.size(), 2);
    assert_eq!(a.back.size(), 4);
    assert_eq!(a.size(), 6);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('x'));
    assert_eq!(a.get(4), Some('y'));
    assert_eq!(a.get(5), Some('d'));

    assert_eq!(a.remove(0), Some('a'));
    assert_eq!(a.front.size(), 2);
    assert_eq!(a.back.size(), 3);
    assert_eq!(a.size(), 5);
    assert_eq!(a.get(0), Some('b'));
    assert_eq!(a.get(1), Some('c'));
    assert_eq!(a.get(2), Some('x'));
    assert_eq!(a.get(3), Some('y'));
    assert_eq!(a.get(4), Some('d'));
}