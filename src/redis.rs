use crate::{
    channel::build_channel,
    subscription::inform_subs_on_channel,
    MandelaMsg,
};
use lazy_static::lazy_static;
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use std::env;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

const REDIS_PUBSUB_CHANNEL: &str = "_mandela-pubsub";

fn redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL not found in ENV");

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    return pool;
}

lazy_static! {
    // No need of Mutex here as RedisConnectionManager does the job
    static ref REDIS_POOL: RedisPool = redis_pool();
}

pub async fn init_redis() {
    // publish fails if there are connection problems
    publish_to_redis(String::from("ctrl:Hello")).await;
}

// TODO: msg should be of type MandelaMsgInternal
pub async fn publish_to_redis(msg: String) {

    let rconn_r = REDIS_POOL.get();
    let mut rconn = match rconn_r {
        Ok(c) => c,
        Err(e) => {
            println!("Error getting Redis connection: {}", e);
            eprintln!("Error getting Redis connection: {}", e);
            return;
        }
    };

    println!("Redis: Publishing message: {}", msg.clone());
    let _x: () = rconn.publish(REDIS_PUBSUB_CHANNEL, msg.clone()).unwrap();
}

pub async fn redis_pubsub() {
    //
    let con_r = REDIS_POOL.get();
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
