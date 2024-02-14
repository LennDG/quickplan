// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

/// In the quickplan app there are actually no users necessary,
/// Going to keep Ctx because you can put anything you want in here
#[derive(Clone, Debug)]
pub struct Ctx {
    url_id: Option<String>,
}

// Constructors.
impl Ctx {
    pub fn root_ctx() -> Self {
        Self { url_id: None }
    }

    pub fn new(url_id: &str) -> Result<Self> {
        Ok(Self {
            url_id: Some(url_id.to_string()),
        })
    }
}

// Property Accessors.
impl Ctx {
    pub fn url_id(&self) -> Option<String> {
        self.url_id.clone()
    }
}
