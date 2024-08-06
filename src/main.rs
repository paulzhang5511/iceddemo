use std::borrow::Cow;

use iced::{
    font::{load, Family, Stretch, Style, Weight},
    widget::Column,
    Element, Font, Settings,
};

#[derive(Default)]
struct Hello;

#[derive(Debug, Clone)]
struct Messsage {}

impl Hello {
    fn update(&mut self, message: Messsage) {}

    fn view(&self) -> Element<'_, Messsage> {
        "测试".into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

fn main() -> iced::Result {
    // 在本机平台上，此方法将控制当前线程 并且不会回来。
    // 它可能应该是您在函数中调用的最后一件事。
    iced::application("a cool counter", Hello::update, Hello::view)
        .theme(Hello::theme)
        .default_font(Font::with_name("MiSans"))
        .font(include_bytes!("../assets/fonts/MiSans-Regular.ttf"))
        .run()
}
