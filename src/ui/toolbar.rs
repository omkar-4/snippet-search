use ::iced::{
    Element, Length,
    widget::{Space, button, container, row, text},
};

#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    CreatePressed,
}

#[derive(Default)]
pub struct Toolbar;

impl Toolbar {
    pub fn view(&self) -> Element<'_, ToolbarMessage> {
        container(row![
            Space::new().width(Length::Fill),
            button(text("+"))
                .on_press(ToolbarMessage::CreatePressed)
                .padding(8)
        ])
        .width(Length::Fill)
        .padding(6)
        .into()
    }
}
