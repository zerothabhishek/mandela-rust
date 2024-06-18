use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::Number;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::sync::Mutex;

#[derive(Deserialize, Debug, Clone)]
pub struct RecurType {
    pub function: String,
    pub every: Number, // in seconds
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChannelConfig {
    pub label: String,
    pub recur: Option<RecurType>, // Set the default value of recur as None
}

lazy_static! {
  // TODO: rename as CHANNEL_CONFIGS_STORE
  static ref CHANNEL_STORE: Mutex<Vec<ChannelConfig>> = Mutex::new(vec![]);
}

pub async fn channel_config_fake() -> ChannelConfig {
    ChannelConfig {
        label: "test".to_string(),
        recur: None, // TODO: find a way to skip this initialization
    }
}

// call this from main
pub async fn init_channel_configs(channel_config_jsons: Vec<String>) {
    let channel_configs_r = CHANNEL_STORE.lock();
    let mut channel_configs = channel_configs_r.unwrap();

    // let cfgs = read_channel_configs(file_paths);
    let cfgs = channel_config_jsons
        .iter()
        .map(|json| serde_json::from_str(&json).unwrap())
        .collect::<Vec<ChannelConfig>>();

    // TODO: use set_channel_config_store
    *channel_configs = cfgs;
}

pub async fn set_channel_config_store(cfgs: Vec<ChannelConfig>) {
    let channel_configs_r = CHANNEL_STORE.lock();
    let mut channel_configs = channel_configs_r.unwrap();

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
    channel_configs
        .iter()
        .find(|cfg| cfg.label == label)
        .cloned()
}

pub async fn read_channel_config_superfile(path: String) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json_arr = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .map(|file_path| {
            println!("Reading ch-cfg file: {}", file_path);
            let mut file = File::open(file_path).unwrap();
            let mut json_content = String::new();
            file.read_to_string(&mut json_content).unwrap();

            json_content
        })
        .collect::<Vec<String>>();
    json_arr
}
