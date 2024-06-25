

function buildSub(ws, channelName) {
  return {
    ws: ws,
    channelName: channelName,
    sendMsg: (msg) => {
      const msgToSend = {
        m : { ch: channelName, t: 'Data' },
        d: msg
      }
      ws.send(JSON.stringify(msgToSend))
    },
    close: () => {
      const msgToSend = {
        m : { ch: channelName, t: 'UnSub' },
      }
      ws.close();
    }
  }
}

function subscribe({ url, channelName, onMessage }) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(url);

    ws.onopen = () => {
      console.log("Connected to server");
      const reqMsg = {
        m : { ch: channelName, t: 'Sub' },
      }
      ws.send(JSON.stringify(reqMsg))
    };

    ws.onmessage = (event) => {
      // eg: { m: {ch: "Collab#123", t: "Data"}, d: "the-data" }
      const data = JSON.parse(event.data);

      if (!data.m) {
        console.error("Unknown message format", data);
        return;
      }

      switch (data.m.t) {
        case "Data":
          // handle this better: find the sub and pass it along in onMessage
          onMessage(data.d);
          break;
        case "Info":
          if (data.d === "Subscribed") {
            const sub = buildSub(ws, channelName);
            resolve(sub);
          } else {
            console.log("Received Info", data.d);
          }
          break;
        default:
          console.error("Unknown message type", data);
      }
    };

    ws.onerror = (error) => {
      console.error(error);
      reject(error);
    };

    ws.onclose = () => {
      console.log("Disconnected from server");
    };
  });
}


function unSubscribe({ sub, onUnSubscribe }) {
  sub.close();
  onUnSubscribe();
}

const MandelaClient = {
  subscribe,
  unSubscribe
}
export default MandelaClient;
