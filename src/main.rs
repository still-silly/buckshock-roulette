use env_logger::Env;
use log::{error, info};
use rzap_ng::{api_builder::OpenShockAPIBuilder, data_type::ControlType};
use serde::Deserialize;
use std::{
    env::{consts, home_dir},
    fs::File,
    io::{Error, Read},
    process::Stdio,
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    own_name: String,
    shocker_id: String,
    openshock_token: String,
    shock_intensity: u8,
    shock_duration: u16,
    game_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    let game_path = config_holder.game_path.clone().unwrap_or({
        match consts::OS {
            "linux" => {
                home_dir().unwrap_or_default().to_str().unwrap().to_owned()
                    + "/.local/share/Steam/steamapps/common/Buckshot Roulette/Buckshot Roulette_linux/Buckshot Roulette.x86_64"
            }
            "windows" => {
                "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Buckshot Roulette\\Buckshot Roulette.exe"
                    .to_owned()
            }
            _ => panic!("what the fuck else would you be using"),
        }
    });

    let death_indication = format!("death request on instance: {}", config_holder.own_name);

    let mut child = Command::new(game_path)
        .env("SteamAppId", "2835570")
        .env("STEAM_APP_ID", "2835570")
        .env("STEAM_RUNTIME", "1")
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| std::io::Error::other("couldn't get stdout"))?;

    let mut reader = BufReader::new(stdout).lines();

    while let Some(line) = reader.next_line().await? {
        if line.contains(&death_indication) {
            info!("get shocked lmao");
            let openshock_api = Arc::clone(&openshock_api);
            let config_holder = config_holder.clone();

            match openshock_api
                .post_control(
                    config_holder.shocker_id,
                    ControlType::Shock,
                    config_holder.shock_intensity,
                    config_holder.shock_duration,
                    Some(config_holder.openshock_token.clone()),
                )
                .await
            {
                Ok(_) => {}
                Err(e) => error!("couldn't shock :kms: error: {e}"),
            }
        }
    }

    Ok(())
}
