use common::traits::List;
use basic::array_deque::ArrayDeque;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Treque<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    front: ArrayDeque<T>,
    back: ArrayDeque<T>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for Treque<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.get(i).clone()
        } else {
            self.back.get(i - self.front.size()).clone()
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.front.size() {
            self.front.set(i, x)
        } else {
            self.back.set(i - self.front.size(), x)
        }
    }

    fn add(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.add(i, x);
        } else {
            self.back.add(i - self.front.size(), x)
        }
        self.balance();
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = if i < self.front.size() {
            self.front.remove(i)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        x
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Treque<T> {
    pub fn new() -> Self {
        Treque {
            front: ArrayDeque::with_capacity(1),
            back: ArrayDeque::with_capacity(0),
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        Treque {
            front: ArrayDeque::with_capacity(len / 2),
            back: ArrayDeque::with_capacity(len - len / 2),
        }
    }

    fn balance(&mut self) {
        if self.front.size() >= self.back.size() + 2 {
            let x = self.front.remove(self.front.size() - 1).unwrap().clone();
            self.back.add(0, x);
            self.balance();
        }
        if self.back.size() >= self.front.size() + 2 {
            let x = self.back.remove(0).unwrap().clone();
            self.front.add(self.front.size(), x);
            self.balance();
        }
    }
}

#[cfg(test)]
#[test]
fn treque() {
    let mut a: Treque<char> = Treque::with_capacity(0);
    assert_eq!(a.size(), 0);
    a.add(0, 'a');
    a.add(1, 'b');
    a.add(2, 'c');
    a.add(3, 'd');
    assert_eq!(a.size(), 4);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));

    a.remove(2);
    assert_eq!(a.size(), 3);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('d'));

    a.set(1, 'x');
    assert_eq!(a.size(), 3);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('x'));
    assert_eq!(a.get(2), Some('d'));
}