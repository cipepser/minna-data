use common::traits::List;
use std::cmp::max;
use std::iter::repeat;
use std::ptr;

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
            if self.j == 0 {
                unsafe {
                    let len = self.heap.len();
                    ptr::copy(self.heap[0..].as_mut_ptr(), self.heap[len - 1..].as_mut_ptr(), 1);
                    ptr::copy(self.heap[1..].as_mut_ptr(), self.heap[0..].as_mut_ptr(), i - 1);
                }
                self.j = self.heap.len() - 1;
            } else {
                unsafe {
                    ptr::copy(self.heap[self.j..].as_mut_ptr(), self.heap[self.j - 1..].as_mut_ptr(), i);
                }
                self.j -= 1;
            };
        } else {
            let len = self.heap.len();
            if self.j + self.size() == len {
                unsafe {
                    ptr::copy(self.heap[len - 1..].as_mut_ptr(), self.heap[0..].as_mut_ptr(), 1);
                    ptr::copy(self.heap[self.j + i..].as_mut_ptr(), self.heap[self.j + i + 1..].as_mut_ptr(), self.size() - i - 1);
                }
            } else if self.j + self.size() < self.heap.len() {
                unsafe {
                    ptr::copy(self.heap[self.j + i..].as_mut_ptr(), self.heap[self.j + i + 1..].as_mut_ptr(), self.size() - i);
                }
            } else {
                unsafe {
                    ptr::copy(self.heap[0..].as_mut_ptr(), self.heap[1..].as_mut_ptr(), self.size() + self.j - self.heap.len());
                    ptr::copy(self.heap[len - 1..].as_mut_ptr(), self.heap[0..].as_mut_ptr(), 1);
                    ptr::copy(self.heap[self.j + i..].as_mut_ptr(), self.heap[self.j + i + 1..].as_mut_ptr(), self.size() - i - self.j - 1);
                }
            }
        }
        let current = self.heap.get_mut((self.j + i) % self.heap.len()).unwrap();
        *current = x.clone();
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.heap.get((self.j + i) % self.heap.len()).unwrap().clone();

        if i < self.n / 2 {
            let len = self.heap.len();
            if self.j + i <= len - 1 {
                unsafe {
                    ptr::copy(self.heap[self.j..].as_mut_ptr(), self.heap[self.j + 1..].as_mut_ptr(), i);
                }
            } else {
                unsafe {
                    ptr::copy(self.heap[0..].as_mut_ptr(), self.heap[1..].as_mut_ptr(), (self.j + i) % len);
                    ptr::copy(self.heap[len - 1..].as_mut_ptr(), self.heap[0..].as_mut_ptr(), 1);
                    ptr::copy(self.heap[self.j..].as_mut_ptr(), self.heap[self.j + 1..].as_mut_ptr(), i - 1 - (self.j + i) % len);
                }
            }
            self.j = (self.j + 1) % self.heap.len();
        } else {
            let len = self.heap.len();
            if self.j + self.size() <= len {
                unsafe {
                    ptr::copy(self.heap[self.j + i + 1..].as_mut_ptr(), self.heap[self.j + i..].as_mut_ptr(), self.size() - i - 1);
                }
            } else {
                if self.j + i >= len {
                    unsafe {
                        ptr::copy(self.heap[(self.j + i) % len..].as_mut_ptr(), self.heap[(self.j + i + 1) % len..].as_mut_ptr(), self.size() - i - 1);
                    }
                } else {
                    unsafe {
                        ptr::copy(self.heap[self.j + i + 1..].as_mut_ptr(), self.heap[self.j + i..].as_mut_ptr(), len - self.j - i - 1);
                        ptr::copy(self.heap[0..].as_mut_ptr(), self.heap[1..].as_mut_ptr(), 1);
                        ptr::copy(self.heap[1..].as_mut_ptr(), self.heap[0..].as_mut_ptr(), self.size() - len + self.j - 1);
                    }
                }
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
        unsafe {
            ptr::copy_nonoverlapping(self.heap.as_mut_ptr(), buf.as_mut_ptr(), self.size());
        }
        self.j = 0;
        self.heap = buf;
    }
}

#[cfg(test)]
#[test]
fn array_deque_rotate() {
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
