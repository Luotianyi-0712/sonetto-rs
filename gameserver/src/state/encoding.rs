use prost::Message;
use crate::error::AppError;

/// Encodes a protobuf message into a Vec<u8>
pub fn encode_message<T: Message>(msg: &T) -> Result<Vec<u8>, AppError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)
        .map_err(|e| AppError::Custom(e.to_string()))?;
    Ok(buf)
}
