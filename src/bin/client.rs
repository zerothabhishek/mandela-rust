use tungstenite::{connect, Message};

const MSG1: &str = r##"
  {
    "m": {
      "ch": "Collab#123",
      "t": "Info"
    },
    "d": "Hello Mandela"
  }
"##;

const MSG2: &str = r##"
  {
    "m": {
      "ch": "Collab#123",
      "t": "Sub"
    },
    "d":""
  }
"##;

const MSG3: &str = r##"
  {
    "m": {
      "ch": "Collab#123",
      "t": "Data"
    },
    "d": "Hello Mandela. This is some data"
  }
"##;

const MSG4: &str = r##"
  {
    "m": {
      "ch": "Collab#123",
      "t": "UnSub"
    },
    "d": ""
  }
"##;


// Steps to run
//
// Build first:
// cargo build
//
// Start the server:
// ./target/debug/mandela-server -c samples/channel_configs_files.txt
//
// Run client (this program):
// ./target/debug/client

fn main() {
    let server_url = "ws://127.0.0.1:9001";
    let (mut socket, _response) = connect(server_url).expect("Can't connect");

    println!("Connected to the server");

    println!("Sending INFO msg: {:?}", MSG1);
    socket.send(Message::Text(MSG1.into())).unwrap();

    println!("Sending SUB msg: {:?}", MSG2);
    socket.send(Message::Text(MSG2.into())).unwrap();

    println!("Sending Data msg: {:?}", MSG3);
    socket.send(Message::Text(MSG3.into())).unwrap();

    println!("Sending Unsub msg: {:?}", MSG4);
    socket.send(Message::Text(MSG4.into())).unwrap();

    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
    // socket.close(None);
}
