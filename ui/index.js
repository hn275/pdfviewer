// open a websocket
const ADDR = `ws://${window.location.host}/viewer`;
console.log("Connecting to", ADDR);

const ws = new WebSocket(ADDR);

ws.onopen = function(_) {
  console.log("Connection OK");
};

const root = document.getElementById("root");
const iframe = document.createElement("iframe");

ws.addEventListener("message", (e) => {
  try {
    const data = e.data;
    console.log("Received message");
    const url = URL.createObjectURL(data);
    iframe.setAttribute("src", url);
    root.innerHTML = ""; // reset root
    root.appendChild(iframe);
  } catch (e) {
    console.error(e);
  }
});
