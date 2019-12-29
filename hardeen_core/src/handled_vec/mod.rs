//! # Handled Vec
//!
//! Hardeen heavily relies on the use of Handles instead of pointers. This module provides the underlying
//! data structures for this pattern. The basic characteristics of a `HandledVec` are that it allows for
//! economic reuse of memory locations that have been occupied and then freed.
//!
//! A `HandledVec` is build upon a vector, but is decoupled from the latters implementation. It just
//! requires it to be wrapped in a struct that implements the `DataVector`-trait.
//!
//! Hardeen has DataVector-Wrappers for std::vec::Vec as well as im::Vector (Vectors based on RRB trees). The latter
//! is ideal for having a vector that is often copied and mutated as with the result of Hardeens processors.
//!


mod data_vector;
mod handles;

pub use self::data_vector::*;
pub use self::handles::*;

use std::vec::Vec;

use serde::Serialize;
use serde::ser::{SerializeMap, Serializer};

#[derive(Debug, PartialEq)]
pub enum HandledVecError {
    GenerationMismatch,
    IndexDoesNotExist,
    IndexUnoccupied,
    CannotBorrowAsMutable,
}

#[derive(Debug)]
pub struct HandledVec<H: Handle + Serialize + Clone, D: DataVector> {
    data: D,
    free_handles: Vec<H>,
    entity_count: usize,
}

impl<H: Handle + Serialize + Clone, D: DataVector> Serialize for HandledVec<H, D> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s_map = serializer.serialize_map(Some(self.entity_count))?;
        let iter = DataFieldIterator::new(&self.data);

        for (idx, element) in iter.enumerate() {
            if let Some(data) = element {
                s_map.serialize_entry(&idx, data)?;
            }
        }

        s_map.end()
    }
}

impl<H: Handle + Serialize + Clone, D: DataVector> HandledVec<H, D> {
    pub fn new() -> Self {
        HandledVec {
            data: D::new(),
            free_handles: Vec::new(),
            entity_count: 0,
        }
    }

    pub fn add_entry(&mut self, entry: D::EntryDataType) -> H {
        let handle = self.get_free_handle();
        let new_datum = DataField::new(Some(entry), handle.get_generation(), true);

        if self.data.len() == handle.get_index() {
            self.data.push(new_datum);
        } else {
            self.data.set_data_field(handle.get_index(), new_datum);
        }

        self.entity_count += 1;

        handle
    }

    pub fn remove_entry(&mut self, handle: H) -> Result<(), HandledVecError> {
        if self.data.len() < handle.get_index() {
            return Err(HandledVecError::IndexDoesNotExist);
        }

        if let Some(data_field) = self.data.get_mut(handle.get_index()) {
            if data_field.get_generation() != handle.get_generation() {
                return Err(HandledVecError::GenerationMismatch);
            }

            self.data.set_data_field(
                handle.get_index(),
                DataField::new(None, handle.get_generation(), false),
            );

            self.free_handles.push(handle);

            self.entity_count -= 1;
        }

        Ok(())
    }

    pub fn get(&self, handle: &H) -> Result<&D::EntryDataType, HandledVecError> {

        if let Some(data) = self.data.get(handle.get_index()) {
            if data.get_generation() == handle.get_generation() {
                if data.get_occupied() {
                    return Ok(data.get_data().as_ref().unwrap());
                }

                return Err(HandledVecError::IndexUnoccupied);
            }

            return Err(HandledVecError::GenerationMismatch);
        }

        Err(HandledVecError::IndexDoesNotExist)
    }

    pub fn get_mut(&mut self, handle: &H) -> Result<&mut D::EntryDataType, HandledVecError> {
        self.is_handle_valid(handle)?;

        if let Some(data_field) = self.data.get_mut(handle.get_index()) {
            if let Some(data) = data_field.get_data_mut().as_mut() {
                return Ok(data);
            }
        }

        Err(HandledVecError::CannotBorrowAsMutable)
    }

    pub fn update(&mut self, handle: &H, entry: D::EntryDataType) -> Result<(), HandledVecError> {
        self.is_handle_valid(handle)?;

        self.data.set_data_field(
            handle.get_index(),
            DataField::new(Some(entry), handle.get_generation(), true),
        );

        Ok(())
    }

    pub fn get_handle_for_index(&self, index: usize) -> Option<H> {
        if let Some(point_ref) = self.data.get(index) {
            return Some(Handle::new(index, (*point_ref).get_generation()));
        }

        None
    }

    pub fn get_length(&self) -> usize {
        self.data.len()
    }

    pub fn get_entity_count(&self) -> usize {
        self.entity_count
    }

    pub fn get_handle_iterator(&self) -> HandleIterator<'_, H, D> {
        HandleIterator::new(&self.data)
    }

    pub fn get_field_iterator(&self) -> DataFieldIterator<'_, D> {
        DataFieldIterator::new(&self.data)
    }

    pub fn get_iterator(&self) -> DataIterator<'_, D> {
        DataIterator::new(&self.data)
    }

    pub fn is_handle_valid(&self, handle: &H) -> Result<(), HandledVecError> {
        if let Some(data_field) = self.data.get(handle.get_index()) {
            if data_field.get_generation() == handle.get_generation() {
                return Ok(());
            }

            return Err(HandledVecError::GenerationMismatch);
        }

        Err(HandledVecError::IndexDoesNotExist)
    }

    fn get_free_handle(&mut self) -> H {
        if let Some(free_handle) = self.free_handles.pop() {
            return Handle::new(free_handle.get_index(), free_handle.get_generation() + 1);
        }

        Handle::new(self.data.len(), 1)
    }

    pub fn mutate_each<F: FnMut(&mut D::EntryDataType)>(&mut self, mut func: F) {
        let mut current = 0;
        while current < self.data.len() {
            if let Some(data_field) = self.data.get_mut(current) {
                if data_field.get_occupied() {
                    let entity = data_field.get_data_mut().as_mut().expect("");
                    func(entity);
                }
            }

            current += 1;
        }
    }
}

impl<H: Handle+Clone, T: Clone+Serialize> Clone for HandledVec<H, ImmutableVector<T>> {
    fn clone(&self) -> Self {
        let mut handled_vec = HandledVec::new();

        handled_vec.data = self.data.clone();
        handled_vec.entity_count = self.entity_count;
        handled_vec.free_handles = self.free_handles.clone();

        handled_vec
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use data_vector::StdVec;
    use handles::MarkedHandle;

    #[derive(Serialize, PartialEq, Debug)]
    struct MockDataField {
        data: u32
    }

    #[test]
    fn add_entry() {
        let mut handled_vec1 : HandledVec<MarkedHandle<MockDataField>, StdVec<MockDataField>> = HandledVec::new();

        handled_vec1.add_entry(MockDataField { data: 1 });
        handled_vec1.add_entry(MockDataField { data: 2 });
        handled_vec1.add_entry(MockDataField { data: 3 });

        assert_eq!(handled_vec1.entity_count, 3);
        assert_eq!(handled_vec1.data.get(1), 
            Some(&DataField::new(Some(MockDataField { data: 2 }), 1, true )));
    }

    #[test]
    fn remove_entry() {
        let mut handled_vec : HandledVec<MarkedHandle<MockDataField>, StdVec<MockDataField>> = HandledVec::new();

        handled_vec.add_entry(MockDataField { data: 1 });
        let h = handled_vec.add_entry(MockDataField { data: 2 });
        handled_vec.add_entry(MockDataField { data: 3 });

        assert_eq!(handled_vec.entity_count, 3);
        assert_eq!(handled_vec.data.get(1), 
            Some(&DataField::new(Some(MockDataField { data: 2 }), 1, true )));

        assert_eq!(handled_vec.remove_entry(h), Ok(()));

        assert_eq!(handled_vec.entity_count, 2);
        assert_eq!(handled_vec.data.get(1), 
            Some(&DataField::new(None, 1, false )));

        assert_eq!(handled_vec.data.get(0), 
            Some(&DataField::new(Some(MockDataField { data: 1 }), 1, true  )));

        assert_eq!(handled_vec.data.get(2), 
            Some(&DataField::new(Some(MockDataField { data: 3 }), 1, true  )));
        
        assert_eq!(handled_vec.free_handles, vec![ MarkedHandle::new(1,1) ]);

        handled_vec.add_entry(MockDataField { data: 4});

        assert_eq!(handled_vec.entity_count, 3);
        assert_eq!(handled_vec.free_handles, vec![ ]);
        assert_eq!(handled_vec.data.get(1), 
            Some(&DataField::new(Some(MockDataField { data: 4 }), 2, true  )));
    }

    #[test]
    #[should_panic]
    fn remove_entry_error() {
        let mut handled_vec : HandledVec<MarkedHandle<MockDataField>, StdVec<MockDataField>> = HandledVec::new();

        handled_vec.remove_entry(MarkedHandle::new(1,1)).unwrap();
    }

    #[test]
    fn get_entry() {
        let mut handled_vec : HandledVec<MarkedHandle<MockDataField>, StdVec<MockDataField>> = HandledVec::new();

        handled_vec.add_entry(MockDataField { data: 1 });
        let h2 = handled_vec.add_entry(MockDataField { data: 2 });
        handled_vec.add_entry(MockDataField { data: 3 });

        let invalid_handle = MarkedHandle::new(2, 2);

        let entry = handled_vec.get(&h2).unwrap();

        assert_eq!(entry, &MockDataField { data: 2} );

        assert_eq!(handled_vec.get(&invalid_handle), Err(HandledVecError::GenerationMismatch));
    }

}