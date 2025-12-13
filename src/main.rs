use env_logger::Env;
use log::{error, info};
use logwatcher::{LogWatcher, LogWatcherAction};
use rzap_ng::{api_builder::OpenShockAPIBuilder, data_type::ControlType};
use serde::Deserialize;
use std::{
    env::{consts, home_dir},
    fs::File,
    io::Read,
    sync::Arc,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    own_name: String,
    shocker_id: String,
    openshock_token: String,
    shock_intensity: u8,
    shock_duration: u16,
}

#[tokio::main]
async fn main() {
    let env = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();

    let config_path = std::path::Path::new("woaw.toml");
    let mut config_file = File::open(config_path).expect("the config has to exist silly");
    let mut config_file_contents = String::new();

    config_file
        .read_to_string(&mut config_file_contents)
        .expect("the config file contains non UTF-8 characters, what the fuck did you do");

    let config_holder: Config = toml::from_str(&config_file_contents)
        .expect("the config file was not formatted properly and could not be read :c");

    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");

    let openshock_api = Arc::new(
        OpenShockAPIBuilder::new()
            .with_app(app_name.to_string(), Some(app_version.to_string()))
            .with_default_api_token(config_holder.openshock_token.clone())
            .build()
            .unwrap(),
    );

    let log_path = {
        match consts::OS {
            "linux" => {
                home_dir().unwrap_or_default().to_str().unwrap().to_owned()
                    + "/.local/share/godot/app_userdata/Buckshot Roulette/logs/godot.log"
            }
            "windows" => {
                home_dir().unwrap_or_default().to_str().unwrap().to_owned()
                    + "/AppData/Roaming/Godot/app_userdata/Buckshot Roulette/logs/godot.log"
            }
            _ => panic!("what the fuck else would you be using"),
        }
    };

    let death_indication = "death request on instance: ".to_owned() + &config_holder.own_name;

    let mut log_watcher = LogWatcher::register(&log_path).unwrap();

    info!("starting to monitor logs at {log_path}");
    log_watcher.watch(&mut move |line: String| {
        if line.contains(&death_indication) {
            info!("get shocked lmao");
            let openshock_api = Arc::clone(&openshock_api);
            let config_holder = config_holder.clone();

            // ugh
            tokio::spawn(async move {
                if let Err(e) = openshock_api
                    .post_control(
                        config_holder.shocker_id,
                        ControlType::Shock,
                        config_holder.shock_intensity,
                        config_holder.shock_duration,
                        Some(config_holder.openshock_token.clone()),
                    )
                    .await
                {
                    error!("Failed to trigger shock: {:?}", e);
                }
            });
        }

        LogWatcherAction::None
    });
}
