use std::cell;

use ratatui::{style::Stylize, text::Text};

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    AddDialog,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub enum DownloadStatus {
    InProgress,
    Done,
}
pub struct Link {
    pub url: String,
    pub status: DownloadStatus,
}

impl Link {
    pub fn new(link: String) -> Self {
        Self {
            url: link,
            status: DownloadStatus::InProgress,
        }
    }

    pub fn to_row(&self, index: usize) -> Vec<Text> {
        let cell_1 = format!("{} - ", index + 1).green().bold();
        let cell_2 = self.url.clone().green();
        return vec![cell_1.into(), cell_2.into()];
    }
}

pub struct App {
    pub links: Vec<Link>,
    pub current_screen: CurrentScreen,
    pub input_mode: InputMode,
    pub input: String,
    pub character_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            links: vec![],
            current_screen: CurrentScreen::Main,
            input_mode: InputMode::Normal,
            input: String::new(),
            character_index: 0,
        }
    }

    pub fn paste(&mut self, s: &str) {
        self.input.push_str(s)
    }
    pub fn enter_char(&mut self, new_char: char) {
        self.input.push(new_char);
    }

    pub fn delete_char(&mut self) {
        self.input.pop();
    }

    pub(crate) fn add_to_download_list(&mut self) {
        self.links.push(Link::new(self.input.clone()));
        self.reset()
    }

    pub(crate) fn reset(&mut self) {
        self.input = String::new();
        self.current_screen = CurrentScreen::Main;
        self.input_mode = InputMode::Normal;
    }
}
