#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::vec;
use tokio::net::TcpListener;
use std::env;

mod channel;
mod channel_configs;
mod connection;
mod messages;
mod redis;
mod subscription;

use connection::handle_conn;
use redis::{init_redis, redis_pubsub};
use subscription::SubscriptionsHashArc;

pub type ChannelName = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MandelaMsgType {
    Sub,
    UnSub,
    HereNow,
    Data,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandelaMsgMeta {
    pub ch: String,
    pub id: String,
    pub t: MandelaMsgType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandelaMsg {
    pub m: MandelaMsgMeta,
    pub d: String,
}

pub struct MandelaMsgInternal {
    pub m: MandelaMsgMeta,
    pub d: String,
}

#[derive(Clone, Debug)]
pub struct MandelaChannel {
    pub ch: String,
    pub id: String,
}

#[derive(Clone)]
struct MandelaGlobal {
    channel_configs: Vec<channel_configs::ChannelConfig>,
    pool: redis::RedisPool,
    subs: SubscriptionsHashArc,
}

#[tokio::main]
async fn main() {

    // Load environment variables
    dotenvy::dotenv().expect("Failed to load ENV");


    // Initialize Redis
    // Checks and panics if needed
    init_redis().await;
    // Start the Redis pub-sub listener in a separate task
    tokio::spawn(redis_pubsub());


    // Initialize Channel Configs
    // TODO: Get the file list as config or input to main
    let files = vec!["./channel_configs/collab.json".to_string()];
    channel_configs::init_channel_configs(files).await;


    // Start the TCP listener
    let server_ip = env::var("SERVER_IP").expect("SERVER_IP not found in ENV");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT not found in ENV");
    let ip_and_port = format!("{}:{}", server_ip, server_port);
    let server = TcpListener::bind(ip_and_port.clone()).await.unwrap();
    println!("Listening on ws://{}", ip_and_port);


    while let Ok((stream, _)) = server.accept().await {
        // Spawn a task to handle the connection
        tokio::spawn(handle_conn(stream));
    }
    // Ok(())
}
