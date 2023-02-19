use leptos::*;

use crate::api;
use crate::state;

fn send_chat(_cx: Scope, state: &'static state::RenderState, msg: String) {
    log::debug!("send_chat: {msg:?}");
    state.send_msg(api::MsgType::TextFromUser(msg.clone()));
    spawn_local(async {
        match api::send_chat(msg).await {
            Err(e) => {
                log::error!("Error sending chat: {e}");
            }
            Ok(msg) => {
                log::info!("recv: {msg:?}");
                state.recv_msg(msg);
            }
        }
    })
}

#[component]
pub fn Chat(cx: Scope) -> impl IntoView {
    let state = use_context::<&'static state::RenderState>(cx)
        .expect("There should always be a render state");
    view! {
        cx,
        <div class="chat-view">
            <div class="chat-header">
                <div class="chat-header-title">
                    "Chat"
                </div>
            </div>
            <div class="chat-body">
                {move || view! { cx,
                    <ul>
                        <For
                            each=state.msgs
                            key=|msg| msg.idx
                            view=move |msg| {
                                view! {
                                    cx,
                                    <Msg msg/>
                                }
                            }
                        />
                    </ul>
                }.into_any()
                }
                <button on:click=move |_| send_chat(cx, state, "hello again".into()) class="chat-send-btn">
                    "Send"
                </button>
            </div>
        </div>
    }
}

#[component]
fn Msg(cx: Scope, msg: api::Msg) -> impl IntoView {
    view! { cx,
        <li class="chat-msg">
            {match &msg.msg_type {
                api::MsgType::TextFromUser(msg) => view! { cx,  <span class="chat-msg-user">{msg}</span> },
                api::MsgType::TextFromBot(msg) => view! { cx,  <span class="chat-msg-bot">{msg}</span> },
            }}
        </li>
    }
}
