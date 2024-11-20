use iced::{Element, Length, Renderer, Theme};
use iced_table::table;
use iced_widget::{
    button, checkbox, container, horizontal_space, pick_list, scrollable, text, text_input,
};

use crate::{Category, ColumnKind, DBReaderMessage, Message};

pub struct DBReader {
    pub db_connection: sqlite::Connection,
    pub columns: Vec<DBColumn>,
    pub rows: Vec<DBRow>,
    pub header: scrollable::Id,
    pub body: scrollable::Id,
    pub footer: scrollable::Id,
    pub resize_column_enabled: bool,
    pub footer_enabled: bool,
    pub min_width_enabled: bool,
    pub theme: Theme,
    pub query: String,
}

impl Default for DBReader {
    fn default() -> Self {
        Self {
            db_connection: sqlite::open(":memory:").unwrap(),
            columns: vec![],
            rows: vec![],
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
            resize_column_enabled: true,
            footer_enabled: true,
            min_width_enabled: true,
            theme: Theme::Dark,
            query: String::new(),
        }
    }
}

impl DBReader {
    pub fn init(theme: Theme) -> Self {
        Self {
            theme,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DBTable {
    pub columns: Vec<DBColumn>,
    pub rows: Vec<DBRow>,
}

#[derive(Default)]
pub struct DBColumn {
    pub kind: ColumnKind,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl DBColumn {
    pub fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Index => 50.0,
            ColumnKind::Category => 100.0,
            ColumnKind::Enabled => 100.0,
            ColumnKind::Notes => 100.0,
            ColumnKind::Delete => 50.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

#[derive(Default)]
pub struct DBRow {
    pub data: DBCell,
    pub category: Category,
    pub is_enabled: bool,
}

impl DBRow {
    fn generate(index: usize) -> Self {
        let category = match index % 5 {
            0 => Category::A,
            1 => Category::B,
            2 => Category::C,
            3 => Category::D,
            4 => Category::E,
            _ => unreachable!(),
        };
        let is_enabled = index % 2 == 0;

        Self {
            data: DBCell::default(),
            category,
            is_enabled,
        }
    }
}

#[derive(Default)]
pub struct DBCell {
    pub cell: String,
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for DBColumn {
    type Row = DBRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Index => "Index",
            ColumnKind::Category => "Category",
            ColumnKind::Enabled => "Enabled",
            ColumnKind::Notes => "Notes",
            ColumnKind::Delete => "",
        };

        container(text(content)).center_y(Length::Fill).into()
    }

    fn cell(&'a self, _col_index: usize, row_index: usize, row: &'a DBRow) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Index => text(row_index).into(),
            ColumnKind::Category => pick_list(Category::ALL, Some(row.category), move |category| {
                Message::DBReader(DBReaderMessage::Category(row_index, category))
            })
            .into(),
            ColumnKind::Enabled => checkbox("", row.is_enabled)
                .on_toggle(move |enabled| {
                    Message::DBReader(DBReaderMessage::Enabled(row_index, enabled))
                })
                .into(),
            ColumnKind::Notes => text_input("", &row.data.cell)
                .on_input(move |notes| Message::DBReader(DBReaderMessage::Notes(row_index, notes)))
                .width(Length::Fill)
                .into(),
            ColumnKind::Delete => button(text("Delete"))
                .on_press(Message::DBReader(DBReaderMessage::Delete(row_index)))
                .into(),
        };

        container(content)
            .width(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn footer(&'a self, _col_index: usize, rows: &'a [DBRow]) -> Option<Element<'a, Message>> {
        let content = if matches!(self.kind, ColumnKind::Enabled) {
            let total_enabled = rows.iter().filter(|row| row.is_enabled).count();

            Element::from(text(format!("Total Enabled: {total_enabled}")))
        } else {
            horizontal_space().into()
        };

        Some(container(content).center_y(24).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
