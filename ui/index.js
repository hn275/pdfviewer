// open a websocket
const ADDR = `ws://${window.location.host}/viewer`;
console.log("Connecting to", ADDR);

const ws = new WebSocket(ADDR);

ws.onopen = function (_) {
  console.log("Connection OK");
};

const root = document.getElementById("root");
const iframe = document.createElement("iframe");
root.appendChild(iframe);

ws.addEventListener("message", (e) => {
  try {
    const data = e.data;
    console.log("Received message");
    const url = URL.createObjectURL(data);
    iframe.setAttribute("src", url);
  } catch (e) {
    console.error(e);
  }
});
