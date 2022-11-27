use serde::{Deserialize, Serialize};
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    bio: String,
    id: u32,
    learning_language_string: String,
    created: String,
    admin: bool,
    email: String,
    invite_url: String,
    fullname: String,
    avatar: String,
    ui_language: String,
    languages: Vec<Language>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    streak: u32,
    language_string: String,
    points: u32,
    learning: bool,
    language: String,
    level: u32,
    current_learning: bool,
    sentences_translated: u32,
    to_next_level: u32,
}
