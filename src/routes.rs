use ibex::prelude::*;

// use crate::posts::ListEnds;

use super::posts::{PostList, PostRef};
use super::views::use_base;
use super::URL_ROOT;

pub fn at_index(posts: &PostList) -> Document {
    // let ListEnds { first, last } = posts.list_ends();

    view! { @use_base [
            "",
            view! {},
            None,
            &posts,
        ] {
            center {
                [:for post in posts {
                    [:where let post = post.get() {
                        a [href=url!(post.index())] {
                            [post.index()]
                        }
                        br/
                    }]
                }]
            }
        }
    }
    .into()
}

pub fn at_404(posts: &PostList) -> Document {
    view! { @use_base [
            "404",
            view! { "Paĝo ne trovita!" },
            None,
            &posts,
        ] {
            center {
                "404 - Not found"
            }
        }
    }
    .into()
}

pub fn at_post(post: PostRef) -> Document {
    let posts = post.list();
    let post = post.get();

    let json = serde_json::to_string_pretty(&post).unwrap();
    view! { @use_base [
            &format!("{} [{}]", post.title, post.index),
            view! { "[post title]" },
            // view!{ @post.title [&post, false] },
            Some(&format!("static/posts/{}/esperanto.png", post.index)),
            &posts,
        ] {
            [json]
        }
    }
    .into()
}
