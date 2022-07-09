use {
    serde,
    serde_json::Value,
};

pub struct Message {
    pub command_type: i32,
    pub command_content: Value,
}

pub const TYPE_CONTEXT: i32 = 0;
pub const TYPE_SESSION_REQ : i32 = 1;
pub const TYPE_SESSION_ALL : i32 = 2;


// decode message from json value
pub fn decode<T>(message: Value) -> Result<T, String>
    where
    T: serde::de::DeserializeOwned
{
    match serde_json::from_value(message) {
        Ok(message) => Ok(message),
        Err(e) => Err(e.to_string())
    }
}
