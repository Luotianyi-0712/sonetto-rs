include!("../../config/configs/mod.rs");

pub mod configs {
    pub use crate::{GameDB, get, init, try_get};
}
