use serde::Serialize;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub trait Handle: Serialize {
    fn new(index: usize, generation: usize) -> Self;
    fn get_index(&self) -> usize;
    fn get_generation(&self) -> usize;
    fn get(&self) -> (usize, usize);
}

#[derive(Serialize, Debug)]
pub struct MarkedHandle<T> {
    index: usize,
    generation: usize,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Clone for MarkedHandle<T> {
    fn clone(&self) -> Self {
        MarkedHandle {
            index: self.index,
            generation: self.generation,
            _marker: PhantomData,
        }
    }
}

impl<T> Handle for MarkedHandle<T> {
    fn new(index: usize, generation: usize) -> Self {
        MarkedHandle {
            index,
            generation,
            _marker: PhantomData,
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_generation(&self) -> usize {
        self.generation
    }

    fn get(&self) -> (usize, usize) {
        (self.index, self.generation)
    }
}

impl<T> PartialEq for MarkedHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

impl<T> Eq for MarkedHandle<T> {}

impl<T> Hash for MarkedHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.generation.hash(state)
    }
}
