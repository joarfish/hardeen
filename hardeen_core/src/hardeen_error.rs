#[derive(Debug)]
pub enum HardeenError {
    NodeSlotDoesNotExist,
    InvalidInputSlotNumber,
    NodeParameterDoesNotExist,
    NodeInputTypeMismatch,
    NodeRunTypeMismatch,
    NodeOutputHandleInvalid,
    InvalidHandle,
    NodeTypeInvalid,
    GraphOutputNotSet,
    ErrorProcessingNode,
    ExposedParameterDoesNotExist,
}
