//! # Nodes
//!
//! A node is composed of a `ProcessorComponent`, a `InputComponent` and a set of output nodes.
//! It has 3 main jobs:
//!     - Relay get/set properties of its ProcessorComponent
//!     - Store information about incoming and outgoing nodes
//!     - Cache the result of its ProcessorComponent
//! Note that it does not directly invoke the RunComponent. This is done by the `Graph` the node is
//! part of.

use serde::Serialize;
use std::collections::{HashSet};
use std::rc::Rc;
use std::vec::Vec;
use std::cell::RefCell;

use super::parameters::*;

use super::input_component::*;
use super::processor_component::*;
use super::SubgraphHandle;

use crate::handled_vec::MarkedHandle;

use crate::hardeen_error::HardeenError;

pub type NodeHandle<T> = MarkedHandle<Node<T>>;

#[derive(Serialize)]
pub struct Node<T: Serialize> {
    processor_component: ProcessorComponent<T>,
    input_component: InputComponent<NodeHandle<T>>,
    output_nodes: HashSet<NodeHandle<T>>,
    #[serde(skip)]
    cached_output: RefCell<Option<Rc<T>>>,
}

impl<T: Serialize> Node<T> {
    pub fn new_basic_processor_node(processor: Box<dyn BasicProcessor<T>>) -> Self {
        let input_component = (*processor).build_input_component();

        Self::new(
            ProcessorComponent::BasicProcessor(processor),
            input_component
        )
    }

    pub fn new_subgraph_processor_node(processor: Box<dyn SubgraphProcessor<T>>, handle: SubgraphHandle<T>) -> Self {
        let input_component = (*processor).build_input_component();

        Self::new(
            ProcessorComponent::SubgraphProcessor(processor, handle),
            input_component
        )
    }

    fn new(
        run_component: ProcessorComponent<T>,
        input_component: InputComponent<NodeHandle<T>>
    ) -> Self {
        Node {
            processor_component: run_component,
            input_component,
            output_nodes: HashSet::new(),
            cached_output: RefCell::new(None),
        }
    }

    pub fn get_processor_component_mut(&mut self) -> &mut ProcessorComponent<T> {
        &mut self.processor_component
    }

    pub fn get_processor_component(&self) -> &ProcessorComponent<T> {
        &self.processor_component
    }

    pub fn get_input_component(&self) -> &InputComponent<NodeHandle<T>> {
        &self.input_component
    }

    pub fn get_processor_name(&self) -> &str {
        match &self.processor_component {
            ProcessorComponent::BasicProcessor(p) => p.get_processor_name(),
            ProcessorComponent::SubgraphProcessor(p,_) => p.get_processor_name(),
        }
    }

    pub fn set_parameter(
        &mut self,
        parameter_name: &str,
        parameter_value: &str,
    ) -> Result<(), HardeenError> {
        match &mut self.processor_component {
            ProcessorComponent::BasicProcessor(processor) => {
                (*processor).set_parameter(parameter_name, parameter_value)
            }
            ProcessorComponent::SubgraphProcessor(processor,_) => {
                (*processor).set_parameter(parameter_name, parameter_value)
            }
        }
    }

    pub fn get_parameter(&self, parameter_name: &str) -> Result<String, HardeenError> {
        match &self.processor_component {
            ProcessorComponent::BasicProcessor(processor) => {
                (*processor).get_parameter(parameter_name)
            }
            ProcessorComponent::SubgraphProcessor(processor,_) => {
                (*processor).get_parameter(parameter_name)
            }
        }
    }

    pub fn get_parameters(&self) -> &[ProcessorParameter] {
        match &self.processor_component {
            ProcessorComponent::BasicProcessor(processor) => {
                (*processor).get_parameters()
            }
            ProcessorComponent::SubgraphProcessor(processor,_) => {
                (*processor).get_parameters()
            }
        }
    }

    pub fn is_parameter(&self, parameter_name: &str) -> bool {
        match &self.processor_component {
            ProcessorComponent::BasicProcessor(processor) => {
                (*processor).is_parameter(parameter_name)
            }
            ProcessorComponent::SubgraphProcessor(processor,_) => {
                (*processor).is_parameter(parameter_name)
            }
        }
    }

    pub fn is_input_satisfied(&self) -> bool {
        match &self.input_component {
            InputComponent::Multiple(multiple_input) => multiple_input.is_input_satisfied(),
            InputComponent::Slotted(slotted_input) => slotted_input.is_input_satisfied(),
        }
    }

    pub fn connect_input_node(
        &mut self,
        input: &NodeHandle<T>,
        slot_number: usize,
    ) -> Result<(), HardeenError> {
        match &mut self.input_component {
            InputComponent::Multiple(multiple_input) => {
                multiple_input.connect_input(input)?;
                Ok(())
            }
            InputComponent::Slotted(slotted_input) => {
                slotted_input.connect_input(input, slot_number)?;
                Ok(())
            }
        }
    }

    pub fn disconnect_input_node_slotted(&mut self, slot_number: usize) -> Result<(), HardeenError> {
        match &mut self.input_component {
            InputComponent::Multiple(_multiple_input) => Err(HardeenError::NodeInputTypeMismatch),
            InputComponent::Slotted(slotted_input) => {
                slotted_input.disconnect_input(slot_number)?;
                Ok(())
            }
        }
    }

    pub fn disconnect_input_node(
        &mut self,
        node_handle: &NodeHandle<T>,
    ) -> Result<(), HardeenError> {
        match &mut self.input_component {
            InputComponent::Multiple(multiple_input) => {
                multiple_input.disconnect_input(node_handle)?;
                Ok(())
            }
            InputComponent::Slotted(slotted_input) => {
                slotted_input.disconnect_handle(node_handle)?;
                Ok(())
            }
        }
    }

    pub fn connect_output_node(
        &mut self,
        output_handle: &NodeHandle<T>,
    ) -> Result<(), HardeenError> {
        self.output_nodes.insert(output_handle.clone());
        Ok(())
    }

    pub fn disconnect_output_node(
        &mut self,
        output_handle: &NodeHandle<T>,
    ) -> Result<(), HardeenError> {
        match self.output_nodes.remove(output_handle) {
            true => Ok(()),
            false => Err(HardeenError::NodeOutputHandleInvalid),
        }
    }

    pub fn get_all_input_handles(&self) -> Vec<NodeHandle<T>> {
        match &self.input_component {
            InputComponent::Multiple(multiple_input) => multiple_input.get_all_input_handles(),
            InputComponent::Slotted(slotted_input) => slotted_input.get_all_input_handles(),
        }
    }

    pub fn get_all_outputs(&self) -> HashSet<NodeHandle<T>> {
        self.output_nodes.clone()
    }

    pub fn get_cached_output(&self) -> Option<Rc<T>> {
        (*self.cached_output.borrow()).clone()
    }

    pub fn set_cached_output(&self, data: Rc<T>) {
        (*self.cached_output.borrow_mut()) = Some(data);
    }

    pub fn invalidate_cache(&mut self) {
        (*self.cached_output.borrow_mut()) = None;
    }
}
