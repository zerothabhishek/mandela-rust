use std::time::Duration;

use mandela_rs::mandela_start;
use tokio::sync::mpsc;

const CH_CFG1: &str = r##"
  {
    "label": "General"
  }
"##;
const REDIS_URL: &str = "redis://127.0.0.1:6379";
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8082";

#[tokio::test]
async fn test_server_start() {
    // Test-1: server starts correctly ---------------------------------
    let (server_tx, mut server_rx) = mpsc::channel::<String>(32);

    // start the server
    let _server_handle = tokio::spawn(mandela_start(
        vec![CH_CFG1.to_string()],
        REDIS_URL.to_string(),
        SERVER_IP.to_string(),
        SERVER_PORT.to_string(),
        Some(server_tx),
    ));

    // FIXME: for some reason this hangs if its > 200ms
    // Maybe conduct the check in the main-thread
    tokio::time::sleep(Duration::from_millis(100)).await;

    let answer = server_rx.try_recv();

    let server_started = match answer {
        Ok(_msg) => true,
        _ => false, // Empty or Disconnected: means server hasn't sent anything yet
    };
    println!("[Test] server_started: {:?}", server_started);
    assert!(server_started);

    // server_handle.await.unwrap();
}
