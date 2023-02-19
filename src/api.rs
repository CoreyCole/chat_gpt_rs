use leptos::{server, ServerFn, ServerFnError};
use serde::{Deserialize, Serialize};
// use std::env;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = SendChat::register();
}

/// handle function on the server side
#[server(SendChat, "/api")]
pub async fn send_chat_server(msg: String) -> Result<Msg, ServerFnError> {
    // let open_ai_key = env::var("OPEN_AI_KEY").expect("server must have OPEN_AI_KEY set");
    // let client = openai_api::Client::new(&open_ai_key);
    // let args = openai_api::api::CompletionArgs::builder()
    //     .prompt(msg)
    //     .engine("text-davinci-003")
    //     .max_tokens(1024)
    //     .temperature(0.5)
    //     .stop(vec!["\n".into()]);
    // let bot_res = client.complete_prompt(args).await
    //     .map_err(|e| ServerFnError::ServerError(format!("openai_api::Client::complete_prompt err: {}", e.to_string())))?;
    let bot_res = format!("pong: {msg}");
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
