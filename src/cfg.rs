extern crate config;

use config::File;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub redis_addr: String,
    pub java_backend_url: String,
    pub user_session_redis_key_prefix: String,
    pub markdown_dir: String,
    pub gstore_username: String,
    pub gstore_password: String,
    pub gstore_host: String,
    pub gstore_holder10_sp: String,
    pub gstore_holder10_pp: String,
    pub gstore_holder10_op: String,
    pub gstore_holder_manager_op: String,
    pub gstore_holder_manager_m_sp: String,
    pub gstore_holder_manager_h_sp: String,
    pub qichacha_api_host: String,
    pub qichacha_api_key: String,
    pub qichacha_api_secret_key: String,
    pub qichacha_api_cache_prefix: String,
    pub elasticsearch_host: String,
}

pub fn get_settings() -> Config {
    let runtime = env::var("ROCKET_ENV").unwrap_or("development".into());
    let mut settings = config::Config::default();
    settings
        // Add in `./Config.toml`
        .merge(config::File::with_name("config/cfg"))
        .unwrap()
        .merge(File::with_name(&format!("config/{}", runtime)).required(false))
        .unwrap();

    // Print out our settings (as a HashMap)
    settings.try_into::<Config>().unwrap()
}

lazy_static! {
    pub static ref CONFIG: Config = get_settings();
}
