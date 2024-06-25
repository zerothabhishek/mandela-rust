import MandelaClient from "/MandelaClient.js";

export async function chatDemo() {

  console.log("chatDemo !!!")

  const connectButton = document.getElementById("connect");
  const connectionStatusEl = document.getElementById("connection-status");
  const sendMsgButton = document.getElementById("send-msg");
  const dataEl = document.getElementById("data");
  const unSubscribeButton = document.getElementById("unsubscribe");

  const channelName = "Collab#123"
  const url = `ws://localhost:9001`;

  function onSubscribe(sub) {
    connectionStatusEl.innerText = "🟢";

    sendMsgButton.removeAttribute("disabled");
    sendMsgButton.addEventListener("click", (event) => {
      event.preventDefault();
      const inputEl = document.getElementById("msg-to-send");
      const msgToSend = inputEl.value;
      sub.sendMsg(msgToSend);
      inputEl.value = ""
    });

    unSubscribeButton.removeAttribute("disabled");
    unSubscribeButton.addEventListener("click", (event) => {
      event.preventDefault();
      MandelaClient.unSubscribe({ sub, onUnSubscribe });
    });
  }

  function onUnSubscribe() {
    connectionStatusEl.innerText = "⚪️";
    sendMsgButton.setAttribute("disabled", true);
    unSubscribeButton.setAttribute("disabled", true);
  }

  function onMessage(msg) { // its the data.d here
    console.log(msg)
    dataEl.innerHTML = (dataEl.innerHTML || "") + msg + "<br>";
  }

  connectButton.addEventListener("click", async (event) => {
    console.log("connectButton!")
    event.preventDefault();

    const sub = await MandelaClient.subscribe({ url, channelName, onMessage });
    onSubscribe(sub);
  });
}

document.addEventListener("DOMContentLoaded", chatDemo);