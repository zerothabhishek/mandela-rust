use crate::{
    channel::build_channel,
    subscription::inform_subs_on_channel,
    MandelaMsg,
};
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use std::sync::OnceLock;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

const REDIS_PUBSUB_CHANNEL: &str = "_mandela-pubsub";
const REDIS_URL_DEFAULT : &str = "redis://127.0.0.1:6379";


static REDIS_POOL_CELL: OnceLock<RedisPool> = OnceLock::new();

fn build_redis_pool(redis_url: String) -> RedisPool {
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    return pool;
}

pub async fn init_redis(redis_url: String) {
    // publish fails if there are connection problems
    let _pool = REDIS_POOL_CELL.get_or_init(|| build_redis_pool(redis_url) );
    publish_to_redis(String::from("ctrl:Hello")).await;
}

// TODO: msg should be of type MandelaMsgInternal
pub async fn publish_to_redis(msg: String) {

    let pool = REDIS_POOL_CELL.get().unwrap();
    let rconn_r = pool.get();
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
    let pool = REDIS_POOL_CELL.get().unwrap();
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
