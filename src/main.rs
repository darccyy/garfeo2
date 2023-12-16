use ibex::prelude::*;
use ibex::{routes, ssg};

mod posts;
mod routes;

const URL_ROOT: &str = "/garfeo2/";

fn main() {
    let posts = posts::parse_posts().expect("Failed to parse posts");

    let routes = routes! [
        (/)
            => routes::at_index(&posts),
        (/404)
            => routes::at_404(),
        (/[post.get().index()])
            for post in posts
            => routes::at_post(post),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [ibex::meta! {}]
            title { "My Ibex App" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
            @ssg::use_autoreload []
        }
    }
}
