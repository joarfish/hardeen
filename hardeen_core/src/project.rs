//! # Hardeen Project
//!
//! A project offers an interface to work with a number of related graphs. As each graph may have a 
//! number of subgraphs, HardeenProject provided methods to descent the tree-like structure Graphs 
//! form.
//!

use std::vec::Vec;
use serde::Serialize;

use crate::graph::*;
use crate::hardeen_error::HardeenError;

pub struct HardeenProject<T: Serialize> {
    root_graph: Graph<T>,
    current_path: Vec<SubgraphHandle<T>>
}

impl<T: Serialize> HardeenProject<T> {
    pub fn new() -> Self {
        HardeenProject {
            root_graph: Graph::new(),
            current_path: vec![]
        }
    }

    pub fn get_current_graph_mut(&mut self) -> Result<&mut Graph<T>, HardeenError> {
        
        if self.current_path.is_empty() {
            return Ok(&mut self.root_graph);
        }

        let mut current_graph = &mut self.root_graph;

        for subgraph_handle in self.current_path.iter() {
            current_graph = current_graph.get_subgraph_mut(subgraph_handle)?;
        }

        Ok(current_graph)
    }

    pub fn go_level_up(&mut self) {

        if !self.current_path.is_empty() {
            self.current_path.pop();
        }
    }

    pub fn go_level_down(&mut self, subgraph_handle: SubgraphHandle<T>) {
        self.current_path.push(subgraph_handle);
    }
}


