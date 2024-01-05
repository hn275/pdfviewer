// open a websocket
const ADDR = "ws://127.0.0.1:8081/viewer";
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
    const url = URL.createObjectURL(data);
    iframe.setAttribute("src", url);
    root.innerHTML= ""; // reset root
    root.appendChild(iframe)
  } catch (e) {
    console.error(e);
  }
});
