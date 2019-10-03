mod data_vector;
mod handles;

pub use self::data_vector::*;
pub use self::handles::*;

use std::vec::Vec;

use serde::Serialize;
use serde::ser::{SerializeSeq, SerializeMap, Serializer};

#[derive(Debug)]
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
        /*let mut s = serializer.serialize_seq(Some(self.entity_count))?;
        let mut iter = DataFieldIterator::new(&self.data);

        while let Some(element) = iter.next() {
            s.serialize_element(element)?;
        }*/

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
                if data.get_occupied() == true {
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

        return Err(HandledVecError::CannotBorrowAsMutable);
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

    pub fn is_handle_valid(&self, handle: &H) -> Result<bool, HandledVecError> {
        if let Some(data_field) = self.data.get(handle.get_index()) {
            if data_field.get_generation() == handle.get_generation() {
                return Ok(true);
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
