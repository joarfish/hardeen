extern crate wasm_bindgen;
extern crate hardeen_core;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate console_error_panic_hook;

use std::vec::Vec;
use std::collections::HashMap;
use std::rc::Rc;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

use wasm_bindgen::prelude::*;

use hardeen_core::*;
use hardeen_core::Graph;
use hardeen_core::HandleIterator;
use hardeen_core::StdVec;
use hardeen_core::NodeHandle;
use hardeen_core::Handle;
use wasm_bindgen::JsCast;

use serde::Serialize;



/* This is still reeeeaaaally inconvenient: */
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type PointHandle = {
    index: number,
    generation: number
}

export type GroupHandle = {
    index: number,
    generation: number
}

export type ShapeHandle = {
    index: number,
    generation: number
}

export type PointDataHandle = {
    index: number,
    generation: number
}

export type Position = number[];

export type Point = {
    data: PointDataHandle[],
    groups: GroupHandle[],
    in_tangent: Position,
    out_tangent: Position,
    position: Position,
    generation: number
}

export type PointData = {
    name: string
}

export type Group = {
    name: string,
    points: PointHandle[]
}

export type Shape = {
    closed: boolean,
    vertices: PointHandle[],
    generation: number
}

export type GeometryWorld = {
    groups: {
        [key: number]: Group
    },
    points: {
        [key: number]: Point
    },
    shapes: {
        [key: number]: Shape
    }
}

export type NodeTypeParameter = {
    param_name: string,
    param_type: string
}

export type NodeInputType = { type: "Slotted", number_of_slots: number } | { type: "Multiple", zero_allowed: boolean };

export type NodeType = {
    name: string,
    input_type: NodeInputType,
    parameters: NodeTypeParameter[]
}
"#;


#[wasm_bindgen]
pub struct HardeenHandle {
    index: usize,
    generation: usize,
    nodeType: String
}

#[wasm_bindgen]
impl HardeenHandle {
    pub fn new(index: usize, generation: usize, nodeType: &str) -> Self {
        HardeenHandle {
            index,
            generation,
            nodeType: nodeType.to_string()
        }
    }

    pub fn get_node_type(&self) -> String {
        self.nodeType.clone()
    }
}

/*
#[wasm_bindgen]
#[derive(Serialize, Clone)]
pub struct HardeenGraphHandle {
    index: usize,
    generation: usize,
}

impl Handle for HardeenGraphHandle {
    fn new(index: usize, generation: usize) -> Self {
        HardeenGraphHandle {
            index,
            generation
        }
    }

    fn get_generation(&self) -> usize {
        self.generation
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get(&self) -> (usize, usize) {
        (self.index, self.generation)
    }
}*/

#[wasm_bindgen]
struct HardeenCoreInterface {
    nodeTypes: Vec<ProcessorTypeInfo>,
    lastResult: Option<Rc<GeometryWorld>>,
    graph: Graph<GeometryWorld>
   // graphs: HandledVec<HardeenGraphHandle, StdVec<Graph<GeometryWorld>>>
}

//type HardeenGraph = Graph<GeometryWorld>;


#[wasm_bindgen]
pub struct HardeenGraphPath {
    path: Vec<SubgraphHandle<GeometryWorld>>
}

#[wasm_bindgen]
impl HardeenCoreInterface {

    pub fn new() -> HardeenCoreInterface {
        console_error_panic_hook::set_once();

        HardeenCoreInterface {
            nodeTypes: vec![
                hardeen_core::Empty::get_processor_type_info(),
                hardeen_core::CreateRectangle::get_processor_type_info(),
                hardeen_core::ScatterPoints::get_processor_type_info(),
                hardeen_core::Scale::get_processor_type_info(),
                hardeen_core::RandomTangents::get_processor_type_info(),
                hardeen_core::AddPoints::get_processor_type_info(),
                hardeen_core::Merge::get_processor_type_info(),
                hardeen_core::CopyPointsAndOffset::get_processor_type_info(),
                hardeen_core::SortPointsX::get_processor_type_info(),
                hardeen_core::CreateShapeFromGroup::get_processor_type_info(),
                hardeen_core::Translate::get_processor_type_info(),
                hardeen_core::RandomTranslate::get_processor_type_info(),
                hardeen_core::CopyPointsAndRandomOffset::get_processor_type_info(),
                hardeen_core::CreateShapeFromAllGroups::get_processor_type_info(),
                hardeen_core::SmoothTangents::get_processor_type_info(),
                hardeen_core::InstanceOnPoints::get_processor_type_info(),
            ],
            lastResult: None,
            graph: Graph::new()
            //graphs: HandledVec::new()
        }
    }

    pub fn get_root_path(&self) -> HardeenGraphPath {
        HardeenGraphPath {
            path: vec![]
        }
    }

    fn get_subgraph_from_path_mut(&mut self, graph_path: &HardeenGraphPath) -> &mut Graph<GeometryWorld> {
        if graph_path.path.len() == 0 {
            &mut self.graph
        }
        else {
            let mut parent = &mut self.graph;
            for subgraph_handle in graph_path.path.iter().rev() {
                parent = parent.get_subgraph_mut(subgraph_handle).unwrap()
            }

            parent
        }
    }

    fn get_subgraph_from_path(&self, graph_path: &HardeenGraphPath) -> &Graph<GeometryWorld> {
        if graph_path.path.len() == 0 {
            &self.graph
        }
        else {
            let mut parent = &self.graph;
            for subgraph_handle in graph_path.path.iter().rev() {
                parent = parent.get_subgraph(subgraph_handle).unwrap()
            }

            parent
        }
    }

    pub fn get_graph_path(&self, parent_path: &HardeenGraphPath, handle: &HardeenHandle) -> HardeenGraphPath {
        let graph = self.get_subgraph_from_path(parent_path);
        let subgraph_handle = graph.get_subgraph_handle(&NodeHandle::new(handle.index, handle.generation)).unwrap();
        let mut path = parent_path.path.clone();
        path.push(subgraph_handle);

        HardeenGraphPath {
            path
        }
    }

    pub fn set_output_node(&mut self, path: &HardeenGraphPath, handle: &HardeenHandle) {
        let graph = self.get_subgraph_from_path_mut(path);

        graph.set_output_node_handle(NodeHandle::new(handle.index, handle.generation));
    }

    pub fn is_node_subgraph_processor(&self, path: &HardeenGraphPath, handle: &HardeenHandle) -> bool {
        let graph = self.get_subgraph_from_path(path);
        graph.is_node_subgraph_processor(&NodeHandle::new(handle.index, handle.generation)).unwrap()
    }

    pub fn add_processor_node(&mut self, path: &HardeenGraphPath, typeName: &str) -> HardeenHandle {
        let graph = self.get_subgraph_from_path_mut(path);
        let handle = graph.add_processor_node_by_type(typeName);

        HardeenHandle::new(handle.get_index(), handle.get_generation(), typeName)
    }

    pub fn remove_node(&mut self, path: &HardeenGraphPath, handle: HardeenHandle) {
        let graph = self.get_subgraph_from_path_mut(path);
        graph.remove_node(NodeHandle::new(handle.index, handle.generation));
    }

    pub fn connect_nodes_slotted(&mut self, path: &HardeenGraphPath, from: &HardeenHandle, to: &HardeenHandle, slot: usize) {
        let graph = self.get_subgraph_from_path_mut(path);
        graph.connect_to_slot(
            &NodeHandle::new(from.index, from.generation), 
            &NodeHandle::new(to.index, to.generation), slot);
    }

    pub fn connect_nodes(&mut self, path: &HardeenGraphPath, from: &HardeenHandle, to: &HardeenHandle) {
        let graph = self.get_subgraph_from_path_mut(path);
        graph.connect(
            &NodeHandle::new(from.index, from.generation),
            &NodeHandle::new(to.index, to.generation));
    }

    pub fn disconnect_nodes_slotted(&mut self, path: &HardeenGraphPath, from: &HardeenHandle, to: &HardeenHandle, slot: usize) {
        let graph = self.get_subgraph_from_path_mut(path);
        graph.disconnect_from_slot(
            &NodeHandle::new(from.index, from.generation), 
            &NodeHandle::new(to.index, to.generation), slot);
    }

    pub fn disconnect_nodes(&mut self, path: &HardeenGraphPath, from: &HardeenHandle, to: &HardeenHandle) {
        let graph = self.get_subgraph_from_path_mut(path);
        graph.disconnect(
            &NodeHandle::new(from.index, from.generation),
            &NodeHandle::new(to.index, to.generation));
    }

    pub fn set_node_parameter(&mut self, path: &HardeenGraphPath, handle: &HardeenHandle, parameter: &str, value: &str) -> bool {
        let h_handle = NodeHandle::new(handle.index, handle.generation);
        let graph = self.get_subgraph_from_path_mut(path);

        if let Ok(node) = graph.get_node_mut(&h_handle)
            {
                node.set_parameter(parameter, value);
                graph.invalidate_cache(&h_handle);
                return true;
            }

        return false;
    }

     pub fn get_node_parameter(&mut self, path: &HardeenGraphPath, handle: &HardeenHandle, parameter: &str) -> JsValue {
        let h_handle = NodeHandle::new(handle.index, handle.generation);
        let graph = self.get_subgraph_from_path_mut(path);
        
        if let Ok(node) = graph.get_node(&h_handle)
            {
                if let Ok(value) = node.get_parameter(parameter) {
                    return JsValue::from_str(&value.to_string());
                }
            }

        JsValue::from_str("Error")
    }

    pub fn run_processors(&mut self, path: &HardeenGraphPath) -> JsValue {
        let graph = self.get_subgraph_from_path_mut(path);
        if let Ok(result) = graph.process_graph_output() {
            self.lastResult = Some(result.clone());
            return JsValue::from_serde(&(*result)).unwrap()
        }

        JsValue::from_str("No result")
    }

    pub fn get_geometry_bounding_rect(&self) -> JsValue {
        if let Some(last_result) = &self.lastResult  {
            return JsValue::from_serde(&last_result.get_bounding_rect()).unwrap();
        }

        JsValue::from_serde(&(Position(0.0, 0.0), Position( 0.0, 0.0))).unwrap()
    }

    pub fn get_processor_parameters(&mut self, path: &HardeenGraphPath, handle: &HardeenHandle) -> JsValue {
        let hHandle = NodeHandle::new(handle.index, handle.generation);
        let graph = self.get_subgraph_from_path_mut(path);

        if let Ok(node) = graph.get_node(&hHandle)
        {
            return JsValue::from_serde(node.get_parameters()).unwrap();
        }

        JsValue::from_str("No result")
    }

    pub fn get_node_types(&self) -> JsValue {

        JsValue::from_serde(&self.nodeTypes).unwrap()
    }

    pub fn is_input_satisfied(&mut self, path: &HardeenGraphPath, handle: &HardeenHandle) -> bool {
        let hardeen_handle = NodeHandle::new(handle.index, handle.generation);
        let graph = self.get_subgraph_from_path_mut(path);

        if let Ok(node) = graph.get_node(&hardeen_handle) {
            return node.is_input_satisfied();
        }

        false
    }

    /*
    pub fn get_shape_iterator(&self) -> Result<ShapeHandleIterator, ()> {
        if let Some(world) = &self.lastResult {
            Ok((*world).get_shape_iterator())
        }

        Err(())
    }*/
}