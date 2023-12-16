mod structs;

use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::Path,
};

pub type PostList = structs::List<Post>;
pub type PostRef<'a> = structs::ItemRef<'a, Post>;

#[derive(Debug, Serialize)]
pub struct Post {
    index: usize,
    pub title: String,
    pub errata: Option<Errata>,
    pub version: u32,
}

#[derive(Debug, Serialize)]
pub struct Errata {
    pub items: Vec<(String, String)>,
}

impl Post {
    pub fn index(&self) -> String {
        format!("{:04}", self.index)
    }
}

pub fn parse_posts() -> Result<PostList> {
    let dir_posts = Path::new("static/posts");
    let dir_old = Path::new("static/old");

    let mut folders: Vec<_> = fs::read_dir(dir_posts)?.flatten().collect();
    folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let old_versions =
        get_old_versions(dir_old).with_context(|| format!("Failed to parse old posts"))?;

    let mut existing_titles = Vec::new();

    let mut posts = Vec::new();
    for (index, folder) in folders.into_iter().enumerate() {
        let version = *old_versions.get(&index).unwrap_or(&0);
        let post = parse_post(index, folder, version, &existing_titles)
            .with_context(|| format!("For post [{}]", index))?;
        existing_titles.push(post.title.clone());
        posts.push(post);
    }

    posts.reverse();
    Ok(PostList::new(posts))
}

fn get_old_versions(dir: &Path) -> Result<HashMap<usize, u32>> {
    let mut old_folders: Vec<_> = fs::read_dir(dir)?.flatten().collect();
    old_folders.sort_by_key(|folder| {
        let path = folder.path();
        path.to_string_lossy().to_string()
    });

    let mut old_versions = HashMap::new();
    for folder in old_folders {
        let name = folder.file_name();
        let name = name.to_string_lossy().to_string();
        let mut split = name.split(':');

        let index: Option<usize> = split.next().and_then(|string| string.parse().ok());
        let version: Option<u32> = split.next().and_then(|string| string.parse().ok());
        let Some((index, version)) = index.zip(version) else {
            bail!("For folder `{}`", folder.path().display());
        };

        let expected_version = match old_versions.get(&index) {
            Some(prev) => prev + 1,
            None => 0,
        };

        if version != expected_version {
            bail!(
                "Revised post version out of order. Expected `:{}`, found `:{}`",
                expected_version,
                version
            );
        }

        old_versions.insert(index, version);
    }

    Ok(old_versions)
}

fn parse_post(
    index: usize,
    folder: DirEntry,
    version: u32,
    existing_titles: &[String],
) -> Result<Post> {
    let folder_name = folder.file_name();
    let folder_name = folder_name.to_string_lossy();

    let expected_name = pad_index(index);
    if folder_name != expected_name {
        bail!(
            "Invalid folder name. Expected `{}`, found `{}`",
            expected_name,
            folder_name
        );
    }

    let path = folder.path().join(Path::new("title"));
    let title = fs::read_to_string(&path)?.trim().to_string();
    if !title.starts_with("Garfildo ") {
        bail!("Title of {index} does not start with 'Garfildo'");
    }
    if existing_titles.contains(&title) {
        bail!("Multiple posts have '{title}' as title");
    }

    let path = folder.path().join(Path::new("errata"));
    let errata = if path.exists() {
        let file = fs::read_to_string(&path)?;
        Some(parse_errata(&file).with_context(|| format!("Failed to parse errata file"))?)
    } else {
        None
    };

    Ok(Post {
        index,
        title,
        errata,
        version,
    })
}

fn pad_index(index: usize) -> String {
    format!("{:04}", index)
}

fn parse_errata(file: &str) -> Result<Errata> {
    if file.trim().is_empty() {
        bail!("Empty errata file".to_string());
    }

    let mut items = Vec::new();
    for line in file.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut split = line.split(">>");

        let Some(bad) = split.next() else {
            bail!("Missing incorrect phrase".to_string());
        };
        let Some(good) = split.next() else {
            bail!(format!("Missing correct phrase for '{}'", bad));
        };

        items.push((bad.trim().to_string(), good.trim().to_string()));
    }

    Ok(Errata { items })
}
