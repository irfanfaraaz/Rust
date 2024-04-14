use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post {
            title: title,
            body: body,
        }
    }
}
