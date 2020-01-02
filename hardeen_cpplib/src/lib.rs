extern crate hardeen_core;

use hardeen_core::{ HardeenProject, NodeHandle };
use hardeen_core::GeometryWorld;
use std::boxed::Box;
use std::ffi::CString;

#[repr(C)]
pub enum HardeenResult {
    Ok,
    GotNullPointer,
    InvalidReference,
    NodeSlotDoesNotExist,
    InvalidInputSlotNumber,
    NodeParameterDoesNotExist,
    NodeInputTypeMismatch,
    NodeRunTypeMismatch,
    NodeOutputHandleInvalid,
    NodeInputNotSatisfied,
    InvalidHandle,
    NodeTypeInvalid,
    GraphOutputNotSet,
    ErrorProcessingNode,
    ExposedParameterDoesNotExist
}

impl From<hardeen_core::HardeenError> for HardeenResult {
    fn from(error: hardeen_core::HardeenError) -> Self {
        match error {
            hardeen_core::HardeenError::NodeSlotDoesNotExist => HardeenResult::NodeSlotDoesNotExist,
            hardeen_core::HardeenError::InvalidInputSlotNumber => HardeenResult::InvalidInputSlotNumber,
            hardeen_core::HardeenError::NodeParameterDoesNotExist => HardeenResult::NodeParameterDoesNotExist,
            hardeen_core::HardeenError::NodeInputTypeMismatch => HardeenResult::NodeInputTypeMismatch,
            hardeen_core::HardeenError::NodeRunTypeMismatch => HardeenResult::NodeRunTypeMismatch,
            hardeen_core::HardeenError::NodeOutputHandleInvalid => HardeenResult::NodeOutputHandleInvalid,
            hardeen_core::HardeenError::NodeInputNotSatisfied => HardeenResult::NodeInputNotSatisfied,
            hardeen_core::HardeenError::InvalidHandle => HardeenResult::InvalidHandle,
            hardeen_core::HardeenError::NodeTypeInvalid => HardeenResult::NodeTypeInvalid,
            hardeen_core::HardeenError::GraphOutputNotSet => HardeenResult::GraphOutputNotSet,
            hardeen_core::HardeenError::ErrorProcessingNode => HardeenResult::ErrorProcessingNode,
            hardeen_core::HardeenError::ExposedParameterDoesNotExist => HardeenResult::ExposedParameterDoesNotExist
        }
    }
}


#[repr(C)]
pub enum ProcessorType {
    Empty,
    CreateRectangle,
    ScatterPoints,
    Scale,
    RandomTangents,
    SmoothTangents,
    AddPoints,
    Merge,
    CopyPointsAndOffset,
    SortPointsX,
    CreateShapeFromGroup,
    CreateShapeFromAllGroups,
    Translate,
    RandomTranslate,
    CopyPointsAndRandomOffset,
    InstanceOnPoints,
    ExtrudeShape,
    GroupPoints
}

impl Into<hardeen_core::ProcessorType> for ProcessorType {
    fn into(self) -> hardeen_core::ProcessorType {
        match self {
            _ => hardeen_core::ProcessorType::Empty
        }
    }  
}

impl From<hardeen_core::ProcessorType> for ProcessorType {
    fn from(processor_type: hardeen_core::ProcessorType) -> Self {
        match processor_type {
            _ => ProcessorType::Empty
        }
    }
}

pub struct HCProject {
    project: HardeenProject<GeometryWorld>
}

pub struct HCNodeHandle {
    handle: NodeHandle<GeometryWorld>
}

#[no_mangle]
pub extern "C" fn hardeen_project_new() -> *mut HCProject {

    Box::into_raw(Box::new( HCProject { project: HardeenProject::new() } ))
}

/// # Safety
///
/// Frees memory allocated for HCProject
#[no_mangle]
pub unsafe extern "C" fn hardeen_project_free(project: *mut HCProject) {
    Box::from_raw(project);
}


/// # Safety
///
/// Creates a new processor node and returns a node handle. This node handle should be
/// freed after use!
#[no_mangle]
pub unsafe extern "C" fn add_processor_node(project: *mut HCProject, processor_type: ProcessorType) -> *mut HCNodeHandle {
    let graph = (*project).project.get_current_graph_mut().unwrap();
    Box::into_raw( Box::new( HCNodeHandle { handle: graph.add_processor_node_by_type(processor_type.into()) } ) )
}

/// # Safety
///
/// Frees a node handle
#[no_mangle]
pub unsafe extern "C" fn free_node_handle(handle: *mut HCNodeHandle) {
    Box::from_raw(handle);
}

#[no_mangle]
pub unsafe extern "C" fn go_to_subgraph(project: *mut HCProject, node_handle: *const HCNodeHandle) -> HardeenResult {

    match node_handle.as_ref() {
        Some(node_handle) => {
            match (*project).project.get_current_graph_mut() {
                Ok(current_graph) => {
                    match current_graph.get_subgraph_handle(&node_handle.handle) {
                        Ok(subgraph_handle) => {
                            (*project).project.go_level_down(subgraph_handle);
                            HardeenResult::Ok
                        },
                        Err(err) => {
                            HardeenResult::from(err)
                        }
                    }
                },
                Err(err) => {
                    HardeenResult::from(err)
                }
            }
        },
        None => HardeenResult::GotNullPointer
    }
}

