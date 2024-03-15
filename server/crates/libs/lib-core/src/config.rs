use lib_utils::envs::{get_env, get_env_parse};
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_URL: String,
    pub DB_TEST_URL: String,
    pub DB_MAX_CONN: u32,
    pub DB_TIMEOUT_MS: f64,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        Ok(CoreConfig {
            // -- Db
            DB_URL: get_env("SERVICE_DB_URL")?,
            DB_TEST_URL: get_env("SERVICE_TEST_DB_URL")?,
            DB_MAX_CONN: get_env_parse("SERVICE_DB_MAX_CONNECTIONS")?,
            DB_TIMEOUT_MS: get_env_parse("SERVICE_DB_TIMEOUT_MS")?,
        })
    }
}
