use futures_util::SinkExt;
use notify::{self, Watcher};
use std::{path::Path, sync::mpsc, thread};
use warp::{filters::ws::WebSocket, ws, Filter};

struct Reloaded;

#[tokio::main]
async fn main() {
    let routes = warp::path("viewer")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(move |ws: ws::Ws| {
            let (tx, rx) = mpsc::channel::<Reloaded>();

            // file watch initialization
            thread::spawn(move || {
                notify::RecommendedWatcher::new(
                    move |event: Result<notify::Event, notify::Error>| {
                        let event = event.unwrap();
                        println!("event emitted");
                        dbg!(&event);
                        tx.send(Reloaded {}).unwrap();
                        use notify::EventKind;
                        match event.kind {
                            EventKind::Modify(msg) => {
                                dbg!(&msg);
                            }
                            _ => {
                                eprintln!("Error");
                            }
                        };
                    },
                    notify::Config::default(),
                )
                .unwrap()
                .watch(
                    Path::new("/home/haln/codes/typst-viewer/test.json"),
                    notify::RecursiveMode::Recursive,
                )
                .unwrap();
            });

            // websocket handling
            ws.on_upgrade(move |websocket| handle_connection(websocket, rx))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}

async fn handle_connection(mut websocket: WebSocket, rx: mpsc::Receiver<Reloaded>) {
    loop {
        while rx.recv().is_ok() {
            websocket
                .send(ws::Message::text("Hello world"))
                .await
                .unwrap();

            websocket
                .send(ws::Message::binary("Hey ho".as_bytes())) // this sends it as blob
                .await
                .unwrap();
        }
    }
}
