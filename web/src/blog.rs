#[derive(Debug)]
#[derive(Clone)]
pub struct Blog {
    id: String,
    title: String,
    content: String,
    date: String,
    author: String
}

use chrono;

impl Blog {
    pub fn new(title: String, content: String, author: String) -> Blog {
        Blog {
            id: "1",
            title,
            content,
            date: chrono::offset::Local::now(),
            author
        }
    }
}
