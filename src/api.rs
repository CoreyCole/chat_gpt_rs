use leptos::{on_cleanup, Scope, Serializable, server, ServerFn, ServerFnError };
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = SendChat::register();
}

/// handle function on the server side
#[server(SendChat, "/api")]
pub async fn send_chat_server(msg: String) -> Result<Msg, ServerFnError> {
    let msg = Msg {
        idx: 10,
        id: "10".to_string(),
        msg_type: MsgType::TextFromUser(format!("pong: {msg}")),
    };
    Ok(msg)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Msg {
    pub id: String,
    pub idx: usize,
    pub msg_type: MsgType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum MsgType {
    TextFromBot(String),
    TextFromUser(String),
}