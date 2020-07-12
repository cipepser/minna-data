use std::cmp::Ordering;

pub trait Queue<T: Clone> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> Option<T>;
}

pub trait Stack<T: Clone> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> Option<T>;
}

pub trait Deque<T: Clone> {
    fn add_first(&mut self, x: T);
    fn add_last(&mut self, x: T);
    fn remove_first(&mut self) -> Option<T>;
    fn remove_last(&mut self) -> Option<T>;
}

pub trait List<T: Clone> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> Option<T>;
    fn set(&mut self, i: usize, x: T) -> Option<T>;
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize) -> Option<T>;
}

pub trait USet<T: PartialEq + Clone> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
}

pub trait SSet<T: Eq + PartialEq + Clone + PartialOrd> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> Option<T>;
    fn find(&self, x: &T) -> Option<T>;
    fn compare(&self, x: &T, y: &T) -> Ordering;
}