use futures_util::SinkExt;
use std::{env, fmt, fs, process, thread::sleep, time::Duration};
use warp::{filters::ws::WebSocket, ws, Filter};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // parse arg
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        error_exit("Invalid number of args.");
    }
    let path = args[1].as_str();
    // file exists?
    fs::metadata(path).expect("File not found.");
    println!("Watching: {}", path);

    // connection
    let routes = warp::path("viewer")
        .and(warp::ws()) // The `ws()` filter will prepare the Websocket handshake.
        .map(move |ws: ws::Ws| ws.on_upgrade(move |websocket| handle_connection(websocket)));

    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
    return Ok(());
}

async fn handle_connection(mut websocket: WebSocket) {
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
    if let Err(err) = websocket.send(ws::Message::binary(file)).await {
        error_exit(err);
        return;
    }

    // pooling for file changes
    loop {
        let file_metadata = fs::metadata(path).unwrap();
        let modified_at = file_metadata.modified().expect("Platform is not supported");

        // no change detected
        if modified_at == last_modified {
            sleep(Duration::from_millis(100)); // TODO: add cli args for this timeout, default to 100 ms
            continue;
        }

        // file changed
        // TODO: add a -v tag to print out logs like this
        println!("changes detected.");
        last_modified = modified_at;

        // send data
        let file = fs::read(path).expect("File not found.");
        if let Err(err) = websocket.send(ws::Message::binary(file)).await {
            error_exit(err);
            return;
        }
    }
}

fn error_exit(err: impl fmt::Display) {
    eprintln!("{}", err);
    process::exit(1);
}
