#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod channel;
mod channel_configs;
mod connection;
mod messages;
mod redis;
mod subscription;
pub mod test_utils;

use connection::handle_conn;
use redis::{init_redis, redis_pubsub};
use subscription::SubscriptionsHashArc;

pub type ChannelName = String; // label#id

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MandelaMsgType {
    Sub,
    UnSub,
    HereNow,
    Data,
    Info,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandelaMsgMeta {
    // pub ch: String,
    // pub id: String,
    pub ch: ChannelName,
    pub t: MandelaMsgType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandelaMsg {
    pub m: MandelaMsgMeta,
    pub d: Option<String>,
}

pub struct MandelaMsgInternal {
    pub m: MandelaMsgMeta,
    // d can be String or none
    pub d: Option<String>,
    // pub d: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MandelaChannel {
    // pub ch: String,
    // pub id: String,
    pub ch: ChannelName, // label#id
}

#[derive(Clone)]
struct MandelaGlobal {
    channel_configs: Vec<channel_configs::ChannelConfig>,
    pool: redis::RedisPool,
    subs: SubscriptionsHashArc,
}


pub async fn mandela_start(
    channel_config_jsons: Vec<String>,
    redis_url: String,
    server_ip: String,
    server_port: String,
    callback_tx: Option<tokio::sync::mpsc::Sender<String>>
) {
    dotenvy::dotenv().expect("Failed to load ENV");

    init_redis(redis_url).await;
    tokio::spawn(redis_pubsub()); // Problem here: blocks the mpsc communication

    // let files = channel_config_files;
    channel_configs::init_channel_configs(channel_config_jsons).await;

    // Start the TCP listener
    // let server_ip = env::var("SERVER_IP").expect("SERVER_IP not found in ENV");
    // let server_port = env::var("SERVER_PORT").expect("SERVER_PORT not found in ENV");
    let ip_and_port = format!("{}:{}", server_ip, server_port);
    let server = TcpListener::bind(ip_and_port.clone()).await.unwrap();
    println!("Listening on ws://{}", ip_and_port);

    if callback_tx.is_some() {
        let tx = callback_tx.unwrap();
        if let Err(e) = tx.send("Started".to_string()).await {
            println!("Failed to send message on tx: {}", e);
        };
    }

    // TODO: handle KILL signal to shutdown gracefully
    loop {
        // tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        let (stream, _) = server.accept().await.unwrap(); // TODO: handle error
        tokio::spawn(handle_conn(stream));
    }
}

pub async fn read_channel_config_superfile(path: String) -> Vec<String> {
    return channel_configs::read_channel_config_superfile(path).await;
}
