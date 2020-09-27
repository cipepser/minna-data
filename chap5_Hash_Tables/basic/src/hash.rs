use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;

const W: usize = std::mem::size_of::<usize>() * 8;

pub fn hashcode<T: Hash>(x: &T) -> usize {
    let mut s = DefaultHasher::new();
    x.hash(&mut s);
    s.finish() as usize
}

pub struct MultiplicativeHashing<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug + Hash> {
    phantom: PhantomData<T>,
    z: usize,
    d: usize,
}

impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug + Hash> MultiplicativeHashing<T> {
    pub fn new() -> Self {
        MultiplicativeHashing {
            phantom: PhantomData,
            z: rand::random::<usize>() | 1,
            d: 1,
        }
    }

    pub fn hash(&self, x: &T) -> usize {
        ((self.z as u128 * hashcode(x) as u128) >> (W - self.d) as u128) as usize
    }
}

// pub struct TabulationHashing<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug + Hash> {
//     phantom: PhantomData<T>,
//     z: usize,
//     d: usize,
// }
//
// impl<T: Clone + PartialEq + Eq + PartialOrd + Default + std::fmt::Debug + Hash> TabulationHashing<T> {
//     pub fn hash(&self, x: &T) -> usize {}
// }
//
