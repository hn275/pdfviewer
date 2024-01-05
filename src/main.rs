use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use futures_util::SinkExt;
use std::{
    env, fmt, fs, process,
    sync::{self, Arc},
    thread,
    time::Duration,
};

#[derive(Clone)]
struct Reloaded;

#[tokio::main]
async fn main() {
    // parse arg
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        error_exit("Invalid number of args.");
    }
    let path = args[1].to_owned();
    // file exists?
    fs::metadata(path.as_str()).expect("File not found.");
    println!("Watching: {}", path);

    // watch file
    let (tx, rx) = sync::mpsc::sync_channel::<u8>(1);
    thread::spawn(move || {
        let mut last_modified = fs::metadata(path.as_str())
            .expect("File not found.")
            .modified()
            .expect("Platform not supported.");
        loop {
            thread::sleep(Duration::from_millis(300));
            let file_modified = fs::metadata(path.as_str())
                .expect("File not found.")
                .modified()
                .expect("Platform not supported.");
            if file_modified == last_modified {
                continue;
            }

            // file changed
            // TODO: add a -v tag to print out logs like this
            println!("changes detected.");
            last_modified = file_modified;

            // send signal
            tx.send(1).expect("Failed to send signal");
        }
    });

    let rx = Arc::new(rx);
    // connection
    let app = Router::new()
        .route("/viewer", get(handle_connection))
        .with_state(Arc::clone(&rx));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    return axum::serve(listener, app).await.unwrap();
}

async fn handle_connection(ws: WebSocketUpgrade) -> Response {
    return ws.on_upgrade(handle_socket);
}

async fn handle_socket(mut socket: WebSocket) {
    println!("New connection detected!");
    // NOTE: since at the file has exists for this function to be called,
    // `unwrap()` is ok for file existence

    // parse arg
    let args = env::args().collect::<Vec<String>>();
    let path = args[1].as_str();

    // read file
    let mut last_modified = fs::metadata(path)
        .unwrap()
        .modified()
        .expect("Platform not supported");

    let file = fs::read(path).unwrap();
    if let Err(err) = socket.send(Message::Binary(file)).await {
        error_exit(err);
        return;
    }
}

fn error_exit(err: impl fmt::Display) {
    eprintln!("{}", err);
    process::exit(1);
}
