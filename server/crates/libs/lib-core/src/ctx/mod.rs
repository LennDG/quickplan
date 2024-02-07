// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

/// In the quickplan app there are actually no users necessary,
/// Going to keep Ctx because you can put anything you want in here
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

// Constructors.
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
