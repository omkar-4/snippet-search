use iced::{
    Element, Length,
    widget::{Space, button, column, container, row, text},
};

#[derive(Debug, Clone)]
pub enum FileCardMessage {
    DeletePressed,
}

#[derive(Default)]
pub struct FileCard {
    pub name: String,
    pub content: String,
}

impl FileCard {
    pub fn view(&self) -> Element<'_, FileCardMessage> {
        let preview = container(text(&self.content).size(13))
            .height(Length::Fixed(80.0))
            .padding(6);

        let filename = truncate(&self.name, 12);

        let footer = row![
            text(filename),
            Space::new().width(Length::Fill),
            button(text("🗑")).on_press(FileCardMessage::DeletePressed)
        ]
        .padding(6);

        container(column![preview, footer]).into()
    }
}

fn truncate(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((idx, _)) => format!("{}...", &s[..idx]),
        None => s.to_string(),
    }
}
