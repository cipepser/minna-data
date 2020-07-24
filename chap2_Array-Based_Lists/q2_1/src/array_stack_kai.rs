use common::traits::List;
use std::cmp::max;
use std::iter::repeat;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArrayStack<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    pub n: usize,
    pub heap: Box<[T]>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        if i >= self.size() {
            return None;
        }
        Some(self.heap[i].clone())
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i >= self.size() {
            return None;
        }

        let previous = self.heap.get_mut(i).unwrap();
        let ret = previous.clone();
        *previous = x;

        Some(ret)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.size() + 1 > self.heap.len() {
            self.resize();
        }

        for j in (i + 1..self.size() + 1).rev() {
            let previous = self.heap
                .get_mut(j - 1).unwrap().clone();

            let current = self.heap.get_mut(j).unwrap();
            *current = previous;
        }
        let current = self.heap.get_mut(i).unwrap();
        *current = x.clone();
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.size() {
            return None;
        }
        let x = self.heap.get(i).unwrap().clone();
        for j in i..self.size() {
            let next = self.heap
                .get_mut(j + 1)
                .unwrap_or(&mut T::default())
                .clone();

            let current = self.heap.get_mut(j).unwrap();
            *current = next;
        }
        self.n -= 1;

        if self.heap.len() >= 3 * self.size() {
            self.resize();
        }
        Some(x)
    }
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> ArrayStack<T> {
    pub fn new() -> Self {
        ArrayStack {
            n: 0,
            heap: Self::allocate(1),
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        ArrayStack {
            n: 0,
            heap: Self::allocate(len),
        }
    }

    fn allocate(len: usize) -> Box<[T]> {
        repeat(T::default())
            .take(len)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let mut buf = Self::allocate(max(2 * self.size(), 1));
        for i in 0..self.size() {
            let ai = self.heap.get(i).unwrap();
            let bi = buf.get_mut(i).unwrap();
            *bi = ai.clone();
        }
        self.heap = buf;
    }

    pub fn add_all(&mut self, i: usize, c: &[T]) {
        while self.size() + c.len() > self.heap.len() {
            self.resize();
        }

        for j in (i + c.len()..self.size() + c.len()).rev() {
            let previous = self.heap
                .get_mut(j - c.len()).unwrap().clone();

            let current = self.heap.get_mut(j).unwrap();
            *current = previous;
        }

        for j in i..i + c.len() {
            let current = self.heap.get_mut(j).unwrap();
            *current = c.get(j - i).unwrap().clone();
        }
        self.n += c.len();
    }

    pub fn _add(&mut self, i: usize, x: T) {
        self.add(i, x)
    }
}

#[cfg(test)]
#[test]
fn array_stack() {
    // same situation as Fig2.1 in an ods-textbook.
    let mut a: ArrayStack<char> = ArrayStack::with_capacity(6);
    assert_eq!(a.size(), 0);
    let c = ['b', 'r', 'e', 'd'];
    a.add_all(0, &c);

    assert_eq!(a.size(), 4);
    assert_eq!(a.heap.len(), 6);
    assert_eq!(a.get(0), Some('b'));
    assert_eq!(a.get(1), Some('r'));
    assert_eq!(a.get(2), Some('e'));
    assert_eq!(a.get(3), Some('d'));
}
