use futures_util::SinkExt;
use warp::{filters::ws::WebSocket, ws, Filter};

#[tokio::main]
async fn main() {
    let routes = warp::path("viewer")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(move |ws: ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(move |websocket| handle_connection(websocket))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}

async fn handle_connection(mut websocket: WebSocket) {
    websocket
        .send(ws::Message::text("Hello world"))
        .await
        .unwrap();

    websocket
        .send(ws::Message::binary("Hey ho".as_bytes())) // this sends it as blob
        .await
        .unwrap();
}
