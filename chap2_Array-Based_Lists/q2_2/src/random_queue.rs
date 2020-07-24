use common::traits::Queue;
use std::cmp::max;
use std::iter::repeat;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RandomQueue<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> {
    pub n: usize,
    pub heap: Box<[T]>,
}

impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> Queue<T> for RandomQueue<T> {
    fn add(&mut self, x: T) {
        if self.size() + 1 > self.heap.len() {
            self.resize();
        }

        let current = self.heap.get_mut(self.size()).unwrap();
        *current = x.clone();
        self.n += 1;
    }

    fn remove(&mut self) -> Option<T> {
        if self.size() == 0 {
            return None;
        }
        let mut rng = thread_rng();
        self.swap(rng.gen_range(0, self.size()));

        let x = self.heap.get_mut(self.size() - 1).unwrap();
        let y = x.clone();
        *x = T::default();
        self.n -= 1;
        if self.heap.len() >= 3 * self.size() {
            self.resize();
        }
        Some(y)
    }
}


impl<T: Clone + PartialEq + Eq + Default + std::fmt::Debug> RandomQueue<T> {
    pub fn new() -> Self {
        RandomQueue {
            n: 0,
            heap: Self::allocate(1),
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        RandomQueue {
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

    fn swap(&mut self, i: usize) {
        self.heap.swap(i, self.size() - 1);
    }

    fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
#[test]
fn random_queue() {
    let mut q: RandomQueue<char> = RandomQueue::with_capacity(1);
    assert_eq!(q.size(), 0);
    q.add('a');
    q.add('b');
    assert_eq!(q.size(), 2);
    assert!(q.remove().is_some());
    assert_eq!(q.size(), 1);

    assert!(q.remove().is_some());
    assert!(q.remove().is_none());
}