use iced::{
    Element, Length, Theme,
    widget::{button, container, row, text, text_input},
};

#[derive(Debug, Clone)]
pub enum SearchBarMessage {
    InputChanged(String),
    SearchPressed,
}

#[derive(Default)]
pub struct SearchBar {
    pub value: String,
}

impl SearchBar {
    pub fn view(&self) -> Element<'_, SearchBarMessage> {
        let input = text_input("search...", &self.value)
            .on_input(SearchBarMessage::InputChanged)
            .padding(8)
            .size(14);
        let btn = button(text("?"))
            .on_press(SearchBarMessage::SearchPressed)
            .padding(8);

        container(row![input, btn].spacing(8))
            .padding(6)
            .width(Length::Fill)
            .style(container_style)
            .into()
    }
}

fn container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        border: iced::Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}
