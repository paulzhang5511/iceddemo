use std::borrow::Cow;

use iced::{
    font::{load, Family, Stretch, Style, Weight},
    widget::{column, text, Column},
    Element, Font, Settings,
};

#[derive(Default)]
struct Hello;

#[derive(Debug, Clone)]
struct Messsage {}

impl Hello {
    fn update(&mut self, message: Messsage) {}

    fn view(&self) -> Element<'_, Messsage> {
        column![text!("测试").shaping(text::Shaping::Advanced),]
            .padding(30.0)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

fn main() -> iced::Result {
    iced::application("a cool counter", Hello::update, Hello::view)
        .theme(Hello::theme)
        .default_font(Font::MONOSPACE)
        .font(include_bytes!("../assets/fonts/MiSans-Regular.ttf"))
        .run()
}
