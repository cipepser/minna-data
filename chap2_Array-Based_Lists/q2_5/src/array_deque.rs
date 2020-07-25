use common::traits::List;
use std::cmp::max;
use std::iter::repeat;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArrayDeque<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    j: usize,
    n: usize,
    heap: Box<[T]>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize { self.n }

    fn get(&self, i: usize) -> Option<T> {
        Some(self.heap[(self.j + i) % self.heap.len()].clone())
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        let previous = self.heap.get_mut((self.j + i) % self.heap.len()).unwrap();
        let ret = previous.clone();
        *previous = x;

        Some(ret)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.size() + 1 > self.heap.len() {
            self.resize();
        }

        if i < self.n / 2 {
            self.j = if self.j == 0 {
                self.heap.len() - 1
            } else {
                self.j - 1
            };

            for k in 0..i {
                let next = self.heap
                    .get_mut((self.j + k + 1) % self.heap.len()).unwrap().clone();

                let current = self.heap.get_mut((self.j + k) % self.heap.len()).unwrap();
                *current = next;
            }
        } else {
            for k in (i + 1..self.n + 1).rev() {
                let previous = self.heap
                    .get_mut((self.j + k - 1) % self.heap.len()).unwrap().clone();

                let current = self.heap.get_mut((self.j + k) % self.heap.len()).unwrap();
                *current = previous;
            }
        }
        let current = self.heap.get_mut((self.j + i) % self.heap.len()).unwrap();
        *current = x.clone();
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.heap.get((self.j + i) % self.heap.len()).unwrap().clone();

        if i < self.n / 2 {
            for k in (1..i + 1).rev() {
                let previous = self.heap
                    .get_mut((self.j + k - 1) % self.heap.len()).unwrap().clone();

                let current = self.heap.get_mut((self.j + k) % self.heap.len()).unwrap();
                *current = previous;

                let previous = self.heap
                    .get_mut((self.j + k - 1) % self.heap.len()).unwrap();
                *previous = T::default();
            }
            self.j = (self.j + 1) % self.heap.len();
        } else {
            for k in i..self.n - 1 {
                let next = self.heap
                    .get_mut((self.j + k + 1) % self.heap.len()).unwrap().clone();

                let current = self.heap.get_mut((self.j + k) % self.heap.len()).unwrap();
                *current = next;
                let next = self.heap
                    .get_mut((self.j + k + 1) % self.heap.len()).unwrap();
                *next = T::default();
            }
        }
        self.n -= 1;
        if 3 * self.size() < self.heap.len() {
            self.resize();
        }
        Some(x)
    }
}


impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> ArrayDeque<T> {
    pub fn new() -> Self {
        ArrayDeque {
            j: 0,
            n: 0,
            heap: Self::allocate(1),
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        ArrayDeque {
            j: 0,
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
        for k in 0..self.size() {
            let ai = self.heap.get((self.j + k) % self.heap.len()).unwrap();
            let bi = buf.get_mut(k).unwrap();
            *bi = ai.clone();
        }
        self.j = 0;
        self.heap = buf;
    }

    pub fn rotate(&mut self, r: usize) {
        if r < self.n / 2 {
            for k in (self.size() - r + 1..self.size() + 1).rev() {
                let old = self.heap.get((self.j + k - 1) % self.heap.len()).unwrap().clone();

                let new = self.heap.get_mut((self.heap.len() + self.j + self.size() - k - 1) % self.heap.len()).unwrap();
                *new = old;

                let old = self.heap.get_mut((self.j + k - 1) % self.heap.len()).unwrap();
                *old = T::default();
            }
            self.j = (self.heap.len() + self.j - r) % self.heap.len();
        } else {
            for k in 0..self.size() - r {
                let old = self.heap.get((self.j + k) % self.heap.len()).unwrap().clone();

                let new = self.heap.get_mut((self.j + self.size() + k) % self.heap.len()).unwrap();
                *new = old;

                let old = self.heap.get_mut((self.j + k) % self.heap.len()).unwrap();
                *old = T::default();
            }
            self.j = (self.heap.len() + self.j + self.size() - r) % self.heap.len();
        }
    }
}


#[cfg(test)]
#[test]
fn array_deque_rotate() {
    let mut a: ArrayDeque<char> = ArrayDeque::with_capacity(5);
    assert_eq!(a.size(), 0);
    a.add(0, 'b');
    a.add(1, 'r');
    a.add(2, 'e');
    a.add(3, 'd');

    a.rotate(1);
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('r'));
    assert_eq!(a.get(3), Some('e'));
    assert_eq!(a.get(0), Some('d'));

    a.rotate(3);
    assert_eq!(a.get(0), Some('b'));
    assert_eq!(a.get(1), Some('r'));
    assert_eq!(a.get(2), Some('e'));
    assert_eq!(a.get(3), Some('d'));
}
