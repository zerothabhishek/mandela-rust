use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Deserialize, Debug, Clone)]
pub struct ChannelConfig {
    pub label: String,
}

lazy_static!{
  // TODO: rename as CHANNEL_CONFIGS_STORE
  static ref CHANNEL_STORE: Mutex<Vec<ChannelConfig>> = Mutex::new(vec![]);
}


pub async fn channel_config_fake() -> ChannelConfig {
    ChannelConfig {
        label: "test".to_string(),
    }
}

// call this from main
pub async fn init_channel_configs(file_paths: Vec<String>) {
  let channel_configs_r = CHANNEL_STORE.lock();
  let mut channel_configs = channel_configs_r.unwrap();
  
  let cfgs = read_channel_configs(file_paths);
  *channel_configs = cfgs;
}

pub fn read_channel_configs(file_paths: Vec<String>) -> Vec<ChannelConfig> {
    // Read files
    return file_paths
        .iter()
        .map(|file_path| {
            println!("Reading file: {}", file_path);
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);

            let cfg: ChannelConfig = serde_json::from_reader(reader).unwrap();
            cfg
        })
        .collect::<Vec<ChannelConfig>>();
}

pub fn find_channel_config(label: String) -> Option<ChannelConfig> {
  let channel_configs = CHANNEL_STORE.lock().unwrap();
  channel_configs.iter().find(|cfg| cfg.label == label).cloned()
}
