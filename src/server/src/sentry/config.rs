use std::fs;
use std::env;
use std::prelude::*;

#[derive(Clone, Deserialize)]
pub struct ArduinoConfig {
    pub device: String,
    pub baud: u32,
    pub max_pitch_speed: u32,
    pub max_yaw_speed: u32,
}

#[derive(Clone, Deserialize)]
pub struct VideoConfig {
    pub device: String,
    pub bitrate: u32,
    pub height: u16,
    pub stun_server: String,
}

#[derive(Clone, Deserialize)]
pub struct WsServerConfig {
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct HttpServerConfig {
    pub port: u16,
    pub directory: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub websocket: WsServerConfig,
    pub http_server: HttpServerConfig,
    pub video: VideoConfig,
    pub arduino: ArduinoConfig,
}

pub fn load() -> Result<Config, String> {
    let mut path = env::current_exe().map_err(|err| err.to_string())?;
    path.pop();
    path.push("config.toml");
    let path = path.to_str().unwrap();
    info!("Reading configuration at \"{}\"...", path);

    Ok(toml::from_str::<Config>(
        fs::read_to_string(path)
            .map_err(|err|
                format!("Could not read configuration file \"{}\": {}", path, err.to_string())
            )?
            .as_str()
        )
        .map_err(|err|
            format!("Could not parse configuration file \"{}\": {}", path, err.to_string())
        )?
    )
}