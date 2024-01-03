// open a websocket
const ADDR: string = "ws://127.0.0.1:8081/viewer";
try {
  const ws = new WebSocket(ADDR);

  ws.onopen = function(_) {
    console.log("Ok messaged sent");
  };

  ws.addEventListener("message", (e) => {
    console.log(e.data);
  });
} catch (e) {
  console.error(e);
}
