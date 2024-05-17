use crate::channel::build_channel;
use crate::redis::publish_to_redis;
use crate::subscription::{add_sub_to_sub_store, build_sub_from_msg, find_sub_for_msg};
use crate::{MandelaMsg, MandelaMsgType };
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

type RedisPool = r2d2::Pool<RedisConnectionManager>;
pub type WsWriter = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type ClientHash = Arc<Mutex<HashMap<String, SplitSink<WebSocketStream<TcpStream>, Message>>>>;

// TODO: get String for msg
pub async fn actual_broadcast(msg: Message, clients: ClientHash) {
    println!("Broadcasting: {:?}", msg);
    let mut clients1 = clients.lock().await;
    for (addr, writer) in clients1.iter_mut() {
        println!("Broadcasting: to: {}, msg: {}", addr, msg);
        // Handle error here: by this time the connection could've gone
        match writer.send(msg.clone()).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error sending message: {}", e);
            }
        };
    }
}

async fn send_to_redis1(msg: String, _addr: String, rpool: RedisPool) {
    // let rclient = redis::Client::open("redis://127.0.0.1").unwrap();
    // let mut _conn = rclient.get_connection().unwrap();

    let mut rconn = rpool.get().unwrap();

    // TODO: attach the _addr to the message before publishing
    let _x: () = rconn.publish("mandela-pubsub", msg.clone()).unwrap();
    println!("Published message: {} to mandela-pubsub", msg);
}

async fn handle_ws_read(
    mut read_stream: SplitStream<WebSocketStream<TcpStream>>,
    addr: String,
    clients: ClientHash,
    _rpool: RedisPool,
) -> Result<(), Error> {
    loop {
        let msg = read_stream.next().await;

        match msg {
            None => break,
            Some(Err(e)) => {
                eprintln!("Error reading message: {}", e);
                break;
            }
            Some(Ok(msg1)) => {
                match msg1 {
                    Message::Text(msg2) => {
                        println!("Received a message: {:?}", msg2);
                        // TODO: process_message instead
                        // send_to_redis(msg2.clone(), addr.clone(), rpool.clone()).await;
                        // messages::process_message(msg2.clone()).await;
                    }
                    Message::Close(_) => {
                        // remove the client from the list
                        clients.lock().await.remove(addr.as_str());
                    }
                    _ => {
                        // do nothing
                    }
                }
            }
        }
    }
    Ok(())
}

// pub async fn handle_conn1(
//     stream: TcpStream,
//     clients: ClientHash,
//     rpool: RedisPool,
//     _channel_configs: Vec<channel_configs::ChannelConfig>) -> Result<(), Error> {
//     let ws_stream = tokio_tungstenite::accept_async(stream)
//         .await
//         .expect("Error during websocket handshake");

//     let addr = ws_stream.get_ref().peer_addr()?.to_string();
//     println!("Accepted connection from: {:?}", addr);

//     let (write, read) = ws_stream.split();

//     // TODO: Do this only if its a request for subscription
//     // add the write stream to the list of clients
//     clients.lock().await.insert(addr.clone(), write);

//     let addr_list = clients
//         .lock()
//         .await
//         .keys()
//         .cloned()
//         .collect::<Vec<String>>();
//     println!("Connected clients: {:?}", addr_list);

//     // let mut rconn = rpool.get().unwrap();
//     // let _: () = rconn.publish("mandela-pubsub", "hello").unwrap();

//     // TODO: pass the write stream too, perhaps the whole ws_stream to handle SUB/UNSUB
//     // handle the read stream
//     handle_ws_read(read, addr, clients.clone(), rpool).await?;

//     Ok(())
// }

async fn handle_sub(
    msg: MandelaMsg,
    addr: String,
    write_stream: Arc<Mutex<WsWriter>>,
) {
    // handle sub
    // Add connection to the list of subs, under the channel name
    // Needs the write stream and the ip address
    // - Create a new Subscription object (write stream, channel name, ip address)
    // - Add Subscription to subs
    // - Send back a welcome message

    let sub_op =
        build_sub_from_msg(msg.clone(), addr.clone(), write_stream).await;

    let sub = match sub_op {
        Some(_sub) => _sub,
        None => {
            eprintln!("handle_sub: Coundnt create sub: {:?}", msg);
            return;
        }
    };

    add_sub_to_sub_store(sub.clone()).await;
    // Send welcome message
    let _ = sub.send_text(String::from("Welcome")).await;
}
// let write_stream1 = *write_stream;

async fn handle_unsub(msg: MandelaMsg, addr: String) {
    // handle unsub
    // Find sub associated with the channel and the ip address
    // Remove the subscription from mandela_g.subs
    // Send back a goodbye message

    // let sub = find_sub(mchannel, addr.clone(), mandela_g.subs.clone()).await;
    let sub_op = find_sub_for_msg(msg.clone(), addr.clone()).await;

    let sub = match sub_op {
        Some(_sub) => _sub,
        None => {
            eprintln!("handle_unsub: Sub not found: {:?}", addr);
            return;
        }
    };

    // let sub1 = sub.unwrap();
    // sub.unsub();
    let _ = sub.send_text(String::from("Goodbye")).await;
}

async fn handle_disconnect(_addr: String) {
    // remove the client from the sub list if subscribed
    // we cant tell the channel here, but we have the address
    // clients.lock().await.remove(addr.as_str());
}

async fn handle_data(msg: MandelaMsg, _addr: String) {
    // handle data
    // check if msg has channel info
    // publish on Redis

    // let msg1 = msg.clone();
    // let channel_o = find_channel(msg1.m.ch, msg1.m.id, mandela_g.channel_configs.clone()).await;
    let channel_o = build_channel(msg.clone());
    if channel_o.is_none() {
        eprintln!("Channel not found: {:?}", msg);
        return;
    }

    // TODO: add addr to the message meta
    let msg_json = serde_json::to_string(&msg.clone()).unwrap();
    publish_to_redis(msg_json).await;
}

pub async fn handle_conn(stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during websocket handshake");

    let addr = ws_stream.get_ref().peer_addr().unwrap().to_string();
    println!("Accepted connection from: {:?}", addr);

    let (write_stream, read_stream) = ws_stream.split();
    let mut read_stream = read_stream;

    let write_stream1 = Arc::new(Mutex::new(write_stream));

    loop {
        let data = read_stream.next().await;

        // let mandela_g1 = MandelaGlobal {
        //     channel_configs: mandela_g.channel_configs.clone(),
        //     pool: mandela_g.pool.clone(),
        //     subs: mandela_g.subs.clone()
        // };
        // let mandela_g1 = mandela_g.clone();

        match data {
            None => continue,
            Some(Err(e)) => {
                eprintln!("Error reading message: {}", e);
                continue;
            }
            Some(Ok(data1)) => {
                match data1 {
                    Message::Text(data2) => {
                        println!("Received a message: {:?}", data2);

                        let json = serde_json::from_str(&data2);
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
                                handle_sub(msg, addr.clone(), write_stream1.clone())
                                    .await;
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
                        }
                    }
                    Message::Close(_) => {
                        // remove the client from the sub list if subscribed
                        // we cant tell the channel here, but we have the address
                        handle_disconnect(addr.clone()).await;
                        continue;
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
