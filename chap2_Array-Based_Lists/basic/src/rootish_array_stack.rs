use common::traits::List;
use crate::array_stack::ArrayStack;

#[derive(Debug, Clone, Default)]
pub struct RootishArrayStack<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    blocks: ArrayStack<ArrayStack<T>>,
    n: usize,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for RootishArrayStack<T> {
    fn size(&self) -> usize { self.n }

    fn get(&self, i: usize) -> Option<T> {
        let b = Self::i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get(b).unwrap().get(j)
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let b = Self::i2b(i);
        let j = i - b * (b + 1) / 2;

        let mut block = self.blocks.get(b).unwrap().clone();
        let y = block.get(j).clone();
        if block.set(j, x.clone()).is_none() {
            block.add(j, x.clone());
        };
        self.blocks.set(b, block.clone());
        y
    }

    fn add(&mut self, i: usize, x: T) {
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.size() + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i + 1..self.size()).rev() {
            self.set(j, self.get(j - 1).unwrap().clone());
        }
        self.set(i, x.clone());
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.get(i).unwrap().clone();
        for j in i..self.size() - 1 {
            self.set(j, self.get(j + 1).unwrap().clone());
        }
        self.n -= 1;

        let r = self.blocks.size();
        if (r - 2) * (r - 1) / 2 >= self.size() {
            self.shrink();
        }
        Some(x)
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> RootishArrayStack<T> {
    pub fn new() -> Self {
        let inner: ArrayStack<T> = ArrayStack::with_capacity(1);
        let mut buf = ArrayStack::with_capacity(1);
        buf.add(0, inner);
        RootishArrayStack {
            blocks: buf,
            n: 0,
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        let mut ras = Self::new();
        let mut n = 1;
        while n < len {
            ras.grow();
            n += ras.blocks.size();
        }
        ras
    }

    fn i2b(i: usize) -> usize {
        let db = (-3.0 + (9.0 + 8 as f64 * i as f64).sqrt()) / 2.0;
        db.ceil() as usize
    }

    pub fn grow(&mut self) {
        let new_block = ArrayStack::with_capacity(self.blocks.size() + 1);
        self.blocks.add(self.blocks.size(), new_block);
    }

    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        while r > 0 && (r - 2) * (r - 1) / 2 >= self.size() {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

#[cfg(test)]
#[test]
fn rootish_array_stack() {
    // same situation as Fig2.1 in an ods-textbook.
    let mut a: RootishArrayStack<char> = RootishArrayStack::with_capacity(15);
    assert_eq!(a.size(), 0);
    a.add(0, 'a');
    a.add(1, 'b');
    a.add(2, 'c');
    a.add(3, 'd');
    a.add(4, 'e');
    a.add(5, 'f');
    a.add(6, 'g');
    a.add(7, 'h');

    assert_eq!(a.size(), 8);
    assert_eq!(a.blocks.size(), 5);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('f'));
    assert_eq!(a.get(6), Some('g'));
    assert_eq!(a.get(7), Some('h'));

    a.add(2, 'x');
    assert_eq!(a.size(), 9);
    assert_eq!(a.blocks.size(), 5);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('x'));
    assert_eq!(a.get(3), Some('c'));
    assert_eq!(a.get(4), Some('d'));
    assert_eq!(a.get(5), Some('e'));
    assert_eq!(a.get(6), Some('f'));
    assert_eq!(a.get(7), Some('g'));
    assert_eq!(a.get(8), Some('h'));

    assert_eq!(a.remove(1), Some('b'));
    assert_eq!(a.size(), 8);
    assert_eq!(a.blocks.size(), 5);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('x'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('f'));
    assert_eq!(a.get(6), Some('g'));
    assert_eq!(a.get(7), Some('h'));

    assert_eq!(a.remove(7), Some('h'));
    assert_eq!(a.size(), 7);
    assert_eq!(a.blocks.size(), 5);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('x'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('f'));
    assert_eq!(a.get(6), Some('g'));

    assert_eq!(a.remove(6), Some('g'));
    assert_eq!(a.size(), 6);
    assert_eq!(a.blocks.size(), 4);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('x'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('f'));
}
