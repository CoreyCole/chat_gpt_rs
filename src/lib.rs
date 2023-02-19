use cfg_if::cfg_if;
use leptos::{component, view, IntoView, Scope, provide_context};
use leptos_meta::*;
use leptos_router::*;
pub mod api;
pub mod error_template;
pub mod fallback;
pub mod handlers;
mod routes;
use routes::chat::*;
mod state;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let render_state: &'static state::RenderState = Box::leak(Box::new(state::RenderState::new(cx)));
    provide_context::<&'static state::RenderState>(cx, render_state);
    view! {
        cx,
        <>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Stylesheet id="leptos" href="/pkg/chat_gpt_rs.css"/>
            <Meta name="description" content="Leptos implementation of ChatGPT."/>
            <Router>
                <main>
                    <Routes>
                        <Route path=":chat?" view=|cx| view! { cx,  <Chat/> }/>
                    </Routes>
                </main>
            </Router>
        </>
    }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            leptos::mount_to_body(move |cx| {
                view! { cx, <App/> }
            });
        }
    }
}
