use super::handles::Handle;

use std::marker::PhantomData;

use im::Vector;
use serde::Serialize;
use std::iter::Iterator;
use std::vec::Vec;

use serde::ser::{SerializeSeq, Serializer};

#[derive(Serialize, Debug, Clone)]
pub enum DataFieldData<T> {
    Taken(T),
    Empty
}

#[derive(Serialize, Debug, Clone)]
pub struct DataField<T> {
    #[serde(flatten)]
    data: Option<T>,
    generation: usize,
    occupied: bool,
}

impl<T> DataField<T> {
    pub fn new(data: Option<T>, generation: usize, occupied: bool) -> Self {
        DataField {
            data,
            generation,
            occupied,
        }
    }

    pub fn set_data(&mut self, data: Option<T>) {
        self.data = data;
    }

    pub fn set_generation(&mut self, generation: usize) {
        self.generation = generation;
    }

    pub fn set_occupied(&mut self, occupied: bool) {
        self.occupied = occupied;
    }

    pub fn get_data(&self) -> &Option<T> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Option<T> {
        &mut self.data
    }

    pub fn get_generation(&self) -> usize {
        self.generation
    }

    pub fn get_occupied(&self) -> bool {
        self.occupied
    }
}

pub trait DataVector {
    type EntryDataType: Serialize;

    fn get(&self, index: usize) -> Option<&DataField<Self::EntryDataType>>;
    fn get_mut(&mut self, index: usize) -> Option<&mut DataField<Self::EntryDataType>>;
    fn len(&self) -> usize;
    fn push(&mut self, data: DataField<Self::EntryDataType>);
    fn set_data_field(&mut self, index: usize, data_field: DataField<Self::EntryDataType>);
    fn new() -> Self;
}

pub struct DataFieldIterator<'a, D: DataVector> {
    current_index: usize,
    data: &'a D,
}

impl<'a, D: DataVector> DataFieldIterator<'a, D> {
    pub fn new(data: &'a D) -> Self {
        DataFieldIterator {
            current_index: 0,
            data,
        }
    }
}

impl<'a, D: DataVector> Iterator for DataFieldIterator<'a, D> {
    type Item = &'a Option<D::EntryDataType>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.data.get(self.current_index) {
            self.current_index += 1;
            return Some(&entry.data);
        }
        None
    }
}

pub struct DataIterator<'a, D: DataVector> {
    current_index: usize,
    data: &'a D,
}

impl<'a, D: DataVector> DataIterator<'a, D> {
    pub fn new(data: &'a D) -> Self {
        DataIterator {
            current_index: 0,
            data,
        }
    }

    fn find_next(&mut self) -> Option<&'a D::EntryDataType> {
        while self.current_index < self.data.len() {
            if let Some(data_field) = self.data.get(self.current_index) {
                if let Some(data) = &data_field.data {
                    self.current_index += 1;
                    return Some(data);
                }
            }

            self.current_index += 1;
        }

        None
    }
}

impl<'a, D: DataVector> Iterator for DataIterator<'a, D> {
    type Item = &'a D::EntryDataType;

    fn next(&mut self) -> Option<Self::Item> {
        self.find_next()
    }
}

pub struct HandleIterator<'a, H: Handle, D: DataVector> {
    current_index: usize,
    data: &'a D,
    _marker: PhantomData<H>,
}

impl<'a, H: Handle, D: DataVector> HandleIterator<'a, H, D> {
    pub fn new(data: &'a D) -> Self {
        HandleIterator {
            current_index: 0,
            data,
            _marker: PhantomData,
        }
    }

    fn find_next(&mut self) -> Option<H> {
        while self.current_index < self.data.len() {
            if let Some(data_field) = self.data.get(self.current_index) {
                if let Some(_data) = &data_field.data {
                    self.current_index += 1;
                    return Some(Handle::new(self.current_index-1, data_field.generation));
                }
            }

            self.current_index += 1;
        }

        None
    }
}

impl<'a, H: Handle, D: DataVector> Iterator for HandleIterator<'a, H, D> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        self.find_next()
    }
}

#[derive(Clone)]
pub struct ImmutableVector<T: Clone> {
    data: Vector<DataField<T>>,
}

impl<T: Serialize+Clone> Serialize for ImmutableVector<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.data.len()))?;
        for field in &self.data {
            s.serialize_element(&field)?;
        }
        s.end()
    }
}

impl<T: Serialize + Clone> DataVector for ImmutableVector<T> {
    type EntryDataType = T;

    fn new() -> Self {
        ImmutableVector {
            data: Vector::new(),
        }
    }

    fn get(&self, index: usize) -> Option<&DataField<T>> {
        self.data.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut DataField<T>> {
        self.data.get_mut(index)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, data: DataField<T>) {
        self.data.push_back(data)
    }

    fn set_data_field(&mut self, index: usize, data_field: DataField<T>) {
        self.data.set(index, data_field);
    }
}


pub struct StdVec<T> {
    data: Vec<DataField<T>>,
}

impl<T: Serialize> Serialize for StdVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.data.len()))?;
        for field in &self.data {
            s.serialize_element(&field)?;
        }
        s.end()
    }
}

impl<T: Serialize> DataVector for StdVec<T> {
    type EntryDataType = T;

    fn new() -> Self {
        StdVec { data: Vec::new() }
    }

    fn get(&self, index: usize) -> Option<&DataField<T>> {
        self.data.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut DataField<T>> {
        self.data.get_mut(index)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, data: DataField<T>) {
        self.data.push(data)
    }

    fn set_data_field(&mut self, index: usize, data_field: DataField<T>) {
        *self.get_mut(index).unwrap() = data_field;
    }
}
