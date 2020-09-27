use std::iter::repeat;

use common::traits::USet;
use array_based_lists::array_stack::ArrayStack;

#[derive(Clone, Default, Debug)]
pub struct ChainedHashTable<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> {
    t: Box<[ArrayStack<T>]>,
    n: usize,
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> ChainedHashTable<T> {
    pub fn new() -> Self {
        Self::with_capacity(2)
    }

    pub fn with_capacity(len: usize) -> Self {
        ChainedHashTable {
            t: Self::allocate(len),
            n: 0,
        }
    }

    fn allocate(len: usize) -> Box<[ArrayStack<T>]> {
        repeat(ArrayStack::new())
            .take(len)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug> USet<T> for ChainedHashTable<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn add(&mut self, x: T) -> bool {
        unimplemented!()
    }

    fn remove(&mut self, x: &T) -> Option<T> {
        unimplemented!()
    }

    fn find(&self, x: &T) -> Option<T> {
        unimplemented!()
    }
}


#[cfg(test)]
#[test]
fn chained_hash_table() {}