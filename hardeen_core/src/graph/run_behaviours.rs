use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::rc::Rc;

use crate::hardeen_error::HardeenError;

use super::parameters::*;
use super::Graph;
use super::SubgraphHandle;

use super::input_behaviours::*;
use super::NodeHandle;

pub trait ParameteredProcessor<T: Serialize> {
    fn number_inputs(&self) -> usize;

    fn build_input_behaviour(&self) -> InputBehaviour<NodeHandle<T>>;

    fn set_parameter(&mut self, param: &str, value: &str) -> Result<(), HardeenError>;
    fn get_parameter(&self, param: &str) -> Result<String, HardeenError>;
    fn get_parameters(&self) -> &[ProcessorParameter];
    fn is_parameter(&self, param: &str) -> bool;
    fn get_processor_name(&self) -> &'static str;
}

pub trait Processor<T: Serialize> : ParameteredProcessor<T> {
    fn run(
        &self,
        inputs: std::vec::Vec<Rc<T>>
    ) -> Rc<T>;
}

impl<T: Serialize> Serialize for Box<dyn Processor<T>> {
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

pub trait SubgraphProcessor<T: Serialize>: ParameteredProcessor<T> {
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
pub enum RunBehaviour<T: Serialize> {
    Processor(Box<dyn Processor<T>>),
    SubgraphProcessor(Box<dyn SubgraphProcessor<T>>, SubgraphHandle<T>),
}
