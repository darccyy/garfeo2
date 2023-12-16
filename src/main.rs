use ibex::{routes, ssg};

mod posts;
mod routes;
mod views;

const URL_ROOT: &str = "/garfeo2/";

fn main() {
    let posts = posts::parse_posts().expect("Failed to parse posts");

    let routes = routes! [
        (/)
            => routes::at_index(&posts),
        (/404)
            => routes::at_404(&posts),
        (/[post.get().index()])
            for post in posts
            => routes::at_post(post),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}
