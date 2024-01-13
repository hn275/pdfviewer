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
use dirs;
use std::{fmt, fs, io, process, thread};
use tower_http::services::ServeDir;

mod cli;

#[derive(Clone)]
struct Reloaded(bool);

#[derive(Clone)]
struct AppState {
    chan: crossbeam_channel::Receiver<Reloaded>,
    cli: cli::Arg,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli_args = cli::Arg::new()?;

    let path = cli_args.file().to_owned().clone();
    // file exists?
    fs::metadata(&path).expect("File not found.");
    println!("Watching file: {}", path);

    // WATCH FILE
    // channels and file paths
    let (tx, rx) = crossbeam_channel::unbounded();
    let cli = cli_args.clone();

    // polling in a separate thread
    thread::spawn(move || {
        let path = path.to_string();
        let mut last_modified = fs::metadata(&cli.file())
            .expect("File not found.")
            .modified()
            .expect("Platform not supported.");

        loop {
            thread::sleep(cli.polling_duration());
            let file_modified = fs::metadata(path.as_str())
                .expect("File not found.")
                .modified()
                .expect("Platform not supported.");

            if file_modified == last_modified {
                continue;
            }

            // file changed
            cli.write_stdin("changes detected.");
            last_modified = file_modified;

            // send signal
            tx.send(Reloaded(true)).expect("Failed to send signal");
        }
    });

    // CONNECTION
    let app_state = AppState {
        chan: rx,
        cli: cli_args.clone(),
    };

    // static assets
    let mut static_dir = dirs::home_dir().expect("env `$HOME` not found");
    static_dir.push(".pdfviewer");
    static_dir.push("ui");

    let app = Router::new()
        .nest_service("/", ServeDir::new(static_dir)) // serving the ui to the browser
        .route("/viewer", get(handle_ws_conn))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(cli_args.host()).await?;

    // OPEN BROWSER
    let full_addr = format!("http://localhost:{}", &cli_args.port());
    open::that(&full_addr)?;
    println!("Serving at: {full_addr}");

    return axum::serve(listener, app).await;
}

async fn handle_ws_conn(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    return ws.on_upgrade(move |soc| async {
        process_socket(soc, state).await;
    });
}

async fn process_socket(mut socket: WebSocket, state: AppState) {
    state.cli.write_stdin("new connection detected");
    // send files when client is connected
    let file = fs::read(&state.cli.file()).unwrap();
    if let Err(err) = socket.send(Message::Binary(file)).await {
        error_exit(err);
        return;
    }

    // send files when signal is received
    while state.chan.recv().is_ok() {
        let file = fs::read(&state.cli.file()).unwrap();
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
