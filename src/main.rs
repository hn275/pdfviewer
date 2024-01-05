use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use crossbeam_channel;
use std::{env, fmt, fs, process, sync::Arc, thread, time::Duration};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct Reloaded(bool);

#[derive(Clone)]
struct AppState {
    path: String,
    chan: crossbeam_channel::Receiver<Reloaded>,
}

const POLLING_DELAY: Duration = Duration::from_millis(300);
const ADDR: &'static str = "0.0.0.0:8080";

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
    println!("Watching file: {}", path);

    // WATCH FILE
    // channels and file paths
    let (tx, rx) = crossbeam_channel::unbounded();
    let file_path = Arc::new(path);
    let path = Arc::clone(&file_path);
    // polling
    thread::spawn(move || {
        let path = path.to_string();
        let mut last_modified = fs::metadata(path.as_str())
            .expect("File not found.")
            .modified()
            .expect("Platform not supported.");
        loop {
            thread::sleep(POLLING_DELAY);
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
            tx.send(Reloaded(true)).expect("Failed to send signal");
        }
    });

    // CONNECTION
    let app_state = AppState {
        path: file_path.to_string(),
        chan: rx,
    };
    let app = Router::new()
        .nest_service("/", ServeDir::new("ui"))
        .route("/viewer", get(handle_connection))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();

    // OPEN BROWSER
    let mut full_addr = "http://".to_string();
    full_addr.push_str(ADDR);
    open::that(full_addr).unwrap();

    return axum::serve(listener, app).await.unwrap();
}

async fn handle_connection(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    return ws.on_upgrade(move |soc| async {
        handle_socket(soc, state).await;
    });
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    println!("New connection detected!");
    // send files when connection is created
    let file = fs::read(&state.path).unwrap();
    if let Err(err) = socket.send(Message::Binary(file)).await {
        error_exit(err);
        return;
    }

    while state.chan.recv().is_ok() {
        let file = fs::read(&state.path).unwrap();
        if let Err(err) = socket.send(Message::Binary(file)).await {
            error_exit(err);
            return;
        }
    }
}

fn error_exit(err: impl fmt::Display) {
    eprintln!("{}", err);
    process::exit(1);
}
