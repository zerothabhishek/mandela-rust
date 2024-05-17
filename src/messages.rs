use crate::{
    channel_configs::{self, ChannelConfig},
    MandelaChannel,
};

// TODO: remove 
pub async fn find_channel_x(
    ch: String,
    id: String,
    _channel_configs: Vec<ChannelConfig>,
) -> Option<MandelaChannel> {
    //

    let channel_configs = read_channel_configs_x();

    let exists = channel_configs.iter().any(|ch_cfg| ch_cfg.label == ch);
    if !exists {
        return None;
    }
    Some(MandelaChannel { ch, id })
}

fn read_channel_configs_x() -> Vec<ChannelConfig> {
    let files = vec!["./channel_configs/collab.json".to_string()];
    return channel_configs::read_channel_configs(files);
}

// lazy_static! {
//     static ref CHANNEL_CONFIGS_CACHE: Mutex<Option<Vec<ChannelConfig>>> = Mutex::new(None);
// }

// fn read_channel_configs() -> Vec<ChannelConfig> {
//     let mut cache = CHANNEL_CONFIGS_CACHE.lock().unwrap();
//     if cache.is_none() {
//         let files = vec!["./channel_configs/collab.json".to_string()];
//         *cache = Some(channel_configs::read_channel_configs(files));
//     }
//     cache.as_ref().unwrap().clone()
// }

// pub async fn process_message(msg: String) {
//   let res = serde_json::from_str(&msg);
//   let msg: MandeleMsg = match res {
//     Ok(msg) => msg,
//     Err(e) => {
//       println!("Message cant be parsed: {}", e);
//       return;
//     }
//   };

//   // TODO: find channel from msg, fail if not found
//   //
//   let mch = match find_channel(msg.m) {
//     Ok(mch) => mch,
//     Err(e) => {
//       println!("Channel not found: {}", e);
//       return;
//     }
//   };

//   match msg.m.t {
//     MandelaMsgType::SUB => {
//       println!("Received a SUB message: {:?}", msg);
//       // handle sub
//       // Add connection to the list of subs, under the channel name
//       // Needs the write stream and the ip address
//       // - Create a new Subscription object (write stream, channel name, ip address)
//       // - Add Subscription to ClientHash
//       // - Send back a welcome message
//     }
//     MandelaMsgType::UNSUB => {
//       println!("Received a UNSUB message: {:?}", msg);
//       // Remove connection from the list of subs, under the channel name
//       // - Remove related (Subscription object) from ClientHash
//       // - Send back a goodbye message
//     }
//     MandelaMsgType::HERE_NOW => {
//       println!("Received a HERE_NOW message: {:?}", msg);
//       // Do nothing
//     }
//     MandelaMsgType::DATA => {
//       println!("Received a DATA message: {:?}", msg);
//       // send to Redis for broadcasting / add custom handling
//       // - Find the channel
//       // - Figure out the associated Subscriptions
//       // - Send message to all the Subscriptions
//     }
//   }
// }
