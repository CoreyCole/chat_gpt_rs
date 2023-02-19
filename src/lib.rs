use cfg_if::cfg_if;
use leptos::{component, provide_context, view, IntoView, Scope};
use leptos_meta::*;
use leptos_router::*;
pub mod api;
mod routes;
use routes::chat::*;
mod state;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let render_state: &'static state::RenderState =
        Box::leak(Box::new(state::RenderState::new(cx)));
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
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            mount_to_body(|cx| {
                view! { cx,  <App/> }
            });
        }
    }
}
