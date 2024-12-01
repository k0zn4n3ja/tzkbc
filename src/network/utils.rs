use super::message::Message;
use bincode;

pub fn serialize_message(message: &Message) -> Vec<u8> {
    bincode::serialize(message).expect("Failed to serialize message")
}

pub fn deserialize_message(data: &[u8]) -> Message {
    bincode::deserialize(data).expect("Failed to deserialize message")
}