use serde::Serialize;
use std::collections::{HashSet};
use std::rc::Rc;
use std::vec::Vec;
use std::cell::RefCell;

use super::parameters::*;

use super::input_behaviours::*;
use super::run_behaviours::*;
use super::SubgraphHandle;

use crate::handled_vec::MarkedHandle;

use crate::hardeen_error::HardeenError;

pub type NodeHandle<T> = MarkedHandle<Node<T>>;

#[derive(Serialize)]
pub struct Node<T: Serialize> {
    run_component: RunBehaviour<T>,
    input_component: InputBehaviour<NodeHandle<T>>,
    output_nodes: HashSet<NodeHandle<T>>,
    #[serde(skip)]
    cached_output: RefCell<Option<Rc<T>>>,
}

impl<T: Serialize> Node<T> {
    pub fn new_processor_node(processor: Box<dyn Processor<T>>) -> Self {
        let input_behaviour = (*processor).build_input_behaviour();

        Self::new(
            RunBehaviour::Processor(processor),
            input_behaviour
        )
    }

    pub fn new_subgraph_processor_node(processor: Box<dyn SubgraphProcessor<T>>, handle: SubgraphHandle<T>) -> Self {
        let input_component = (*processor).build_input_behaviour();

        Self::new(
            RunBehaviour::SubgraphProcessor(processor, handle),
            input_component
        )
    }

    fn new(
        run_component: RunBehaviour<T>,
        input_component: InputBehaviour<NodeHandle<T>>
    ) -> Self {
        Node {
            run_component,
            input_component,
            output_nodes: HashSet::new(),
            cached_output: RefCell::new(None),
        }
    }

    pub fn get_run_behaviour_mut(&mut self) -> &mut RunBehaviour<T> {
        &mut self.run_component
    }

    pub fn get_run_behaviour(&self) -> &RunBehaviour<T> {
        &self.run_component
    }

    pub fn get_input_behaviour(&self) -> &InputBehaviour<NodeHandle<T>> {
        &self.input_component
    }

    pub fn get_processor_name(&self) -> &str {
        match &self.run_component {
            RunBehaviour::Processor(p) => p.get_processor_name(),
            RunBehaviour::SubgraphProcessor(p,_) => p.get_processor_name(),
        }
    }

    pub fn set_parameter(
        &mut self,
        parameter_name: &str,
        parameter_value: &str,
    ) -> Result<(), HardeenError> {
        match &mut self.run_component {
            RunBehaviour::Processor(processor) => {
                (*processor).set_parameter(parameter_name, parameter_value)
            }
            RunBehaviour::SubgraphProcessor(processor,_) => {
                (*processor).set_parameter(parameter_name, parameter_value)
            }
        }
    }

    pub fn get_parameter(&self, parameter_name: &str) -> Result<String, HardeenError> {
        match &self.run_component {
            RunBehaviour::Processor(processor) => {
                (*processor).get_parameter(parameter_name)
            }
            RunBehaviour::SubgraphProcessor(processor,_) => {
                (*processor).get_parameter(parameter_name)
            }
        }
    }

    pub fn get_parameters(&self) -> &[ProcessorParameter] {
        match &self.run_component {
            RunBehaviour::Processor(processor) => {
                (*processor).get_parameters()
            }
            RunBehaviour::SubgraphProcessor(processor,_) => {
                (*processor).get_parameters()
            }
        }
    }

    pub fn is_parameter(&self, parameter_name: &str) -> bool {
        match &self.run_component {
            RunBehaviour::Processor(processor) => {
                (*processor).is_parameter(parameter_name)
            }
            RunBehaviour::SubgraphProcessor(processor,_) => {
                (*processor).is_parameter(parameter_name)
            }
        }
    }

    pub fn is_input_satisfied(&self) -> bool {
        return match &self.input_component {
            InputBehaviour::Multiple(multiple_input) => multiple_input.is_input_satisfied(),
            InputBehaviour::Slotted(slotted_input) => slotted_input.is_input_satisfied(),
        };
    }

    /*fn get_input_node(&self, slot_number: u8) -> Result<&Option<NodeHandle<T>>, HardeenError> {
        match &self.input_behaviour {
            InputBehaviour::Multiple(multiple) => {
                Err(NodeError::InputTypeMismatch)
            },
            InputBehaviour::Slotted(slotted_input) => {
                Ok(slotted_input.get_input(slot_number))
            }
        }
    }*/

    pub fn connect_input_node(
        &mut self,
        input: &NodeHandle<T>,
        slot_number: usize,
    ) -> Result<(), HardeenError> {
        match &mut self.input_component {
            InputBehaviour::Multiple(multiple_input) => {
                multiple_input.connect_input(input)?;
                Ok(())
            }
            InputBehaviour::Slotted(slotted_input) => {
                slotted_input.connect_input(input, slot_number)?;
                Ok(())
            }
        }
    }

    pub fn disconnect_input_node_slotted(&mut self, slot_number: usize) -> Result<(), HardeenError> {
        match &mut self.input_component {
            InputBehaviour::Multiple(_multiple_input) => Err(HardeenError::NodeInputTypeMismatch),
            InputBehaviour::Slotted(slotted_input) => {
                slotted_input.disconnect_input(slot_number)?;
                Ok(())
            }
        }
    }

    pub fn disconnect_input_node(
        &mut self,
        node_handle: &NodeHandle<T>,
    ) -> Result<(), HardeenError> {
        return match &mut self.input_component {
            InputBehaviour::Multiple(multiple_input) => {
                multiple_input.disconnect_input(node_handle)?;
                Ok(())
            }
            InputBehaviour::Slotted(slotted_input) => {
                slotted_input.disconnect_handle(node_handle)?;
                Ok(())
            }
        };
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
        return match self.output_nodes.remove(output_handle) {
            true => Ok(()),
            false => Err(HardeenError::NodeOutputHandleInvalid),
        };
    }

    pub fn get_all_input_handles(&self) -> Vec<NodeHandle<T>> {
        return match &self.input_component {
            InputBehaviour::Multiple(multiple_input) => multiple_input.get_all_input_handles(),
            InputBehaviour::Slotted(slotted_input) => slotted_input.get_all_input_handles(),
        };
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

    /*pub fn run(&mut self, inputs: std::vec::Vec<Rc<T>>) -> Rc<T> {
        let result = match &mut self.run_behaviour {
            RunBehaviour::Processor(processor) => (*processor).run(inputs),
            RunBehaviour::SubgraphProcessor(processor) => {
                (*processor).run(inputs)
            }
        };

        self.cached_output = Some(result.clone());

        result
    }*/

    /*pub fn get_subgraph(&mut self) -> Graph<T> {
        match &mut self.run_behaviour {
            RunBehaviour::Processor(_processor) => panic!("Node isn't a Subgraph Processor!"),
            RunBehaviour::SubgraphProcessor(processor) => {
                (*processor).get_subgraph()
            }
        }
    }*/

    /*
    fn get_output_node(&self, to_node_index: &usize) -> Result<&NodeHandle, NodeError> {
        return match self.output_nodes.get(to_node_index) {
            Some(output_handle) => Ok(output_handle),
            None => Err(NodeError::SlotDoesNotExist)
        }
    }

    fn set_output_node(&mut self, to_node_index: usize, output: NodeHandle) {
        self.output_nodes.insert(to_node_index, output);
    }*/
}
