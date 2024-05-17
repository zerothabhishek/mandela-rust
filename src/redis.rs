use crate::{
    channel::build_channel,
    subscription::inform_subs_on_channel,
    MandelaMsg,
};
use lazy_static::lazy_static;
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use tokio::sync::Mutex;
// use serde::{Deserialize, Serialize};
// use tokio_tungstenite::tungstenite::Message;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

const REDIS_PUBSUB_CHANNEL: &str = "_mandela-pubsub";
const REDIS_URL: &str = "redis://127.0.0.1"; // TODO: get from ENV

fn redis_pool() -> RedisPool {
    // TODO: error handling
    let manager = RedisConnectionManager::new(REDIS_URL).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    return pool;
}

lazy_static! {
    static ref REDIS_POOL: Mutex<RedisPool> = Mutex::new(redis_pool());
}

pub async fn init_redis() {
    // publish fails if there are connection problems
    publish_to_redis(String::from("ctrl:Hello")).await;
}

// TODO: msg should be of type MandelaMsgInternal
pub async fn publish_to_redis(msg: String) {
    let rpool = REDIS_POOL.lock().await;
    let rconn_r = rpool.get();
    let mut rconn = match rconn_r {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error getting Redis connection: {}", e);
            return;
        }
    };

    let _x: () = rconn.publish(REDIS_PUBSUB_CHANNEL, msg.clone()).unwrap();
    println!("Published message: {} to {}", msg, REDIS_PUBSUB_CHANNEL);
}

pub async fn redis_pubsub() {
    //
    // let rclient = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut con = rclient.get_connection().unwrap();
    // let pool = mandela_g.pool.clone();

    let pool = REDIS_POOL.lock().await;
    let con_r = pool.get();
    let mut con = match con_r {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error getting Redis connection: {}", e);
            return;
        }
    };

    let mut pubsub = con.as_pubsub();

    pubsub.subscribe(REDIS_PUBSUB_CHANNEL).unwrap();
    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);

        // TODO: don't inform ctrl messages
        // TODO: do this in a separate thread 
        inform(payload).await;

        // Doesn't work correctly with tokio::spawn
        // actual_broadcast(Message::Text(payload), clients.clone()).await;
        // tokio::spawn(actual_broadcast(
        //     Message::Text(payload),
        //     clients.clone(),
        // ));
    }
}

async fn inform(msg: String) {
    // Parse msg as MandelaMsg
    // Find channel from msg
    // Find Subscriptions for channel using mandela_g.subs
    // Send message to all Subscriptions
    
    // TODO: msg_r should be of type MandelaMsgInternal
    let msg_r = serde_json::from_str(&msg);
    let mandela_msg: MandelaMsg = match msg_r {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing message: {}", e);
            return;
        }
    };

    // let channel_op = find_channel(mandela_msg.m.ch, mandela_msg.m.id, mandela_g.channel_configs.clone()).await;
    let channel_op = build_channel(mandela_msg).clone();
    let channel = match channel_op {
        Some(ch) => ch,
        None => {
            println!("Channel not found: {}", msg);
            return;
        }
    };

    inform_subs_on_channel(msg.clone(), channel).await;
}

// async fn inform_subs_on_channel(msg: String, channel: MandelaChannel) {
//     // let subs_h = subs.lock().await;
//     // let subs_for_channel_op = subs_h.get(&channel.ch);
//     // let subs_for_channel = match subs_for_channel_op {
//     //     Some(s) => s,
//     //     None => {
//     //         println!("No subscriptions found for channel: {}", channel.ch);
//     //         return;
//     //     }
//     // };
//     let subs_for_channel = find_subs_for_channel(channel)
//     for sub in subs_for_channel {
//         let _ = sub.send_text(msg.clone()).await;
//         // TODO: handle error
//     }
// }

// pub async fn new_redis_pool() -> RedisPool {
//     let manager = RedisConnectionManager::new(REDIS_URL).unwrap();
//     let pool = r2d2::Pool::builder().build(manager).unwrap();
//     pool
// }
