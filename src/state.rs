use crate::api;

use std::sync::{Arc, Mutex};
use std::fmt::Debug;

use futures::{channel::mpsc::{Sender, Receiver}, StreamExt};
use leptos::{spawn_local, use_context, Scope, RwSignal, create_rw_signal };

#[derive(Clone)]
pub struct RenderState {
    pub msgs: RwSignal<Vec<api::Msg>>,
}

impl RenderState {
    pub fn new(cx: Scope) -> Self {
        RenderState {
            msgs: create_rw_signal(cx, vec![
                api::Msg {
                    idx: 0,
                    id: "0".to_string(),
                    msg_type: api::MsgType::TextFromUser("Hello".to_string()),
                },
                api::Msg {
                    idx: 1,
                    id: "1".to_string(),
                    msg_type: api::MsgType::TextFromBot("Hi".to_string()),
                },
            ]),
        }
    }

    pub fn send_msg(&'static self, msg_type: api::MsgType) {
        self.msgs.update(move |msgs| {
            msgs.push(api::Msg {
                idx: msgs.len(),
                id: msgs.len().to_string(),
                msg_type,
            });
        });
    }

    pub fn recv_msg(&'static self, msg: api::Msg) {
        self.msgs.update(move |msgs| {
            msgs.push(msg);
        });
    }
}
