use tokio_tungstenite::connect_async;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use tungstenite::Message;


const MSG1: &str = r##"
  {
    "m": {
      "ch": "Collab#123",
      "t": "Info"
    },
    "d": "Hello Mandela"
  }
"##;



#[tokio::main]
async fn main() {

  println!("Starting client2");

  let url = "ws://127.0.0.1:9001";
  let (ws_stream, res) = connect_async(url).await.expect("Error connecting!");
  println!("{:?}", res);

  let (mut ws_write, mut _ws_read) = ws_stream.split();

  println!("Sending INFO msg: {:?}", MSG1);
  // Message::Text(MSG1.into());

  ws_write.send(Message::Text(MSG1.into())).await.unwrap();

  // let msg = ws_read.next().await.unwrap().unwrap();
  // println!("{:?}", msg);
  // ws_write.close(None, None).await.unwrap();
}