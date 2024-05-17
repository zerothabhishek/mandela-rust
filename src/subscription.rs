use crate::channel::build_channel;
// use crate::channel_configs::find_channel_config;
use crate::connection;
use crate::{ChannelName, MandelaChannel, MandelaMsg};
use core::fmt::Error;
use futures_util::SinkExt;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Clone, Debug)]
pub struct Subscription {
    pub channel: MandelaChannel,
    pub writer: Arc<Mutex<connection::WsWriter>>,
    pub ip: String, // rename to ip_port
}

pub type SubscriptionsHash = HashMap<ChannelName, Vec<Subscription>>;
pub type SubscriptionsHashArc = Arc<Mutex<SubscriptionsHash>>;

// TODO: implement with OneCell/OneLock
lazy_static! {
    static ref SUBS_STORE: Mutex<SubscriptionsHash> = Mutex::new(HashMap::new());
}

impl Subscription {
    // implement send text for Subsciprtion
    pub async fn send_text(&self, text: String) -> Result<(), Error> {
        let msg = Message::Text(text);
        let mut writer1 = self.writer.lock().await;
        let res = writer1.send(msg).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error sending message: {}", e);
            }
        }
        Result::Ok(())
    }
}

pub async fn add_sub_to_sub_store(sub: Subscription) {
    // let mut subs_safe = sub_store.lock().await;
    let mut subs_store = SUBS_STORE.lock().await;

    let ch = sub.channel.ch.clone();

    let sub_arr = subs_store.entry(ch).or_insert(vec![]);
    sub_arr.push(sub.clone());
}

pub async fn build_sub(
    mchannel: MandelaChannel,
    addr: String,
    writer: Arc<Mutex<connection::WsWriter>>,
) -> Subscription {
    Subscription {
        channel: mchannel,
        writer: writer,
        ip: addr,
    }
}

pub async fn build_sub_from_msg(
    msg: MandelaMsg,
    addr: String,
    writer: Arc<Mutex<connection::WsWriter>>,
) -> Option<Subscription> {
    let mchannel = build_channel(msg.clone());

    if mchannel.is_none() {
        eprintln!("build_sub: Channel not found: {:?}", msg.clone());
        return None;
    }

    Some(Subscription {
        channel: mchannel.unwrap(),
        writer,
        ip: addr,
    })
}

pub async fn inform_subs_on_channel(msg: String, channel: MandelaChannel) {
    let subs_for_channel = find_subs_for_channel(channel).await;
    for sub in subs_for_channel {
        let _ = sub.send_text(msg.clone()).await;
        // TODO: handle error
    }
}

pub async fn find_subs_for_channel(mchannel: MandelaChannel) -> Vec<Subscription> {
    let sub_store = SUBS_STORE.lock().await;
    let subs_arr_ = sub_store.get(&mchannel.ch);

    if subs_arr_.is_none() {
        return vec![];
    }
    return subs_arr_.unwrap().clone();
}

pub async fn find_sub(
    mchannel: MandelaChannel,
    addr: String, // Ip address and port
) -> Option<Subscription> {
    // let sub_store = SUBS_STORE.lock().await;
    // let subs_arr_ = sub_store.get(&mchannel.ch);
    let subs_arr = find_subs_for_channel(mchannel).await;
    let sub_ = subs_arr.iter().find(|s| s.ip == addr);
    if sub_.is_none() {
        return None;
    }
    let sub = sub_.unwrap();
    return Some(sub.clone());
}

pub async fn find_sub_for_msg(msg: MandelaMsg, addr: String) -> Option<Subscription> {
    let mchannel_ = build_channel(msg.clone());

    let mchannel = match mchannel_ {
        Some(_mchannel) => _mchannel,
        None => {
            eprintln!("find_sub_for_msg: Channel not found: {:?}", msg);
            return None;
        }
    };

    find_sub(mchannel, addr).await
}

pub async fn unsub(sub: Subscription, sub_store: SubscriptionsHashArc) {
    let mut subs_safe = sub_store.lock().await;
    let subs_arr = subs_safe.get(&sub.channel.ch);
    if subs_arr.is_none() {
        return;
    }

    let subs = subs_arr.unwrap();
    // TODO: remove  sub from subs
    let subs_new = subs
        .iter()
        .filter(|s| s.ip != sub.ip)
        .cloned()
        .collect::<Vec<Subscription>>();
    subs_safe
        .entry(sub.channel.ch)
        .and_modify(|subs| *subs = subs_new);
}
