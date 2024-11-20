use std::path::Path;

use iced::{
    alignment::{Horizontal, Vertical},
    Length, Task,
};
use iced_widget::{column, container, text, text_editor::Action, text_input, Container};
use iced_widget::text_editor;

use crate::{Message, SearchMessage};

#[derive(Default)]
pub struct SearchItems {
    pub search_input: String,
    pub search_results: text_editor::Content,
}

impl SearchItems {
    pub fn view(&self) -> Container<'_, Message> {
        let column = column![
                    text("Search for Wynncraft Items")
                        .size(30),
                    text("This tool allows you to search through all Wynncraft items using various parameters.")
                        .size(20),
                    text("By pressing enter, the output will be displayed in the text editor below.")
                        .size(20),
                    text("For a list of available parameters, type '--help' or '-h'.")
                        .size(20),
                    text("For example: '--type boots -s hp -l 10' will show you the 10 highest hp boots.")
                        .size(20),
                    // Search input
                    text_input(
                        "Enter search parameters...",
                        &self.search_input
                    )
                    .on_input(|input| Message::Search(SearchMessage::InputChanged(input)))
                    .on_submit(Message::Search(SearchMessage::InputSubmitted))
                    .padding(10)
                    .width(Length::Fill),
                    text_editor(
                        &self.search_results
                    )
                    .placeholder("Output will appear here...")
                    .on_action(|action| Message::Search(
                        SearchMessage::ItemEditorAction(action)
                    ))
                ]
                .spacing(20)
                .align_x(Horizontal::Center);

        container(column)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: SearchMessage) -> Task<Message> {
        match message {
            SearchMessage::InputChanged(input_text) => {
                self.search_input = input_text;
            }
            SearchMessage::InputSubmitted => {
                let binary_path = if cfg!(windows) {
                    Path::new("search_item.exe")
                } else {
                    Path::new("search_item")
                };

                let args: Vec<&str> = self
                    .search_input
                    .split_whitespace()
                    .collect();

                let output = match std::process::Command::new(binary_path).args(args).output() {
                    Ok(output) => match output.status.success() {
                        true => String::from_utf8_lossy(&output.stdout).to_string(),
                        false => String::from_utf8_lossy(&output.stderr).to_string(),
                    },
                    Err(e) => format!("Error: Could not execute search_item binary: {}", e),
                };

                self.search_results = text_editor::Content::with_text(&output);
            }
            SearchMessage::ItemEditorAction(action) => match action {
                Action::Edit(_) => (),
                Action::Move(_) => (),
                Action::Select(motion) => {
                    self.search_results
                        .perform(Action::Select(motion));
                }
                Action::SelectWord => {
                    self.search_results
                        .perform(Action::SelectWord);
                }
                Action::SelectLine => {
                    self.search_results
                        .perform(Action::SelectLine);
                }
                Action::SelectAll => {
                    self.search_results
                        .perform(Action::SelectAll);
                }
                Action::Click(point) => {
                    self.search_results
                        .perform(Action::Click(point));
                }
                Action::Drag(point) => {
                    self.search_results
                        .perform(Action::Drag(point));
                }
                Action::Scroll { lines } => {
                    self.search_results
                        .perform(Action::Scroll { lines });
                }
            },
        }
        Task::none()
    }
}
