//! # Processor Graph
//!
//! `processor_graph` implements a directed, acyclic graph. The purpose of a processor graph
//! is to take some input and push it through a number of processors and produce one output.
//! Each node has a fixed number of ingoing edges and an arbitrary number of outgoing edges.
//!

use serde::Serialize;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

mod input_behaviours;
mod nodes;
mod parameters;
mod run_behaviours;

pub use nodes::*;
pub use parameters::*;
pub use run_behaviours::*;
pub use input_behaviours::*;

use crate::hardeen_error::HardeenError;
use crate::handled_vec::{HandledVec, HandledVecError, MarkedHandle, StdVec};

pub type NodeHandle<T> = MarkedHandle<Node<T>>;
pub type SubgraphHandle<T> = MarkedHandle<Graph<T>>;
pub type ParameterHandle = MarkedHandle<Parameter>;

pub type GraphInputBehaviour<T> = InputBehaviour<NodeHandle<T>>;

#[derive(Serialize)]
pub struct ExposedParameter<T: Serialize> {
    node_handle: NodeHandle<T>,
    parameter_name: String,
}

impl<T: Serialize> ExposedParameter<T> {
    fn new(node_handle: &NodeHandle<T>, parameter_name: &str) -> Self {
        ExposedParameter {
            node_handle: node_handle.clone(),
            parameter_name: String::from(parameter_name),
        }
    }

    pub fn get_node_handle(&self) -> &NodeHandle<T> {
        &self.node_handle
    }

    fn get_parameter_name(&self) -> &str {
        &self.parameter_name
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ProcessorInputType {
    Slotted{number_of_slots: u8},
    Multiple{zero_allowed: bool}
}

#[derive(Serialize)]
pub struct ProcessorTypeInfo {
    name: &'static str,
    input_type: ProcessorInputType,
    parameters: Vec<ProcessorParameter>
}

impl ProcessorTypeInfo {
    pub fn new(name: &'static str, input_type: ProcessorInputType, parameters: Vec<ProcessorParameter>) -> Self {
        ProcessorTypeInfo {
            name,
            input_type,
            parameters
        }
    }
}

#[derive(Serialize)]
pub struct Graph<T: Serialize> {
    input: Vec<Option<T>>,
    nodes: HandledVec<NodeHandle<T>, StdVec<Node<T>>>,
    exposed_parameters: HashMap<String, ExposedParameter<T>>,
    subgraphs: HandledVec<SubgraphHandle<T>, StdVec<Graph<T>>>,
    output_node_handle: Option<NodeHandle<T>>,
    processor_types: Vec<ProcessorTypeInfo>
}

impl std::convert::From<HandledVecError> for HardeenError {
    fn from(_error: HandledVecError) -> HardeenError {
        HardeenError::InvalidHandle
    }
}

impl<T: Serialize> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            input: Vec::new(),
            nodes: HandledVec::new(),
            exposed_parameters: HashMap::new(),
            subgraphs: HandledVec::new(),
            output_node_handle: None,
            processor_types: Vec::new()
        }
    }

    pub fn get_processor_types(&self) -> &[ProcessorTypeInfo] {
        &self.processor_types
    }

    pub fn add_processor_node(&mut self, processor: Box<dyn Processor<T>>) -> NodeHandle<T> {
        self.nodes.add_entry(Node::new_processor_node(processor))
    }

    pub fn add_subgraph_processor_node(
        &mut self,
        processor: Box<dyn SubgraphProcessor<T>>,
    ) -> NodeHandle<T> {

        let subgraph = Graph::new();
        let subgraph_handle = self.subgraphs.add_entry(subgraph);

        let handle = self.nodes
            .add_entry(Node::new_subgraph_processor_node(processor, subgraph_handle));

        handle
    }

    pub fn get_subgraph_handle(&self, node_handle: &NodeHandle<T>) -> Result<SubgraphHandle<T>, HardeenError> {
        let node = self.get_node(node_handle)?;

        match node.get_run_behaviour() {
            RunBehaviour::Processor(_) => Err(HardeenError::NodeRunTypeMismatch),
            RunBehaviour::SubgraphProcessor(_, subgraph_handle) => Ok(subgraph_handle.clone())
        }
    }

    pub fn get_subgraph(&self, handle: &SubgraphHandle<T>) -> Result<&Graph<T>, HardeenError> {
        Ok(self.subgraphs.get(handle)?)
    }

    pub fn get_subgraph_mut(&mut self, handle: &SubgraphHandle<T>) -> Result<&mut Graph<T>, HardeenError> {
        Ok(self.subgraphs.get_mut(handle)?)
    }

    pub fn get_subgraph_for_node(&mut self, node_handle: &NodeHandle<T>) -> Result<&mut Graph<T>, HardeenError> {
        let subgraph_handle = self.get_subgraph_handle(node_handle)?;
        self.get_subgraph_mut(&subgraph_handle)
    }

    pub fn is_node_subgraph_processor(&self, node_handle: &NodeHandle<T>) -> Result<bool, HardeenError> {
        let node = self.get_node(node_handle)?;
        match node.get_run_behaviour() {
            RunBehaviour::Processor(_) => Ok(false),
            RunBehaviour::SubgraphProcessor(_,_) => Ok(true)
        }
    }

    pub fn set_exposed_parameter(
        &mut self,
        name: &str,
        value: &str
    ) -> Result<(), HardeenError> {
        return match self.exposed_parameters.get(name) {
            Some(exposed_parameter) => {
                let node_handle = exposed_parameter.get_node_handle();
                (*self.nodes.get_mut(node_handle)?)
                    .set_parameter(exposed_parameter.get_parameter_name(), value)?;

                Ok(())
            }
            None => Err(HardeenError::ExposedParameterDoesNotExist),
        };
    }

    pub fn expose_parameter(
        &mut self,
        exposed_name: &str,
        node_handle: &NodeHandle<T>,
        parameter_name: &str,
    ) -> Result<(), HardeenError> {
        if let Ok(node) = self.nodes.get(node_handle) {
            if node.is_parameter(parameter_name) {
                self.exposed_parameters.insert(
                    String::from(exposed_name),
                    ExposedParameter::new(node_handle, parameter_name),
                );
            }
        }

        Ok(())
    }

    pub fn remove_node(&mut self, handle: NodeHandle<T>) -> Result<(), HardeenError> {
        self.nodes.is_handle_valid(&handle)?;
        self.disconnect_all_nodes(&handle)?;
        self.nodes.remove_entry(handle)?;
        Ok(())
    }

    pub fn get_node(&self, handle: &NodeHandle<T>) -> Result<&Node<T>, HardeenError> {
        Ok(self.nodes.get(handle)?)
    }

    pub fn get_node_mut(&mut self, handle: &NodeHandle<T>) -> Result<&mut Node<T>, HardeenError> {
        Ok(self.nodes.get_mut(handle)?)
    }

    pub fn connect_to_slot(
        &mut self,
        from: &NodeHandle<T>,
        to: &NodeHandle<T>,
        slot: usize,
    ) -> Result<(), HardeenError> {
        if !self.is_handle_valid(from) || !self.is_handle_valid(to) {
            panic!("NodeHandle is invalid!");
        }

        if self.path_exists(to, from) {
            panic!("Circles are not allowed!");
        }

        return match self
            .nodes
            .get_mut(to)
            .unwrap()
            .connect_input_node(from, slot)
            .and(self.nodes.get_mut(from).unwrap().connect_output_node(to))
        {
            Ok(()) => Ok(()),
            Err(_error) => Err(HardeenError::InvalidHandle),
        };
    }

    pub fn connect(
        &mut self,
        from: &NodeHandle<T>,
        to: &NodeHandle<T>,
    ) -> Result<(), HardeenError> {
        self.connect_to_slot(from, to, 0)
    }

    pub fn disconnect_all_nodes(&mut self, handle: &NodeHandle<T>) -> Result<(), HardeenError> {
        if let Err(_error) = self.nodes.get(handle) {
            return Err(HardeenError::InvalidHandle);
        }

        let node = self.nodes.get(handle).unwrap();
        let input_node_handles = node.get_all_input_handles();
        let output_node_handles = node.get_all_outputs();

        for other_handle in output_node_handles.iter() {
            match self.nodes.get_mut(&other_handle) {
                Ok(other_node) => {
                    other_node.disconnect_output_node(handle)?;
                }
                Err(_error) => {}
            }
        }

        for (slot_number, other_handle) in input_node_handles.iter().enumerate() {
            match self.nodes.get_mut(&other_handle) {
                Ok(other_node) => {
                    other_node.disconnect_input_node_slotted(slot_number)?;
                }
                Err(_error) => {}
            }
        }

        Ok(())
    }

    pub fn disconnect_from_slot(
        &mut self,
        from: &NodeHandle<T>,
        to: &NodeHandle<T>,
        slot: usize,
    ) -> Result<(), HardeenError> {
        let from_node = self.nodes.get_mut(from).unwrap();

        from_node.disconnect_output_node(to).and(
            self.nodes
                .get_mut(to)
                .unwrap()
                .disconnect_input_node_slotted(slot),
        )
    }

    pub fn disconnect(
        &mut self,
        from: &NodeHandle<T>,
        to: &NodeHandle<T>,
    ) -> Result<(), HardeenError> {
        let from_node = self.nodes.get_mut(from).unwrap();

        from_node
            .disconnect_output_node(to)
            .and(self.nodes.get_mut(to).unwrap().disconnect_input_node(from))
    }

    pub fn set_output_node_handle(&mut self, output_node_handle: NodeHandle<T>) {
        if !self.is_handle_valid(&output_node_handle) {
            panic!("Node with this handle does not exist!");
        }
        if let RunBehaviour::SubgraphProcessor(_,_) = self.get_node(&output_node_handle).unwrap().get_run_behaviour() {
            self.invalidate_cache(&output_node_handle);
        }
        self.output_node_handle = Some(output_node_handle);
    }

    pub fn process_graph_output(&self, use_caches: bool) -> Result<Rc<T>, HardeenError> {
        if let Some(output_node_handle) = self.output_node_handle.clone() {
            return self.process_node(&output_node_handle, use_caches);
        }

        Err(HardeenError::GraphOutputNotSet)
    }

    fn process_node(&self, node_handle: &NodeHandle<T>, use_caches: bool) -> Result<Rc<T>, HardeenError> {
        let mut inputs: Vec<Rc<T>> = Vec::new();

        let node = self.get_node(node_handle)?;

        if !node.is_input_satisfied() {
            return Err(HardeenError::NodeInputNotSatisfied);
        }

        if use_caches {
            if let Some(cached_output) = node.get_cached_output() {
                return Ok(cached_output);
            }
        }

        for input_node_handle in node.get_all_input_handles().iter() {
            if let Ok(result) = self.process_node(input_node_handle, use_caches) {
                inputs.push(result);
            } else {
                return Err(HardeenError::ErrorProcessingNode);
            }
        }

        let result = match node.get_run_behaviour() {
            RunBehaviour::Processor(processor) => {
                (*processor).run(inputs)
            },
            RunBehaviour::SubgraphProcessor(processor, subgraph_handle) => {
                let subgraph = self.subgraphs.get(&subgraph_handle)?;
                (*processor).run(inputs, subgraph)
            }
        };

        node.set_cached_output(result.clone());

        Ok(result)
    }

    pub fn invalidate_cache(&mut self, node_handle: &NodeHandle<T>) {
        let node = self
            .get_node_mut(node_handle)
            .expect("Node to process does not exist!");
        node.invalidate_cache();

        let node = self
            .get_node(node_handle)
            .expect("Node to process does not exist!");

        for out_node_handle in node.get_all_outputs().iter() {
            self.invalidate_cache(&out_node_handle);
        }
    }

    pub fn get_output_node(&self) -> Result<&Node<T>, HardeenError> {
        return match &self.output_node_handle {
            None => Err(HardeenError::GraphOutputNotSet),
            Some(handle) => match self.nodes.get(handle) {
                Err(_error) => Err(HardeenError::InvalidHandle),
                Ok(node) => Ok(node),
            },
        };
    }

    pub fn get_output_node_handle(&self) -> Option<NodeHandle<T>> {
        self.output_node_handle.clone()
    }

    pub fn is_output_node_set(&self) -> bool {
        match self.output_node_handle {
            Some(_) => true,
            None => false
        }
    }

    fn path_exists(&self, from: &NodeHandle<T>, to: &NodeHandle<T>) -> bool {
        if from == to {
            return true;
        }

        let start = self.nodes.get(from).expect("Node does not exist!");

        let start_output_nodes = start.get_all_outputs();

        if start_output_nodes.is_empty() {
            false
        } else {
            for kv in start_output_nodes.iter() {
                if self.path_exists(kv, to) {
                    return true;
                }
            }
            false
        }
    }

    fn is_handle_valid(&self, handle: &NodeHandle<T>) -> bool {
        match self.nodes.is_handle_valid(handle) {
            Ok(_true) => true,
            Err(_error) => false,
        }
    }
}
