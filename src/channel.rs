use crate::channel_configs::find_channel_config;
use crate::{MandelaChannel, MandelaMsg};

pub fn build_channel(msg: MandelaMsg) -> Option<MandelaChannel> {
    let msg1 = msg.clone();
    let ch_name = msg1.m.ch;

    let ch_name_split: Vec<&str> = ch_name.split('#').collect();
    let label = ch_name_split[0].to_string();
    let _id = ch_name_split[1].to_string();

    let ch_cfg = find_channel_config(label.clone());
    if ch_cfg.is_none() {
        return None;
    }
    Some(MandelaChannel { ch: ch_name })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        channel_configs::{set_channel_config_store, ChannelConfig},
        MandelaMsgMeta, MandelaMsgType,
    };
    use tokio;

    #[tokio::test]
    async fn build_channel_test() {
        let ch_cfg1 = ChannelConfig {
            label: String::from("General"),
            recur: None
        };
        let ch_cfg2 = ChannelConfig {
            label: String::from("Random"),
            recur: None
        };
        let cfgs = vec![ch_cfg1, ch_cfg2];
        set_channel_config_store(cfgs).await;

        // Test case 1: Channel config is present
        let channel_name = "General#123".to_string();
        let msg = MandelaMsg {
            m: MandelaMsgMeta {
                ch: channel_name,
                t: MandelaMsgType::Data,
            },
            d: None,
        };

        let channel = build_channel(msg).unwrap();
        assert_eq!(channel.ch, "General#123");

        // Test case 2: Channel config NOT present
        let channel_name2 = "Missing#456".to_string();
        let msg2 = MandelaMsg {
            m: MandelaMsgMeta {
                ch: channel_name2,
                t: MandelaMsgType::Data,
            },
            d: None,
        };

        let channel = build_channel(msg2);
        assert_eq!(channel, None);

    }
}
