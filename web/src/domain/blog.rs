#[derive(Debug)]
#[derive(Clone)]
pub struct Blog {
    id: String,
    pub title: String,
    pub content: String,
    pub date: String,
    author: String,
}

use chrono;

impl Blog {
    pub fn new(title: String, content: String, author: String) -> Blog {
        Blog {
            id: String::from("1"),
            title,
            content,
            date: chrono::offset::Local::now().to_string(),
            author,
        }
    }

    pub fn update_title(&mut self, title: String) -> &mut Blog {
        self.title = title;

        self
    }
}
