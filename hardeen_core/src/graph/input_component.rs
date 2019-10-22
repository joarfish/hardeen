//! # InputComponents
//!
//! Depending on the type of processor a node needs different logics for its input. Some nodes don't
//! need any input at all, some need a fixed number and others can accept an arbitrary number of incoming
//! edges.

use std::collections::HashSet;
use std::vec::Vec;

use crate::Handle;
use itertools::Itertools;
use serde::Serialize;
use std::hash::Hash;

use crate::hardeen_error::HardeenError;

#[derive(Debug, Serialize)]
pub struct SlottedInput<H: Handle + Clone + PartialEq> {
    input_handles: Vec<Option<H>>,
    slots_taken: usize,
}

impl<H: Handle + Clone + PartialEq> SlottedInput<H> {
    pub fn new(number_of_slots: u8) -> Self {
        SlottedInput {
            input_handles: vec![None; number_of_slots as usize],
            slots_taken: 0,
        }
    }

    pub fn connect_input(&mut self, handle: &H, slot_number: usize) -> Result<(), HardeenError> {
        if slot_number as usize > self.input_handles.len() {
            return Err(HardeenError::InvalidInputSlotNumber);
        }

        self.input_handles[slot_number as usize] = Some(handle.clone());
        self.slots_taken += 1;

        Ok(())
    }

    pub fn disconnect_input(&mut self, slot_number: usize) -> Result<(), HardeenError> {
        if slot_number as usize > self.input_handles.len() {
            return Err(HardeenError::InvalidInputSlotNumber);
        }

        self.input_handles[slot_number as usize] = None;
        self.slots_taken -= 1;

        Ok(())
    }

    pub fn disconnect_handle(&mut self, handle: &H) -> Result<(), HardeenError> {
        for input_handle_slot in self.input_handles.iter_mut() {
            if let Some(input_handle) = input_handle_slot {
                if input_handle == handle {
                    *input_handle_slot = None;
                }
            }
        }

        Ok(())
    }

    pub fn is_slot_occupied(&self, slot_number: usize) -> Result<bool, HardeenError> {
        if slot_number as usize > self.input_handles.len() {
            return Err(HardeenError::InvalidInputSlotNumber);
        }

        Ok(self.input_handles[slot_number as usize] != None)
    }

    pub fn is_input_satisfied(&self) -> bool {
        self.slots_taken as usize == self.input_handles.len()
    }

    pub fn get_input(&self, slot_number: usize) -> Result<Option<H>, HardeenError> {
        if slot_number as usize > self.input_handles.len() {
            return Err(HardeenError::InvalidInputSlotNumber);
        }

        Ok(self.input_handles[slot_number as usize].clone())
    }

    pub fn get_all_input_handles(&self) -> Vec<H> {
        self.input_handles
            .iter()
            .filter(|slot| **slot != None)
            .map(|slot| slot.clone().unwrap())
            .collect()
    }
}

#[derive(Debug, Serialize)]
pub struct MultipleInput<H: Handle + Hash + Eq> {
    input_handles: HashSet<H>,
    zero_inputs_allowed: bool,
}

impl<H: Handle + Hash + Eq + Clone> MultipleInput<H> {
    pub fn new(zero_inputs_allowed: bool) -> Self {
        MultipleInput {
            input_handles: HashSet::new(),
            zero_inputs_allowed,
        }
    }

    pub fn connect_input(&mut self, handle: &H) -> Result<(), HardeenError> {
        self.input_handles.insert((*handle).clone());

        Ok(())
    }

    pub fn is_input_satisfied(&self) -> bool {
        !self.input_handles.is_empty() || self.zero_inputs_allowed
    }

    pub fn disconnect_input(&mut self, handle: &H) -> Result<(), HardeenError> {
        self.input_handles.remove(handle);

        Ok(())
    }

    pub fn get_all_input_handles(&self) -> Vec<H> {
        self.input_handles.iter().cloned().collect_vec()
    }
}

#[derive(Debug, Serialize)]
pub enum InputComponent<H: Handle + Hash + PartialEq + Eq + Clone> {
    Slotted(SlottedInput<H>),
    Multiple(MultipleInput<H>),
}
