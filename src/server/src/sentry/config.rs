use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Deserialize)]
pub struct ArduinoConfig {
    pub device: String,
    pub baud: u32,
    pub pitch_max_speed: u32,
    pub yaw_max_speed: u32,
    pub pitch_homing_speed: u32,
    pub yaw_homing_speed: u32,
}

#[derive(Clone, Deserialize)]
pub struct VideoConfig {
    pub encoder: String,
    pub decoder: String,
    pub host: String,
}

#[derive(Clone, Deserialize)]
pub struct TcpServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: TcpServerConfig,
    pub video: VideoConfig,
    pub camera: HashMap<String, String>,
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
            .map_err(|err| {
                format!(
                    "Could not read configuration file \"{}\": {}",
                    path,
                    err.to_string()
                )
            })?
            .as_str(),
    )
    .map_err(|err| {
        format!(
            "Could not parse configuration file \"{}\": {}",
            path,
            err.to_string()
        )
    })?)
}
