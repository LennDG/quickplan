// region:    --- Modules

use crate::model::{self, ModelManager};
use tokio::sync::OnceCell;
use tracing::info;

// endregion: --- Modules

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INIT
        .get_or_init(|| async {
            // NOTE: Rare occasion where unwrap is kind of ok.
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}
