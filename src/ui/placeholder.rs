use super::file_card::FileCard;

pub fn mock_card() -> FileCard {
    FileCard {
        name: "untitled.md".into(),
        content: "# Markdown\n## Heading\n- list\n**bold**".into(),
    }
}
