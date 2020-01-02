#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

enum class HardeenResult {
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
  ExposedParameterDoesNotExist,
};

enum class ProcessorType {
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
  GroupPoints,
};

struct HCNodeHandle;

struct HCProject;

template<typename T>
struct Vec;

struct ProcessorParameter {
  CString name;
  CString ptype;
};

struct ProcessorTypeInfo {
  CString name;
  Vec<ProcessorParameter> parameters;
};

extern "C" {

/// # Safety
///
/// Creates a new processor node and returns a node handle. This node handle should be
/// freed after use!
HCNodeHandle *add_processor_node(HCProject *project, ProcessorType processor_type);

/// # Safety
///
/// Frees a node handle
void free_node_handle(HCNodeHandle *handle);

Vec<ProcessorTypeInfo> get_processor_node_infos();

HardeenResult go_to_subgraph(HCProject *project, const HCNodeHandle *node_handle);

/// # Safety
///
/// Frees memory allocated for HCProject
void hardeen_project_free(HCProject *project);

HCProject *hardeen_project_new();

} // extern "C"
