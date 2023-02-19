use leptos::{Scope, Serializable, server, ServerFn, ServerFnError};
use serde::{Deserialize, Serialize};

mod openai;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = SendChat::register();
}

/// handle function on the server side
#[server(SendChat, "/api")]
pub async fn send_chat(msg: String) -> Result<Msg, ServerFnError> {
    let bot_res = openai::complete(msg).await?;
    let msg = Msg {
        idx: 10,
        id: "10".to_string(),
        msg_type: MsgType::TextFromUser(bot_res),
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
