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
}


#[cfg(test)]
#[test]
fn array_deque() {
    // same situation as Fig2.3 in an ods-textbook.
    let mut a: ArrayDeque<char> = ArrayDeque::with_capacity(12);
    assert_eq!(a.size(), 0);
    a.add(0, 'a');
    a.add(1, 'b');
    a.add(2, 'c');
    a.add(3, 'd');
    a.add(4, 'e');
    a.add(5, 'f');
    a.add(6, 'g');
    a.add(7, 'h');

    assert_eq!(a.j, 0);
    assert_eq!(a.size(), 8);
    assert_eq!(a.heap.len(), 12);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('c'));
    assert_eq!(a.get(3), Some('d'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('f'));
    assert_eq!(a.get(6), Some('g'));
    assert_eq!(a.get(7), Some('h'));

    assert_eq!(a.remove(2), Some('c'));
    assert_eq!(a.j, 1);
    assert_eq!(a.size(), 7);
    assert_eq!(a.heap.len(), 12);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('d'));
    assert_eq!(a.get(3), Some('e'));
    assert_eq!(a.get(4), Some('f'));
    assert_eq!(a.get(5), Some('g'));
    assert_eq!(a.get(6), Some('h'));

    a.add(4, 'x');
    assert_eq!(a.j, 1);
    assert_eq!(a.size(), 8);
    assert_eq!(a.heap.len(), 12);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('d'));
    assert_eq!(a.get(3), Some('e'));
    assert_eq!(a.get(4), Some('x'));
    assert_eq!(a.get(5), Some('f'));
    assert_eq!(a.get(6), Some('g'));
    assert_eq!(a.get(7), Some('h'));

    a.add(3, 'y');
    assert_eq!(a.j, 0);
    assert_eq!(a.size(), 9);
    assert_eq!(a.heap.len(), 12);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('d'));
    assert_eq!(a.get(3), Some('y'));
    assert_eq!(a.get(4), Some('e'));
    assert_eq!(a.get(5), Some('x'));
    assert_eq!(a.get(6), Some('f'));
    assert_eq!(a.get(7), Some('g'));
    assert_eq!(a.get(8), Some('h'));

    a.add(3, 'z');
    assert_eq!(a.j, 11);
    assert_eq!(a.size(), 10);
    assert_eq!(a.heap.len(), 12);
    assert_eq!(a.get(0), Some('a'));
    assert_eq!(a.get(1), Some('b'));
    assert_eq!(a.get(2), Some('d'));
    assert_eq!(a.get(3), Some('z'));
    assert_eq!(a.get(4), Some('y'));
    assert_eq!(a.get(5), Some('e'));
    assert_eq!(a.get(6), Some('x'));
    assert_eq!(a.get(7), Some('f'));
    assert_eq!(a.get(8), Some('g'));
    assert_eq!(a.get(9), Some('h'));
}