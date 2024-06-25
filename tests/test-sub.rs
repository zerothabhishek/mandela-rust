use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use mandela_rs::mandela_start;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, WebSocketStream};
use tungstenite::Message;


const CH_CFG1: &str = r##"
  {
    "label": "General"
  }
"##;
const REDIS_URL: &str = "redis://127.0.0.1:6379";
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8082";

const SUB_REQ_MSG: &str = r##"
  {
    "m": {
      "ch": "General#123",
      "t": "Sub"
    },
    "d":""
  }
"##;


type WsStream = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
type WsStreamWriter = SplitSink<WsStream, Message>;

// TODO: move to test-utils file
async fn start_client(client_label: String, tx: mpsc::Sender<String>, num: i32) -> WsStreamWriter {
    let server_url = format!("ws://{}:{}", SERVER_IP, SERVER_PORT);
    println!("[Test] Client connecting to {:?}", server_url);

    let (ws_stream, _res) = connect_async(server_url)
        .await
        .expect("Client can't connect");
    let (ws_write, mut ws_read) = ws_stream.split();

    let client_label1 = client_label.clone();

    let client_receiver = async move {
        let mut count = 0;
        loop {
            let client_label1 = client_label.clone();
            let msg = ws_read.next().await.unwrap().unwrap();
            let msg_s = msg.to_string();

            println!("[Test][{:?}] Received: {:?}", client_label1, msg_s);
            tx.send(msg_s).await.unwrap();

            // TODO: add a timeout based exit too
            // Exit after receiving num messages
            count += 1;
            if count == num {
                break;
            }
        }
    };
    let _client_receiver_handle = tokio::spawn(client_receiver);

    println!("[Test] Connected: {:?}", client_label1.clone());
    return ws_write;
}

#[tokio::test]
async fn test_subs() {
    // Test-1: server starts correctly ---------------------------------
    let (server_tx, mut _server_rx) = mpsc::channel::<String>(32);
    // start the server
    let _server_handle = tokio::spawn(mandela_start(
        vec![CH_CFG1.to_string()],
        REDIS_URL.to_string(),
        SERVER_IP.to_string(),
        SERVER_PORT.to_string(),
        Some(server_tx),
    ));

    // Test-2: single client can subscribe and unsub  ---------------------------
    let (client_tx, mut client_rx) = mpsc::channel::<String>(442);

    let client_tx1 = client_tx.clone();
    let client_handle = tokio::spawn(async move {
        let mut ws_writer = start_client("cl1".to_string(), client_tx1, 1).await;
        ws_writer
            .send(Message::Text(SUB_REQ_MSG.to_string()))
            .await
            .unwrap();
    });

    let mut msgs_received = Vec::new();
    while let Some(msg) = client_rx.recv().await {
        // println!("[Test] GOT client-rx: = {}", msg);
        msgs_received.push(msg);
        break;
    }


    // use the function data_part defined in the module mandela_rs::test_utils
    assert_eq!(mandela_rs::test_utils::data_part(msgs_received[0].clone()), "Subscribed");

    // assert_eq!(mandela_rs::test_utils::data_part(msgs_received[0].clone()), "Subscribed");
    client_handle.await.unwrap();
}
