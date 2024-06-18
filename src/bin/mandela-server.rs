use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mandela-server", about = "Mandela")]
struct CliArgs {
    #[arg(short, long)]
    channel_configs_superfile: PathBuf,

    #[arg(short, long, default_value_t = String::from("redis://127.0.0.1"))]
    redis_url: String,

    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    server_ip: String,

    #[arg(short, long, default_value_t = String::from("9001"))]
    port: String, // server port
}

#[tokio::main]
async fn main() {
    //
    let args = CliArgs::parse();
    //
    let superfile = args.channel_configs_superfile.to_str().unwrap().to_string();
    let ch_cfg_jsons = mandela_rs::read_channel_config_superfile(superfile).await;
    mandela_rs::mandela_start(ch_cfg_jsons, args.redis_url, args.server_ip, args.port).await;
}
