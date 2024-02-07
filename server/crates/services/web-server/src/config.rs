use std::sync::OnceLock;
use utils::envs::get_env;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
}

impl WebConfig {
    fn load_from_env() -> utils::envs::Result<WebConfig> {
        Ok(WebConfig {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}
