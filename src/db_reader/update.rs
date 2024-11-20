use std::path::Path;

use iced::Task;
use iced_widget::scrollable;

use crate::{DBReaderMessage, Message, ThemeConfig};

use super::DBReader;

impl DBReader {
    pub fn update(&mut self, message: DBReaderMessage) -> Task<Message> {
        match message {
            DBReaderMessage::Sync(offset) => {
                return Task::batch(vec![
                    scrollable::scroll_to(self.header.clone(), offset),
                    scrollable::scroll_to(self.footer.clone(), offset),
                ])
            }
            DBReaderMessage::Resizing(index, offset) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            DBReaderMessage::Resized => self.columns.iter_mut().for_each(|column| {
                if let Some(offset) = column.resize_offset.take() {
                    column.width += offset;
                }
            }),
            DBReaderMessage::ResizeColumnsEnabled(enabled) => self.resize_column_enabled = enabled,
            DBReaderMessage::FooterEnabled(enabled) => self.footer_enabled = enabled,
            DBReaderMessage::MinWidthEnabled(enabled) => self.min_width_enabled = enabled,
            DBReaderMessage::Theme(theme) => {
                self.theme = theme.clone();

                // Save the theme to settings folder
                let theme_config = ThemeConfig {
                    theme: theme.clone(),
                };
                let settings_dir = Path::new("settings");
                let theme_path = settings_dir.join("theme.toml");
                let theme_toml = toml::to_string(&theme_config).unwrap();
                let _ = std::fs::write(theme_path, theme_toml);
                return Task::none();
            }
            DBReaderMessage::Category(index, category) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.category = category;
                }
            }
            DBReaderMessage::Enabled(index, is_enabled) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.is_enabled = is_enabled;
                }
            }
            DBReaderMessage::Notes(index, notes) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.data.cell = notes;
                }
            }
            DBReaderMessage::Delete(index) => {
                self.rows.remove(index);
            }
        }

        Task::none()
    }
}
