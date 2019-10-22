//! # Processors
//!
//! Processors provide the core functionality of hardeen. A Processor gets some input and produces
//! some output. At the moment there are two overall types of processor. A BasicProcessor is just
//! dependent on its inputs. A SubgraphProcessor on the other hand apart from its input works with
//! another instance of a `Graph`, a subgraph. This allows for nesting and instancing of graphs within
//! other graphs as well as looping.
//!
//! Note that this module provides just traits and not concrete implementations of processors. These
//! traits are agnostic to their input type.

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::rc::Rc;

use crate::hardeen_error::HardeenError;

use super::parameters::*;
use super::Graph;
use super::SubgraphHandle;

use super::input_component::*;
use super::NodeHandle;

pub trait Processor<T: Serialize> {
    fn number_inputs(&self) -> usize;

    fn build_input_component(&self) -> InputComponent<NodeHandle<T>>;

    fn set_parameter(&mut self, param: &str, value: &str) -> Result<(), HardeenError>;
    fn get_parameter(&self, param: &str) -> Result<String, HardeenError>;
    fn get_parameters(&self) -> &[ProcessorParameter];
    fn is_parameter(&self, param: &str) -> bool;
    fn get_processor_name(&self) -> &'static str;
}

pub trait BasicProcessor<T: Serialize> : Processor<T> {
    fn run(
        &self,
        inputs: std::vec::Vec<Rc<T>>
    ) -> Rc<T>;
}

impl<T: Serialize> Serialize for Box<dyn BasicProcessor<T>> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(self.get_processor_name(), 1)?;
        s.serialize_field("number_inputs", &self.number_inputs())?;
        //s.serialize_field("parameters", &self.get_parameters())?;
        s.end()
    }
}

pub trait SubgraphProcessor<T: Serialize>: Processor<T> {
    fn run(&self, inputs: std::vec::Vec<Rc<T>>, subgraph: &Graph<T>) -> Rc<T>;
}

impl<T: Serialize> Serialize for Box<dyn SubgraphProcessor<T>> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(self.get_processor_name(), 1)?;
        s.serialize_field("number_inputs", &self.number_inputs())?;
        //s.serialize_field("parameters", &self.get_parameters())?;
        s.end()
    }
}

#[derive(Serialize)]
pub enum ProcessorComponent<T: Serialize> {
    BasicProcessor(Box<dyn BasicProcessor<T>>),
    SubgraphProcessor(Box<dyn SubgraphProcessor<T>>, SubgraphHandle<T>),
}
