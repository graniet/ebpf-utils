use crate::loader::LoadContext;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, sync::mpsc};

pub struct SessionMessage {
    pub message_type: i32,
    pub message_content: serde_json::Value,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SessionReq {
    pub probe: String, // fitered search (soon)
    pub comm: String, // filtered search (soon)
    pub hook: String, // filtered search (soon)
    pub value: String, // global value
}

pub struct Session {
    pub rx: mpsc::Receiver<SessionMessage>,
    pub context: HashMap<String, Vec<LoadContext>>,
}

pub fn new() -> (Session, mpsc::Sender<SessionMessage>) {
    let (tx, rx) = mpsc::channel::<SessionMessage>();
    (
        Session {
            rx,
            context: HashMap::new(),
        },
        tx,
    )
}
