//! Web Hooks
//! =========
//!
//! This example runs through how to manage web hooks.
//! To trigger it, you can use the stripe cli.
//!
//! See: https://stripe.com/docs/webhooks/test
//!
//! TLDR;
//! ```
//! stripe listen --forward-to localhost:4242/stripe_webhooks
//! stripe trigger checkout.session.completed
//! ```

use std::net::SocketAddr;

use axum::{routing::post, Json, Router};
use stripe::Event;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/stripe_webhooks", post(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 4242));
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

// basic handler that parses stripe webhooks
async fn root(Json(event): Json<Event>) {
    println!("received event '{}' ({}) ", event.type_, event.id);
}
