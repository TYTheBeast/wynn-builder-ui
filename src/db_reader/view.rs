use iced::widget::responsive;
use iced::Length;
use iced_table::table;
use iced_widget::{checkbox, container, pick_list};
use iced_widget::{column, Container};

use crate::{DBReaderMessage, Message, Themes};

use super::DBReader;

impl DBReader {
    pub fn view(&self) -> Container<Message> {
        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                |offset| Message::DBReader(DBReaderMessage::Sync(offset)),
            );

            if self.resize_column_enabled {
                table = table.on_column_resize(
                    |index, offset| Message::DBReader(DBReaderMessage::Resizing(index, offset)),
                    Message::DBReader(DBReaderMessage::Resized),
                );
            }

            if self.footer_enabled {
                table = table.footer(self.footer.clone());
            }

            if self.min_width_enabled {
                table = table.min_width(size.width);
            }

            table.into()
        });

        let content = column![
            checkbox("Resize Columns", self.resize_column_enabled)
                .on_toggle(|_| Message::DBReader(DBReaderMessage::ResizeColumnsEnabled(true))),
            checkbox("Footer", self.footer_enabled)
                .on_toggle(|_| Message::DBReader(DBReaderMessage::FooterEnabled(true))),
            checkbox("Min Width", self.min_width_enabled)
                .on_toggle(|_| Message::DBReader(DBReaderMessage::MinWidthEnabled(true))),
            pick_list(Themes::to_vec(), Some(self.theme.clone()), |theme| {
                Message::DBReader(DBReaderMessage::Theme(theme))
            },),
            table
        ]
        .spacing(6);

        container(content)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
