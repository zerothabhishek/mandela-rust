use crate::channel::build_channel;
use crate::redis::publish_to_redis;
use crate::subscription::{
    add_sub_to_sub_store, build_sub_from_msg, find_sub_for_msg, remove_subs_for_ip_addr,
};
use crate::{MandelaMsg, MandelaMsgType};
use futures_util::stream::SplitSink;
use futures_util::StreamExt;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

type RedisPool = r2d2::Pool<RedisConnectionManager>;
pub type WsWriter = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type ClientHash = Arc<Mutex<HashMap<String, SplitSink<WebSocketStream<TcpStream>, Message>>>>;

// TODO: get String for msg
// pub async fn actual_broadcast(msg: Message, clients: ClientHash) {
//     println!("Broadcasting: {:?}", msg);
//     let mut clients1 = clients.lock().await;
//     for (addr, writer) in clients1.iter_mut() {
//         println!("Broadcasting: to: {}, msg: {}", addr, msg);
//         // Handle error here: by this time the connection could've gone
//         match writer.send(msg.clone()).await {
//             Ok(_) => {}
//             Err(e) => {
//                 eprintln!("Error sending message: {}", e);
//             }
//         };
//     }
// }

async fn handle_sub(msg: MandelaMsg, addr: String, write_stream: Arc<Mutex<WsWriter>>) {
    // handle sub
    // Add connection to the list of subs, under the channel name
    // Needs the write stream and the ip address
    // - Create a new Subscription object (write stream, channel name, ip address)
    // - Add Subscription to subs
    // - Send back a welcome message

    let sub_op = build_sub_from_msg(msg.clone(), addr.clone(), write_stream).await;

    let sub = match sub_op {
        Some(_sub) => _sub,
        None => {
            eprintln!("handle_sub: Coundnt create sub: {:?}", msg);
            return;
        }
    };

    add_sub_to_sub_store(sub.clone()).await;
    // Send welcome message
    let _ = sub.send_info(String::from("Subscribed")).await;
    println!("Subscribed: {:?}", addr);
}

async fn handle_unsub(msg: MandelaMsg, addr: String) {
    // handle unsub
    // Find sub associated with the channel and the ip address
    // Remove the subscription from mandela_g.subs
    // Send back a goodbye message

    let sub_op = find_sub_for_msg(msg.clone(), addr.clone()).await;

    let sub = match sub_op {
        Some(_sub) => _sub,
        None => {
            eprintln!("handle_unsub: Sub not found: {:?}", addr);
            return;
        }
    };

    let _ = sub.send_text(String::from("Goodbye")).await;
}

async fn handle_disconnect(addr: String) {
    // TODO: remove the client from the sub list if subscribed
    // we cant tell the channel here, but we have the address
    // clients.lock().await.remove(addr.as_str());
    remove_subs_for_ip_addr(addr.clone()).await;
    println!("Disconnected: {:?}", addr);
}

async fn handle_data(msg: MandelaMsg, _addr: String) {
    // handle data
    // check if msg has channel info
    // publish on Redis

    let channel_o = build_channel(msg.clone());
    if channel_o.is_none() {
        eprintln!("Channel not found: {:?}", msg);
        return;
    }

    // TODO: add addr to the message meta
    let msg_json = serde_json::to_string(&msg.clone()).unwrap();
    println!("Sending msg JSON to Redis: {:?}", msg_json);
    publish_to_redis(msg_json).await;
}

async fn handle_here_now(_msg: MandelaMsg) {
    // Let Redis hold a list of connections (addrs)
    // SUB => add to the redis-connection-list
    // UNSUB => remove from redis-connection-list
    // DISCONNECT => remove from redis-connection-list

    // Garbage-removal:
    // - In a separate task (thread)
    // - Iterate over connections in SUBS_STORE;
    // - remove those that are not responding
}

pub async fn handle_conn(stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during websocket handshake");

    let addr = ws_stream.get_ref().peer_addr().unwrap().to_string();
    println!("Accepted connection from: {:?}", addr);

    let (write_stream, read_stream) = ws_stream.split();
    let mut read_stream = read_stream;

    let write_stream_arc = Arc::new(Mutex::new(write_stream));

    loop {
        let ws_msg_ = read_stream.next().await;

        match ws_msg_ {
            None => continue,
            Some(Err(e)) => {
                eprintln!("Error reading message: {}", e);
                continue;
            }
            Some(Ok(ws_msg)) => {
                match ws_msg {
                    Message::Text(ws_msg_str) => {
                        println!("Received a message: {:?}", ws_msg_str);
                        let json = serde_json::from_str(&ws_msg_str);

                        let msg: MandelaMsg = match json {
                            Ok(msg) => msg,
                            Err(e) => {
                                println!("Message cant be parsed: {}", e);
                                continue;
                            }
                        };

                        match msg.m.t {
                            MandelaMsgType::Sub => {
                                println!("Received a SUB message: {:?}", msg);
                                handle_sub(msg, addr.clone(), write_stream_arc.clone()).await;
                                continue;
                            }
                            MandelaMsgType::UnSub => {
                                println!("Received a UNSUB message: {:?}", msg);
                                handle_unsub(msg, addr.clone()).await;
                                continue;
                            }
                            MandelaMsgType::HereNow => {
                                println!("Received a HERE_NOW message: {:?}", msg);
                                // TODO: implement
                                continue;
                            }
                            MandelaMsgType::Data => {
                                println!("Received a DATA message: {:?}", msg);
                                handle_data(msg, addr.clone()).await;
                                continue;
                            }
                            MandelaMsgType::Info => {
                                println!("Received a INFO message: {:?}", msg);
                                continue;
                            } // _ => {
                              //     println!("Received an unknown message: {:?}", msg);
                              //     continue;
                              // }
                        }
                    }
                    Message::Close(_) => {
                        // remove the client from the sub list if subscribed
                        // we cant tell the channel here, but we have the address
                        println!("Received a CLOSE message: {:?}", addr);
                        handle_disconnect(addr.clone()).await;
                        break;
                        // clients.lock().await.remove(addr.as_str());
                    }
                    _ => {
                        // do nothing
                    }
                } // end of match data1
            } // end of Some data1
        }; // end of match data
    } // end of loop
}
