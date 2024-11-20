mod config_update;
mod config_view;
mod gear;
pub mod style;

use crate::build_config::Config;
pub use gear::{Gear, GearList, GearSelections, GearType};

#[derive(Default)]
pub struct ConfigFile {
    pub error_message: Option<String>,
    pub gear: GearSelections,
    pub config: Config,
}

impl ConfigFile {
    pub fn save_config(&mut self) {
        self.config
            .save_config("config/config.toml")
            .unwrap_or_default();
    }
}
