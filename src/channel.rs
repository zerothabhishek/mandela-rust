use crate::channel_configs::find_channel_config;
use crate::{ MandelaMsg, MandelaChannel };


pub fn build_channel(
    msg: MandelaMsg,
) -> Option<MandelaChannel> {
    let msg1 = msg.clone();
    let ch = msg1.m.ch;
    let id = msg1.m.id;

    // let exists = channel_configs.iter().any(|ch_cfg| ch_cfg.label == ch);
    let ch_cfg = find_channel_config(ch.clone());
    if ch_cfg.is_none() {
        return None;
    }
    Some(MandelaChannel { ch, id })
}
