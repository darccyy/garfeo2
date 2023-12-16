use ibex::prelude::*;

use super::posts::{PostList, PostRef};
use super::{use_base, URL_ROOT};

pub fn at_post(post: PostRef) -> Document {
    let json = serde_json::to_string_pretty(&post.get()).unwrap();
    view! { @use_base []
        [json]
    }
    .into()
}

pub fn at_index(posts: &PostList) -> Document {
    view! { @use_base []
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
    .into()
}

pub fn at_404() -> Document {
    view! { @use_base []
        center {
            "404 - Not found"
        }
    }
    .into()
}
