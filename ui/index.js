// open a websocket
var ADDR = "ws://127.0.0.1:8081/viewer";
try {
    var ws = new WebSocket(ADDR);
    ws.onopen = function (_) {
        console.log("Ok messaged sent");
    };
    ws.addEventListener("message", function (e) {
        console.log(e.data);
    });
}
catch (e) {
    console.error(e);
}
