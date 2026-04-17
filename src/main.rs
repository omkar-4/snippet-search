use iced::{Element, Size, Task};
mod ui;

use ui::{
    file_card::FileCard,
    search_bar::{SearchBar, SearchBarMessage},
    toolbar::{Toolbar, ToolbarMessage},
};

use crate::ui::{file_card::FileCardMessage, placeholder};

#[derive(Default)]
struct App {
    search: SearchBar,
    toolbar: Toolbar,
    card: FileCard,
}

#[derive(Debug, Clone)]
enum Message {
    SearchBar(SearchBarMessage),
    Toolbar(ToolbarMessage),
    FileCard(FileCardMessage),
}

fn update(app: &mut App, msg: Message) -> Task<Message> {
    match msg {
        Message::SearchBar(SearchBarMessage::InputChanged(v)) => {
            app.search.value = v;
        }
        Message::SearchBar(SearchBarMessage::SearchPressed) => {}
        Message::Toolbar(_) => {}
        Message::FileCard(_) => {}
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    iced::widget::column![
        app.search.view().map(Message::SearchBar),
        app.toolbar.view().map(Message::Toolbar),
        app.card.view().map(Message::FileCard)
    ]
    .into()
}

fn boot() -> (App, Task<Message>) {
    (
        App {
            card: placeholder::mock_card(),
            ..Default::default()
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    // iced::run(update, view)
    iced::application(boot, update, view)
        .window_size(Size::new(400.0, 500.0))
        .title(|_: &App| String::from("App Title"))
        .run()
}
