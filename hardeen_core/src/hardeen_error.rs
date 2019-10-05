#[derive(Debug)]
pub enum HardeenError {
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
}
