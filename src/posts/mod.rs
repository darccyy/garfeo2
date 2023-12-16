mod parse;
mod structs;
mod transcript;

use serde::Serialize;

use self::transcript::Transcript;
pub use parse::parse_posts;

pub type PostList = structs::List<Post>;
pub type PostRef<'a> = structs::ItemRef<'a, Post>;

pub use structs::{ListEnds, Neighbors};

#[derive(Debug, Serialize)]
pub struct Post {
    pub index: Index,
    pub title: String,
    pub date: String,
    pub is_sunday: bool,
    pub transcript: Option<Transcript>,
    pub errata: Option<Errata>,
    pub version: u32,
    pub is_old: bool,
    pub image_bytes: u64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Index(usize);

#[derive(Debug, Serialize)]
pub struct Errata {
    pub items: Vec<(String, String)>,
}

impl Post {
    pub fn index(&self) -> String {
        self.index.to_string()
    }
}

impl Index {
    pub fn as_int(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}
