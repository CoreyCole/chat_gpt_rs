use cfg_if::cfg_if;
use leptos::*;

// boilerplate to run in different modes
cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{
        Router,
        routing::get,
        extract::Extension,
    };
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::sync::Arc;
    use chat_gpt_rs::fallback::file_and_error_handler;

    #[tokio::main]
    async fn main() {
        use chat_gpt_rs::*;
        chat_gpt_rs::api::register_server_functions();

        let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_address.clone();
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

        // build our application with a route
        let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> } )
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

    // client-only main for Trunk
    else {
        pub fn main() {
            // cannot work in a Client-Side-Rendered only
            // app as a server is required to maintain state
        }
    }
}