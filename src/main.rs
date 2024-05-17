#![allow(dead_code)]

use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::vec;
use tokio::net::TcpListener;
// use tokio::sync::Mutex;

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

const IP_AND_PORT: &str = "127.0.0.1:9001";

#[tokio::main]
async fn main() {
    // Checks and panics if needed
    init_redis().await;

    // ws_broadcast().await.unwrap();
    let server = TcpListener::bind(IP_AND_PORT).await.unwrap();
    println!("Listening on ws://${}", IP_AND_PORT);

    // TODO: remove
    // let _clients: ClientHash = Arc::new(Mutex::new(HashMap::new()));

    // TODO: Get the file list as config or input to main
    let files = vec!["./channel_configs/collab.json".to_string()];
    // let channel_configs = channel_configs::read_channel_configs(files);
    channel_configs::init_channel_configs(files).await;

    // let subs_fake = Arc::new(Mutex::new(SubscriptionsHash::new()));
    // let pool = new_redis_pool().await;

    // let ch_cfg_fake = channel_configs::ChannelConfig {
    //     label: "test".to_string(),
    // };
    // let channel_configs_fake = vec![ch_cfg_fake];

    // let mg = MandelaGlobal {
    //     channel_configs: channel_configs_fake,
    //     pool,
    //     subs: subs_fake,
    // };

    // Start the Redis pub-sub listener in a separate task
    tokio::spawn(redis_pubsub());

    while let Ok((stream, _)) = server.accept().await {
        // Spawn a task to handle the connection
        tokio::spawn(handle_conn(stream));
    }

    // Ok(())
}
